# `ra_ap_project_model::cargo_workspace`

Crate `ra_ap_project_model` · 14 public items · structured records in [`model/ra_ap_project_model.cargo_workspace.json`](../model/ra_ap_project_model.cargo_workspace.json)

## CargoFeatures

`enum` · `ra_ap_project_model::cargo_workspace::CargoFeatures`

Also reachable as `ra_ap_project_model::CargoFeatures`

```rust
enum CargoFeatures
```

**Variants**: `All`, `Selected`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

---

## DepKind

`enum` · `ra_ap_project_model::cargo_workspace::DepKind`

```rust
enum DepKind
```

**Variants**: `Normal`, `Dev`, `Build`

---

## RustLibSource

`enum` · `ra_ap_project_model::cargo_workspace::RustLibSource`

Also reachable as `ra_ap_project_model::RustLibSource`

```rust
enum RustLibSource
```

**Variants**: `Path`, `Discover`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Describes how to set the rustc source directory.

---

## TargetDirectoryConfig

`enum` · `ra_ap_project_model::cargo_workspace::TargetDirectoryConfig`

Also reachable as `ra_ap_project_model::TargetDirectoryConfig`

```rust
enum TargetDirectoryConfig
```

**Variants**: `None`, `UseSubdirectory`, `Directory`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn target_dir<'a>(&'a self, ws_target_dir: Option<&'a Utf8Path>) -> Option<Cow<'a, Utf8Path>>
```

---

## TargetKind

`enum` · `ra_ap_project_model::cargo_workspace::TargetKind`

Also reachable as `ra_ap_project_model::TargetKind`

```rust
enum TargetKind
```

**Variants**: `Bin`, `Lib`, `Example`, `Test`, `Bench`, `BuildScript`, `Other`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn as_cargo_target(self) -> Option<&'static str>
fn is_executable(self) -> bool
fn is_proc_macro(self) -> bool
fn new(kinds: &[cargo_metadata::TargetKind]) -> TargetKind
```

---

## CargoConfig

`struct` · `ra_ap_project_model::cargo_workspace::CargoConfig`

Also reachable as `ra_ap_project_model::CargoConfig`

```rust
struct CargoConfig
```

**Fields**: `all_targets`, `features`, `target`, `sysroot`, `sysroot_src`, `rustc_source`, `extra_includes`, `cfg_overrides`, `wrap_rustc_in_build_scripts`, `run_build_script_command`, `extra_args`, `metadata_extra_args`, `config_path`, `extra_env`, `invocation_strategy`, `target_dir_config`, `set_test`, `no_deps`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

---

## CargoMetadataConfig

`struct` · `ra_ap_project_model::cargo_workspace::CargoMetadataConfig`

Also reachable as `ra_ap_project_model::CargoMetadataConfig`

```rust
struct CargoMetadataConfig
```

**Fields**: `features`, `targets`, `extra_args`, `metadata_extra_args`, `config_path`, `extra_env`, `kind`, `toolchain_version`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

---

## CargoWorkspace

`struct` · `ra_ap_project_model::cargo_workspace::CargoWorkspace`

Also reachable as `ra_ap_project_model::CargoWorkspace`

```rust
struct CargoWorkspace
```

**Implements**: `core::ops::index::Index`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (14)

```rust
fn build_directory(&self) -> Option<&AbsPath>
fn env(&self) -> &Env
fn is_sysroot(&self) -> bool
fn is_virtual_workspace(&self) -> bool
fn manifest_path(&self) -> &ManifestPath
fn new(meta: cargo_metadata::Metadata, ws_manifest_path: ManifestPath, cargo_env: Env, is_sysroot: bool) -> CargoWorkspace
fn package_flag(&self, package: &PackageData) -> String
fn packages(&self) -> impl ExactSizeIterator<Item = Package> + '_
fn parent_manifests(&self, manifest_path: &ManifestPath) -> Option<Vec<ManifestPath>>
fn requires_rustc_private(&self) -> bool
fn target_by_root(&self, root: &AbsPath) -> Option<Target>
fn target_directory(&self) -> &AbsPath
fn workspace_features(&self) -> FxHashSet<String>
fn workspace_root(&self) -> &AbsPath
```

**via `core::ops::index::Index`**

```rust
fn index(&self, index: Target) -> &TargetData
fn index(&self, index: Package) -> &PackageData
```

[`CargoWorkspace`] represents the logical structure of, well, a Cargo
workspace. It pretty closely mirrors `cargo metadata` output.

Note that internally, rust-analyzer uses a different structure:
`CrateGraph`. `CrateGraph` is lower-level: it knows only about the crates,
while this knows about `Packages` & `Targets`: purely cargo-related
concepts.

We use absolute paths here, `cargo metadata` guarantees to always produce
abs paths.

---

## PackageData

`struct` · `ra_ap_project_model::cargo_workspace::PackageData`

Also reachable as `ra_ap_project_model::PackageData`

```rust
struct PackageData
```

**Fields**: `version`, `name`, `repository`, `manifest`, `targets`, `is_local`, `is_member`, `dependencies`, `edition`, `features`, `active_features`, `id`, `authors`, `description`, `homepage`, `license`, `license_file`, `readme`, `rust_version`, `metadata`, `all_member_deps`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Information associated with a cargo crate

---

## PackageDependency

`struct` · `ra_ap_project_model::cargo_workspace::PackageDependency`

Also reachable as `ra_ap_project_model::PackageDependency`

```rust
struct PackageDependency
```

**Fields**: `pkg`, `name`, `kind`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## RustAnalyzerPackageMetaData

`struct` · `ra_ap_project_model::cargo_workspace::RustAnalyzerPackageMetaData`

```rust
struct RustAnalyzerPackageMetaData
```

**Fields**: `rustc_private`

---

## TargetData

`struct` · `ra_ap_project_model::cargo_workspace::TargetData`

Also reachable as `ra_ap_project_model::TargetData`

```rust
struct TargetData
```

**Fields**: `package`, `name`, `root`, `kind`, `required_features`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Information associated with a package's target

---

## Package

`type_alias` · `ra_ap_project_model::cargo_workspace::Package`

Also reachable as `ra_ap_project_model::Package`

```rust
type Package = la_arena::Idx<PackageData>
```

---

## Target

`type_alias` · `ra_ap_project_model::cargo_workspace::Target`

Also reachable as `ra_ap_project_model::Target`

```rust
type Target = la_arena::Idx<TargetData>
```

---
