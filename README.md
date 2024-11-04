# CodeMerge

CodeMerge is a command-line tool for merging multiple code files into a single output file. It provides an easy way to combine code from different files while ignoring certain files/directories and calculating token counts.

## Features

- Merge multiple code files into one output file
- Specify files/directories to ignore during merging
- Calculate token counts for each file and total tokens
- Verbose output mode for detailed information
- Written in Rust for fast performance
- Can be used with AI large language models (LLMs) for context-aware code generation

## Installation

To install CodeMerge, you can download the pre-built binary from the [releases page](https://github.com/gelleson/codemerge/releases) or build it from source using Cargo:

```bash
cargo install codemerge
```

## Usage

### Merging Files

To merge code files, use the `merge` command:

```bash
codemerge merge -o merged.txt -f "**/*.rs"
```

This will merge all `.rs` files into the output file `merged.txt`.

Additional options:
- `-o, --output`: Specify the output file name (required for `merge` command)
- `-i, --ignores`: Specify files/directories to ignore (can be used multiple times)
- `-v, --verbose`: Enable verbose output mode
- `-f, --filter`: Specify a filter pattern (can be used multiple times)
- `-n, --file-names-only`: Print only file names

### Calculating Tokens

To calculate token counts without merging, use the `tokens` command:

```bash
codemerge tokens -c 5
```

This will calculate the token counts for each file and display the top 5 files with the most tokens.

Additional options:
- `-c, --count`: Specify the number of top files to display
- `-i, --ignores`: Specify files/directories to ignore (can be used multiple times)
- `-v, --verbose`: Enable verbose output mode
- `-f, --filter`: Specify a filter pattern (can be used multiple times)

## Configuration

CodeMerge respects `.gitignore` files for specifying files/directories to ignore during merging and token calculation. You can also use the `-i` flag to specify additional ignore patterns.

## Previous Go Version

The original version of CodeMerge was written in Go and is available in the `/go` branch of this repository. However, this version will not be actively maintained going forward. The Rust version was rewritten to take advantage of Rust's improved performance, safety, and ecosystem, particularly the `tiktoken-rs` library for efficient token counting.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

CodeMerge is open-source software released under the [MIT License](LICENSE).
