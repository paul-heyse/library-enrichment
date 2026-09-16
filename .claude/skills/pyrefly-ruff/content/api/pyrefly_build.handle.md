# `pyrefly_build::handle`

Crate `pyrefly_build` · 1 public items · structured records in [`model/pyrefly_build.handle.json`](../model/pyrefly_build.handle.json)

## Handle

`struct` · `pyrefly_build::handle::Handle`

```rust
struct Handle
```

**Implements**: `dupe::Dupe`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn from_with_module_name_kind(module: ModuleNameWithKind, path: ModulePath, sys_info: SysInfo) -> Self
fn is_fallback(&self) -> bool
fn module(&self) -> ModuleName
fn module_kind(&self) -> ModuleNameWithKind
fn new(module: ModuleName, path: ModulePath, sys_info: SysInfo) -> Self
fn path(&self) -> &ModulePath
fn sys_info(&self) -> &SysInfo
```

A handle to a Python module, containing its name, path, and system information.

---
