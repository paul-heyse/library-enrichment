# `ruff_python_ast`

Crate `ruff_python_ast` · 3 public items · structured records in [`model/ruff_python_ast.json`](../model/ruff_python_ast.json)

## PySourceType

`enum` · `ruff_python_ast::PySourceType`

```rust
enum PySourceType
```

**Variants**: `Python`, `Stub`, `Ipynb`

**Implements**: `core::convert::From`, `ruff_python_parser::AsMode`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn from_extension(extension: &str) -> Self
const fn is_ipynb(self) -> bool
const fn is_py_file(self) -> bool
const fn is_py_file_or_stub(self) -> bool
const fn is_stub(self) -> bool
fn try_from_extension(extension: &str) -> Option<Self>
fn try_from_path(path: impl AsRef<Path>) -> Option<Self>
```

**via `core::convert::From`**

```rust
fn from(path: P) -> Self
```

---

## SourceType

`enum` · `ruff_python_ast::SourceType`

```rust
enum SourceType
```

**Variants**: `Python`, `Toml`, `Markdown`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn as_mut_python(&mut self) -> Option<&mut PySourceType>
fn as_mut_toml(&mut self) -> Option<&mut TomlSourceType>
fn as_python(&self) -> Option<&PySourceType>
fn as_toml(&self) -> Option<&TomlSourceType>
fn expect_python(self) -> PySourceType where Self: ::std::fmt::Debug
fn expect_toml(self) -> TomlSourceType where Self: ::std::fmt::Debug
fn from_extension(ext: &str) -> Self
const fn is_markdown(&self) -> bool
const fn is_python(&self) -> bool
const fn is_toml(&self) -> bool
fn python(self) -> Option<PySourceType>
fn toml(self) -> Option<TomlSourceType>
```

**via `core::convert::From`**

```rust
fn from(path: P) -> Self
```

The type of a source file.

---

## TomlSourceType

`enum` · `ruff_python_ast::TomlSourceType`

```rust
enum TomlSourceType
```

**Variants**: `Pyproject`, `Ruff`, `Pipfile`, `Poetry`, `Unrecognized`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
const fn is_pipfile(&self) -> bool
const fn is_poetry(&self) -> bool
const fn is_pyproject(&self) -> bool
const fn is_ruff(&self) -> bool
const fn is_unrecognized(&self) -> bool
```

---
