# `bstr::utf8`

Crate `bstr` · 7 public items · structured records in [`model/bstr.utf8.json`](../model/bstr.utf8.json)

## decode

`function` · `bstr::utf8::decode`

Also reachable as `bstr::decode_utf8`

```rust
fn decode<B: AsRef<[u8]>>(slice: B) -> (Option<char>, usize)
```

UTF-8 decode a single Unicode scalar value from the beginning of a slice.

When successful, the corresponding Unicode scalar value is returned along
with the number of bytes it was encoded with. The number of bytes consumed
for a successful decode is always between 1 and 4, inclusive.

When unsuccessful, `None` is returned along with the number of bytes that
make up a maximal prefix of a valid UTF-8 code unit sequence. When there is
no prefix of a valid UTF-8 code unit sequence, then 1 byte is consumed.
Thus, for a non-empty slice given, the number of bytes consumed is always
at least `1`. `0` is only returned when `slice` is empty.

# Examples

Basic usage:

```
use bstr::decode_utf8;

// Decoding a valid codepoint.
let (ch, size) = decode_utf8(b"\xE2\x98\x83");
assert_eq!(Some('☃'), ch);
assert_eq!(3, size);

// Decoding an incomplete codepoint.
let (ch, size) = decode_utf8(b"\xE2\x98");
assert_eq!(None, ch);
assert_eq!(2, size);
```

This example shows how to iterate over all codepoints in UTF-8 encoded
bytes, while replacing invalid UTF-8 sequences with the replacement
codepoint:

```
use bstr::{B, decode_utf8};

let mut bytes = B(b"\xE2\x98\x83\xFF\xF0\x9D\x9E\x83\xE2\x98\x61");
let mut chars = vec![];
while !bytes.is_empty() {
    let (ch, size) = decode_utf8(bytes);
    bytes = &bytes[size..];
    chars.push(ch.unwrap_or('\u{FFFD}'));
}
assert_eq!(vec!['☃', '\u{FFFD}', '𝞃', '\u{FFFD}', 'a'], chars);
```

---

## decode_last

`function` · `bstr::utf8::decode_last`

Also reachable as `bstr::decode_last_utf8`

```rust
fn decode_last<B: AsRef<[u8]>>(slice: B) -> (Option<char>, usize)
```

UTF-8 decode a single Unicode scalar value from the end of a slice.

When successful, the corresponding Unicode scalar value is returned along
with the number of bytes it was encoded with. The number of bytes consumed
for a successful decode is always between 1 and 4, inclusive.

When unsuccessful, `None` is returned along with the number of bytes that
make up a maximal prefix of a valid UTF-8 code unit sequence. In this case,
the number of bytes consumed is always between 0 and 3, inclusive, where
0 is only returned when `slice` is empty.

# Examples

Basic usage:

```
use bstr::decode_last_utf8;

// Decoding a valid codepoint.
let (ch, size) = decode_last_utf8(b"\xE2\x98\x83");
assert_eq!(Some('☃'), ch);
assert_eq!(3, size);

// Decoding an incomplete codepoint.
let (ch, size) = decode_last_utf8(b"\xE2\x98");
assert_eq!(None, ch);
assert_eq!(2, size);
```

This example shows how to iterate over all codepoints in UTF-8 encoded
bytes in reverse, while replacing invalid UTF-8 sequences with the
replacement codepoint:

```
use bstr::{B, decode_last_utf8};

let mut bytes = B(b"\xE2\x98\x83\xFF\xF0\x9D\x9E\x83\xE2\x98\x61");
let mut chars = vec![];
while !bytes.is_empty() {
    let (ch, size) = decode_last_utf8(bytes);
    bytes = &bytes[..bytes.len()-size];
    chars.push(ch.unwrap_or('\u{FFFD}'));
}
assert_eq!(vec!['a', '\u{FFFD}', '𝞃', '\u{FFFD}', '☃'], chars);
```

---

## CharIndices

`struct` · `bstr::utf8::CharIndices`

Also reachable as `bstr::CharIndices`

```rust
struct CharIndices<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<(usize, usize, char)>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<(usize, usize, char)>
```

An iterator over Unicode scalar values in a byte string and their
byte index positions.

When invalid UTF-8 byte sequences are found, they are substituted with the
Unicode replacement codepoint (`U+FFFD`) using the
["maximal subpart" strategy](https://www.unicode.org/review/pr-121.html).

Note that this is slightly different from the `CharIndices` iterator
provided by the standard library. Aside from working on possibly invalid
UTF-8, this iterator provides both the corresponding starting and ending
byte indices of each codepoint yielded. The ending position is necessary to
slice the original byte string when invalid UTF-8 bytes are converted into
a Unicode replacement codepoint, since a single replacement codepoint can
substitute anywhere from 1 to 3 invalid bytes (inclusive).

This iterator is created by the
[`char_indices`](trait.ByteSlice.html#method.char_indices) method provided
by the [`ByteSlice`](trait.ByteSlice.html) extension trait for `&[u8]`.

---

## Chars

`struct` · `bstr::utf8::Chars`

Also reachable as `bstr::Chars`

```rust
struct Chars<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_bytes(&self) -> &'a [u8]
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<char>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<char>
```

An iterator over Unicode scalar values in a byte string.

When invalid UTF-8 byte sequences are found, they are substituted with the
Unicode replacement codepoint (`U+FFFD`) using the
["maximal subpart" strategy](https://www.unicode.org/review/pr-121.html).

This iterator is created by the
[`chars`](trait.ByteSlice.html#method.chars) method provided by the
[`ByteSlice`](trait.ByteSlice.html) extension trait for `&[u8]`.

---

## Utf8Chunk

`struct` · `bstr::utf8::Utf8Chunk`

Also reachable as `bstr::Utf8Chunk`

```rust
struct Utf8Chunk<'a>
```

**Methods** (3)

```rust
fn incomplete(&self) -> bool
fn invalid(&self) -> &'a [u8]
fn valid(&self) -> &'a str
```

A chunk of valid UTF-8, possibly followed by invalid UTF-8 bytes.

This is yielded by the
[`Utf8Chunks`](struct.Utf8Chunks.html)
iterator, which can be created via the
[`ByteSlice::utf8_chunks`](trait.ByteSlice.html#method.utf8_chunks)
method.

The `'a` lifetime parameter corresponds to the lifetime of the bytes that
are being iterated over.

---

## Utf8Chunks

`struct` · `bstr::utf8::Utf8Chunks`

Also reachable as `bstr::Utf8Chunks`

```rust
struct Utf8Chunks<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Utf8Chunk<'a>>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over chunks of valid UTF-8 in a byte slice.

See [`utf8_chunks`](trait.ByteSlice.html#method.utf8_chunks).

---

## Utf8Error

`struct` · `bstr::utf8::Utf8Error`

Also reachable as `bstr::Utf8Error`

```rust
struct Utf8Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn error_len(&self) -> Option<usize>
fn valid_up_to(&self) -> usize
```

**via `core::error::Error`**

```rust
fn description(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

An error that occurs when UTF-8 decoding fails.

This error occurs when attempting to convert a non-UTF-8 byte
string to a Rust string that must be valid UTF-8. For example,
[`to_str`](trait.ByteSlice.html#method.to_str) is one such method.

# Example

This example shows what happens when a given byte sequence is invalid,
but ends with a sequence that is a possible prefix of valid UTF-8.

```
use bstr::{B, ByteSlice};

let s = B(b"foobar\xF1\x80\x80");
let err = s.to_str().unwrap_err();
assert_eq!(err.valid_up_to(), 6);
assert_eq!(err.error_len(), None);
```

This example shows what happens when a given byte sequence contains
invalid UTF-8.

```
use bstr::ByteSlice;

let s = b"foobar\xF1\x80\x80quux";
let err = s.to_str().unwrap_err();
assert_eq!(err.valid_up_to(), 6);
// The error length reports the maximum number of bytes that correspond to
// a valid prefix of a UTF-8 encoded codepoint.
assert_eq!(err.error_len(), Some(3));

// In contrast to the above which contains a single invalid prefix,
// consider the case of multiple individual bytes that are never valid
// prefixes. Note how the value of error_len changes!
let s = b"foobar\xFF\xFFquux";
let err = s.to_str().unwrap_err();
assert_eq!(err.valid_up_to(), 6);
assert_eq!(err.error_len(), Some(1));

// The fact that it's an invalid prefix does not change error_len even
// when it immediately precedes the end of the string.
let s = b"foobar\xFF";
let err = s.to_str().unwrap_err();
assert_eq!(err.valid_up_to(), 6);
assert_eq!(err.error_len(), Some(1));
```

---
