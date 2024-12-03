import { describe, expect, it } from "bun:test";
import { load, loadAll } from "./loader.ts";
import { join } from "path";
import { writeFileSync, unlinkSync } from "fs";

describe("loader", () => {
    it("load", async () => {
        // Create a temporary test file
        const testPath = join(import.meta.dir, "test.txt");
        const testContent = "Hello, World!";
        writeFileSync(testPath, testContent);

        try {
            const result = await load(testPath);

            expect(result.path).toBe(testPath);
            expect(result.content).toBe(testContent);
            expect(result.tokens).toBeGreaterThan(0);
        } finally {
            // Cleanup
            unlinkSync(testPath);
        }
    });

    it("loadAll", async () => {
        // Create multiple test files
        const files = [
            { path: join(import.meta.dir, "test1.txt"), content: "Hello" },
            { path: join(import.meta.dir, "test2.txt"), content: "World" }
        ];

        files.forEach(file => writeFileSync(file.path, file.content));

        try {
            const results = await loadAll(files.map(f => f.path));

            expect(results.length).toBe(files.length);
            results.forEach((result, i) => {
                expect(result.path).toBe(files[i].path);
                expect(result.content).toBe(files[i].content);
                expect(result.tokens).toBeGreaterThan(0);
            });
        } finally {
            // Cleanup
            files.forEach(file => unlinkSync(file.path));
        }
    });
});