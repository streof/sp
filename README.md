# grrs

grrs is a basic implementation of grep/ripgrep. It can be used to find patterns/words
in files. 

## Options 

```
USAGE:
    grrs [OPTIONS] <PATTERN> <PATH>

ARGS:
    <PATTERN>    A pattern used for matching a sub-slice
    <PATH>       A file to search

OPTIONS:
    -c, --count              Suppress normal output and show number of matching lines
    -e, --ends-with          Only show matches containing fields ending with PATTERN
    -h, --help               Prints help information
    -i, --ignore-case        Case insensitive search
    -m, --max-count <NUM>    Limit number of shown matches
    -n, --no-line-number     Do not show line number which is enabled by default
    -s, --starts-with        Only show matches containing fields starting with PATTERN
    -V, --version            Prints version information
    -w, --words              Whole words search (i.e. non-word characters are stripped)
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

Tha main goal of this project was to reimplement a small number of grep/ripgrep
alike features. Performance was not a strict requirement althought, in my
opinion, cli's should at least be perceived as fast by their users. Performance
is obviously a trade-off and for grss depends on things like:

- system calls
- memory allocation
- cpu utilization
- heuristics
- matching algorithm

Here are some more details of my exploration process:

- An easy way to gain more information about a process is by using something like
`strace` (on Linux) or `dtruss` (on macOS). When comparing to a smart cli such
as ripgrep, one will see that in some cases the number of `read` syscalls is
significantly lower than when using a hobby project such as grrs. One of the
reasons for that is heuristics in [ripgrep](https://github.com/BurntSushi/ripgrep)'s
searcher which, among others things, determines whether or not to use memory
maps and looks for potential candidates restricting the search space in some
cases.
- A simple way to reduce memory allocation is by using an IO buffer. Rust's
standard library provides for example the very convenient [`lines`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines) API but also the lower level [`read_line`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line) and [`read_until`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_until) methods. Initially, I used `read_line` but
then I read this [reddit thread](https://www.reddit.com/r/rust/comments/cqpswx/processing_data_line_by_line_from_stdin_rust/) where [linereader](https://github.com/Freaky/rust-linereader)
was mentioned. I ended up using [bstr](https://github.com/BurntSushi/bstr) which
offers a good balance between rich, ergonomic API and performance (see this [commit](https://github.com/BurntSushi/bstr/commit/66dee497c8da16f397c1d0952e58dadf04b66b5c)).
- Counts in grrs rely on a very naive implementation that does not take any
advantage of modern CPU capabilities (see for example [bytecount](https://github.com/llogiq/bytecount))

<!-- TODO: Matching algorithms -->