# `memchr::arch::all::shiftor`

Crate `memchr` · 1 public items · structured records in [`model/memchr.arch.all.shiftor.json`](../model/memchr.arch.all.shiftor.json)

## Finder

`struct` · `memchr::arch::all::shiftor::Finder`

```rust
struct Finder
```

**Derives**: Debug

**Methods** (2)

```rust
fn find(&self, haystack: &[u8]) -> Option<usize>
fn new(needle: &[u8]) -> Option<Finder>
```

A forward substring searcher using the Shift-Or algorithm.

---
