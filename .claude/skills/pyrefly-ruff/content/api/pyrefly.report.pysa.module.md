# `pyrefly::report::pysa::module`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.report.pysa.module.json`](../model/pyrefly.report.pysa.module.json)

## ModuleId

`struct` · `pyrefly::report::pysa::module::ModuleId`

```rust
struct ModuleId
```

**Implements**: `dupe::Dupe`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn to_int(self) -> u32
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a unique identifier for a module

---

## ModuleIds

`struct` · `pyrefly::report::pysa::module::ModuleIds`

```rust
struct ModuleIds
```

**Methods** (3)

```rust
fn get_from_handle(&self, handle: &Handle) -> ModuleId
fn get_from_module(&self, module: &Module, sys_info: SysInfo) -> ModuleId
fn new(handles: &[Handle]) -> ModuleIds
```

Thread-safe map from `Handle` to `ModuleId`.

Typeshed modules and project handles are pre-assigned deterministic IDs
in sorted order. Dependency modules discovered during type checking get
IDs assigned lazily on first access via `get_or_insert`.

---
