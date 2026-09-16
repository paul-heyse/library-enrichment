# `pyrefly::error::legacy`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.error.legacy.json`](../model/pyrefly.error.legacy.json)

## BaselineError

`struct` · `pyrefly::error::legacy::BaselineError`

```rust
struct BaselineError
```

**Fields**: `column`, `path`, `name`, `concise_description`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## LegacyError

`struct` · `pyrefly::error::legacy::LegacyError`

Also reachable as `pyrefly::library::library::library::library::LegacyError`

```rust
struct LegacyError
```

**Fields**: `column`, `path`, `name`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn from_error(relative_to: &Path, error: &Error) -> Self
fn severity(&self) -> Severity
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Legacy error structure in Pyre1. Needs to be consistent with the following file:
<https://www.internalfb.com/code/fbsource/fbcode/tools/pyre/facebook/arc/lib/error.rs>

Used to serialize errors in a Pyre1-compatible format.

---
