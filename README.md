# CodeMerge

CodeMerge is a command-line tool for merging multiple code files into a single output file. It provides an easy way to combine code from different files while ignoring certain files/directories and calculating token counts.

## Features

- Merge multiple code files into one output file
- Specify files/directories to ignore during merging
- Calculate token counts for each file and total tokens
- Verbose output mode for detailed information
- Written in Go for fast performance
- Can be used with AI large language models (LLMs) for context-aware code generation


## Installation

To install CodeMerge, you need to have Go installed on your system. Then you can use `go get` to download and install the tool:

```bash
go install github.com/gelleson/codemerge
```

This will download the source code, compile it, and install the `codemerge` binary in your `$GOPATH/bin` directory.

## Usage

### Merging Files

To merge code files, use the `merge` command:

```bash
codemerge merge -o merged.txt
```


This will merge files into the output file `merged.txt`.

Additional options:
- `-o, --output`: Specify the output file name (required)
- `-i, --ignores`: Specify files/directories to ignore (can be used multiple times)
- `-v, --verbose`: Enable verbose output mode

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

## Configuration

CodeMerge respects `.gitignore` files for specifying files/directories to ignore during merging and token calculation. You can also use the `-i` flag to specify additional ignore patterns.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

CodeMerge is open-source software released under the [MIT License](LICENSE).
