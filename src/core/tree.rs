//! Directory tree construction and rendering logic.
//!
//! Provides structures and functions to construct an in-memory hierarchical
//! directory tree from a flat list of files and visually format it as an ASCII tree.

use super::file::FileData;
use serde::Serialize;
use std::collections::HashMap;

/// A node in the directory tree structure.
#[derive(Debug, Serialize, Clone)]
pub struct TreeNode {
    /// The name of the file or directory.
    pub path: String,
    /// The sum of tokens in this file or directory (including children).
    pub tokens: usize,
    /// Sub-nodes for a directory, or empty if it's a file.
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    /// Initialize a new tree node.
    pub fn new(path: String) -> Self {
        Self {
            path,
            tokens: 0,
            children: Vec::new(),
        }
    }

    /// Add a child node to this directory, accumulating its token count.
    pub fn add_child(&mut self, child: TreeNode) {
        self.tokens += child.tokens;
        self.children.push(child);
    }
}

/// Construct an in-memory hierarchical tree from a flat list of processed files.
///
/// # Arguments
///
/// * `files` - A slice of `FileData` instances containing the resolved paths and token counts.
///
/// # Returns
///
/// * `TreeNode` - The root node of the constructed tree structure.
pub fn build_tree(files: &[FileData]) -> TreeNode {
    let mut root = TreeNode::new(String::new());
    let mut path_map: HashMap<String, Vec<String>> = HashMap::new();

    // First, organize files by their path components
    for file in files {
        let components: Vec<String> = file.path.split('/').map(String::from).collect();
        path_map.insert(file.path.clone(), components);
    }

    // Build the tree structure
    for file in files {
        let components = path_map.get(&file.path).unwrap();
        let mut current = &mut root;
        let mut current_path = String::new();

        for (i, component) in components.iter().enumerate() {
            if !current_path.is_empty() {
                current_path.push('/');
            }
            current_path.push_str(component);

            // Find or create the child node
            let is_file = i == components.len() - 1;
            let child_pos = current
                .children
                .iter()
                .position(|child| child.path == *component);

            match child_pos {
                Some(pos) => {
                    if is_file {
                        current.children[pos].tokens = file.tokens;
                    }
                    current = &mut current.children[pos];
                }
                None => {
                    let mut new_node = TreeNode::new(component.clone());
                    if is_file {
                        new_node.tokens = file.tokens;
                    }
                    current.add_child(new_node);
                    current = current.children.last_mut().unwrap();
                }
            }
        }
    }

    // Update directory token counts
    update_directory_tokens(&mut root);
    root
}

fn update_directory_tokens(node: &mut TreeNode) -> usize {
    if node.children.is_empty() {
        return node.tokens;
    }

    let mut total = 0;
    for child in &mut node.children {
        total += update_directory_tokens(child);
    }
    node.tokens = total;
    total
}

/// Recursively formats the tree structure into a visually aligned ASCII representation.
///
/// # Arguments
///
/// * `tree` - A reference to the root `TreeNode` to be formatted.
/// * `indent` - The current indentation string (typically empty initially).
/// * `is_last` - A boolean indicating if the current node is the last child of its parent.
///
/// # Returns
///
/// * `String` - A formatted string representing the directory tree, ready for standard output.
pub fn format_tree(tree: &TreeNode, indent: &str, is_last: bool) -> String {
    let mut result = String::new();

    if !tree.path.is_empty() {
        let marker = if is_last { "└── " } else { "├── " };
        result.push_str(&format!(
            "{}{}{} ({} tokens)\n",
            indent, marker, tree.path, tree.tokens
        ));
    }

    let child_indent = if tree.path.is_empty() {
        indent.to_string()
    } else if is_last {
        format!("{}    ", indent)
    } else {
        format!("{}│   ", indent)
    };

    for (i, child) in tree.children.iter().enumerate() {
        let is_last_child = i == tree.children.len() - 1;
        result.push_str(&format_tree(child, &child_indent, is_last_child));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_building() {
        let files = vec![
            FileData::new("src/main.rs", "content1"),
            FileData::new("src/lib.rs", "content2"),
            FileData::new("README.md", "content3"),
        ];

        let tree = build_tree(&files);

        // Root should have two children: "src" and "README.md"
        assert_eq!(tree.children.len(), 2);

        // Find src directory
        let src = tree
            .children
            .iter()
            .find(|node| node.path == "src")
            .expect("src directory not found");

        // src should have two children: main.rs and lib.rs
        assert_eq!(src.children.len(), 2);

        // Verify README.md
        let readme = tree
            .children
            .iter()
            .find(|node| node.path == "README.md")
            .expect("README.md not found");
        assert_eq!(readme.children.len(), 0);
    }

    #[test]
    fn test_tree_formatting() {
        let files = vec![
            FileData::new("src/main.rs", "content1"),
            FileData::new("src/lib.rs", "content2"),
        ];

        let tree = build_tree(&files);
        let formatted = format_tree(&tree, "", true);

        assert!(formatted.contains("src"));
        assert!(formatted.contains("main.rs"));
        assert!(formatted.contains("lib.rs"));
        assert!(formatted.contains("tokens"));
    }
}
