# `memchr::arch::all::memchr`

Crate `memchr` · 6 public items · structured records in [`model/memchr.arch.all.memchr.json`](../model/memchr.arch.all.memchr.json)

## One

`struct` · `memchr::arch::all::memchr::One`

```rust
struct One
```

**Derives**: Clone, Copy, Debug

**Methods** (8)

```rust
fn count(&self, haystack: &[u8]) -> usize
unsafe fn count_raw(&self, start: *const u8, end: *const u8) -> usize
fn find(&self, haystack: &[u8]) -> Option<usize>
unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>
fn iter<'a, 'h>(&'a self, haystack: &'h [u8]) -> OneIter<'a, 'h>
fn new(needle: u8) -> One
fn rfind(&self, haystack: &[u8]) -> Option<usize>
unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>
```

Finds all occurrences of a single byte in a haystack.

---

## OneIter

`struct` · `memchr::arch::all::memchr::OneIter`

```rust
struct OneIter<'a, 'h>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

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

This iterator is created by the [`One::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

---

## Three

`struct` · `memchr::arch::all::memchr::Three`

```rust
struct Three
```

**Derives**: Clone, Copy, Debug

**Methods** (6)

```rust
fn find(&self, haystack: &[u8]) -> Option<usize>
unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>
fn iter<'a, 'h>(&'a self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>
fn new(needle1: u8, needle2: u8, needle3: u8) -> Three
fn rfind(&self, haystack: &[u8]) -> Option<usize>
unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>
```

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

---

## ThreeIter

`struct` · `memchr::arch::all::memchr::ThreeIter`

```rust
struct ThreeIter<'a, 'h>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

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

This iterator is created by the [`Three::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

---

## Two

`struct` · `memchr::arch::all::memchr::Two`

```rust
struct Two
```

**Derives**: Clone, Copy, Debug

**Methods** (6)

```rust
fn find(&self, haystack: &[u8]) -> Option<usize>
unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>
fn iter<'a, 'h>(&'a self, haystack: &'h [u8]) -> TwoIter<'a, 'h>
fn new(needle1: u8, needle2: u8) -> Two
fn rfind(&self, haystack: &[u8]) -> Option<usize>
unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

---

## TwoIter

`struct` · `memchr::arch::all::memchr::TwoIter`

```rust
struct TwoIter<'a, 'h>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

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

This iterator is created by the [`Two::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

---
