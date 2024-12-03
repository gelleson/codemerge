import yargs from 'yargs';
import {
    buildMergeCLI,
    merge,
} from '@modules/handlers/merge.ts';
import { buildTreeCLI, tree } from "@modules/handlers/tree.ts";
import { buildTokensCLI, tokens } from "@modules/handlers/tokens.ts";

/**
 * Entry point for the command-line interface (CLI) application.
 * Utilizes yargs to define and handle various CLI commands.
 */
const cli = yargs(process.argv.slice(2));

cli
    .scriptName("codemerge") // Sets the script name for the CLI application
    .command(
        'merge',     // Command name
        'Performs a merge operation combining specified files.', // Detailed description of the 'merge' command
        buildMergeCLI, // Function to setup CLI options for the merge command
        merge         // Function to execute the merge command logic
    )
    .command(
        'tree',      // Command name
        'Displays a hierarchical tree structure of files based on filtering criteria.', // Detailed description of the 'tree' command
        buildTreeCLI, // Function to setup CLI options for the tree command
        tree          // Function to execute the tree command logic
    )
    .command(
        'tokens',    // Command name
        'Calculates and displays token information for specified files.', // Detailed description of the 'tokens' command
        //@ts-ignore - buildTokensCLI is not a valid type
        buildTokensCLI, // Function to setup CLI options for the tokens command
        tokens          // Function to execute the tokens command logic
    )
    .version()
    .argv;            // Triggers the argv parsing