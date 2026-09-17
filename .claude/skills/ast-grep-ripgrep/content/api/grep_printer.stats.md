# `grep_printer::stats`

Crate `grep-printer` · 1 public items · structured records in [`model/grep_printer.stats.json`](../model/grep_printer.stats.json)

## Stats

`struct` · `grep_printer::stats::Stats`

Also reachable as `grep_printer::Stats`

```rust
struct Stats
```

**Implements**: `core::ops::arith::Add`, `core::ops::arith::AddAssign`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (15)

```rust
fn add_bytes_printed(&mut self, n: u64)
fn add_bytes_searched(&mut self, n: u64)
fn add_elapsed(&mut self, duration: Duration)
fn add_matched_lines(&mut self, n: u64)
fn add_matches(&mut self, n: u64)
fn add_searches(&mut self, n: u64)
fn add_searches_with_match(&mut self, n: u64)
fn bytes_printed(&self) -> u64
fn bytes_searched(&self) -> u64
fn elapsed(&self) -> Duration
fn matched_lines(&self) -> u64
fn matches(&self) -> u64
fn new() -> Stats
fn searches(&self) -> u64
fn searches_with_match(&self) -> u64
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: &'a Stats) -> Stats
fn add(self, rhs: Stats) -> Stats
```

**via `core::ops::arith::AddAssign`**

```rust
fn add_assign(&mut self, rhs: Stats)
fn add_assign(&mut self, rhs: &'a Stats)
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error>
```

Summary statistics produced at the end of a search.

When statistics are reported by a printer, they correspond to all searches
executed with that printer.

---
