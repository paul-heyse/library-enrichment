# `pyrefly_config::environment::interpreters`

Crate `pyrefly_config` · 1 public items · structured records in [`model/pyrefly_config.environment.interpreters.json`](../model/pyrefly_config.environment.interpreters.json)

## Interpreters

`struct` · `pyrefly_config::environment::interpreters::Interpreters`

```rust
struct Interpreters
```

**Fields**: `skip_interpreter_query`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn is_empty(&self) -> bool
fn set_lsp_python_interpreter(&mut self, interpreter: PathBuf)
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
