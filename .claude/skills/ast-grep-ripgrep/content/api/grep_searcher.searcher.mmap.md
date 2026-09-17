# `grep_searcher::searcher::mmap`

Crate `grep-searcher` · 1 public items · structured records in [`model/grep_searcher.searcher.mmap.json`](../model/grep_searcher.searcher.mmap.json)

## MmapChoice

`struct` · `grep_searcher::searcher::mmap::MmapChoice`

Also reachable as `grep_searcher::MmapChoice`

```rust
struct MmapChoice
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
unsafe fn auto() -> MmapChoice
fn never() -> MmapChoice
```

Controls the strategy used for determining when to use memory maps.

If a searcher is called in circumstances where it is possible to use memory
maps, and memory maps are enabled, then it will attempt to do so if it
believes it will make the search faster.

By default, memory maps are disabled.

---
