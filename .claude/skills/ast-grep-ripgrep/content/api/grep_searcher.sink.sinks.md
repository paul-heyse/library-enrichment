# `grep_searcher::sink::sinks`

Crate `grep-searcher` · 3 public items · structured records in [`model/grep_searcher.sink.sinks.json`](../model/grep_searcher.sink.sinks.json)

## Bytes

`struct` · `grep_searcher::sink::sinks::Bytes`

Also reachable as `grep_searcher::sinks::Bytes`

```rust
struct Bytes<F> where F: FnMut(u64, &[u8]) -> Result<bool, io::Error>
```

**Implements**: `grep_searcher::sink::Sink`

**Derives**: Clone, Debug

**via `grep_searcher::sink::Sink`**

```rust
fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error>
```

A sink that provides line numbers and matches as raw bytes while
ignoring everything else.

This implementation will return an error on the first match if the
searcher was not configured to count lines.

The closure accepts two parameters: a line number and a raw byte string
containing the matched data. The closure returns a
`Result<bool, std::io::Error>`. If the `bool` is `false`, then the
search stops immediately. Otherwise, searching continues.

If multi line mode was enabled, the line number refers to the line
number of the first line in the match.

---

## Lossy

`struct` · `grep_searcher::sink::sinks::Lossy`

Also reachable as `grep_searcher::sinks::Lossy`

```rust
struct Lossy<F> where F: FnMut(u64, &str) -> Result<bool, io::Error>
```

**Implements**: `grep_searcher::sink::Sink`

**Derives**: Clone, Debug

**via `grep_searcher::sink::Sink`**

```rust
fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error>
```

A sink that provides line numbers and matches as (lossily converted)
strings while ignoring everything else.

This is like `UTF8`, except that if a match contains invalid UTF-8,
then it will be lossily converted to valid UTF-8 by substituting
invalid UTF-8 with Unicode replacement characters.

This implementation will return an error on the first match if the
searcher was not configured to count lines.

The closure accepts two parameters: a line number and a UTF-8 string
containing the matched data. The closure returns a
`Result<bool, std::io::Error>`. If the `bool` is `false`, then the
search stops immediately. Otherwise, searching continues.

If multi line mode was enabled, the line number refers to the line
number of the first line in the match.

---

## UTF8

`struct` · `grep_searcher::sink::sinks::UTF8`

Also reachable as `grep_searcher::sinks::UTF8`

```rust
struct UTF8<F> where F: FnMut(u64, &str) -> Result<bool, io::Error>
```

**Implements**: `grep_searcher::sink::Sink`

**Derives**: Clone, Debug

**via `grep_searcher::sink::Sink`**

```rust
fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error>
```

A sink that provides line numbers and matches as strings while ignoring
everything else.

This implementation will return an error if a match contains invalid
UTF-8 or if the searcher was not configured to count lines. Errors
on invalid UTF-8 can be suppressed by using the `Lossy` sink instead
of this one.

The closure accepts two parameters: a line number and a UTF-8 string
containing the matched data. The closure returns a
`Result<bool, std::io::Error>`. If the `bool` is `false`, then the
search stops immediately. Otherwise, searching continues.

If multi line mode was enabled, the line number refers to the line
number of the first line in the match.

---
