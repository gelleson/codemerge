use crate::error::{Error, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct GitignoreManager {
    root: PathBuf,
    ignore_patterns: HashMap<PathBuf, GlobSet>,
}

impl GitignoreManager {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        let mut manager = Self {
            root: root.clone(),
            ignore_patterns: HashMap::new(),
        };

        manager.load_gitignore_files()?;
        manager.load_predefined_patterns()?;
        Ok(manager)
    }

    fn load_gitignore_files(&mut self) -> Result<()> {
        for entry in WalkDir::new(&self.root) {
            let entry = entry.map_err(|e| Error::Path(e.to_string()))?;
            if entry.file_type().is_file() && entry.file_name() == ".gitignore" {
                let abs_dir = entry.path().parent().unwrap().to_path_buf();
                let rel_dir = abs_dir
                    .strip_prefix(&self.root)
                    .unwrap_or(&abs_dir)
                    .to_path_buf();
                let patterns = self.parse_gitignore(entry.path())?;
                self.ignore_patterns.insert(rel_dir, patterns);
            }
        }
        Ok(())
    }

    fn load_predefined_patterns(&mut self) -> Result<()> {
        let predefined_patterns = [
            "**/.git/**",
            "**/node_modules/**",
            "**/target/**",
            "**/.idea/**",
            "**/.DS_Store",
            "**/target/debug/**",
            "**/target/release/**",
            "**/*.o",
            "**/*.rmeta",
            "**/*.rlib",
            "**/*.dll",
            "**/*.dylib",
            "**/*.so",
            "**/*.exe",
            "**/.next/**",
            "**/.vercel/**",
            "**/*.svg",
            "**/.yarn/**",
            "**/*.lock",
            "**/.jest/**",
            "**/.cache/**",
            "**/.pnpm-lock.yaml",
            "**/.yarn-lock.yaml",
            "**/.*/**",
            "**/coverage/**",
            "**/go.sum",
        ];

        let mut builder = GlobSetBuilder::new();
        for pattern in predefined_patterns {
            builder.add(Glob::new(pattern).map_err(|e| Error::Filter(e.to_string()))?);
        }

        let glob_set = builder.build().map_err(|e| Error::Filter(e.to_string()))?;
        self.ignore_patterns.insert(PathBuf::from(""), glob_set);
        Ok(())
    }

    fn normalize_path_for_matching(&self, path: &Path) -> String {
        let path_str = path
            .to_string_lossy()
            .replace('\\', "/")
            .trim_start_matches("./")
            .to_string();

        path_str.trim_start_matches('/').to_string()
    }

    fn normalize_pattern(&self, pattern: &str) -> String {
        let pattern = pattern.replace('\\', "/");
        let mut components = Vec::new();
        for component in pattern.split('/') {
            match component {
                "." => continue,
                ".." => {
                    if !components.is_empty() {
                        components.pop();
                    }
                }
                "" => continue,
                _ => components.push(component),
            }
        }

        let mut result = components.join("/");
        if pattern.ends_with('/') {
            result.push('/');
        }

        if pattern.starts_with('/') {
            result = format!("/{}", result);
        }

        result
    }

    fn parse_gitignore(&self, path: &Path) -> Result<GlobSet> {
        let content = std::fs::read_to_string(path)?;
        let mut builder = GlobSetBuilder::new();

        let gitignore_dir = path
            .parent()
            .and_then(|p| p.strip_prefix(&self.root).ok())
            .unwrap_or_else(|| Path::new(""));

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut pattern = line.to_string();
            let is_negation = pattern.starts_with('!');
            if is_negation {
                pattern = pattern[1..].to_string();
            }

            pattern = self.normalize_pattern(&pattern);

            let patterns_to_add = if pattern.starts_with('/') {
                vec![pattern.trim_start_matches('/').to_string()]
            } else {
                vec![
                    if gitignore_dir.as_os_str().is_empty() {
                        pattern.clone()
                    } else {
                        self.normalize_pattern(&format!("{}/{}", gitignore_dir.display(), pattern))
                    },
                    format!("**/{}", pattern),
                ]
            };

            for mut pat in patterns_to_add {
                if pat.ends_with('/') {
                    pat.push_str("**");
                }

                let glob = if is_negation {
                    Glob::new(&format!("!{}", pat))
                } else {
                    Glob::new(&pat)
                }
                .map_err(|e| Error::Filter(e.to_string()))?;

                builder.add(glob);
            }
        }

        builder.build().map_err(|e| Error::Filter(e.to_string()))
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        let relative_path = if path.is_absolute() {
            path.strip_prefix(&self.root).unwrap_or(path)
        } else {
            path
        };

        let normalized_path = self.normalize_path_for_matching(relative_path);

        // (Changed variable name: dir → _dir)
        for patterns in self.ignore_patterns.values() {
            if patterns.is_match(&normalized_path) {
                return true;
            }
        }

        false
    }

    pub fn walk(&self) -> impl Iterator<Item = Result<PathBuf>> + '_ {
        let root = self.root.clone();
        WalkDir::new(&root)
            .into_iter()
            .filter_map(move |entry| match entry {
                Ok(entry) => {
                    let path = entry.path().to_path_buf();
                    if !entry.file_type().is_file() {
                        return None;
                    }
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.eq_ignore_ascii_case(".gitignore") {
                            return None;
                        }
                    }
                    if !self.is_ignored(&path) {
                        Some(Ok(path))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(Error::Path(e.to_string()))),
            })
    }

    pub fn walk_iter(&self) -> impl Iterator<Item = Result<PathBuf>> + '_ {
        self.walk()
    }
}
