# CodeMerge

CodeMerge is a command-line interface (CLI) tool built with Rust for merging and analyzing codebases. It leverages Rust for performance-critical tasks like file I/O, token counting, and caching, while also incorporating a modern CLI design to offer flexible options. The tool allows you to merge files, display file tree structures, and calculate token counts—all while respecting .gitignore files and offering various filtering and caching options.

## Features

- **File Merging:** Combines the content of multiple files into a single output, configurable via globs and ignores.
- **File Tree Generation:** Visualizes the project's file structure with token counts for each file and directory.
- **Token Counting:** Calculates the number of tokens in files using the `gpt-tokenizer` library, useful for estimating costs associated with large language models.
- **`.gitignore` Support:** Respects .gitignore files for comprehensive filtering, handling multiple .gitignore files within nested directories.
- **Flexible Filtering:** Supports glob patterns, budget limits (minimum and maximum token counts), and the ability to ignore specific file types.
- **Output Formatting:** Presents results in a clear, human-readable format.
- **Caching:**

  - _Cache Providers:_ Choose among three caching strategies – SQLite, RocksDB, or disable caching altogether using the "none" provider.
  - _Configuration Options:_ Easily specify a custom cache directory using the `--cache_dir` flag, disable caching via `--no_cache`, or clear the cache with `--clear_cache`.
  - _Usage Integration:_ Cached file results optimize repeated runs by avoiding redundant file reads.

- **Performance:** Uses Rust for high-speed file processing, token counting, and caching to enhance overall efficiency.

## Installation

### One-Line Install

To install CodeMerge on Linux or macOS, run:

curl -sL https://raw.githubusercontent.com/gelleson/codemerge/main/scripts/install.sh | sudo bash

After installation, use the `codemerge` command directly.

### Manual Installation

1. **Clone the Repository:**

git clone https://github.com/gelleson/codemerge.git
cd codemerge

2. **Build and Install from Source:**

   - **Install Rust:** Ensure you have Rust installed. You can install Rust using `rustup`:

     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   - **Build and Install:**

     cargo build --release
     sudo mv target/release/codemerge /usr/local/bin

   This will build the project and install the CodeMerge binary.

## Usage

CodeMerge provides several commands along with new caching capabilities. Run `codemerge --help` to see the full list of options.

### 1. Merging Files (`merge`)

Merge the contents of multiple files into one output:

codemerge merge [options]

Options include:

- `<path>`: The directory or file paths to merge.
- `--ignores, -i`: Glob patterns for files or directories to ignore.
- `--filters, -f`: Glob patterns for files or directories to include (defaults to `**`).
- `--format`: Output format (default: `text`).
- `--output`: Path to save the merged output; if not provided, output is sent to stdout.
- `--max-budget, -hb`: Maximum token budget (default: 10,000).
- `--min-budget, -lb`: Minimum token budget (default: 0).
- `--limit-by-high-budget, -lhb`: Apply maximum budget filtering.
- `--limit-by-low-budget, -llb`: Apply minimum budget filtering.
- **Caching Options:**
  - `--cache_provider`: Choose a cache provider (sqlite, rocksdb, or none). (Default is `sqlite`.)
  - `--cache_dir`: Specify a custom directory for cache storage.
  - `--no_cache`: Disable caching completely.
  - `--clear_cache`: Clear cache before processing.

**Example:**

codemerge merge ./src --filters "\*.ts" --output merged_code.txt

### 2. Generating File Tree (`tree`)

Display a visual representation of the file tree along with token counts:

codemerge tree <path> [options]

Options include similar filtering and budgeting flags as in the merge command.

**Example:**

codemerge tree

### 3. Calculating Token Counts (`tokens`)

Count tokens in files, which is useful when estimating costs for large language model interactions:

codemerge tokens [options]

Options include:

- `<path>`: The target directory.
- `--ignores, -i`: Glob patterns for files/directories to ignore.
- `--filters, -f`: Glob patterns for files/directories to include (default: `**`).
- `--total, -n`: Maximum number of files to display (default: all).
- Budget and caching options as seen in the merge command.

### 4. Managing Cache (`cache`)

Manage cache operations independently using a dedicated subcommand. This allows you to inspect or clear the cache without running a file merge or tree operation.

Usage:

codemerge cache <operation> [--provider <provider>] [--dir <cache_directory>]

Where `<operation>` can be:

- `clear`: Deletes all cache entries.
- `info`: Displays cache provider and cache directory information.

**Example:**

codemerge cache clear --provider sqlite
codemerge cache info --dir /custom/cache/dir

## Caching Details

CodeMerge now integrates a caching layer to speed up file processing by avoiding repeated disk I/O for unchanged files. The caching module supports:

- **SQLite Cache:** Stores cached file data in a local SQLite database (`cache.db` in the cache directory).
- **RocksDB Cache:** Uses RocksDB for a high-performance, embedded key-value store.
- **None:** Disables caching altogether, so every run reads all files from disk.

Cache configuration options include specifying a custom cache directory, disabling caching entirely, or clearing the cache before executing a command. Cache choice can be controlled via the `--cache_provider` flag.

## Previous Versions

The initial version of CodeMerge was developed in Go (available on the `/go` branch) but is no longer maintained. The current version is a complete rewrite in Rust that offers improved performance, safety, and additional features like caching.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License
