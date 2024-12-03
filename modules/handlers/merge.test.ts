import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { merge, type MergeOptions } from "@modules/handlers/merge.ts";

import { FileTree, type TreeNode } from "@modules/testing";
import type { ArgumentsCamelCase } from "yargs";
const basicTree: TreeNode = {
    name: "",
    children: [
        {
            name: ".gitignore",
            content: `
            test2.txt
            `,
        },
        {
            name: "src",
            children: [
                {
                    name: "test.txt",
                    content: "test",
                },
            ],
        },
    ],
};
describe("Merge CLI", () => {
    let tree: FileTree;
    beforeEach(() => {
        tree = new FileTree();
    });
    afterEach(async () => {
        await tree.cleanup();
    });

    test("merge", async () => {
        await tree.build(basicTree);
        const result = await merge({
            path: tree.getPath(),
            ignores: [],
            filters: ["**"],
            format: "text",
            absolute: true,
        } as unknown as ArgumentsCamelCase<MergeOptions>);
    });
});