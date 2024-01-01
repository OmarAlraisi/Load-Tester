# Load-Tester
This is a server's load tester written in Rust.

## Usage

cargo run -- -u <url> [...options]

### Options

- `-u` Reads a url from the command line.
- `-n` Number of requests. (Default 1)
- `-c` Number of concurrent threads. (Default 1)
- `-f` Reads line-seperated urls from a file.
