import { join, dirname } from "path";
import { temporaryDirectory } from "tempy";
import { mkdir, rmdir, writeFile } from "node:fs/promises";

export interface TreeNode {
    name: string;
    content?: string;
    children?: TreeNode[];
}

export class FileTree {
    private readonly root: string;

    constructor(root?: string) {
        this.root = root ?? temporaryDirectory();
    }

    async build(structure: TreeNode): Promise<void> {
        await this.createNode(structure, this.root);
    }

    private async createNode(node: TreeNode, parentPath: string): Promise<void> {
        // Validate: a node cannot be both a file and a directory
        if (node.content !== undefined && node.children?.length) {
            throw new Error(
                `Node "${node.name}" cannot have both content and children. ` +
                `A node must be either a file (with content) or a directory (with children).`
            );
        }

        const path = join(parentPath, node.name);

        if (node.content !== undefined) {
            // File case
            await mkdir(dirname(path), { recursive: true });
            await writeFile(path, node.content);
        } else {
            // Directory case
            await mkdir(path, { recursive: true });

            // Process children if any
            if (node.children?.length) {
                await Promise.all(
                    node.children.map(child => this.createNode(child, path))
                );
            }
        }
    }

    async cleanup(): Promise<void> {
        await rmdir(this.root, { recursive: true });
    }

    getPath(): string {
        return this.root;
    }

    resolvePath(name: string): string {
        return join(this.root, name);
    }
}