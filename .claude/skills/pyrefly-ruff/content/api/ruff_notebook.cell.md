# `ruff_notebook::cell`

Crate `ruff_notebook` · 1 public items · structured records in [`model/ruff_notebook.cell.json`](../model/ruff_notebook.cell.json)

## CellOffsets

`struct` · `ruff_notebook::cell::CellOffsets`

Also reachable as `ruff_notebook::CellOffsets`

```rust
struct CellOffsets
```

**Implements**: `core::ops::deref::Deref`, `core::ops::deref::DerefMut`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn containing_range(&self, offset: TextSize) -> Option<TextRange>
fn content_ranges(&self) -> impl Iterator<Item = TextRange>
fn has_cell_boundary(&self, range: TextRange) -> bool
fn ranges(&self) -> impl Iterator<Item = TextRange>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

Cell offsets are used to keep track of the start and end offsets of each
cell in the concatenated source code. These offsets are in sorted order.

---
