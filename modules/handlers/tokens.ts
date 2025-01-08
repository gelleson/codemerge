import type { Argv } from "yargs";
import { isCWD, matchOrStdIn } from "@modules/matcher";
import { sort } from "@modules/tokens";
import { loadAll, type LoadResult } from "@modules/loader";
import { staticFilters } from "@modules/filters";
import { load, mergeWithDefaults } from "@modules/config";

/**
 * Token options interface to specify configurations for token operations
 */
type TokensOptions = {
    path: string;
    ignores: string[];
    filters: string[];
    total?: number;
    absolute?: boolean;
    glob?: boolean;
    maxBudget: number;
    minBudget: number;
    limitByHighBudget: boolean;
    limitByLowBudget: boolean;
    config?: string;
    ignoreConfig?: boolean;
    configPath?: string;
    context?: string;
    input?: boolean;
};

/**
 * Builds the Command-Line Interface (CLI) for the token operations.
 *
 * @param cli - A yargs.Argv instance to configure CLI options.
 * @returns Updated yargs.Argv instance with new options.
 */
export function buildTokensCLI(cli: Argv<any>): Argv<any> {
    cli.positional('path', {
        type: 'string',
        group: 'Path',
        description: 'Path where files are located.',
        //@ts-ignore - Bun.cwd is not a valid type
        default: Bun.cwd,
    });

    cli.option('i', {
        alias: 'ignores',
        type: 'array',
        group: 'Filters',
        description: 'List of paths to ignore.',
        default: [],
    });

    cli.option('f', {
        alias: 'filters',
        type: 'array',
        group: 'Filters',
        description: 'Glob patterns to match paths.',
        default: ['**'],
    });

    cli.option('absolute', {
        alias: 'a',
        hidden: true,
        type: 'boolean',
        description: 'Whether to use absolute paths.',
        default: false,
    });

    cli.option('total', {
        type: 'number',
        alias: 'n',
        group: 'Filters',
        default: 10,
        description: 'Total number of tokens to display.',
    });

    cli.option('max-budget', {
        type: 'number',
        alias: 'hb',
        group: 'Budget',
        description: 'Maximum budget for high budget filter.',
        default: 10_000,
    });

    cli.option('min-budget', {
        type: 'number',
        alias: 'lb',
        group: 'Budget',
        description: 'Minimum budget for low budget filter.',
        default: 0,
    });

    cli.option('limit-by-high-budget', {
        type: 'boolean',
        alias: 'lhb',
        group: 'Budget',
        description: 'Apply high budget limit filter.',
        default: false,
    });

    cli.option('limit-by-low-budget', {
        type: 'boolean',
        alias: 'llb',
        group: 'Budget',
        description: 'Apply low budget limit filter.',
        default: false,
    });
    cli.option('config', {
        type: 'string',
        alias: 'c',
        group: 'Config',
        description: 'Path to config file.',
        default: undefined,
    });
    cli.option('ignore-config', {
        type: 'boolean',
        group: 'Config',
        description: 'Ignore config file.',
        default: false,
    });
    cli.option('config-path', {
        type: 'string',
        group: 'Config',
        description: 'Path to config file.',
        default: undefined,
    });
    cli.option('context', {
        type: 'string',
        group: 'Config',
        description: 'Context to use.',
        default: undefined,
    });
    cli.option('input', {
        type: 'boolean',
        group: 'Config',
        description: 'Read input from stdin.',
        default: false,
    });

    return cli;
}

/**
 * Main function to execute token operations based on provided options.
 *
 * @param options - Configuration parameters for token operations.
 */
export async function tokens(options: TokensOptions) {
    const config = await load({
        cwd: options.path,
        configPath: options.configPath,
        silent: options.configPath === undefined
    });

    if (!options.ignoreConfig && config) {
        options = mergeWithDefaults(options, config);
    }
    const files = await matchOrStdIn({
        path: options.path,
        ignores: options.ignores,
        filters: options.filters,
        absolute: isCWD(options.path),
        input: options.input,
    });

    const res = await loadAll(files);

    // Compose the necessary filters
    const filters = staticFilters({
        maxBudget: options.maxBudget,
        minBudget: options.minBudget,
        limitByHighBudget: options.limitByHighBudget,
        limitByLowBudget: options.limitByLowBudget,
    });

    const filtered = filters(res);
    const results = sort(filtered);
    const total = options.total ?? results.length;
    const sorted = results.slice(0, total);

    if (results.length === 0) {
        console.log("No files found");
        return;
    }
    printTokenBoard(sorted);
    printTokenTotal(results);
}

/**
 * Prints a formatted board of token statistics.
 *
 * @param results - An array of load results containing file paths and token counts.
 */
export function printTokenBoard(results: LoadResult[]) {
    const maxPathLength = Math.max(...results.map(r => r.path.length));

    console.log('\nToken Statistics:');
    console.log('─'.repeat(maxPathLength + 20));

    const sorted = [...results].sort((a, b) => b.tokens - a.tokens);

    sorted.forEach(result => {
        const padding = ' '.repeat(maxPathLength - result.path.length);
        console.log(`${result.path}${padding} │ ${result.tokens.toString().padStart(8)} tokens`);
    });

    console.log('─'.repeat(maxPathLength + 20));
}

/**
 * Prints the total number of tokens across all results.
 *
 * @param results - An array of LoadResult objects.
 */
export function printTokenTotal(results: LoadResult[]) {
    const total = results.reduce((sum, result) => sum + result.tokens, 0);
    console.log(`Total tokens: ${total}`);
}