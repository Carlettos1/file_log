# File Log

The `file_log` module provides functionality for logging messages to files. Using different indexes for each run of your code.

The idea of this crate is to maintain all previous logs without needing to change the code.

This crate creates a file called `log_index` to maintain the last index created. Unless the environment variable `FILE_LOG_INDEX` is found.

## Installation
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
file_log = "0.1.0"
```

## Quick usage

```rust
use file_log::{log, index};

fn main() {
    log!("log", "Hello"); // this will append "hello" to log_<index>.log
    log!("log" "xyz", "{} {} {}", index(), index(), index()); // this will append "<index> <index> <index>" to log_<index>.xyz
}
```
By running:
```shell
$ cargo run
```
You create the `log_index` file that mantains the last index used, and every log file used in the `log!` macro.

Or, using:
```shell
$ FILE_LOG_INDEX=10 cargo run
```
The `log_index` file will not be created, and the index in the `file_log::index()` function, will be 10.

## Usage

See the brief example at [Example](example/)

-----------------------------

This project is licensed under the [MIT License](LICENSE).