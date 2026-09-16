# `ruff_db::source`

Crate `ruff_db` · 4 public items · structured records in [`model/ruff_db.source.json`](../model/ruff_db.source.json)

## SourceTextError

`enum` · `ruff_db::source::SourceTextError`

```rust
enum SourceTextError
```

**Variants**: `FailedToReadNotebook`, `FailedToReadFile`

**Implements**: `core::error::Error`, `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## line_index

`function` · `ruff_db::source::line_index`

```rust
fn line_index<'db>(db: &'db dyn Db, file: files::File) -> ruff_source_file::LineIndex
```

Computes the [`LineIndex`] for `file`.

---

## source_text

`function` · `ruff_db::source::source_text`

```rust
fn source_text<'db>(db: &'db dyn Db, file: files::File) -> SourceText
```

Reads the source text of a python text file (must be valid UTF8) or notebook.

---

## SourceText

`struct` · `ruff_db::source::SourceText`

```rust
struct SourceText
```

**Implements**: `core::ops::deref::Deref`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn as_notebook(&self) -> Option<&Notebook>
fn as_str(&self) -> &str
fn is_notebook(&self) -> bool
fn read_error(&self) -> Option<&SourceTextError>
fn to_bytes(&self) -> Cow<'_, [u8]>
fn with_text(&self, new_text: String, source_map: &SourceMap) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &str
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

The source text of a file containing python code.

The file containing the source text can either be a text file or a notebook.

Cheap cloneable in `O(1)`.

---
