# `pyrefly_build`

Crate `pyrefly_build` · 2 public items · structured records in [`model/pyrefly_build.json`](../model/pyrefly_build.json)

## BuildSystemArgs

`enum` · `pyrefly_build::BuildSystemArgs`

```rust
enum BuildSystemArgs
```

**Variants**: `Buck`, `Custom`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

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

## BuildSystem

`struct` · `pyrefly_build::BuildSystem`

```rust
struct BuildSystem
```

**Fields**: `args`, `ignore_if_build_system_missing`, `search_path_prefix`, `catch_all_targets`, `catch_all_targets_only`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn get_source_db(&mut self, config_root: PathBuf) -> Option<anyhow::Result<ArcId<Box<dyn source_db::SourceDatabase + 'static>>>>
fn new(isolation_dir: Option<String>, extras: Option<Vec<String>>, ignore_if_build_system_missing: bool, search_path_prefix: Vec<PathBuf>, catch_all_targets: Vec<Target>, catch_all_targets_only: bool) -> Self
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
