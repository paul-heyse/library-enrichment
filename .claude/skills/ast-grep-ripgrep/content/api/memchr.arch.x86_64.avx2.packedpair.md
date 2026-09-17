# `memchr::arch::x86_64::avx2::packedpair`

Crate `memchr` · 1 public items · structured records in [`model/memchr.arch.x86_64.avx2.packedpair.json`](../model/memchr.arch.x86_64.avx2.packedpair.json)

## Finder

`struct` · `memchr::arch::x86_64::avx2::packedpair::Finder`

```rust
struct Finder
```

**Derives**: Clone, Copy, Debug

**Methods** (7)

```rust
fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>
fn find_prefilter(&self, haystack: &[u8]) -> Option<usize>
fn is_available() -> bool
fn min_haystack_len(&self) -> usize
fn new(needle: &[u8]) -> Option<Finder>
fn pair(&self) -> &Pair
fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>
```

A "packed pair" finder that uses 256-bit vector operations.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`] of bytes given matches.

---
