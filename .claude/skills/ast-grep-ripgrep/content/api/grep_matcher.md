# `grep_matcher`

Crate `grep-matcher` · 8 public items · structured records in [`model/grep_matcher.json`](../model/grep_matcher.json)

## LineMatchKind

`enum` · `grep_matcher::LineMatchKind`

```rust
enum LineMatchKind
```

**Variants**: `Confirmed`, `Candidate`

**Derives**: Clone, Copy, Debug

The type of match for a line oriented matcher.

---

## ByteSet

`struct` · `grep_matcher::ByteSet`

```rust
struct ByteSet
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn add(&mut self, byte: u8)
fn add_all(&mut self, start: u8, end: u8)
fn contains(&self, byte: u8) -> bool
fn empty() -> ByteSet
fn full() -> ByteSet
fn remove(&mut self, byte: u8)
fn remove_all(&mut self, start: u8, end: u8)
```

A set of bytes.

In this crate, byte sets are used to express bytes that can never appear
anywhere in a match for a particular implementation of the `Matcher` trait.
Specifically, if such a set can be determined, then it's possible for
callers to perform additional operations on the basis that certain bytes
may never match.

For example, if a search is configured to possibly produce results that
span multiple lines but a caller provided pattern can never match across
multiple lines, then it may make sense to divert to more optimized line
oriented routines that don't need to handle the multi-line match case.

---

## LineTerminator

`struct` · `grep_matcher::LineTerminator`

```rust
struct LineTerminator
```

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn as_byte(&self) -> u8
fn as_bytes(&self) -> &[u8]
fn byte(byte: u8) -> LineTerminator
fn crlf() -> LineTerminator
fn is_crlf(&self) -> bool
fn is_suffix(&self, slice: &[u8]) -> bool
```

A line terminator.

A line terminator represents the end of a line. Generally, every line is
either "terminated" by the end of a stream or a specific byte (or sequence
of bytes).

Generally, a line terminator is a single byte, specifically, `\n`, on
Unix-like systems. On Windows, a line terminator is `\r\n` (referred to
as `CRLF` for `Carriage Return; Line Feed`).

The default line terminator is `\n` on all platforms.

---

## Match

`struct` · `grep_matcher::Match`

```rust
struct Match
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (9)

```rust
fn end(&self) -> usize
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(start: usize, end: usize) -> Match
fn offset(&self, amount: usize) -> Match
fn start(&self) -> usize
fn with_end(&self, end: usize) -> Match
fn with_start(&self, start: usize) -> Match
fn zero(offset: usize) -> Match
```

The type of a match.

The type of a match is a possibly empty range pointing to a contiguous
block of addressable memory.

Every `Match` is guaranteed to satisfy the invariant that `start <= end`.

# Indexing

This type is structurally identical to `std::ops::Range<usize>`, but
is a bit more ergonomic for dealing with match indices. In particular,
this type implements `Copy` and provides methods for building new `Match`
values based on old `Match` values. Finally, the invariant that `start`
is always less than or equal to `end` is enforced.

A `Match` can be used to slice a `&[u8]`, `&mut [u8]` or `&str` using
range notation. e.g.,

```
use grep_matcher::Match;

let m = Match::new(2, 5);
let bytes = b"abcdefghi";
assert_eq!(b"cde", &bytes[m]);
```

---

## NoCaptures

`struct` · `grep_matcher::NoCaptures`

```rust
struct NoCaptures
```

**Implements**: `grep_matcher::Captures`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new() -> NoCaptures
```

**via `grep_matcher::Captures`**

```rust
fn get(&self, _: usize) -> Option<Match>
fn len(&self) -> usize
```

NoCaptures provides an always-empty implementation of the `Captures` trait.

This type is useful for implementations of `Matcher` that don't support
capturing groups.

---

## NoError

`struct` · `grep_matcher::NoError`

```rust
struct NoError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `core::error::Error`**

```rust
fn description(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

NoError provides an error type for matchers that never produce errors.

This error type implements the `std::error::Error` and `std::fmt::Display`
traits for use in matcher implementations that can never produce errors.

The `std::fmt::Debug` and `std::fmt::Display` impls for this type panics.

---

## Captures

`trait` · `grep_matcher::Captures`

```rust
trait Captures
```

**Implementors** (3)

- `grep_matcher::NoCaptures`
- `grep_pcre2::matcher::RegexCaptures`
- `grep_regex::matcher::RegexCaptures`

**Methods** (5)

```rust
fn as_match(&self) -> Match
fn get(&self, i: usize) -> Option<Match>
fn interpolate<F>(&self, name_to_index: F, haystack: &[u8], replacement: &[u8], dst: &mut Vec<u8>) where F: FnMut(&str) -> Option<usize>
fn is_empty(&self) -> bool
fn len(&self) -> usize
```

A trait that describes implementations of capturing groups.

When a matcher supports capturing group extraction, then it is the
matcher's responsibility to provide an implementation of this trait.

Principally, this trait provides a way to access capturing groups
in a uniform way that does not require any specific representation.
Namely, different matcher implementations may require different in-memory
representations of capturing groups. This trait permits matchers to
maintain their specific in-memory representation.

Note that this trait explicitly does not provide a way to construct a new
capture value. Instead, it is the responsibility of a `Matcher` to build
one, which might require knowledge of the matcher's internal implementation
details.

---

## Matcher

`trait` · `grep_matcher::Matcher`

```rust
trait Matcher
```

**Implementors** (2)

- `grep_pcre2::matcher::RegexMatcher`
- `grep_regex::matcher::RegexMatcher`

**Methods** (25)

```rust
fn capture_count(&self) -> usize
fn capture_index(&self, _name: &str) -> Option<usize>
fn captures(&self, haystack: &[u8], caps: &mut Self::Captures) -> Result<bool, Self::Error>
fn captures_at(&self, _haystack: &[u8], _at: usize, _caps: &mut Self::Captures) -> Result<bool, Self::Error>
fn captures_iter<F>(&self, haystack: &[u8], caps: &mut Self::Captures, matched: F) -> Result<(), Self::Error> where F: FnMut(&Self::Captures) -> bool
fn captures_iter_at<F>(&self, haystack: &[u8], at: usize, caps: &mut Self::Captures, matched: F) -> Result<(), Self::Error> where F: FnMut(&Self::Captures) -> bool
fn find(&self, haystack: &[u8]) -> Result<Option<Match>, Self::Error>
fn find_at(&self, haystack: &[u8], at: usize) -> Result<Option<Match>, Self::Error>
fn find_candidate_line(&self, haystack: &[u8]) -> Result<Option<LineMatchKind>, Self::Error>
fn find_iter<F>(&self, haystack: &[u8], matched: F) -> Result<(), Self::Error> where F: FnMut(Match) -> bool
fn find_iter_at<F>(&self, haystack: &[u8], at: usize, matched: F) -> Result<(), Self::Error> where F: FnMut(Match) -> bool
fn is_match(&self, haystack: &[u8]) -> Result<bool, Self::Error>
fn is_match_at(&self, haystack: &[u8], at: usize) -> Result<bool, Self::Error>
fn line_terminator(&self) -> Option<LineTerminator>
fn new_captures(&self) -> Result<Self::Captures, Self::Error>
fn non_matching_bytes(&self) -> Option<&ByteSet>
fn replace<F>(&self, haystack: &[u8], dst: &mut Vec<u8>, append: F) -> Result<(), Self::Error> where F: FnMut(Match, &mut Vec<u8>) -> bool
fn replace_with_captures<F>(&self, haystack: &[u8], caps: &mut Self::Captures, dst: &mut Vec<u8>, append: F) -> Result<(), Self::Error> where F: FnMut(&Self::Captures, &mut Vec<u8>) -> bool
fn replace_with_captures_at<F>(&self, haystack: &[u8], at: usize, caps: &mut Self::Captures, dst: &mut Vec<u8>, append: F) -> Result<(), Self::Error> where F: FnMut(&Self::Captures, &mut Vec<u8>) -> bool
fn shortest_match(&self, haystack: &[u8]) -> Result<Option<usize>, Self::Error>
fn shortest_match_at(&self, haystack: &[u8], at: usize) -> Result<Option<usize>, Self::Error>
fn try_captures_iter<F, E>(&self, haystack: &[u8], caps: &mut Self::Captures, matched: F) -> Result<Result<(), E>, Self::Error> where F: FnMut(&Self::Captures) -> Result<bool, E>
fn try_captures_iter_at<F, E>(&self, haystack: &[u8], at: usize, caps: &mut Self::Captures, matched: F) -> Result<Result<(), E>, Self::Error> where F: FnMut(&Self::Captures) -> Result<bool, E>
fn try_find_iter<F, E>(&self, haystack: &[u8], matched: F) -> Result<Result<(), E>, Self::Error> where F: FnMut(Match) -> Result<bool, E>
fn try_find_iter_at<F, E>(&self, haystack: &[u8], at: usize, matched: F) -> Result<Result<(), E>, Self::Error> where F: FnMut(Match) -> Result<bool, E>
```

A matcher defines an interface for regular expression implementations.

While this trait is large, there are only two required methods that
implementors must provide: `find_at` and `new_captures`. If captures aren't
supported by your implementation, then `new_captures` can be implemented
with [`NoCaptures`]. If your implementation does support capture groups,
then you should also implement the other capture related methods, as
dictated by the documentation. Crucially, this includes `captures_at`.

The rest of the methods on this trait provide default implementations on
top of `find_at` and `new_captures`. It is not uncommon for implementations
to be able to provide faster variants of some methods; in those cases,
simply override the default implementation.

---
