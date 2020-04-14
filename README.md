# grrs

grrs is a simple hobby project that mainly follows the [cli book](https://rust-cli.github.io/book/).

## Features

This project extends the basic example from the cli book:

- Less memory allocation (see [Memory allocation](#memory-allocation) below)
- More features (line numbers, case insensitive search, limit shown matches, etc.)
- More modulair codebase
- More tests

## Options 

```
USAGE:
    grrs [OPTIONS] <PATTERN> <PATH>

ARGS:
    <PATTERN>    A pattern used for matching a sub-slice
    <PATH>       A file to search

OPTIONS:
    -h, --help               Prints help information
    -i, --ignore-case        Case insensitive search
    -m, --max-count <NUM>    Limit number of shown matches
    -n, --no-line-number     Do not show line number which is enabled by default
    -s, --starts-with        Only show matches containing words starting with PATTERN
    -V, --version            Prints version information
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
grrs 0.1.2
```

## Running tests

grrs includes a few unit and integration test. Run them as follows

```
$ cargo test
```

## Considerations

### Performance

IMHO cli's should be at least perceived as fast by their users. Grrs being very 
much a basic hobby project means that performance wasn't a strict requirement. 

I did however look into several crates (including the awesome [ripgrep](https://github.com/BurntSushi/ripgrep)) in order 
to explore the available options. Obviously, performance of a cli such as grrs 
depends on many factors including:

- system calls
- memory allocation
- matching algorithm

#### System calls

An easy way to gain more information about a process is by using something like 
`strace` (on Linux) or `dtruss` (on macOS). When comparing to a smart cli such
as ripgrep, one will see that the number of `read` syscalls is significantly
lower than when using a hobby project such as grrs. One of the reasons for that 
is ripgrep's [`searcher`](https://github.com/BurntSushi/ripgrep/tree/master/crates/searcher) 
who, among others things, looks for potential candidates and restricts the search 
space resulting in lower number of `read` syscalls.


#### Memory allocation

A simple way to reduce memory allocation is by using an IO buffer. Rust's 
standard library provides for example the very convenient [`lines`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines) API but also the lower level [`read_line`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line) and [`read_until`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_until) methods. Initially, I used `read_line` but 
then I read this [reddit thread](https://www.reddit.com/r/rust/comments/cqpswx/processing_data_line_by_line_from_stdin_rust/) where [linereader](https://github.com/Freaky/rust-linereader)
was mentioned. I ended up using [bstr](https://github.com/BurntSushi/bstr) which
offers a good balance between rich, ergonomic API and performance (see this [commit](https://github.com/BurntSushi/bstr/commit/66dee497c8da16f397c1d0952e58dadf04b66b5c)).

#### Matching algorithm

[TODO]
