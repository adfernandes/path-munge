# path-munge

This utility is designed to help with managing `PATH`-type shell variables.

For example, given
```bash
$ echo $MANPATH
:/usr/local/man:/home/andrew/.local/share/man
```
we can prepend directories via
```bash
$ path-munge MANPATH prepend force a b c
a:b:c:/usr/local/man:/home/andrew/.local/share/man
```
or append then via
```bash
$ path-munge MANPATH append force a b c
/usr/local/man:/home/andrew/.local/share/man:a:b:c
```

Note that extra path-separator characters are trimmed from the result.

## Usage

To use it in your `.bashrc`, you would need to use it as part of an expansion,
since `path-munge` never modifies the environment itself.

For example, to prepend `/opt/bin` and `/foo/bar` to your path, use
```bash
PATH="$(path-munge PATH prepend force /opt/bin /foo/bar)"
```

In addition to the `prepend` and `append` options, you can also use `keep`
to retain the precedence of a path if it already exists, or use `force` to
remove its existing precedence and replace it with the new one. 

## Installation

Install with `cargo`:
```bash
cargo install path-munge
```
or download a binary from the [releases page](https://github.com/adfernandes/path-munge/releases).
