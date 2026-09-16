# `ruff_diagnostics::edit`

Crate `ruff_diagnostics` · 1 public items · structured records in [`model/ruff_diagnostics.edit.json`](../model/ruff_diagnostics.edit.json)

## Edit

`struct` · `ruff_diagnostics::edit::Edit`

Also reachable as `ruff_diagnostics::Edit`

```rust
struct Edit
```

**Implements**: `get_size2::GetSize`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (10)

```rust
fn content(&self) -> Option<&str>
const fn deletion(start: TextSize, end: TextSize) -> Self
fn insertion(content: String, at: TextSize) -> Self
fn into_content(self) -> Option<Box<str>>
fn is_deletion(&self) -> bool
fn is_insertion(&self) -> bool
fn is_replacement(&self) -> bool
const fn range_deletion(range: TextRange) -> Self
fn range_replacement(content: String, range: TextRange) -> Self
fn replacement(content: String, start: TextSize, end: TextSize) -> Self
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A text edit to be applied to a source file. Inserts, deletes, or replaces
content at a given location.

---
