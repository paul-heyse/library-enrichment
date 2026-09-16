# `pyrefly_build::query::buck`

Crate `pyrefly_build` · 1 public items · structured records in [`model/pyrefly_build.query.buck.json`](../model/pyrefly_build.query.buck.json)

## BxlArgs

`struct` · `pyrefly_build::query::buck::BxlArgs`

```rust
struct BxlArgs
```

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn get_repo_root(&self, cwd: &Path) -> anyhow::Result<PathBuf>
fn new(isolation_dir: Option<String>, extras: Option<Vec<String>>) -> Self
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

---
