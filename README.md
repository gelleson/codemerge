# Codemerge: A CLI Tool for Merging and Analyzing Code

`codemerge` is a command-line interface (CLI) tool built with Bun and Rust for merging and analyzing codebases. It leverages Rust for performance-critical tasks like file I/O and token counting, while using TypeScript and Bun for the CLI and higher-level logic. The tool allows you to merge files, display file tree structures, and calculate token counts, all while respecting `.gitignore` files and offering various filtering options.

## Features

* **File Merging:** Combines the content of multiple files into a single output, configurable via globs and ignores.
* **File Tree Generation:** Visualizes the project's file structure with token counts for each file and directory.
* **Token Counting:** Calculates the number of tokens in files using the `gpt-tokenizer` library, useful for estimating costs associated with large language models.
* **`.gitignore` Support:** Respects `.gitignore` files for comprehensive filtering, handling multiple `.gitignore` files within nested directories.
* **Filtering:** Supports flexible filtering using glob patterns, budget limits (minimum and maximum token counts), and the ability to ignore specific file types.
* **Output Formatting:** Presents results in a clear, human-readable text format.
* **Performance:** Uses Rust for file processing and token counting to enhance speed and efficiency.

## Installation

1. **Install Bun:** If you don't have Bun installed, download it from [https://bun.sh/](https://bun.sh/).

2. **Clone the Repository:**
   ```bash
   git clone https://github.com/gelleson/codemerge.git
   cd codemerge
   ```

3. **Build Rust Dependencies:** Navigate into the `rust` directory and run:

   ```bash
   just build
   ```

## Usage

The `codemerge` CLI has three main commands: `merge`, `tree`, and `tokens`. Run `codemerge --help` to see the full list of options for each command.

**1. Merging Files (`merge`)**

```bash
codemerge merge  [options]
```

* `<path>`: The path to the directory or file you want to merge.
* `-i`, `--ignores`: (Array) Files or directories to ignore (glob patterns).
* `-f`, `--filters`: (Array) Files or directories to include (glob patterns), defaults to `**`.
* `--format`: Output format (`text`, default).
* `--output`: Output file path (defaults to stdout).
* `--max-budget`, `-hb`: Maximum token budget (default: 10,000).
* `--min-budget`, `-lb`: Minimum token budget (default: 0).
* `--limit-by-high-budget`, `-lhb`: Apply the maximum budget limit filter.
* `--limit-by-low-budget`, `-llb`: Apply the minimum budget limit filter.

**Example:**

```bash
codemerge merge ./src --filters "*.ts" --output merged_code.txt
```

**2. Generating File Tree (`tree`)**

```bash
codemerge tree <path> [options]
```

* `<path>`: The path to the directory.
* `-i`, `--ignores`: (Array) Files or directories to ignore (glob patterns).
* `-f`, `--filters`: (Array) Files or directories to include (glob patterns), defaults to `**`.
* `--max-budget`, `-hb`: Maximum token budget (default: 10,000).
* `--min-budget`, `-lb`: Minimum token budget (default: 0).
* `--limit-by-high-budget`, `-lhb`: Apply the maximum budget limit filter.
* `--limit-by-low-budget`, `-llb`: Apply the minimum budget limit filter.

**Example:**

```bash
codemerge tree 
```

**3. Calculating Token Counts (`tokens`)**

```bash
codemerge tokens [options]
```

* `<path>`: The path to the directory.
* `-i`, `--ignores`: (Array) Files or directories to ignore (glob patterns).
* `-f`, `--filters`: (Array) Files or directories to include (glob patterns), defaults to `**`.
* `-n`, `--total`: Maximum number of files to display (default: all).
* `--max-budget`, `-hb`: Maximum token budget (default: 10,000).
* `--min-budget`, `-lb`: Minimum token budget (default: 0).
* `--limit-by-high-budget`, `-lhb`: Apply the maximum budget limit filter.
* `--limit-by-low-budget`, `-llb`: Apply the minimum budget limit filter.


## Previous Versions and Evolution of CodeMerge
The initial version of CodeMerge was developed in Go, and it is available on the `/go` branch of the repository, although it is no longer maintained. The subsequent version was rewritten in Rust, offering enhanced performance, safety, and ecosystem benefits, especially with the use of the `tiktoken-rs` library for token counting.
The latest, third-generation version combines Bun JS and Rust. This modern approach improves maintainability and performance, capitalizing on Bun for the command-line interface and TypeScript for higher-level logic, while utilizing Rust for performance-critical tasks.



## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

