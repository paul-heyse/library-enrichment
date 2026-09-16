# `pyrefly_build::source_db::map_db`

Crate `pyrefly_build` · 1 public items · structured records in [`model/pyrefly_build.source_db.map_db.json`](../model/pyrefly_build.source_db.map_db.json)

## MapDatabase

`struct` · `pyrefly_build::source_db::map_db::MapDatabase`

```rust
struct MapDatabase
```

**Implements**: `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `pyrefly_build::source_db::ModuleEnumerator`, `pyrefly_build::source_db::SourceDatabase`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn insert(&mut self, name: ModuleName, path: ModulePath)
fn new(sys_info: SysInfo) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

**via `pyrefly_build::source_db::ModuleEnumerator`**

```rust
fn modules_to_check(&self) -> Vec<Handle>
```

**via `pyrefly_build::source_db::SourceDatabase`**

```rust
fn as_live_source_database(&self) -> Option<&dyn LiveSourceDatabase>
fn handle_from_module_path(&self, module_path: &ModulePath) -> Option<Handle>
fn lookup(&self, module: ModuleName, _: Option<&Path>, style: Option<ModuleStyle>) -> Option<ModulePath>
fn may_contain_module(&self, module: ModuleName) -> bool
```

A simple [`SourceDatabase`] that can be used for easy setup and testing.

---
