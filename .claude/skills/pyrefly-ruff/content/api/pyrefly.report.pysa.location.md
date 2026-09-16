# `pyrefly::report::pysa::location`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.report.pysa.location.json`](../model/pyrefly.report.pysa.location.json)

## PysaLocation

`struct` · `pyrefly::report::pysa::location::PysaLocation`

```rust
struct PysaLocation
```

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn as_key(&self) -> String
fn col(&self) -> u32
fn end_col(&self) -> u32
fn end_line(&self) -> u32
fn from_text_range(location: TextRange, module: &pyrefly_python::module::Module) -> Self
fn line(&self) -> u32
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

---
