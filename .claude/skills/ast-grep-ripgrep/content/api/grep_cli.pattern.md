# `grep_cli::pattern`

Crate `grep-cli` · 6 public items · structured records in [`model/grep_cli.pattern.json`](../model/grep_cli.pattern.json)

## pattern_from_bytes

`function` · `grep_cli::pattern::pattern_from_bytes`

Also reachable as `grep_cli::pattern_from_bytes`

```rust
fn pattern_from_bytes(pattern: &[u8]) -> Result<&str, InvalidPatternError>
```

Convert arbitrary bytes into a regular expression pattern.

This conversion fails if the given pattern is not valid UTF-8, in which
case, a targeted error with more information about where the invalid UTF-8
occurs is given. The error also suggests the use of hex escape sequences,
which are supported by many regex engines.

---

## pattern_from_os

`function` · `grep_cli::pattern::pattern_from_os`

Also reachable as `grep_cli::pattern_from_os`

```rust
fn pattern_from_os(pattern: &std::ffi::OsStr) -> Result<&str, InvalidPatternError>
```

Convert an OS string into a regular expression pattern.

This conversion fails if the given pattern is not valid UTF-8, in which
case, a targeted error with more information about where the invalid UTF-8
occurs is given. The error also suggests the use of hex escape sequences,
which are supported by many regex engines.

---

## patterns_from_path

`function` · `grep_cli::pattern::patterns_from_path`

Also reachable as `grep_cli::patterns_from_path`

```rust
fn patterns_from_path<P: AsRef<std::path::Path>>(path: P) -> io::Result<Vec<String>>
```

Read patterns from a file path, one per line.

If there was a problem reading or if any of the patterns contain invalid
UTF-8, then an error is returned. If there was a problem with a specific
pattern, then the error message will include the line number and the file
path.

---

## patterns_from_reader

`function` · `grep_cli::pattern::patterns_from_reader`

Also reachable as `grep_cli::patterns_from_reader`

```rust
fn patterns_from_reader<R: io::Read>(rdr: R) -> io::Result<Vec<String>>
```

Read patterns from any reader, one per line.

If there was a problem reading or if any of the patterns contain invalid
UTF-8, then an error is returned. If there was a problem with a specific
pattern, then the error message will include the line number.

Note that this routine uses its own internal buffer, so the caller should
not provide their own buffered reader if possible.

# Example

This shows how to parse patterns, one per line.

```
use grep_cli::patterns_from_reader;

let patterns = "\
foo
bar\\s+foo
[a-z]{3}
";

assert_eq!(patterns_from_reader(patterns.as_bytes())?, vec![
    r"foo",
    r"bar\s+foo",
    r"[a-z]{3}",
]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## patterns_from_stdin

`function` · `grep_cli::pattern::patterns_from_stdin`

Also reachable as `grep_cli::patterns_from_stdin`

```rust
fn patterns_from_stdin() -> io::Result<Vec<String>>
```

Read patterns from stdin, one per line.

If there was a problem reading or if any of the patterns contain invalid
UTF-8, then an error is returned. If there was a problem with a specific
pattern, then the error message will include the line number and the fact
that it came from stdin.

---

## InvalidPatternError

`struct` · `grep_cli::pattern::InvalidPatternError`

Also reachable as `grep_cli::InvalidPatternError`

```rust
struct InvalidPatternError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn valid_up_to(&self) -> usize
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An error that occurs when a pattern could not be converted to valid UTF-8.

The purpose of this error is to give a more targeted failure mode for
patterns written by end users that are not valid UTF-8.

---
