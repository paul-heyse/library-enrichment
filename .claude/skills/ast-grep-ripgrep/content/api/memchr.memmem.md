# `memchr::memmem`

Crate `memchr` · 9 public items · structured records in [`model/memchr.memmem.json`](../model/memchr.memmem.json)

## find

`function` · `memchr::memmem::find`

```rust
fn find(haystack: &[u8], needle: &[u8]) -> Option<usize>
```

Returns the index of the first occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`Finder`] once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::find(haystack, b"foo"));
assert_eq!(Some(4), memmem::find(haystack, b"bar"));
assert_eq!(None, memmem::find(haystack, b"quux"));
```

---

## find_iter

`function` · `memchr::memmem::find_iter`

```rust
fn find_iter<'h, 'n, N: 'n + ?Sized + AsRef<[u8]>>(haystack: &'h [u8], needle: &'n N) -> FindIter<'h, 'n>
```

Returns an iterator over all non-overlapping occurrences of a substring in
a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::find_iter(haystack, b"foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

---

## rfind

`function` · `memchr::memmem::rfind`

```rust
fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize>
```

Returns the index of the last occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`FinderRev`] once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::rfind(haystack, b"foo"));
assert_eq!(Some(4), memmem::rfind(haystack, b"bar"));
assert_eq!(Some(8), memmem::rfind(haystack, b"ba"));
assert_eq!(None, memmem::rfind(haystack, b"quux"));
```

---

## rfind_iter

`function` · `memchr::memmem::rfind_iter`

```rust
fn rfind_iter<'h, 'n, N: 'n + ?Sized + AsRef<[u8]>>(haystack: &'h [u8], needle: &'n N) -> FindRevIter<'h, 'n>
```

Returns a reverse iterator over all non-overlapping occurrences of a
substring in a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::rfind_iter(haystack, b"foo");
assert_eq!(Some(16), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(0), it.next());
assert_eq!(None, it.next());
```

---

## FindIter

`struct` · `memchr::memmem::FindIter`

```rust
struct FindIter<'h, 'n>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn into_owned(self) -> FindIter<'h, 'static>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over non-overlapping substring matches.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

---

## FindRevIter

`struct` · `memchr::memmem::FindRevIter`

```rust
struct FindRevIter<'h, 'n>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn into_owned(self) -> FindRevIter<'h, 'static>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
```

An iterator over non-overlapping substring matches in reverse.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

---

## Finder

`struct` · `memchr::memmem::Finder`

```rust
struct Finder<'n>
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn as_ref(&self) -> Finder<'_>
fn find(&self, haystack: &[u8]) -> Option<usize>
fn find_iter<'a, 'h>(&'a self, haystack: &'h [u8]) -> FindIter<'h, 'a>
fn into_owned(self) -> Finder<'static>
fn needle(&self) -> &[u8]
fn new<B: ?Sized + AsRef<[u8]>>(needle: &'n B) -> Finder<'n>
```

A single substring searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general, using
[`find`] is good enough, but `Finder` is useful when you can meaningfully
observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `Finder` that is not connected to
the lifetime of its needle.

---

## FinderBuilder

`struct` · `memchr::memmem::FinderBuilder`

```rust
struct FinderBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn build_forward<'n, B: ?Sized + AsRef<[u8]>>(&self, needle: &'n B) -> Finder<'n>
fn build_forward_owned<B: Into<alloc::boxed::Box<[u8]>>>(&self, needle: B) -> Finder<'static>
fn build_forward_with_ranker<'n, R: HeuristicFrequencyRank, B: ?Sized + AsRef<[u8]>>(&self, ranker: R, needle: &'n B) -> Finder<'n>
fn build_forward_with_ranker_owned<R: HeuristicFrequencyRank, B: Into<alloc::boxed::Box<[u8]>>>(&self, ranker: R, needle: B) -> Finder<'static>
fn build_reverse<'n, B: ?Sized + AsRef<[u8]>>(&self, needle: &'n B) -> FinderRev<'n>
fn build_reverse_owned<B: Into<alloc::boxed::Box<[u8]>>>(&self, needle: B) -> FinderRev<'static>
fn new() -> FinderBuilder
fn prefilter(&mut self, prefilter: Prefilter) -> &mut FinderBuilder
```

A builder for constructing non-default forward or reverse memmem finders.

A builder is primarily useful for configuring a substring searcher.
Currently, the only configuration exposed is the ability to disable
heuristic prefilters used to speed up certain searches.

---

## FinderRev

`struct` · `memchr::memmem::FinderRev`

```rust
struct FinderRev<'n>
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn as_ref(&self) -> FinderRev<'_>
fn into_owned(self) -> FinderRev<'static>
fn needle(&self) -> &[u8]
fn new<B: ?Sized + AsRef<[u8]>>(needle: &'n B) -> FinderRev<'n>
fn rfind<B: AsRef<[u8]>>(&self, haystack: B) -> Option<usize>
fn rfind_iter<'a, 'h>(&'a self, haystack: &'h [u8]) -> FindRevIter<'h, 'a>
```

A single substring reverse searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general,
using [`rfind`] is good enough, but `FinderRev` is useful when you can
meaningfully observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `FinderRev` that is not connected to
the lifetime of its needle.

---
