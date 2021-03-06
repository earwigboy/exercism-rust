# Grep

Welcome to Grep on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Search a file for lines matching a regular expression pattern. Return the line
number and contents of each matching line.

The Unix [`grep`](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html) command can be used to search for lines in one or more files
that match a user-provided search query (known as the *pattern*).

The `grep` command takes three arguments:

1. The pattern used to match lines in a file.
2. Zero or more flags to customize the matching behavior.
3. One or more files in which to search for matching lines.

Your task is to implement the `grep` function, which should read the contents
of the specified files, find the lines that match the specified pattern
and then output those lines as a single string. Note that the lines should
be output in the order in which they were found, with the first matching line
in the first file being output first.

As an example, suppose there is a file named "input.txt" with the following contents:

```text
hello
world
hello again
```

If we were to call `grep "hello" input.txt`, the returned string should be:

```text
hello
hello again
```

## Flags

As said earlier, the `grep` command should also support the following flags:

- `-n` Print the line numbers of each matching line.
- `-l` Print only the names of files that contain at least one matching line.
- `-i` Match line using a case-insensitive comparison.
- `-v` Invert the program -- collect all lines that fail to match the pattern.
- `-x` Only match entire lines, instead of lines that contain a match.

If we run `grep -n "hello" input.txt`, the `-n` flag will require the matching
lines to be prefixed with its line number:

```text
1:hello
3:hello again
```

And if we run `grep -i "HELLO" input.txt`, we'll do a case-insensitive match,
and the output will be:

```text
hello
hello again
```

The `grep` command should support multiple flags at once.

For example, running `grep -l -v "hello" file1.txt file2.txt` should
print the names of files that do not contain the string "hello".

## Error handling

This exercise introduces the `anyhow` crate, which makes it easy to handle arbitrary error types.
Its intent is to ensure that when you're writing an application, you don't have to worry about what
particular errors your called function is returning, but to just do the right thing when propagating them.

N.B.: it is actually somewhat bad form to use `anyhow` when writing a library, as we are here; it's more
explicit and more useful to write your own `Error` enum when writing a library (potentially with the aid of helper
macros such as are provided by the [`thiserror` crate](https://crates.io/crates/thiserror)). However, we are
intentionally and explicitly doing so here to demonstrate the use of this crate.

To learn more about this crate refer to its [documentation](https://docs.rs/anyhow/1.0.32/anyhow/).

## Additional reading

While this exercise asks you to implement only the most basic functions of `grep`,
there is actually a project to fully re-implement `grep` in Rust - [ripgrep](https://github.com/BurntSushi/ripgrep).

If you liked the concept of rewriting the basic util programs in Rust be sure to check the following projects:

- [fd](https://github.com/sharkdp/fd) - a clone of `find`
- [exa](https://github.com/ogham/exa) - a clone of `ls`
- [bat](https://github.com/sharkdp/bat) - a clone of `cat`
- [coreutils](https://github.com/uutils/coreutils) - a rewrite of the GNU coreutils

## Source

### Created by

- @ZapAnton

### Contributed to by

- @AndrewKvalheim
- @coriolinus
- @cwhakes
- @efx
- @ErikSchierboom
- @k33nice
- @lutostag
- @petertseng
- @rofrol
- @stringparser

### Based on

Conversation with Nate Foster. - http://www.cs.cornell.edu/Courses/cs3110/2014sp/hw/0/ps0.pdf