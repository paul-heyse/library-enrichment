# `buoyant_kernel::column_trie`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.column_trie.json`](../model/buoyant_kernel.column_trie.json)

## ColumnTrie

`struct` · `buoyant_kernel::column_trie::ColumnTrie`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.column_trie.ColumnTrie.md)

Also reachable as `delta_kernel::column_trie::ColumnTrie`

```rust
struct ColumnTrie<'col>
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn contains_prefix_of(&self, path: &[String]) -> bool
fn from_columns(columns: &'col [ColumnName]) -> Self
fn insert(&mut self, column: &'col ColumnName)
fn is_terminal(&self, path: &[String]) -> bool
fn new() -> Self
```

A trie (prefix tree) for efficient column path matching.

The lifetime `'col` ties this trie to the column names it was built from,
allowing it to borrow string slices instead of cloning.

The `Default` implementation creates an empty trie node with no children and
`is_terminal = false`. This is used both for creating a new root trie and for
creating intermediate nodes during insertion (via `or_default()`).

---
