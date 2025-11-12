# path-munge

[![Crates.io](https://img.shields.io/crates/v/path-munge)](https://crates.io/crates/path-munge)
[![Crates.io](https://img.shields.io/crates/d/path-munge)](https://crates.io/crates/path-munge)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
![Release Workflow](https://img.shields.io/github/actions/workflow/status/adfernandes/path-munge/release.yml)

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

## `MANPATH` Warning

Note that `MANPATH` is a bit special. `path-munge` removes prefix, postfix, and double colons.

**However**, the [`man`](https://www.man7.org/linux/man-pages/man5/manpath.5.html) manual page, under "SEARCH PATH" has an interesting note:

> _If the value of `$MANPATH` starts with a colon, then the default search path is added at its start. If the value of `$MANPATH` ends with a colon, then the default search path is added at its end. If the value of `$MANPATH` contains a double colon (`::`), then the default search path is inserted in the middle of the value, between the two colons._

What this means is that unless you add back a leading or trailing colon when setting `MANPATH`, you will be **unable** to search the default system `MANPATH`!

