# `ruff_linter::source_kind`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.source_kind.json`](../model/ruff_linter.source_kind.json)

## SourceError

`enum` · `ruff_linter::source_kind::SourceError`

```rust
enum SourceError
```

**Variants**: `Io`, `Notebook`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: io::Error) -> Self
fn from(source: NotebookError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## SourceKind

`enum` · `ruff_linter::source_kind::SourceKind`

```rust
enum SourceKind
```

**Variants**: `Python`, `IpyNotebook`, `Markdown`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (13)

```rust
fn as_ipy_notebook(&self) -> Option<&Notebook>
fn as_markdown(&self) -> Option<&str>
fn as_python(&self) -> Option<&str>
fn diff<'a>(&'a self, other: &'a Self, path: Option<&'a Path>) -> Option<SourceKindDiff<'a>>
fn expect_ipy_notebook(self) -> Notebook
fn expect_markdown(self) -> String
fn expect_python(self) -> String
fn from_path(path: &Path, source_type: SourceType) -> Result<Option<Self>, SourceError>
fn from_source_code(source_code: String, source_type: SourceType) -> Result<Option<Self>, SourceError>
fn ipy_notebook(notebook: Notebook) -> Self
fn py_source_type(&self) -> PySourceType
fn source_code(&self) -> &str
fn write(&self, writer: &mut dyn Write) -> Result<(), SourceError>
```

---

## SourceKindDiff

`struct` · `ruff_linter::source_kind::SourceKindDiff`

```rust
struct SourceKindDiff<'a>
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn from_text(original: &'a str, modified: &'a str, path: Option<&'a Path>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

---
