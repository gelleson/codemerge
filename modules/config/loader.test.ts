import { describe, expect, test, beforeEach, afterEach } from "bun:test";
import { loadConfig } from "@modules/config/loader.ts";
import { temporaryDirectory } from "tempy";
import { rm } from "node:fs/promises";

describe("loadConfig", () => {
    let dir: string;
    beforeEach(() => {
        dir = temporaryDirectory();
    });

    afterEach(() => {
        rm(dir, {recursive: true});
    });

    test("loads config from file with default context", async () => {
        const contextYaml = `
        version: 1
        contexts:
           - context: default
             ignores: 
                - node_modules
                - dist
             filters:
                - src
           - context: test
             ignores: 
                - node_modules
                - dist
             filters:
                - src
        `;
        const path = dir + "/config.test.yaml";
        await Bun.write(path, contextYaml);

        const result = await loadConfig(path);
        expect(result).toEqual({
            context: "default",
            ignores: ['node_modules', 'dist'],
            filters: ['src']
        });
    });

    test("throws an error if the config file does not exist", async () => {
        const nonExistentPath = dir + "/nonexistent-config.yaml";

        expect(loadConfig(nonExistentPath)).rejects.toThrow();
    });

    test("loads config from file with specified context", async () => {
        const contextYaml = `
        version: 1
        contexts:
           - context: default
             ignores: 
                - node_modules
                - dist
             filters:
                - src
           - context: test
             ignores: 
                - cache
             filters:
                - lib
        `;
        const path = dir + "/config.test.yaml";
        await Bun.write(path, contextYaml);

        const result = await loadConfig(path, "test");
        expect(result).toEqual({
            context: "test",
            ignores: ['cache'],
            filters: ['lib']
        });
    });

    test("throws an error for non-existent context", async () => {
        const contextYaml = `
        version: 1
        contexts:
           - context: default
             ignores: 
                - node_modules
                - dist
             filters:
                - src
        `;
        const path = dir + "/config.test.yaml";
        await Bun.write(path, contextYaml);

        expect(loadConfig(path, "nonexistent")).rejects.toThrow("Context 'nonexistent' not found in config.");
    });

    test("throws an error for configuration with empty contexts", async () => {
        const contextYaml = `
        version: 1
        contexts: []
        `;
        const path = dir + "/config.test.yaml";
        await Bun.write(path, contextYaml);

        expect(loadConfig(path)).rejects.toThrow("Context 'default' not found in config.");
    });

    test("throws an error for invalid configuration schema", async () => {
        const invalidYaml = `
        version: 1
        context: {}
        `;
        const path = dir + "/invalidConfig.test.yaml";
        await Bun.write(path, invalidYaml);

        expect(loadConfig(path)).rejects.toThrow();
    });
});