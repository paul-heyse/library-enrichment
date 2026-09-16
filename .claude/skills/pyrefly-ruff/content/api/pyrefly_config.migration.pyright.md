# `pyrefly_config::migration::pyright`

Crate `pyrefly_config` · 2 public items · structured records in [`model/pyrefly_config.migration.pyright.json`](../model/pyrefly_config.migration.pyright.json)

## DiagnosticLevel

`enum` · `pyrefly_config::migration::pyright::DiagnosticLevel`

```rust
enum DiagnosticLevel
```

**Variants**: `None`, `Hint`, `Information`, `Warning`, `Error`

**Implements**: `serde_core::de::Deserialize`

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---

## DiagnosticLevelOrBool

`enum` · `pyrefly_config::migration::pyright::DiagnosticLevelOrBool`

```rust
enum DiagnosticLevelOrBool
```

**Variants**: `DiagnosticLevel`, `Bool`

**Implements**: `serde_core::de::Deserialize`

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

---
