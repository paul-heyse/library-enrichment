# `memchr::arch::all::twoway`

Crate `memchr` · 2 public items · structured records in [`model/memchr.arch.all.twoway.json`](../model/memchr.arch.all.twoway.json)

## Finder

`struct` · `memchr::arch::all::twoway::Finder`

```rust
struct Finder
```

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>
fn new(needle: &[u8]) -> Finder
```

A forward substring searcher that uses the Two-Way algorithm.

---

## FinderRev

`struct` · `memchr::arch::all::twoway::FinderRev`

```rust
struct FinderRev
```

**Derives**: Clone, Copy, Debug

**Methods** (2)

```rust
fn new(needle: &[u8]) -> FinderRev
fn rfind(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

A reverse substring searcher that uses the Two-Way algorithm.

---
