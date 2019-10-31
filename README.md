# minipython-interpreter

A simple interpreter for minipython, a minimal, turing complete, sort-of subset of Python.

## CLI

```
USAGE:
    minipython.exe [FLAGS] <file>

FLAGS:
    -h, --help           Prints help information
    -i, --interactive    Run in interactive mode, allowing you to specify input and output in the script
    -t, --trace          Trace execution (displays context before and after each statement
    -V, --version        Prints version information

ARGS:
    <file>    The script file to run
```

## Building

`cargo build` for a debug build
`cargo build --release` for a release build
`cargo install --path .` to install the binary

## Code

Warning: this code is full of bugs, is non-idiomatic Rust, is unperformant and horribly hacky.
