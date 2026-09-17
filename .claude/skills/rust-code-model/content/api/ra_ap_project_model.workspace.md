# `ra_ap_project_model::workspace`

Crate `ra_ap_project_model` · 4 public items · structured records in [`model/ra_ap_project_model.workspace.json`](../model/ra_ap_project_model.workspace.json)

## ProjectWorkspaceKind

`enum` · `ra_ap_project_model::workspace::ProjectWorkspaceKind`

Also reachable as `ra_ap_project_model::ProjectWorkspaceKind`

```rust
enum ProjectWorkspaceKind
```

**Variants**: `Cargo`, `Json`, `DetachedFile`

**Derives**: Clone

---

## PackageRoot

`struct` · `ra_ap_project_model::workspace::PackageRoot`

Also reachable as `ra_ap_project_model::PackageRoot`

```rust
struct PackageRoot
```

**Fields**: `is_local`, `include`, `exclude`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

`PackageRoot` describes a package root folder.
Which may be an external dependency, or a member of
the current workspace.

---

## ProjectWorkspace

`struct` · `ra_ap_project_model::workspace::ProjectWorkspace`

Also reachable as `ra_ap_project_model::ProjectWorkspace`

```rust
struct ProjectWorkspace
```

**Fields**: `kind`, `sysroot`, `rustc_cfg`, `toolchain`, `target`, `cfg_overrides`, `extra_includes`, `set_test`

**Derives**: Clone, Debug

**Methods** (17)

```rust
fn buildfiles(&self) -> Vec<AbsPathBuf>
fn eq_ignore_build_data(&self, other: &Self) -> bool
fn find_sysroot_proc_macro_srv(&self) -> Option<anyhow::Result<AbsPathBuf>>
fn is_json(&self) -> bool
fn load(manifest: ProjectManifest, config: &CargoConfig, progress: &dyn Fn(String) + Sync) -> anyhow::Result<ProjectWorkspace>
fn load_detached_file(detached_file: &ManifestPath, config: &CargoConfig) -> anyhow::Result<ProjectWorkspace>
fn load_detached_files(detached_files: Vec<ManifestPath>, config: &CargoConfig) -> Vec<anyhow::Result<ProjectWorkspace>>
fn load_inline(project_json: ProjectJson, config: &CargoConfig, progress: &dyn Fn(String) + Sync) -> ProjectWorkspace
fn manifest(&self) -> Option<&ManifestPath>
fn manifest_or_root(&self) -> &AbsPath
fn n_packages(&self) -> usize
fn run_all_build_scripts(workspaces: &[ProjectWorkspace], config: &CargoConfig, progress: &dyn Fn(String), working_directory: &AbsPathBuf) -> Vec<anyhow::Result<WorkspaceBuildScripts>>
fn run_build_scripts(&self, config: &CargoConfig, progress: &dyn Fn(String)) -> anyhow::Result<WorkspaceBuildScripts>
fn set_build_scripts(&mut self, bs: WorkspaceBuildScripts)
fn to_crate_graph(&self, load: FileLoader<'_>, extra_env: &FxHashMap<String, Option<String>>) -> (CrateGraphBuilder, ProcMacroPaths)
fn to_roots(&self) -> Vec<PackageRoot>
fn workspace_root(&self) -> &AbsPath
```

---

## FileLoader

`type_alias` · `ra_ap_project_model::workspace::FileLoader`

Also reachable as `ra_ap_project_model::FileLoader`

```rust
type FileLoader<'a> = &'a mut dyn FnMut(&'b paths::AbsPath) -> Option<span::FileId>
```

---
