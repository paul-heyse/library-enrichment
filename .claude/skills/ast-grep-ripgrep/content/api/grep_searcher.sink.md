# `grep_searcher::sink`

Crate `grep-searcher` · 6 public items · structured records in [`model/grep_searcher.sink.json`](../model/grep_searcher.sink.json)

## SinkContextKind

`enum` · `grep_searcher::sink::SinkContextKind`

Also reachable as `grep_searcher::SinkContextKind`

```rust
enum SinkContextKind
```

**Variants**: `Before`, `After`, `Other`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

The type of context reported by a searcher.

---

## SinkContext

`struct` · `grep_searcher::sink::SinkContext`

Also reachable as `grep_searcher::SinkContext`

```rust
struct SinkContext<'b>
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn absolute_byte_offset(&self) -> u64
fn bytes(&self) -> &'b [u8]
fn kind(&self) -> &SinkContextKind
fn line_number(&self) -> Option<u64>
```

A type that describes a contextual line reported by a searcher.

---

## SinkFinish

`struct` · `grep_searcher::sink::SinkFinish`

Also reachable as `grep_searcher::SinkFinish`

```rust
struct SinkFinish
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn binary_byte_offset(&self) -> Option<u64>
fn byte_count(&self) -> u64
```

Summary data reported at the end of a search.

This reports data such as the total number of bytes searched and the
absolute offset of the first occurrence of binary data, if any were found.

A searcher that stops early because of an error does not call `finish`.
A searcher that stops early because the `Sink` implementor instructed it
to will still call `finish`.

---

## SinkMatch

`struct` · `grep_searcher::sink::SinkMatch`

Also reachable as `grep_searcher::SinkMatch`

```rust
struct SinkMatch<'b>
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn absolute_byte_offset(&self) -> u64
fn buffer(&self) -> &'b [u8]
fn bytes(&self) -> &'b [u8]
fn bytes_range_in_buffer(&self) -> std::ops::Range<usize>
fn line_number(&self) -> Option<u64>
fn lines(&self) -> LineIter<'b>
```

A type that describes a match reported by a searcher.

---

## Sink

`trait` · `grep_searcher::sink::Sink`

Also reachable as `grep_searcher::Sink`

```rust
trait Sink
```

**Implementors** (7)

- `alloc::boxed::Box`
- `grep_printer::json::JSONSink`
- `grep_printer::standard::StandardSink`
- `grep_printer::summary::SummarySink`
- `grep_searcher::sink::sinks::Bytes`
- `grep_searcher::sink::sinks::Lossy`
- `grep_searcher::sink::sinks::UTF8`

**Methods** (6)

```rust
fn begin(&mut self, _searcher: &Searcher) -> Result<bool, Self::Error>
fn binary_data(&mut self, _searcher: &Searcher, _binary_byte_offset: u64) -> Result<bool, Self::Error>
fn context(&mut self, _searcher: &Searcher, _context: &SinkContext<'_>) -> Result<bool, Self::Error>
fn context_break(&mut self, _searcher: &Searcher) -> Result<bool, Self::Error>
fn finish(&mut self, _searcher: &Searcher, _: &SinkFinish) -> Result<(), Self::Error>
fn matched(&mut self, _searcher: &Searcher, _mat: &SinkMatch<'_>) -> Result<bool, Self::Error>
```

A trait that defines how results from searchers are handled.

In this crate, a searcher follows the "push" model. What that means is that
the searcher drives execution, and pushes results back to the caller. This
is in contrast to a "pull" model where the caller drives execution and
takes results as they need them. These are also known as "internal" and
"external" iteration strategies, respectively.

For a variety of reasons, including the complexity of the searcher
implementation, this crate chooses the "push" or "internal" model of
execution. Thus, in order to act on search results, callers must provide
an implementation of this trait to a searcher, and the searcher is then
responsible for calling the methods on this trait.

This trait defines several behaviors:

* What to do when a match is found. Callers must provide this.
* What to do when an error occurs. Callers must provide this via the
  [`SinkError`] trait. Generally, callers can just use `std::io::Error` for
  this, which already implements `SinkError`.
* What to do when a contextual line is found. By default, these are
  ignored.
* What to do when a gap between contextual lines has been found. By
  default, this is ignored.
* What to do when a search has started. By default, this does nothing.
* What to do when a search has finished successfully. By default, this does
  nothing.

Callers must, at minimum, specify the behavior when an error occurs and
the behavior when a match occurs. The rest is optional. For each behavior,
callers may report an error (say, if writing the result to another
location failed) or simply return `false` if they want the search to stop
(e.g., when implementing a cap on the number of search results to show).

When errors are reported (whether in the searcher or in the implementation
of `Sink`), then searchers quit immediately without calling `finish`.

For simpler uses of `Sink`, callers may elect to use one of
the more convenient but less flexible implementations in the
[`sinks`] module.

---

## SinkError

`trait` · `grep_searcher::sink::SinkError`

Also reachable as `grep_searcher::SinkError`

```rust
trait SinkError: Sized
```

**Implementors** (2)

- `alloc::boxed::Box`
- `core::io::error::Error`

**Methods** (3)

```rust
fn error_config(err: ConfigError) -> Self
fn error_io(err: io::Error) -> Self
fn error_message<T: std::fmt::Display>(message: T) -> Self
```

A trait that describes errors that can be reported by searchers and
implementations of `Sink`.

Unless you have a specialized use case, you probably don't need to
implement this trait explicitly. It's likely that using `std::io::Error`
(which implements this trait) for your error type is good enough,
largely because most errors that occur during search will likely be an
`std::io::Error`.

---
