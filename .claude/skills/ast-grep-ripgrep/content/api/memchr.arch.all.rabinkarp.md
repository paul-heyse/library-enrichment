# `memchr::arch::all::rabinkarp`

Crate `memchr` · 2 public items · structured records in [`model/memchr.arch.all.rabinkarp.json`](../model/memchr.arch.all.rabinkarp.json)

## Finder

`struct` · `memchr::arch::all::rabinkarp::Finder`

```rust
struct Finder
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>
unsafe fn find_raw(&self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>
fn new(needle: &[u8]) -> Finder
```

A forward substring searcher using the Rabin-Karp algorithm.

Note that, as a lower level API, a `Finder` does not have access to the
needle it was constructed with. For this reason, executing a search
with a `Finder` requires passing both the needle and the haystack,
where the needle is exactly equivalent to the one given to the `Finder`
at construction time. This design was chosen so that callers can have
more precise control over where and how many times a needle is stored.
For example, in cases where Rabin-Karp is just one of several possible
substring search algorithms.

---

## FinderRev

`struct` · `memchr::arch::all::rabinkarp::FinderRev`

```rust
struct FinderRev
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn new(needle: &[u8]) -> FinderRev
fn rfind(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>
unsafe fn rfind_raw(&self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>
```

A reverse substring searcher using the Rabin-Karp algorithm.

---
