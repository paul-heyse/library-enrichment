# `ruff_python_ast::python_version`

Crate `ruff_python_ast` · 2 public items · structured records in [`model/ruff_python_ast.python_version.json`](../model/ruff_python_ast.python_version.json)

## PythonVersionDeserializationError

`enum` · `ruff_python_ast::python_version::PythonVersionDeserializationError`

Also reachable as `ruff_python_ast::PythonVersionDeserializationError`

```rust
enum PythonVersionDeserializationError
```

**Variants**: `WrongPeriodNumber`, `InvalidMajorVersion`, `InvalidMinorVersion`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## PythonVersion

`struct` · `ruff_python_ast::python_version::PythonVersion`

Also reachable as `ruff_python_ast::PythonVersion`

```rust
struct PythonVersion
```

**Fields**: `major`, `minor`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (9)

```rust
const fn as_tuple(self) -> (u8, u8)
fn defers_annotations(self) -> bool
fn free_threaded_build_available(self) -> bool
fn iter() -> impl Iterator<Item = PythonVersion>
const fn latest() -> Self
fn latest_preview() -> Self
const fn latest_ty() -> Self
const fn lowest() -> Self
fn supports_pep_701(self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: (u8, u8)) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: (i64, i64)) -> Result<Self, Self::Error>
fn try_from(value: (&str, &str)) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Representation of a Python version.

N.B. This does not necessarily represent a Python version that we actually support.

---
