# `pyrefly_config::pyproject`

Crate `pyrefly_config` · 1 public items · structured records in [`model/pyrefly_config.pyproject.json`](../model/pyrefly_config.pyproject.json)

## PyProject

`struct` · `pyrefly_config::pyproject::PyProject`

```rust
struct PyProject
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug

**Methods** (2)

```rust
fn new(cfg: ConfigFile) -> Self
fn update(pyproject_path: &Path, config: ConfigFile) -> anyhow::Result<()>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Wrapper used to (de)serialize pyrefly configs from pyproject.toml files.

---
