import { describe, expect, test } from "bun:test";
import { TextFormatter } from "@modules/formatters/text.ts";
import type { LoadResult } from "@modules/loader";

describe("TextFormatter", () => {
    test("format writes content correctly", async () => {
        const formatter = new TextFormatter({
            type: "text",
        });
        const written: string[] = [];

        // Mock WritableStreamDefaultWriter
        const mockWriter = {
            write: async (content: string) => {
                written.push(content);
                return Promise.resolve();
            }
        } as WritableStreamDefaultWriter;

        const results: LoadResult[] = [
            {
                path: "/test/file1.txt",
                content: "content1",
                tokens: 1
            },
            {
                path: "/test/file2.txt",
                content: "content2",
                tokens: 1
            }
        ];

        await formatter.format(mockWriter, results);

        // Check the written content
        expect(written).toEqual([
            "=== Result ===",
            "\n",
            "File: /test/file1.txt\n",
            "content1",
            "File: /test/file2.txt\n",
            "content2"
        ]);
    });

    test("format handles empty results", async () => {
        const formatter = new TextFormatter({
            type: "text",
        });
        const written: string[] = [];

        const mockWriter = {
            write: async (content: string) => {
                written.push(content);
                return Promise.resolve();
            }
        } as WritableStreamDefaultWriter;

        await formatter.format(mockWriter, []);

        expect(written).toEqual([
            "=== Result ===",
            "\n"
        ]);
    });
});