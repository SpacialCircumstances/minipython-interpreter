# minipython-interpreter

A simple interpreter for (a) minipython, a minimal, turing complete, python-like language. As minipython is not actually a subset of Python (f.e. incrementing variables without creating them, the range of an int), it cannot be reliably interpreted by a python interpreter. This interpreter is capable of running minipython programs, with the restriction that identation-based syntax is not fully supported, which means that while-blocks have to end with an `#endwhile` comment and function definitions with an `#enddef`.

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
