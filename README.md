# grsp

`grsp` is a command-line utility written in Rust that searches for patterns within a text file and displays matching lines along with their surrounding context.

## Features

- Search for a specific pattern using regular expressions.
- Display matching lines with a configurable number of surrounding context lines.
- Handles file input and error reporting gracefully.

## Requirements

- [Rust](https://www.rust-lang.org/) installed on your system.

## Usage

### Building the Program

Clone this repository and navigate to the project directory. Then, build the program using:

```bash
cargo build --release

### Running the program
```bash
cargo run -- <pattern> -- <filename>

### Arguments:
<pattern>:

The regular expression pattern to search for in the file.
Example: "strong" to search for the word "strong".
<filename>:

The path to the file where the search will be performed.
Example: example.txt for a file named example.txt.

## Error Handling
If the file cannot be opened, an error message will be displayed.
If no matches are found, the program will exit without producing output.
## Notes
The number of context lines is currently set to 2 by default. This can be adjusted in the source code if needed.