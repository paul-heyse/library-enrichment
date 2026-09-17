# `ra_ap_load_cargo`

Crate `ra_ap_load-cargo` · 8 public items · structured records in [`model/ra_ap_load_cargo.json`](../model/ra_ap_load_cargo.json)

## ProcMacroServerChoice

`enum` · `ra_ap_load_cargo::ProcMacroServerChoice`

```rust
enum ProcMacroServerChoice
```

**Variants**: `Sysroot`, `Explicit`, `None`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## load_proc_macro

`function` · `ra_ap_load_cargo::load_proc_macro`

```rust
fn load_proc_macro(server: &proc_macro_api::ProcMacroClient, path: &vfs::AbsPath, ignored_macros: &[Box<str>]) -> hir_expand::proc_macro::ProcMacroLoadResult
```

Load the proc-macros for the given lib path, disabling all expanders whose names are in `ignored_macros`.

---

## load_workspace

`function` · `ra_ap_load_cargo::load_workspace`

```rust
fn load_workspace(ws: project_model::ProjectWorkspace, extra_env: &ide_db::FxHashMap<String, Option<String>>, load_config: &LoadCargoConfig) -> anyhow::Result<(ide_db::RootDatabase, vfs::Vfs, Option<proc_macro_api::ProcMacroClient>)>
```

---

## load_workspace_at

`function` · `ra_ap_load_cargo::load_workspace_at`

```rust
fn load_workspace_at(root: &std::path::Path, cargo_config: &project_model::CargoConfig, load_config: &LoadCargoConfig, progress: &dyn Fn(String) + Sync) -> anyhow::Result<(ide_db::RootDatabase, vfs::Vfs, Option<proc_macro_api::ProcMacroClient>)>
```

---

## load_workspace_into_db

`function` · `ra_ap_load_cargo::load_workspace_into_db`

```rust
fn load_workspace_into_db(ws: project_model::ProjectWorkspace, extra_env: &ide_db::FxHashMap<String, Option<String>>, load_config: &LoadCargoConfig, db: &mut ide_db::RootDatabase) -> anyhow::Result<(vfs::Vfs, Option<proc_macro_api::ProcMacroClient>)>
```

---

## LoadCargoConfig

`struct` · `ra_ap_load_cargo::LoadCargoConfig`

```rust
struct LoadCargoConfig
```

**Fields**: `load_out_dirs_from_check`, `with_proc_macro_server`, `prefill_caches`, `num_worker_threads`, `proc_macro_processes`

**Derives**: Debug

---

## ProjectFolders

`struct` · `ra_ap_load_cargo::ProjectFolders`

```rust
struct ProjectFolders
```

**Fields**: `load`, `watch`, `source_root_config`

**Derives**: Default

**Methods** (1)

```rust
fn new(workspaces: &[ProjectWorkspace], global_excludes: &[AbsPathBuf], user_config_dir_path: Option<&AbsPath>) -> ProjectFolders
```

---

## SourceRootConfig

`struct` · `ra_ap_load_cargo::SourceRootConfig`

```rust
struct SourceRootConfig
```

**Fields**: `fsc`, `local_filesets`

**Derives**: Debug, Default

**Methods** (3)

```rust
fn partition(&self, vfs: &vfs::Vfs) -> Vec<SourceRoot>
fn path_is_library(&self, path: &VfsPath) -> bool
fn source_root_parent_map(&self) -> FxHashMap<SourceRootId, SourceRootId>
```

---
