# grrs

grrs is a simple hobby project that mainly follows the [cli book](https://rust-cli.github.io/book/).

## Features

This project extends the basic example from the cli book by:

- Reducing the number of allocations by using IO buffer ([read_line](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line) API)
- Adding verbose option for depicting failed reads

## Options 

```
USAGE:
    grrs [FLAGS] <PATTERN> <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints any warning or error messages

ARGS:
    <PATTERN>    A pattern used for matching a sub-slice
    <PATH>       A file to search
```

## Building

This is a Rust project so first you have to make sure that [Rust](https://www.rust-lang.org/) in running
on your machine.

To build this project:

```
$ git clone https://github.com/streof/grrs
$ cd grrs
$ cargo build --release
$ ./target/release/grrs --version
grrs 0.1.0
```

## Running tests

grrs includes a few unit and integration test. Run them as follows

```bash
$ cargo test
```