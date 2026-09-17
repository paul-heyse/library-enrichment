# `grep_cli::escape`

Crate `grep-cli` · 4 public items · structured records in [`model/grep_cli.escape.json`](../model/grep_cli.escape.json)

## escape

`function` · `grep_cli::escape::escape`

Also reachable as `grep_cli::escape`

```rust
fn escape(bytes: &[u8]) -> String
```

Escapes arbitrary bytes into a human readable string.

This converts `\t`, `\r` and `\n` into their escaped forms. It also
converts the non-printable subset of ASCII in addition to invalid UTF-8
bytes to hexadecimal escape sequences. Everything else is left as is.

The dual of this routine is [`unescape`].

# Example

This example shows how to convert a byte string that contains a `\n` and
invalid UTF-8 bytes into a `String`.

Pay special attention to the use of raw strings. That is, `r"\n"` is
equivalent to `"\\n"`.

```
use grep_cli::escape;

assert_eq!(r"foo\nbar\xFFbaz", escape(b"foo\nbar\xFFbaz"));
```

---

## escape_os

`function` · `grep_cli::escape::escape_os`

Also reachable as `grep_cli::escape_os`

```rust
fn escape_os(string: &std::ffi::OsStr) -> String
```

Escapes an OS string into a human readable string.

This is like [`escape`], but accepts an OS string.

---

## unescape

`function` · `grep_cli::escape::unescape`

Also reachable as `grep_cli::unescape`

```rust
fn unescape(s: &str) -> Vec<u8>
```

Unescapes a string.

It supports a limited set of escape sequences:

* `\t`, `\r` and `\n` are mapped to their corresponding ASCII bytes.
* `\xZZ` hexadecimal escapes are mapped to their byte.

Everything else is left as is, including non-hexadecimal escapes like
`\xGG`.

This is useful when it is desirable for a command line argument to be
capable of specifying arbitrary bytes or otherwise make it easier to
specify non-printable characters.

The dual of this routine is [`escape`].

# Example

This example shows how to convert an escaped string (which is valid UTF-8)
into a corresponding sequence of bytes. Each escape sequence is mapped to
its bytes, which may include invalid UTF-8.

Pay special attention to the use of raw strings. That is, `r"\n"` is
equivalent to `"\\n"`.

```
use grep_cli::unescape;

assert_eq!(&b"foo\nbar\xFFbaz"[..], &*unescape(r"foo\nbar\xFFbaz"));
```

---

## unescape_os

`function` · `grep_cli::escape::unescape_os`

Also reachable as `grep_cli::unescape_os`

```rust
fn unescape_os(string: &std::ffi::OsStr) -> Vec<u8>
```

Unescapes an OS string.

This is like [`unescape`], but accepts an OS string.

Note that this first lossily decodes the given OS string as UTF-8. That
is, an escaped string (the thing given) should be valid UTF-8.

---
