# `pyrefly_build::source_db::buck_check`

Crate `pyrefly_build` · 1 public items · structured records in [`model/pyrefly_build.source_db.buck_check.json`](../model/pyrefly_build.source_db.buck_check.json)

## BuckCheckSourceDatabase

`struct` · `pyrefly_build::source_db::buck_check::BuckCheckSourceDatabase`

```rust
struct BuckCheckSourceDatabase
```

**Implements**: `pyrefly_build::source_db::ModuleEnumerator`, `pyrefly_build::source_db::SourceDatabase`

**Derives**: Debug

**Methods** (1)

```rust
fn from_manifest_files(source_manifests: &[PathBuf], dependency_manifests: &[PathBuf], typeshed_manifests: &[PathBuf], sys_info: SysInfo, check_dependencies: bool, skip_dependency_modules: Vec<Regex>) -> anyhow::Result<Self>
```

**via `pyrefly_build::source_db::ModuleEnumerator`**

```rust
fn modules_to_check(&self) -> Vec<Handle>
```

**via `pyrefly_build::source_db::SourceDatabase`**

```rust
fn as_live_source_database(&self) -> Option<&dyn LiveSourceDatabase>
fn handle_from_module_path(&self, module_path: &ModulePath) -> Option<Handle>
fn lookup(&self, module: ModuleName, _: Option<&Path>, _: Option<ModuleStyle>) -> Option<ModulePath>
fn may_contain_module(&self, module: ModuleName) -> bool
```

---
