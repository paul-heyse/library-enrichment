# `ra_ap_project_model`

Crate `ra_ap_project_model` · 5 public items · structured records in [`model/ra_ap_project_model.json`](../model/ra_ap_project_model.json)

## InvocationStrategy

`enum` · `ra_ap_project_model::InvocationStrategy`

```rust
enum InvocationStrategy
```

**Variants**: `Once`, `PerWorkspace`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

---

## ProjectManifest

`enum` · `ra_ap_project_model::ProjectManifest`

```rust
enum ProjectManifest
```

**Variants**: `ProjectJson`, `CargoToml`, `CargoScript`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn discover(path: &AbsPath) -> io::Result<Vec<ProjectManifest>>
fn discover_all(paths: &[AbsPathBuf]) -> Vec<ProjectManifest>
fn discover_single(path: &AbsPath) -> anyhow::Result<ProjectManifest>
fn from_manifest_file(path: AbsPathBuf) -> anyhow::Result<ProjectManifest>
fn manifest_path(&self) -> &ManifestPath
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## RustSourceWorkspaceConfig

`enum` · `ra_ap_project_model::RustSourceWorkspaceConfig`

```rust
enum RustSourceWorkspaceConfig
```

**Variants**: `CargoMetadata`, `Json`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn default_cargo() -> Self
```

---

## CfgOverrides

`struct` · `ra_ap_project_model::CfgOverrides`

```rust
struct CfgOverrides
```

**Fields**: `global`, `selective`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn apply(&self, cfg_options: &mut cfg::CfgOptions, name: &str)
fn len(&self) -> usize
```

A set of cfg-overrides per crate.

---

## ProjectJsonFromCommand

`struct` · `ra_ap_project_model::ProjectJsonFromCommand`

```rust
struct ProjectJsonFromCommand
```

**Fields**: `data`, `buildfile`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---
