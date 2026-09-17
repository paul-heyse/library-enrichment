# `ra_ap_project_model::sysroot`

Crate `ra_ap_project_model` · 2 public items · structured records in [`model/ra_ap_project_model.sysroot.json`](../model/ra_ap_project_model.sysroot.json)

## RustLibSrcWorkspace

`enum` · `ra_ap_project_model::sysroot::RustLibSrcWorkspace`

```rust
enum RustLibSrcWorkspace
```

**Variants**: `Workspace`, `Json`, `Stitched`, `Empty`

---

## Sysroot

`struct` · `ra_ap_project_model::sysroot::Sysroot`

Also reachable as `ra_ap_project_model::Sysroot`

```rust
struct Sysroot
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (17)

```rust
fn discover(dir: &AbsPath, extra_env: &FxHashMap<String, Option<String>>) -> Sysroot
fn discover_proc_macro_srv(&self) -> Option<anyhow::Result<AbsPathBuf>>
fn discover_rust_lib_src_dir(sysroot_dir: AbsPathBuf) -> Sysroot
fn discover_rustc_src(&self) -> Option<ManifestPath>
fn discover_with_src_override(current_dir: &AbsPath, extra_env: &FxHashMap<String, Option<String>>, rust_lib_src_dir: AbsPathBuf) -> Sysroot
const fn empty() -> Sysroot
fn error(&self) -> Option<&str>
fn is_rust_lib_src_empty(&self) -> bool
fn load_workspace(&self, sysroot_source_config: &RustSourceWorkspaceConfig, no_deps: bool, progress: &dyn Fn(String)) -> Option<RustLibSrcWorkspace>
fn metadata_error(&self) -> Option<&str>
fn new(sysroot_dir: Option<AbsPathBuf>, rust_lib_src_dir: Option<AbsPathBuf>) -> Sysroot
fn num_packages(&self) -> usize
fn root(&self) -> Option<&AbsPath>
fn rust_lib_src_root(&self) -> Option<&AbsPath>
fn set_workspace(&mut self, workspace: RustLibSrcWorkspace)
fn tool(&self, tool: Tool, current_dir: impl AsRef<Path>, envs: &FxHashMap<String, Option<String>>) -> Command
fn tool_path(&self, tool: Tool, current_dir: impl AsRef<Path>, envs: &Env) -> Utf8PathBuf
```

---
