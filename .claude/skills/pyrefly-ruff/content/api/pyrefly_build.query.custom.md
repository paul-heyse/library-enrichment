# `pyrefly_build::query::custom`

Crate `pyrefly_build` · 1 public items · structured records in [`model/pyrefly_build.query.custom.json`](../model/pyrefly_build.query.custom.json)

## CustomQueryArgs

`struct` · `pyrefly_build::query::custom::CustomQueryArgs`

Also reachable as `pyrefly_build::CustomQueryArgs`

```rust
struct CustomQueryArgs
```

**Fields**: `command`, `repo_root`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn get_repo_root(&self, cwd: &std::path::Path) -> anyhow::Result<PathBuf>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Args and settings for querying a custom source DB.

---
