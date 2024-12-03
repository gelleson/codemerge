import type { LoadResult } from "@modules/loader/loader.ts";

export type Tree = {
    path: string
    tokens: number
    children: Tree[]
}

export function buildTree(loadResults: LoadResult[]): Tree {
    const root: Tree = { path: '', children: [], tokens: 0 };

    loadResults.forEach(result => {
        const parts = result.path.split('/');
        let currentLevel = root;

        parts.forEach((part, index) => {
            const isFile = index === parts.length - 1;
            let node = currentLevel.children.find(child => child.path === part);

            if (!node) {
                node = { path: part, children: [], tokens: 0 };
                currentLevel.children.push(node);
            }

            if (isFile) {
                node.tokens = result.tokens; // Assign tokens to file node
            }

            currentLevel = node;
        });
    });

    function updateTotalTokens(node: Tree): number {
        if (node.children.length > 0) {
            node.tokens = node.children.reduce((sum, child) => sum + updateTotalTokens(child), 0);
        }
        return node.tokens;
    }

    updateTotalTokens(root);
    return root;
}