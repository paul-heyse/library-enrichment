# `ruff_notebook::index`

Crate `ruff_notebook` · 2 public items · structured records in [`model/ruff_notebook.index.json`](../model/ruff_notebook.index.json)

## CellStart

`struct` · `ruff_notebook::index::CellStart`

Also reachable as `ruff_notebook::CellStart`

```rust
struct CellStart
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn cell_index(&self) -> OneIndexed
fn start_row(&self) -> OneIndexed
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## NotebookIndex

`struct` · `ruff_notebook::index::NotebookIndex`

Also reachable as `ruff_notebook::NotebookIndex`

```rust
struct NotebookIndex
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn cell(&self, row: OneIndexed) -> Option<OneIndexed>
fn cell_row(&self, row: OneIndexed) -> Option<OneIndexed>
fn iter(&self) -> impl Iterator<Item = CellStart> + '_
fn translate_line_column(&self, source_location: &LineColumn) -> LineColumn
fn translate_source_location(&self, source_location: &SourceLocation) -> SourceLocation
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Jupyter Notebook indexing table

When we lint a jupyter notebook, we have to translate the row/column based on
[`ruff_text_size::TextSize`] to jupyter notebook cell/row/column.

---
