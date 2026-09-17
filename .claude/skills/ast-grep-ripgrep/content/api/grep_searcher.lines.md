# `grep_searcher::lines`

Crate `grep-searcher` · 2 public items · structured records in [`model/grep_searcher.lines.json`](../model/grep_searcher.lines.json)

## LineIter

`struct` · `grep_searcher::lines::LineIter`

Also reachable as `grep_searcher::LineIter`

```rust
struct LineIter<'b>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(line_term: u8, bytes: &'b [u8]) -> LineIter<'b>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'b [u8]>
```

An iterator over lines in a particular slice of bytes.

Line terminators are considered part of the line they terminate. All lines
yielded by the iterator are guaranteed to be non-empty.

`'b` refers to the lifetime of the underlying bytes.

---

## LineStep

`struct` · `grep_searcher::lines::LineStep`

Also reachable as `grep_searcher::LineStep`

```rust
struct LineStep
```

**Derives**: Debug

**Methods** (2)

```rust
fn new(line_term: u8, start: usize, end: usize) -> LineStep
fn next(&mut self, bytes: &[u8]) -> Option<(usize, usize)>
```

An explicit iterator over lines in a particular slice of bytes.

This iterator avoids borrowing the bytes themselves, and instead requires
callers to explicitly provide the bytes when moving through the iterator.
While not idiomatic, this provides a simple way of iterating over lines
that doesn't require borrowing the slice itself, which can be convenient.

Line terminators are considered part of the line they terminate. All lines
yielded by the iterator are guaranteed to be non-empty.

---
