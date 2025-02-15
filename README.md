Merging and Analyzing Code

`codemerge` is a command-line interface (CLI) tool built with Rust for merging and analyzing codebases. It leverages Rust for performance-critical tasks like file I/O and token counting, while using TypeScript for the CLI and higher-level logic. The tool allows you to merge files, display file tree structures, and calculate token counts, all while respecting `.gitignore` files and offering various filtering options.

## Features

- **File Merging:** Combines the content of multiple files into a single output, configurable via globs and ignores.
- **File Tree Generation:** Visualizes the project's file structure with token counts for each file and directory.
- **Token Counting:** Calculates the number of tokens in files using the `gpt-tokenizer` library, useful for estimating costs associated with large language models.
- **`.gitignore` Support:** Respects `.gitignore` files for comprehensive filtering, handling multiple `.gitignore` files within nested directories.
- **Filtering:** Supports flexible filtering using glob patterns, budget limits (minimum and maximum token counts), and the ability to ignore specific file types.
- **Output Formatting:** Presents results in a clear, human-readable text format.
- **Performance:** Uses Rust for file processing and token counting to enhance speed and efficiency.

## Installation

### One-Line Install

To install `codemerge` on Linux or macOS, run the following command:

```bash
curl -sL https://raw.githubusercontent.com/gelleson/codemerge/main/scripts/install.sh | sudo bash
```

After installation, you can use the `codemerge` command directly.

### Manual Installation

1. **Clone the Repository:**

```bash
git clone https://github.com/gelleson/codemerge.git
cd codemerge
```

2. **Build and Install from Source:**

   - **Install Rust:** Ensure you have Rust installed. You can install Rust using `rustup`:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - **Build and Install:**

     ```bash
     cargo build --release
     sudo mv target/release/codemerge /usr/local/bin
     ```

     This will build the project and install the `codemerge` binary.

## Usage

The `codemerge` CLI has three main commands: `merge`, `tree`, and `tokens`. Run `codemerge --help` to see the full list of options for each command.

**1. Merging Files (`merge`)**

```bash
codemerge merge  [options]
```

- `<path>`: The path to the directory or file you want to merge.
- `-i`, `--ignores`: (Array) Files or directories to ignore (glob patterns).
- `-f`, `--filters`: (Array) Files or directories to include (glob patterns), defaults to `**`.
- `--format`: Output format (`text`, default).
- `--output`: Output file path (defaults to stdout).
- `--max-budget`, `-hb`: Maximum token budget (default: 10,000).
- `--min-budget`, `-lb`: Minimum token budget (default: 0).
- `--limit-by-high-budget`, `-lhb`: Apply the maximum budget limit filter.
- `--limit-by-low-budget`, `-llb`: Apply the minimum budget limit filter.

**Example:**

```bash
codemerge merge ./src --filters "*.ts" --output merged_code.txt
```

**2. Generating File Tree (`tree`)**

```bash
codemerge tree <path> [options]
```

- `<path>`: The path to the directory.
- `-i`, `--ignores`: (Array) Files or directories to ignore (glob patterns).
- `-f`, `--filters`: (Array) Files or directories to include (glob patterns), defaults to `**`.
- `--max-budget`, `-hb`: Maximum token budget (default: 10,000).
- `--min-budget`, `-lb`: Minimum token budget (default: 0).
- `--limit-by-high-budget`, `-lhb`: Apply the maximum budget limit filter.
- `--limit-by-low-budget`, `-llb`: Apply the minimum budget limit filter.

**Example:**

```bash
codemerge tree
```

**3. Calculating Token Counts (`tokens`)**

```bash
codemerge tokens [options]
```

- `<path>`: The path to the directory.
- `-i`, `--ignores`: (Array) Files or directories to ignore (glob patterns).
- `-f`, `--filters`: (Array) Files or directories to include (glob patterns), defaults to `**`.
- `-n`, `--total`: Maximum number of files to display (default: all).
- `--max-budget`, `-hb`: Maximum token budget (default: 10,000).
- `--min-budget`, `-lb`: Minimum token budget (default: 0).
- `--limit-by-high-budget`, `-lhb`: Apply the maximum budget limit filter.
- `--limit-by-low-budget`, `-llb`: Apply the minimum budget limit filter.

## Previous Versions and Evolution of CodeMerge

The initial version of CodeMerge was developed in Go, and it is available on the `/go` branch of the repository, although it is no longer maintained. The subsequent version was rewritten in Rust, offering enhanced performance, safety, and ecosystem benefits, especially with the use of the `tiktoken-rs` library for token counting.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License
