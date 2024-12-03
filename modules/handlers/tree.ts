import { isCWD, match } from "@modules/matcher/matcher.ts";
import { loadAll } from "@modules/loader";
import { buildTree, type Tree } from "@modules/loader/tree.ts";
import type { ArgumentsCamelCase, Argv } from "yargs";
import { compose, enabled, removeHighBudget, removeLowBudget, staticFilters } from "@modules/filters";
import { emptyContentFilter } from "@modules/filters/content.ts";

/**
 * Options for configuring the tree command functionality.
 */
type TreeOptions = {
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
};

/**
 * Sets up the command-line interface (CLI) options for tree structure generation.
 *
 * @param cli - The yargs.Argv instance used for CLI option definitions.
 * @returns The configured yargs.Argv instance with tree options.
 */
export function buildTreeCLI(cli: Argv<any>): Argv<any> {
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

    return cli;
}

/**
 * Executes the logic to display a tree structure of files based on the provided options.
 *
 * @param options - The parsed options from the command-line arguments.
 */
export async function tree(options: ArgumentsCamelCase<TreeOptions>) {
    // Retrieve files based on matching options
    const files = await match({
        path: options.path,
        ignores: options.ignores,
        filters: options.filters,
        absolute: isCWD(options.path),
    });

    const data = await loadAll(files);

    const filters = staticFilters({
        maxBudget: options.maxBudget,
        minBudget: options.minBudget,
        limitByHighBudget: options.limitByHighBudget,
        limitByLowBudget: options.limitByLowBudget,
    });

    const filtered = filters(data);

    // Build and print the tree structure
    console.log(printTree(buildTree(filtered)));
}

/**
 * Recursively prints the tree structure of files.
 *
 * @param tree - The tree object representing the file hierarchy.
 * @param indent - The current indentation level for visual representation.
 * @param isLast - Flag to indicate if the item is the last child in a group.
 * @returns A formatted string representing the tree structure.
 */
export function printTree(tree: Tree, indent: string = '', isLast: boolean = true): string {
    let output = '';

    // If the node is the root, process its children
    if (tree.path === '') {
        tree.children.forEach((child, index) => {
            output += printTree(
                child,
                '',
                index === tree.children.length - 1
            );
        });
        return output;
    }

    // Non-root nodes are printed with appropriate markers and indentation
    const marker = isLast ? '└── ' : '├── ';
    output += `${indent}${marker}${tree.path} (${tree.tokens} tokens)\n`;

    const childIndent = indent + (isLast ? '    ' : '│   ');

    tree.children.forEach((child, index) => {
        output += printTree(
            child,
            childIndent,
            index === tree.children.length - 1
        );
    });

    return output;
}