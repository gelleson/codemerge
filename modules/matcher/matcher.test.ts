import { afterEach, beforeEach, describe, expect, test } from "bun:test";
import { match } from "@modules/matcher/matcher.ts";

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
        {
            name: "test.txt",
            content: "test",
        },
    ],
};

describe("Matcher", () => {
    let tree: FileTree;
    beforeEach(() => {
        tree = new FileTree();
    });
    afterEach(async () => {
        await tree.cleanup();
    });

    test("match", async () => {
        await tree.build(basicTree);
        const result = await match({
            path: tree.getPath(),
            ignores: [],
            filters: ["**"],
            absolute: false,
        });

    });

});