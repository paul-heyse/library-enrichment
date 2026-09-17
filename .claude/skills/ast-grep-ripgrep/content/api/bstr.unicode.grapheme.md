# `bstr::unicode::grapheme`

Crate `bstr` · 2 public items · structured records in [`model/bstr.unicode.grapheme.json`](../model/bstr.unicode.grapheme.json)

## GraphemeIndices

`struct` · `bstr::unicode::grapheme::GraphemeIndices`

Also reachable as `bstr::GraphemeIndices`

```rust
struct GraphemeIndices<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<(usize, usize, &'a str)>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<(usize, usize, &'a str)>
```

An iterator over grapheme clusters in a byte string and their byte index
positions.

This iterator is typically constructed by
[`ByteSlice::grapheme_indices`](trait.ByteSlice.html#method.grapheme_indices).

Unicode defines a grapheme cluster as an *approximation* to a single user
visible character. A grapheme cluster, or just "grapheme," is made up of
one or more codepoints. For end user oriented tasks, one should generally
prefer using graphemes instead of [`Chars`](struct.Chars.html), which
always yields one codepoint at a time.

Since graphemes are made up of one or more codepoints, this iterator
yields `&str` elements (along with their start and end byte offsets).
When invalid UTF-8 is encountered, replacement codepoints are
[substituted](index.html#handling-of-invalid-utf-8). Because of this, the
indices yielded by this iterator may not correspond to the length of the
grapheme cluster yielded with those indices. For example, when this
iterator encounters `\xFF` in the byte string, then it will yield a pair
of indices ranging over a single byte, but will provide an `&str`
equivalent to `"\u{FFFD}"`, which is three bytes in length. However, when
given only valid UTF-8, then all indices are in exact correspondence with
their paired grapheme cluster.

This iterator can be used in reverse. When reversed, exactly the same
set of grapheme clusters are yielded, but in reverse order.

This iterator only yields *extended* grapheme clusters, in accordance with
[UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Grapheme_Cluster_Boundaries).

---

## Graphemes

`struct` · `bstr::unicode::grapheme::Graphemes`

Also reachable as `bstr::Graphemes`

```rust
struct Graphemes<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<&'a str>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a str>
```

An iterator over grapheme clusters in a byte string.

This iterator is typically constructed by
[`ByteSlice::graphemes`](trait.ByteSlice.html#method.graphemes).

Unicode defines a grapheme cluster as an *approximation* to a single user
visible character. A grapheme cluster, or just "grapheme," is made up of
one or more codepoints. For end user oriented tasks, one should generally
prefer using graphemes instead of [`Chars`](struct.Chars.html), which
always yields one codepoint at a time.

Since graphemes are made up of one or more codepoints, this iterator yields
`&str` elements. When invalid UTF-8 is encountered, replacement codepoints
are [substituted](index.html#handling-of-invalid-utf-8).

This iterator can be used in reverse. When reversed, exactly the same
set of grapheme clusters are yielded, but in reverse order.

This iterator only yields *extended* grapheme clusters, in accordance with
[UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Grapheme_Cluster_Boundaries).

---
