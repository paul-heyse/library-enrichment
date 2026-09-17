# `bstr::unicode::word`

Crate `bstr` · 4 public items · structured records in [`model/bstr.unicode.word.json`](../model/bstr.unicode.word.json)

## WordIndices

`struct` · `bstr::unicode::word::WordIndices`

Also reachable as `bstr::WordIndices`

```rust
struct WordIndices<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<(usize, usize, &'a str)>
```

An iterator over words in a byte string and their byte index positions.

This iterator is typically constructed by
[`ByteSlice::word_indices`](trait.ByteSlice.html#method.word_indices).

This is similar to the
[`WordsWithBreakIndices`](struct.WordsWithBreakIndices.html) iterator,
except it only returns elements that contain a "word" character. A
word character is defined by UTS #18 (Annex C) to be the combination
of the `Alphabetic` and `Join_Control` properties, along with the
`Decimal_Number`, `Mark` and `Connector_Punctuation` general categories.

Since words are made up of one or more codepoints, this iterator
yields `&str` elements (along with their start and end byte offsets).
When invalid UTF-8 is encountered, replacement codepoints are
[substituted](index.html#handling-of-invalid-utf-8). Because of this, the
indices yielded by this iterator may not correspond to the length of the
word yielded with those indices. For example, when this iterator encounters
`\xFF` in the byte string, then it will yield a pair of indices ranging
over a single byte, but will provide an `&str` equivalent to `"\u{FFFD}"`,
which is three bytes in length. However, when given only valid UTF-8, then
all indices are in exact correspondence with their paired word.

This iterator yields words in accordance with the default word boundary
rules specified in
[UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Word_Boundaries).
In particular, this may not be suitable for Japanese and Chinese scripts
that do not use spaces between words.

---

## Words

`struct` · `bstr::unicode::word::Words`

Also reachable as `bstr::Words`

```rust
struct Words<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a str>
```

An iterator over words in a byte string.

This iterator is typically constructed by
[`ByteSlice::words`](trait.ByteSlice.html#method.words).

This is similar to the [`WordsWithBreaks`](struct.WordsWithBreaks.html)
iterator, except it only returns elements that contain a "word" character.
A word character is defined by UTS #18 (Annex C) to be the combination
of the `Alphabetic` and `Join_Control` properties, along with the
`Decimal_Number`, `Mark` and `Connector_Punctuation` general categories.

Since words are made up of one or more codepoints, this iterator yields
`&str` elements. When invalid UTF-8 is encountered, replacement codepoints
are [substituted](index.html#handling-of-invalid-utf-8).

This iterator yields words in accordance with the default word boundary
rules specified in
[UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Word_Boundaries).
In particular, this may not be suitable for Japanese and Chinese scripts
that do not use spaces between words.

---

## WordsWithBreakIndices

`struct` · `bstr::unicode::word::WordsWithBreakIndices`

Also reachable as `bstr::WordsWithBreakIndices`

```rust
struct WordsWithBreakIndices<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<(usize, usize, &'a str)>
```

An iterator over all word breaks in a byte string, along with their byte
index positions.

This iterator is typically constructed by
[`ByteSlice::words_with_break_indices`](trait.ByteSlice.html#method.words_with_break_indices).

This iterator yields not only all words, but the content that comes between
words. In particular, if all elements yielded by this iterator are
concatenated, then the result is the original string (subject to Unicode
replacement codepoint substitutions).

Since words are made up of one or more codepoints, this iterator
yields `&str` elements (along with their start and end byte offsets).
When invalid UTF-8 is encountered, replacement codepoints are
[substituted](index.html#handling-of-invalid-utf-8). Because of this, the
indices yielded by this iterator may not correspond to the length of the
word yielded with those indices. For example, when this iterator encounters
`\xFF` in the byte string, then it will yield a pair of indices ranging
over a single byte, but will provide an `&str` equivalent to `"\u{FFFD}"`,
which is three bytes in length. However, when given only valid UTF-8, then
all indices are in exact correspondence with their paired word.

This iterator yields words in accordance with the default word boundary
rules specified in
[UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Word_Boundaries).
In particular, this may not be suitable for Japanese and Chinese scripts
that do not use spaces between words.

---

## WordsWithBreaks

`struct` · `bstr::unicode::word::WordsWithBreaks`

Also reachable as `bstr::WordsWithBreaks`

```rust
struct WordsWithBreaks<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a str>
```

An iterator over all word breaks in a byte string.

This iterator is typically constructed by
[`ByteSlice::words_with_breaks`](trait.ByteSlice.html#method.words_with_breaks).

This iterator yields not only all words, but the content that comes between
words. In particular, if all elements yielded by this iterator are
concatenated, then the result is the original string (subject to Unicode
replacement codepoint substitutions).

Since words are made up of one or more codepoints, this iterator yields
`&str` elements. When invalid UTF-8 is encountered, replacement codepoints
are [substituted](index.html#handling-of-invalid-utf-8).

This iterator yields words in accordance with the default word boundary
rules specified in
[UAX #29](https://www.unicode.org/reports/tr29/tr29-33.html#Word_Boundaries).
In particular, this may not be suitable for Japanese and Chinese scripts
that do not use spaces between words.

---
