# minipython-interpreter

A simple interpreter for (a) minipython, a minimal, turing complete, python-like language. As minipython is not actually a subset of Python (f.e. incrementing variables without creating them, the range of an int), it cannot be reliably interpreted by a python interpreter. This interpreter is capable of running minipython programs, with the restriction that identation-based syntax is not fully supported, which means that while-blocks have to end with an `#endwhile` comment and function definitions with an `#enddef`.

## Usage

### Normal script

Assume you have a simple minipython script in a file called dup.mpy:

```
a+=1
a+=1
while a!=0:
    a-=1
    b+=1
    b+=1
#endwhile
```

You can run this script with `minipython -t dup.mpy` (where t is to trace variables) and you will get a result that looks like this:
```
[a: 0, b: 4]
```
Running without `-t` will run the script, but not print anything except errors.

### Interactive script

An interactive script is a script prefaced with a declaration of input and output variables (basically, its a function as a script file).

A simple interactive script that computes f(x) = 2 * x:

```
input: x
output: y
while x!=0:
    x-=1
    y+=1
    y+=1
#endwhile
```

Running this script with `minipython -i -t dup_interactive.mpy` will prompt you for the value of x, then return x * 2.
You **must** always run interactive scripts with `-i`, else the script will fail.

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


## Warnings

Warning: this code is full of bugs, is non-idiomatic Rust, is unperformant and horribly hacky.
