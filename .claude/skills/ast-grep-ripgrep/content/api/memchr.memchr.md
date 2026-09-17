# `memchr::memchr`

Crate `memchr` · 15 public items · structured records in [`model/memchr.memchr.json`](../model/memchr.memchr.json)

## memchr

`function` · `memchr::memchr::memchr`

Also reachable as `memchr::memchr`

```rust
fn memchr(needle: u8, haystack: &[u8]) -> Option<usize>
```

Search for the first occurrence of a byte in a slice.

This returns the index corresponding to the first occurrence of `needle` in
`haystack`, or `None` if one is not found. If an index is returned, it is
guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().position(|&b| b == needle)`, this routine will attempt to
use highly optimized vector operations that can be an order of magnitude
faster (or more).

# Example

This shows how to find the first position of a byte in a byte string.

```
use memchr::memchr;

let haystack = b"the quick brown fox";
assert_eq!(memchr(b'k', haystack), Some(8));
```

---

## memchr2

`function` · `memchr::memchr::memchr2`

Also reachable as `memchr::memchr2`

```rust
fn memchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize>
```

Search for the first occurrence of two possible bytes in a haystack.

This returns the index corresponding to the first occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().position(|&b| b == needle1 || b == needle2)`, this routine
will attempt to use highly optimized vector operations that can be an order
of magnitude faster (or more).

# Example

This shows how to find the first position of one of two possible bytes in a
haystack.

```
use memchr::memchr2;

let haystack = b"the quick brown fox";
assert_eq!(memchr2(b'k', b'q', haystack), Some(4));
```

---

## memchr2_iter

`function` · `memchr::memchr::memchr2_iter`

Also reachable as `memchr::memchr2_iter`

```rust
fn memchr2_iter<'h>(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>
```

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

---

## memchr3

`function` · `memchr::memchr::memchr3`

Also reachable as `memchr::memchr3`

```rust
fn memchr3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize>
```

Search for the first occurrence of three possible bytes in a haystack.

This returns the index corresponding to the first occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().position(|&b| b == needle1 || b == needle2 || b == needle3)`,
this routine will attempt to use highly optimized vector operations that
can be an order of magnitude faster (or more).

# Example

This shows how to find the first position of one of three possible bytes in
a haystack.

```
use memchr::memchr3;

let haystack = b"the quick brown fox";
assert_eq!(memchr3(b'k', b'q', b'u', haystack), Some(4));
```

---

## memchr3_iter

`function` · `memchr::memchr::memchr3_iter`

Also reachable as `memchr::memchr3_iter`

```rust
fn memchr3_iter<'h>(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>
```

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

---

## memchr_iter

`function` · `memchr::memchr::memchr_iter`

Also reachable as `memchr::memchr_iter`

```rust
fn memchr_iter<'h>(needle: u8, haystack: &'h [u8]) -> Memchr<'h>
```

Returns an iterator over all occurrences of the needle in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

---

## memrchr

`function` · `memchr::memchr::memrchr`

Also reachable as `memchr::memrchr`

```rust
fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize>
```

Search for the last occurrence of a byte in a slice.

This returns the index corresponding to the last occurrence of `needle` in
`haystack`, or `None` if one is not found. If an index is returned, it is
guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().rposition(|&b| b == needle)`, this routine will attempt to
use highly optimized vector operations that can be an order of magnitude
faster (or more).

# Example

This shows how to find the last position of a byte in a byte string.

```
use memchr::memrchr;

let haystack = b"the quick brown fox";
assert_eq!(memrchr(b'o', haystack), Some(17));
```

---

## memrchr2

`function` · `memchr::memchr::memrchr2`

Also reachable as `memchr::memrchr2`

```rust
fn memrchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize>
```

Search for the last occurrence of two possible bytes in a haystack.

This returns the index corresponding to the last occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().rposition(|&b| b == needle1 || b == needle2)`, this
routine will attempt to use highly optimized vector operations that can be
an order of magnitude faster (or more).

# Example

This shows how to find the last position of one of two possible bytes in a
haystack.

```
use memchr::memrchr2;

let haystack = b"the quick brown fox";
assert_eq!(memrchr2(b'k', b'o', haystack), Some(17));
```

---

## memrchr2_iter

`function` · `memchr::memchr::memrchr2_iter`

Also reachable as `memchr::memrchr2_iter`

```rust
fn memrchr2_iter(needle1: u8, needle2: u8, haystack: &[u8]) -> core::iter::Rev<Memchr2<'_>>
```

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

---

## memrchr3

`function` · `memchr::memchr::memrchr3`

Also reachable as `memchr::memrchr3`

```rust
fn memrchr3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize>
```

Search for the last occurrence of three possible bytes in a haystack.

This returns the index corresponding to the last occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().rposition(|&b| b == needle1 || b == needle2 || b == needle3)`,
this routine will attempt to use highly optimized vector operations that
can be an order of magnitude faster (or more).

# Example

This shows how to find the last position of one of three possible bytes in
a haystack.

```
use memchr::memrchr3;

let haystack = b"the quick brown fox";
assert_eq!(memrchr3(b'k', b'o', b'n', haystack), Some(17));
```

---

## memrchr3_iter

`function` · `memchr::memchr::memrchr3_iter`

Also reachable as `memchr::memrchr3_iter`

```rust
fn memrchr3_iter(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> core::iter::Rev<Memchr3<'_>>
```

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

---

## memrchr_iter

`function` · `memchr::memchr::memrchr_iter`

Also reachable as `memchr::memrchr_iter`

```rust
fn memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<Memchr<'_>>
```

Returns an iterator over all occurrences of the needle in a haystack, in
reverse.

---

## Memchr

`struct` · `memchr::memchr::Memchr`

Also reachable as `memchr::Memchr`

```rust
struct Memchr<'h>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(needle1: u8, haystack: &'h [u8]) -> Memchr<'h>
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<usize>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn count(self) -> usize
fn next(&mut self) -> Option<usize>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr_iter`] or `[memrchr_iter`]
functions. It can also be created with the [`Memchr::new`] method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

---

## Memchr2

`struct` · `memchr::memchr::Memchr2`

Also reachable as `memchr::Memchr2`

```rust
struct Memchr2<'h>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<usize>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`] or `[memrchr2_iter`]
functions. It can also be created with the [`Memchr2::new`] method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

---

## Memchr3

`struct` · `memchr::memchr::Memchr3`

Also reachable as `memchr::Memchr3`

```rust
struct Memchr3<'h>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>
```

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<usize>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`] or `[memrchr2_iter`]
functions. It can also be created with the [`Memchr3::new`] method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

---
