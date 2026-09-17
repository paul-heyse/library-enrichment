# `regex_syntax::utf8`

Crate `regex-syntax` · 3 public items · structured records in [`model/regex_syntax.utf8.json`](../model/regex_syntax.utf8.json)

## Utf8Sequence

`enum` · `regex_syntax::utf8::Utf8Sequence`

```rust
enum Utf8Sequence
```

**Variants**: `One`, `Two`, `Three`, `Four`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_slice(&self) -> &[Utf8Range]
fn len(&self) -> usize
fn matches(&self, bytes: &[u8]) -> bool
fn reverse(&mut self)
```

Utf8Sequence represents a sequence of byte ranges.

To match a Utf8Sequence, a candidate byte sequence must match each
successive range.

For example, if there are two ranges, `[C2-DF][80-BF]`, then the byte
sequence `\xDD\x61` would not match because `0x61 < 0x80`.

---

## Utf8Range

`struct` · `regex_syntax::utf8::Utf8Range`

```rust
struct Utf8Range
```

**Fields**: `start`, `end`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn matches(&self, b: u8) -> bool
```

A single inclusive range of UTF-8 bytes.

---

## Utf8Sequences

`struct` · `regex_syntax::utf8::Utf8Sequences`

```rust
struct Utf8Sequences
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Debug

**Methods** (1)

```rust
fn new(start: char, end: char) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator over ranges of matching UTF-8 byte sequences.

The iteration represents an alternation of comprehensive byte sequences
that match precisely the set of UTF-8 encoded scalar values.

A byte sequence corresponds to one of the scalar values in the range given
if and only if it completely matches exactly one of the sequences of byte
ranges produced by this iterator.

Each sequence of byte ranges matches a unique set of bytes. That is, no two
sequences will match the same bytes.

# Example

This shows how to match an arbitrary byte sequence against a range of
scalar values.

```rust
use regex_syntax::utf8::{Utf8Sequences, Utf8Sequence};

fn matches(seqs: &[Utf8Sequence], bytes: &[u8]) -> bool {
    for range in seqs {
        if range.matches(bytes) {
            return true;
        }
    }
    false
}

// Test the basic multilingual plane.
let seqs: Vec<_> = Utf8Sequences::new('\u{0}', '\u{FFFF}').collect();

// UTF-8 encoding of 'a'.
assert!(matches(&seqs, &[0x61]));
// UTF-8 encoding of '☃' (`\u{2603}`).
assert!(matches(&seqs, &[0xE2, 0x98, 0x83]));
// UTF-8 encoding of `\u{10348}` (outside the BMP).
assert!(!matches(&seqs, &[0xF0, 0x90, 0x8D, 0x88]));
// Tries to match against a UTF-8 encoding of a surrogate codepoint,
// which is invalid UTF-8, and therefore fails, despite the fact that
// the corresponding codepoint (0xD800) falls in the range given.
assert!(!matches(&seqs, &[0xED, 0xA0, 0x80]));
// And fails against plain old invalid UTF-8.
assert!(!matches(&seqs, &[0xFF, 0xFF]));
```

If this example seems circuitous, that's because it is! It's meant to be
illustrative. In practice, you could just try to decode your byte sequence
and compare it with the scalar value range directly. However, this is not
always possible (for example, in a byte based automaton).

---
