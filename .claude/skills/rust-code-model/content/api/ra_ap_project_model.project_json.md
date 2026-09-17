# `ra_ap_project_model::project_json`

Crate `ra_ap_project_model` · 7 public items · structured records in [`model/ra_ap_project_model.project_json.json`](../model/ra_ap_project_model.project_json.json)

## RunnableKind

`enum` · `ra_ap_project_model::project_json::RunnableKind`

```rust
enum RunnableKind
```

**Variants**: `Check`, `Run`, `TestOne`, `TestMod`, `DocTestOne`, `BenchOne`, `Flycheck`, `Unknown`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

The kind of runnable.

---

## Build

`struct` · `ra_ap_project_model::project_json::Build`

```rust
struct Build
```

**Fields**: `label`, `build_file`, `target_kind`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Additional, build-specific data about a crate.

---

## Crate

`struct` · `ra_ap_project_model::project_json::Crate`

```rust
struct Crate
```

**Fields**: `root_module`, `is_workspace_member`, `build`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn iter_deps(&self) -> impl ExactSizeIterator<Item = CrateArrayIdx>
```

A crate points to the root module of a crate and lists the dependencies of the crate. This is
useful in creating the crate graph.

---

## CrateArrayIdx

`struct` · `ra_ap_project_model::project_json::CrateArrayIdx`

```rust
struct CrateArrayIdx
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Identifies a crate by position in the crates array.

This will differ from `Crate` when multiple `ProjectJson`
workspaces are loaded.

---

## ProjectJson

`struct` · `ra_ap_project_model::project_json::ProjectJson`

Also reachable as `ra_ap_project_model::ProjectJson`

```rust
struct ProjectJson
```

**Implements**: `core::ops::index::Index`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn crate_by_buildfile(&self, path: &AbsPath) -> Option<Build>
fn crate_by_label(&self, label: &str) -> Option<&Crate>
fn crate_by_root(&self, root: &AbsPath) -> Option<&Crate>
fn crates(&self) -> impl Iterator<Item = (CrateArrayIdx, &Crate)>
fn manifest(&self) -> Option<&ManifestPath>
fn manifest_or_root(&self) -> &AbsPath
fn n_crates(&self) -> usize
fn new(manifest: Option<ManifestPath>, base: &AbsPath, data: ProjectJsonData) -> ProjectJson
fn path(&self) -> &AbsPath
fn project_root(&self) -> &AbsPath
fn runnable_template(&self, kind: RunnableKind) -> Option<&Runnable>
fn runnables(&self) -> &[Runnable]
```

**via `core::ops::index::Index`**

```rust
fn index(&self, index: CrateArrayIdx) -> &Self::Output
```

Roots and crates that compose this Rust project.

---

## ProjectJsonData

`struct` · `ra_ap_project_model::project_json::ProjectJsonData`

Also reachable as `ra_ap_project_model::ProjectJsonData`

```rust
struct ProjectJsonData
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Runnable

`struct` · `ra_ap_project_model::project_json::Runnable`

```rust
struct Runnable
```

**Fields**: `program`, `args`, `cwd`, `kind`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

A template-like structure for describing runnables.

These are used for running and debugging binaries and tests without encoding
build system-specific knowledge into rust-analyzer.

# Example

Below is an example of a test runnable. `{label}` and `{test_id}`
are explained in [`Runnable::args`]'s documentation.

```json
{
    "program": "buck",
    "args": [
        "test",
         "{label}",
         "--",
         "{test_id}",
         "--print-passing-details"
    ],
    "cwd": "/home/user/repo-root/",
    "kind": "testOne"
}
```

---
