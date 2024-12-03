import { test, expect, describe } from "bun:test";
import { calcTokens, calcTokensWithLimit } from "@modules/tokens/calc.ts";

describe("tokens", () => {
    test("calcTokens", () => {
        expect(calcTokens("hello")).toBe(1);
    });

    test("calcTokensWithLimit", () => {
        expect(calcTokensWithLimit("hello", 10)).toBe(1);
        expect(calcTokensWithLimit("hello2121", 1)).toBe(false);
    });
});

