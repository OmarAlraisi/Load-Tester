# Load-Tester
This is a server's load tester written in Rust.

## Usage

loadt -u <url> [...options]

### Options

- `-u` To read a url.
- `-n` Number of requests. (Default 1)
- `-c` Concurrent Requests. (Default 1)
- `-f` To read line-seperated urls from a file. If specified, can ommit `-u`.
