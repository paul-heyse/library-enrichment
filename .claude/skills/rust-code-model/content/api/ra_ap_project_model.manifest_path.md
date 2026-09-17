# `ra_ap_project_model::manifest_path`

Crate `ra_ap_project_model` · 1 public items · structured records in [`model/ra_ap_project_model.manifest_path.json`](../model/ra_ap_project_model.manifest_path.json)

## ManifestPath

`struct` · `ra_ap_project_model::manifest_path::ManifestPath`

Also reachable as `ra_ap_project_model::ManifestPath`

```rust
struct ManifestPath
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::convert::TryFrom`, `core::fmt::Display`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn canonicalize(&self) -> never
fn is_rust_manifest(&self) -> bool
fn parent(&self) -> &AbsPath
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &AbsPath
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Utf8Path
fn as_ref(&self) -> &AbsPath
fn as_ref(&self) -> &std::path::Path
fn as_ref(&self) -> &std::ffi::OsStr
```

**via `core::convert::TryFrom`**

```rust
fn try_from(file: AbsPathBuf) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

More or less [`AbsPathBuf`] with non-None parent.

We use it to store path to Cargo.toml, as we frequently use the parent dir
as a working directory to spawn various commands, and its nice to not have
to `.unwrap()` everywhere.

This could have been named `AbsNonRootPathBuf`, as we don't enforce that
this stores manifest files in particular, but we only use this for manifests
at the moment in practice.

---
