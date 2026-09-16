# `pyrefly_util::interned_path`

Crate `pyrefly_util` · 1 public items · structured records in [`model/pyrefly_util.interned_path.json`](../model/pyrefly_util.interned_path.json)

## InternedPath

`struct` · `pyrefly_util::interned_path::InternedPath`

```rust
struct InternedPath
```

**Implements**: `core::fmt::Display`, `core::ops::deref::Deref`, `dupe::Dupe`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn from_path(path: &Path) -> Self
fn new(path: PathBuf) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(d: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

---
