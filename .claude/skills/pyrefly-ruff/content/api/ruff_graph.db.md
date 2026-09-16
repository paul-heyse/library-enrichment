# `ruff_graph::db`

Crate `ruff_graph` · 2 public items · structured records in [`model/ruff_graph.db.json`](../model/ruff_graph.db.json)

## resolve_search_paths

`function` · `ruff_graph::db::resolve_search_paths`

Also reachable as `ruff_graph::resolve_search_paths`

```rust
fn resolve_search_paths(system: &dyn System, src_roots: Vec<ruff_db::system::SystemPathBuf>, venv_path: Option<ruff_db::system::SystemPathBuf>) -> anyhow::Result<ty_module_resolver::SearchPaths>
```

Resolve module search paths for the given source roots and Python environment.

---

## ModuleDb

`struct` · `ruff_graph::db::ModuleDb`

Also reachable as `ruff_graph::ModuleDb`

```rust
struct ModuleDb
```

**Implements**: `ruff_db::Db`, `salsa::database::Database`, `salsa::storage::HasStorage`, `ty_module_resolver::db::Db`

**Derives**: Clone

**Methods** (1)

```rust
fn new<S>(system: S) -> Self where S: System + 'static + Send + Sync + RefUnwindSafe
```

**via `ruff_db::Db`**

```rust
fn files(&self) -> &Files
fn system(&self) -> &dyn System
fn vendored(&self) -> &VendoredFileSystem
```

**via `salsa::storage::HasStorage`**

```rust
fn storage(&self) -> &zalsa_::Storage<Self>
fn storage_mut(&mut self) -> &mut zalsa_::Storage<Self>
```

---
