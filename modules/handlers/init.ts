import type { ArgumentsCamelCase, Argv } from "yargs";
import { join } from "path";  // Importing the `join` method for handling file paths effectively.

/**
 * Defines the options for initializing the configuration file.
 */
export interface InitOptions {
    fileName: string;  // The name of the file that will be created or used.
    force: boolean;  // Whether to force the creation of the file, even if it already exists.
}

/**
 * Configures the Command-Line Interface (CLI) options specific to the initialization process.
 *
 * @param cli - An instance of yargs.Argv used for defining command-line option configurations.
 * @returns The configured yargs.Argv instance after adding the relevant initialization options.
 */
export function buildInitCLI(cli: Argv<{}>): Argv<{}> {
    // Defines a command-line option to specify the name of the configuration file to be created.
    cli.option("file-name", {
        type: "string",  // Data type for the command-line option, indicating it expects a string.
        description: "The name of the file to create",  // Descriptive text guiding the user on what this option does.
        default: ".codemerge.yaml",  // Default value if the user does not provide one.
    });
    cli.option("force", {
        alias: "f",  // Short form of the command-line option for ease of use.
        type: "boolean",  // Data type for the command-line option, indicating it expects a boolean.
        description: "Force the creation of the file, even if it already exists.",  // Descriptive text guiding the user on what this option does.
        default: false,  // Default value if the user does not provide one.
    });
    return cli;  // Return the modified cli instance for further processing or use.
}

/**
 * Initializes the CLI by writing the default configuration file.
 *
 * @param argv - The arguments passed from the command line, typed to ensure expected structure.
 * @returns A promise that resolves when the file has been created.
 */
export async function initCLI(argv: ArgumentsCamelCase<InitOptions>): Promise<void> {
    // Writes the default configuration file to the specified file name.
    //@ts-ignore - Bun.file is not a valid type
    const file = Bun.file(join(Bun.cwd, argv.fileName));  // Constructs a Bun.File instance for the specified file.
    if (await file.exists() && !argv.force) {  // Checks if the file already exists.
        console.log(`File ${argv.fileName} already exists.`);  // Notify the user that the file already exists.
        return;  // Exit the function early.
    }

    await Bun.write(
        //@ts-ignore
        join(Bun.cwd, argv.fileName),  // Constructs the full path to the file using the current working directory.
        `
version: 1  # This specifies the version of the CodeMerge configuration format being used.

# This is a CodeMerge configuration file. It outlines how files should be processed and merged.
# For detailed instructions and options, refer to the project's official documentation:
# https://github.com/gelleson/codemerge

contexts:
    # Contexts define specific scenarios or configurations for merging files.
    - context: default  # The 'default' context is used when no specific context is provided.

      filters:
      # Filters are patterns that specify which files or directories should be considered
      # for merging operations. A pattern of "**" means to include all files/subdirectories.
      - "**"

      ignores:
      # Ignores define patterns for files or directories that should be excluded from merging.
      # These patterns are useful for skipping temporary files, system files, or source control directories.
      - ".git/"  # Ignore all contents within the .git directory used for version control.
      - "*.lock"  # Ignore all lock files, often used by package managers to lock dependencies.
        `
    );
    console.log(`Created ${argv.fileName}`);  // Notify the user that the file has been created successfully.
}