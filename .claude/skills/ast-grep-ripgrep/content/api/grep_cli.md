# `grep_cli`

Crate `grep-cli` · 4 public items · structured records in [`model/grep_cli.json`](../model/grep_cli.json)

## is_readable_stdin

`function` · `grep_cli::is_readable_stdin`

```rust
fn is_readable_stdin() -> bool
```

Returns true if and only if stdin is believed to be readable.

When stdin is readable, command line programs may choose to behave
differently than when stdin is not readable. For example, `command foo`
might search the current directory for occurrences of `foo` where as
`command foo < some-file` or `cat some-file | command foo` might instead
only search stdin for occurrences of `foo`.

Note that this isn't perfect and essentially corresponds to a heuristic.
When things are unclear (such as if an error occurs during introspection to
determine whether stdin is readable), this prefers to return `false`. That
means it's possible for an end user to pipe something into your program and
have this return `false` and thus potentially lead to ignoring the user's
stdin data. While not ideal, this is perhaps better than falsely assuming
stdin is readable, which would result in blocking forever on reading stdin.
Regardless, commands should always provide explicit fallbacks to override
behavior. For example, `rg foo -` will explicitly search stdin and `rg foo
./` will explicitly search the current working directory.

---

## is_tty_stderr

`function` · `grep_cli::is_tty_stderr`

> **Deprecated** — since 0.1.10: use std::io::IsTerminal instead

```rust
fn is_tty_stderr() -> bool
```

Returns true if and only if stderr is believed to be connected to a tty
or a console.

Note that this is now just a wrapper around
[`std::io::IsTerminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html).
Callers should prefer using the `IsTerminal` trait directly. This routine
is deprecated and will be removed in the next semver incompatible release.

---

## is_tty_stdin

`function` · `grep_cli::is_tty_stdin`

> **Deprecated** — since 0.1.10: use std::io::IsTerminal instead

```rust
fn is_tty_stdin() -> bool
```

Returns true if and only if stdin is believed to be connected to a tty
or a console.

Note that this is now just a wrapper around
[`std::io::IsTerminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html).
Callers should prefer using the `IsTerminal` trait directly. This routine
is deprecated and will be removed in the next semver incompatible release.

---

## is_tty_stdout

`function` · `grep_cli::is_tty_stdout`

> **Deprecated** — since 0.1.10: use std::io::IsTerminal instead

```rust
fn is_tty_stdout() -> bool
```

Returns true if and only if stdout is believed to be connected to a tty
or a console.

This is useful for when you want your command line program to produce
different output depending on whether it's printing directly to a user's
terminal or whether it's being redirected somewhere else. For example,
implementations of `ls` will often show one item per line when stdout is
redirected, but will condensed output when printing to a tty.

Note that this is now just a wrapper around
[`std::io::IsTerminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html).
Callers should prefer using the `IsTerminal` trait directly. This routine
is deprecated and will be removed in the next semver incompatible release.

---
