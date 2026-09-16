# `ruff_notebook::notebook`

Crate `ruff_notebook` · 3 public items · structured records in [`model/ruff_notebook.notebook.json`](../model/ruff_notebook.notebook.json)

## NotebookError

`enum` · `ruff_notebook::notebook::NotebookError`

Also reachable as `ruff_notebook::NotebookError`

```rust
enum NotebookError
```

**Variants**: `Io`, `Json`, `InvalidJson`, `InvalidSchema`, `InvalidFormat`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: io::Error) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

An error that can occur while deserializing a Jupyter Notebook.

---

## round_trip

`function` · `ruff_notebook::notebook::round_trip`

Also reachable as `ruff_notebook::round_trip`

```rust
fn round_trip(path: &std::path::Path) -> anyhow::Result<String>
```

Run round-trip source code generation on a given Jupyter notebook file path.

---

## Notebook

`struct` · `ruff_notebook::notebook::Notebook`

Also reachable as `ruff_notebook::Notebook`

```rust
struct Notebook
```

**Derives**: Clone, Debug, Eq, PartialEq

**Methods** (15)

```rust
fn cell_offset(&self, cell: OneIndexed) -> Option<TextSize>
fn cell_offsets(&self) -> &CellOffsets
fn cell_range(&self, cell: OneIndexed) -> Option<TextRange>
fn cells(&self) -> &[Cell]
fn empty() -> Self
fn from_path(path: &Path) -> Result<Self, NotebookError>
fn from_raw_notebook(raw_notebook: RawNotebook, trailing_newline: bool) -> Result<Self, NotebookError>
fn from_source_code(source_code: &str) -> Result<Self, NotebookError>
fn index(&self) -> &NotebookIndex
fn into_index(self) -> NotebookIndex
fn is_python_notebook(&self) -> bool
fn source_code(&self) -> &str
fn trailing_newline(&self) -> bool
fn update(&mut self, source_map: &SourceMap, transformed: String)
fn write(&self, writer: &mut dyn Write) -> Result<(), NotebookError>
```

---
