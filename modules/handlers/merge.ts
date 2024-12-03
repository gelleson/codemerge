import type { ArgumentsCamelCase, Argv } from 'yargs';

import { loadAll } from '@modules/loader';
import { isCWD, match } from "@modules/matcher";
import { formatter } from "@modules/formatters";
import * as process from "node:process";
import { compose, enabled, removeHighBudget, removeLowBudget, staticFilters } from "@modules/filters";
import { emptyContentFilter } from "@modules/filters/content.ts";

/**
 * Options for configuring the merge process.
 */
export type MergeOptions = {
    path: string;
    ignores: string[];
    filters: string[];
    format: 'text';
    output?: string;
    absolute?: boolean;
    maxBudget: number;
    minBudget: number;
    limitByHighBudget: boolean;
    limitByLowBudget: boolean;
}

/**
 * Constructs a Command-Line Interface (CLI) for merging operations.
 *
 * @param cli - The yargs.Argv instance for command-line configuration.
 * @returns The configured yargs.Argv instance with merging options.
 */
export function buildMergeCLI(cli: Argv<{}>): Argv<{}> {
    cli.positional('path', {
        type: 'string',
        group: 'Path',
        description: 'Path to files or directories for merging.',
        //@ts-ignore
        default: Bun.cwd,
    });

    cli.option('i', {
        alias: 'ignores',
        type: 'array',
        group: 'Filters',
        description: 'List of files or directories to ignore.',
        default: [],
    });

    cli.option('f', {
        alias: 'filters',
        type: 'array',
        group: 'Filters',
        description: 'Glob patterns to filter paths.',
        default: ['**'],
    });

    cli.option('format', {
        type: 'string',
        output: 'Output',
        description: 'Output format of the merged results.',
        choices: ['text', 'json'],
        default: 'text',
    });

    cli.option('output', {
        type: 'string',
        group: 'Output',
        description: 'Path to the output file, or use stdout if not specified.',
    });

    cli.option('total', {
        type: 'number',
        alias: 'n',
        group: 'Filters',
        description: 'Total number of tokens to display.',
    });

    cli.option('max-budget', {
        type: 'number',
        alias: 'hb',
        group: 'Budget',
        description: 'Maximum budget constraint for high budget filtering.',
        default: 10_000,
    });

    cli.option('min-budget', {
        type: 'number',
        alias: 'lb',
        group: 'Budget',
        description: 'Minimum budget constraint for low budget filtering.',
        default: 0,
    });

    cli.option('limit-by-high-budget', {
        type: 'boolean',
        alias: 'lhb',
        group: 'Budget',
        description: 'Enable limiting by maximum budget.',
        default: false,
    });

    cli.option('limit-by-low-budget', {
        type: 'boolean',
        alias: 'llb',
        group: 'Budget',
        description: 'Enable limiting by minimum budget.',
        default: false,
    });

    return cli;
}

/**
 * Generates a filter function to exclude files of specified types.
 *
 * @param types - Array of file types to exclude.
 * @returns A function to filter files by types.
 */
export function filterByTypes(types: string[]): (file: string) => boolean {
    return (file: string) => {
        const f = Bun.file(file);
        return !types.includes(f.type);
    };
}

/**
 * Merges files based on provided command-line arguments.
 *
 * @param argv - The parsed arguments from the CLI input.
 */
export async function merge(argv: ArgumentsCamelCase<MergeOptions>) {
    // Match files based on provided options
    const files = await match({
        path: argv.path,
        ignores: argv.ignores,
        filters: argv.filters,
        absolute: isCWD(argv.path),
    });

    const data = await loadAll(files);
    const format = formatter({
        type: argv.format,
    });

    // Compose the necessary filters
    const filters = staticFilters({
        maxBudget: argv.maxBudget,
        minBudget: argv.minBudget,
        limitByHighBudget: argv.limitByHighBudget,
        limitByLowBudget: argv.limitByLowBudget,
    });


    const filtered = filters(data);

    // Determine the output destination
    let output: any = process.stdout;
    if (argv.output) {
        output = Bun.file(argv.output).writer();
    }

    // Write the formatted, filtered data to the specified output
    await format.format(output, filtered);
}