import { describe, expect, test } from "bun:test";
import type { LoadResult } from "@modules/loader/loader.ts";
import { buildTree, type Tree } from "@modules/loader/tree.ts";
import { printTree } from "@modules/handlers/tree.ts";

describe("buildTree", () => {
    test("builds correct tree structure", () => {
        const loadResults: LoadResult[] = [
            { path: "src/index.ts", content: "hello", tokens: 1 },
            { path: "src/utils/helper.ts", content: "hello", tokens: 1 },
            { path: "README.md", content: "hello1", tokens: 1 },
        ];

        const expectedTree = {
            path: "",
            tokens: 3,
            children: [
                {
                    path: "src",
                    tokens: 2,
                    children: [
                        { path: "index.ts", children: [], tokens: 1 },
                        { path: "utils", children: [{ path: "helper.ts", children: [], tokens: 1 }], tokens: 1 },
                    ],
                },
                { path: "README.md", children: [], tokens: 1 },
            ],
        };

        const tree = buildTree(loadResults);
        expect(tree).toEqual(expectedTree);
    });
});


describe("printTree", () => {
    test("prints tree structure correctly", () => {
        const tree: Tree = {
            path: '',
            tokens: 600,
            children: [
                {
                    path: 'src',
                    tokens: 600,
                    children: [
                        {
                            path: 'main.ts',
                            tokens: 400,
                            children: []
                        },
                        {
                            path: 'utils.ts',
                            tokens: 200,
                            children: []
                        }
                    ]
                }
            ]
        };

        const expected =
            `└── src (600 tokens)
    ├── main.ts (400 tokens)
    └── utils.ts (200 tokens)
`;

        expect(printTree(tree)).toBe(expected);
    });

    test("handles empty tree", () => {
        const tree: Tree = {
            path: '',
            tokens: 0,
            children: []
        };

        expect(printTree(tree)).toBe('');
    });
});