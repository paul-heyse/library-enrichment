# `cargo_metadata`

Crate `cargo_metadata` · 31 public items · structured records in [`model/cargo_metadata.json`](../model/cargo_metadata.json)

## CargoOpt

`enum` · `cargo_metadata::CargoOpt`

```rust
enum CargoOpt
```

**Variants**: `AllFeatures`, `NoDefaultFeatures`, `SomeFeatures`

**Derives**: Clone, Debug

Cargo features flags

---

## CrateType

`enum` · `cargo_metadata::CrateType`

```rust
enum CrateType
```

**Variants**: `Bin`, `CDyLib`, `DyLib`, `Lib`, `ProcMacro`, `RLib`, `StaticLib`, `Unknown`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Similar to `kind`, but only reports the
[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):

`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
Everything that's not a proc macro or a library of some kind is reported as "bin".

Other possible values may be added in the future.

---

## DepKindInfoBuilderError

`enum` · `cargo_metadata::DepKindInfoBuilderError`

```rust
enum DepKindInfoBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for DepKindInfoBuilder

---

## Edition

`enum` · `cargo_metadata::Edition`

```rust
enum Edition
```

**Variants**: `E2015`, `E2018`, `E2021`, `E2024`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The Rust edition

As of writing this comment rust editions 2027 and 2030 are not actually a thing yet but are parsed nonetheless for future proofing.

---

## MetadataBuilderError

`enum` · `cargo_metadata::MetadataBuilderError`

```rust
enum MetadataBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for MetadataBuilder

---

## NodeBuilderError

`enum` · `cargo_metadata::NodeBuilderError`

```rust
enum NodeBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for NodeBuilder

---

## NodeDepBuilderError

`enum` · `cargo_metadata::NodeDepBuilderError`

```rust
enum NodeDepBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for NodeDepBuilder

---

## PackageBuilderError

`enum` · `cargo_metadata::PackageBuilderError`

```rust
enum PackageBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for PackageBuilder

---

## ResolveBuilderError

`enum` · `cargo_metadata::ResolveBuilderError`

```rust
enum ResolveBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for ResolveBuilder

---

## TargetBuilderError

`enum` · `cargo_metadata::TargetBuilderError`

```rust
enum TargetBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(s: ::derive_builder::export::core::string::String) -> Self
fn from(s: ::derive_builder::UninitializedFieldError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter<'_>) -> ::derive_builder::export::core::fmt::Result
```

Error type for TargetBuilder

---

## TargetKind

`enum` · `cargo_metadata::TargetKind`

```rust
enum TargetKind
```

**Variants**: `Bench`, `Bin`, `CustomBuild`, `CDyLib`, `DyLib`, `Example`, `Lib`, `ProcMacro`, `RLib`, `StaticLib`, `Test`, `Unknown`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &str) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Kind of target.

The possible values are `example`, `test`, `bench`, `custom-build` and
[Cargo crate types](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field):
`bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.

Other possible values may be added in the future.

---

## DepKindInfo

`struct` · `cargo_metadata::DepKindInfo`

```rust
struct DepKindInfo
```

**Fields**: `kind`, `target`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Information about a dependency kind.

---

## DepKindInfoBuilder

`struct` · `cargo_metadata::DepKindInfoBuilder`

```rust
struct DepKindInfoBuilder
```

**Derives**: Default

**Methods** (3)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<DepKindInfo, DepKindInfoBuilderError>
fn kind<VALUE: ::derive_builder::export::core::convert::Into<DependencyKind>>(self, value: VALUE) -> Self
fn target<VALUE: ::derive_builder::export::core::convert::Into<Option<dependency::Platform>>>(self, value: VALUE) -> Self
```

Builder for [`DepKindInfo`](struct.DepKindInfo.html).

---

## FeatureName

`struct` · `cargo_metadata::FeatureName`

```rust
struct FeatureName<T: AsRef<str> = String>
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::fmt::Display`, `core::ops::deref::Deref`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (2)

```rust
fn into_inner(self) -> T
fn new(name: T) -> Self
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(value: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Feature name newtype

Based on [cargo-util-schema's string newtype] but with two crucial differences:

- This newtype does not verify the wrapped string.
- This newtype allows comparison with arbitrary types that implement `AsRef<str>`.

[cargo-util-schema's string newtype]: https://github.com/epage/cargo/blob/d8975d2901e132c02b3f6b1d107f2f50b275a058/crates/cargo-util-schemas/src/manifest/mod.rs#L1355-L1413

---

## Metadata

`struct` · `cargo_metadata::Metadata`

Also reachable as `ra_ap_project_model::Metadata`

```rust
struct Metadata
```

**Fields**: `packages`, `workspace_members`, `workspace_default_members`, `resolve`, `workspace_root`, `target_directory`, `build_directory`, `workspace_metadata`

**Implements**: `core::ops::index::Index`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn root_package(&self) -> Option<&Package>
fn workspace_default_packages(&self) -> Vec<&Package>
fn workspace_packages(&self) -> Vec<&Package>
```

**via `core::ops::index::Index`**

```rust
fn index(&self, idx: &'a PackageId) -> &Self::Output
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Starting point for metadata returned by `cargo metadata`

---

## MetadataBuilder

`struct` · `cargo_metadata::MetadataBuilder`

```rust
struct MetadataBuilder
```

**Derives**: Default

**Methods** (10)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<Metadata, MetadataBuilderError>
fn build_directory<VALUE: ::derive_builder::export::core::convert::Into<Option<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn packages<VALUE: ::derive_builder::export::core::convert::Into<Vec<Package>>>(self, value: VALUE) -> Self
fn resolve<VALUE: ::derive_builder::export::core::convert::Into<Option<Resolve>>>(self, value: VALUE) -> Self
fn target_directory<VALUE: ::derive_builder::export::core::convert::Into<Utf8PathBuf>>(self, value: VALUE) -> Self
fn version<VALUE: ::derive_builder::export::core::convert::Into<usize>>(self, value: VALUE) -> Self
fn workspace_default_members<VALUE: ::derive_builder::export::core::convert::Into<WorkspaceDefaultMembers>>(self, value: VALUE) -> Self
fn workspace_members<VALUE: ::derive_builder::export::core::convert::Into<Vec<PackageId>>>(self, value: VALUE) -> Self
fn workspace_metadata<VALUE: ::derive_builder::export::core::convert::Into<serde_json::Value>>(self, value: VALUE) -> Self
fn workspace_root<VALUE: ::derive_builder::export::core::convert::Into<Utf8PathBuf>>(self, value: VALUE) -> Self
```

Builder for [`Metadata`](struct.Metadata.html).

---

## MetadataCommand

`struct` · `cargo_metadata::MetadataCommand`

```rust
struct MetadataCommand
```

**Derives**: Clone, Debug, Default

**Methods** (13)

```rust
fn cargo_command(&self) -> Command
fn cargo_path(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand
fn current_dir(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand
fn env<K: Into<OsString>, V: Into<OsString>>(&mut self, key: K, val: V) -> &mut MetadataCommand
fn env_remove<K: Into<OsString>>(&mut self, key: K) -> &mut MetadataCommand
fn exec(&self) -> Result<Metadata>
fn features(&mut self, features: CargoOpt) -> &mut MetadataCommand
fn manifest_path(&mut self, path: impl Into<PathBuf>) -> &mut MetadataCommand
fn new() -> MetadataCommand
fn no_deps(&mut self) -> &mut MetadataCommand
fn other_options(&mut self, options: impl Into<Vec<String>>) -> &mut MetadataCommand
fn parse<T: AsRef<str>>(data: T) -> Result<Metadata>
fn verbose(&mut self, verbose: bool) -> &mut MetadataCommand
```

A builder for configuring `cargo metadata` invocation.

---

## Node

`struct` · `cargo_metadata::Node`

```rust
struct Node
```

**Fields**: `id`, `deps`, `dependencies`, `features`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A node in a dependencies graph

---

## NodeBuilder

`struct` · `cargo_metadata::NodeBuilder`

```rust
struct NodeBuilder
```

**Derives**: Default

**Methods** (5)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<Node, NodeBuilderError>
fn dependencies<VALUE: ::derive_builder::export::core::convert::Into<Vec<PackageId>>>(self, value: VALUE) -> Self
fn deps<VALUE: ::derive_builder::export::core::convert::Into<Vec<NodeDep>>>(self, value: VALUE) -> Self
fn features<VALUE: ::derive_builder::export::core::convert::Into<Vec<FeatureName>>>(self, value: VALUE) -> Self
fn id<VALUE: ::derive_builder::export::core::convert::Into<PackageId>>(self, value: VALUE) -> Self
```

Builder for [`Node`](struct.Node.html).

---

## NodeDep

`struct` · `cargo_metadata::NodeDep`

```rust
struct NodeDep
```

**Fields**: `name`, `pkg`, `dep_kinds`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A dependency in a node

---

## NodeDepBuilder

`struct` · `cargo_metadata::NodeDepBuilder`

```rust
struct NodeDepBuilder
```

**Derives**: Default

**Methods** (4)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<NodeDep, NodeDepBuilderError>
fn dep_kinds<VALUE: ::derive_builder::export::core::convert::Into<Vec<DepKindInfo>>>(self, value: VALUE) -> Self
fn name<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn pkg<VALUE: ::derive_builder::export::core::convert::Into<PackageId>>(self, value: VALUE) -> Self
```

Builder for [`NodeDep`](struct.NodeDep.html).

---

## Package

`struct` · `cargo_metadata::Package`

```rust
struct Package
```

**Fields**: `name`, `version`, `authors`, `id`, `source`, `description`, `dependencies`, `license`, `license_file`, `targets`, `features`, `manifest_path`, `categories`, `keywords`, `readme`, `repository`, `homepage`, `documentation`, `edition`, `metadata`, `links`, `publish`, `default_run`, `rust_version`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn license_file(&self) -> Option<Utf8PathBuf>
fn readme(&self) -> Option<Utf8PathBuf>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One or more crates described by a single `Cargo.toml`

Each [`target`][Package::targets] of a `Package` will be built as a crate.
For more information, see <https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html>.

---

## PackageBuilder

`struct` · `cargo_metadata::PackageBuilder`

```rust
struct PackageBuilder
```

**Derives**: Default

**Methods** (26)

```rust
fn authors<VALUE: ::derive_builder::export::core::convert::Into<Vec<String>>>(self, value: VALUE) -> Self
fn build(self) -> ::derive_builder::export::core::result::Result<Package, PackageBuilderError>
fn categories<VALUE: ::derive_builder::export::core::convert::Into<Vec<String>>>(self, value: VALUE) -> Self
fn default_run<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn dependencies<VALUE: ::derive_builder::export::core::convert::Into<Vec<Dependency>>>(self, value: VALUE) -> Self
fn description<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn documentation<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn edition<VALUE: ::derive_builder::export::core::convert::Into<Edition>>(self, value: VALUE) -> Self
fn features<VALUE: ::derive_builder::export::core::convert::Into<BTreeMap<String, Vec<String>>>>(self, value: VALUE) -> Self
fn homepage<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn id<VALUE: ::derive_builder::export::core::convert::Into<PackageId>>(self, value: VALUE) -> Self
fn keywords<VALUE: ::derive_builder::export::core::convert::Into<Vec<String>>>(self, value: VALUE) -> Self
fn license<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn license_file<VALUE: ::derive_builder::export::core::convert::Into<Option<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn links<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn manifest_path<VALUE: ::derive_builder::export::core::convert::Into<Utf8PathBuf>>(self, value: VALUE) -> Self
fn metadata<VALUE: ::derive_builder::export::core::convert::Into<serde_json::Value>>(self, value: VALUE) -> Self
fn name<VALUE: ::derive_builder::export::core::convert::Into<PackageName>>(self, value: VALUE) -> Self
fn new(name: impl Into<PackageName>, version: impl Into<Version>, id: impl Into<PackageId>, path: impl Into<Utf8PathBuf>) -> Self
fn publish<VALUE: ::derive_builder::export::core::convert::Into<Option<Vec<String>>>>(self, value: VALUE) -> Self
fn readme<VALUE: ::derive_builder::export::core::convert::Into<Option<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn repository<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn rust_version<VALUE: ::derive_builder::export::core::convert::Into<Option<Version>>>(self, value: VALUE) -> Self
fn source<VALUE: ::derive_builder::export::core::convert::Into<Option<Source>>>(self, value: VALUE) -> Self
fn targets<VALUE: ::derive_builder::export::core::convert::Into<Vec<Target>>>(self, value: VALUE) -> Self
fn version<VALUE: ::derive_builder::export::core::convert::Into<Version>>(self, value: VALUE) -> Self
```

Builder for [`Package`](struct.Package.html).

---

## PackageId

`struct` · `cargo_metadata::PackageId`

```rust
struct PackageId
```

**Fields**: `repr`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An "opaque" identifier for a package.

It is possible to inspect the `repr` field, if the need arises, but its
precise format is an implementation detail and is subject to change.

`Metadata` can be indexed by `PackageId`.

---

## PackageName

`struct` · `cargo_metadata::PackageName`

```rust
struct PackageName<T: AsRef<str> = String>
```

**Implements**: `core::borrow::Borrow`, `core::convert::AsRef`, `core::fmt::Display`, `core::ops::deref::Deref`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (2)

```rust
fn into_inner(self) -> T
fn new(name: T) -> Self
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(value: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Package name newtype

Based on [cargo-util-schema's string newtype] but with two crucial differences:

- This newtype does not verify the wrapped string.
- This newtype allows comparison with arbitrary types that implement `AsRef<str>`.

[cargo-util-schema's string newtype]: https://github.com/epage/cargo/blob/d8975d2901e132c02b3f6b1d107f2f50b275a058/crates/cargo-util-schemas/src/manifest/mod.rs#L1355-L1413

---

## Resolve

`struct` · `cargo_metadata::Resolve`

```rust
struct Resolve
```

**Fields**: `nodes`, `root`

**Implements**: `core::ops::index::Index`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::ops::index::Index`**

```rust
fn index(&self, idx: &'a PackageId) -> &Self::Output
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A dependency graph

---

## ResolveBuilder

`struct` · `cargo_metadata::ResolveBuilder`

```rust
struct ResolveBuilder
```

**Derives**: Default

**Methods** (3)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<Resolve, ResolveBuilderError>
fn nodes<VALUE: ::derive_builder::export::core::convert::Into<Vec<Node>>>(self, value: VALUE) -> Self
fn root<VALUE: ::derive_builder::export::core::convert::Into<Option<PackageId>>>(self, value: VALUE) -> Self
```

Builder for [`Resolve`](struct.Resolve.html).

---

## Source

`struct` · `cargo_metadata::Source`

```rust
struct Source
```

**Fields**: `repr`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_crates_io(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The source of a package such as crates.io.

It is possible to inspect the `repr` field, if the need arises, but its
precise format is an implementation detail and is subject to change.

---

## Target

`struct` · `cargo_metadata::Target`

```rust
struct Target
```

**Fields**: `name`, `kind`, `crate_types`, `required_features`, `src_path`, `edition`, `doctest`, `test`, `doc`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn is_bench(&self) -> bool
fn is_bin(&self) -> bool
fn is_cdylib(&self) -> bool
fn is_custom_build(&self) -> bool
fn is_dylib(&self) -> bool
fn is_example(&self) -> bool
fn is_kind(&self, name: TargetKind) -> bool
fn is_lib(&self) -> bool
fn is_proc_macro(&self) -> bool
fn is_rlib(&self) -> bool
fn is_staticlib(&self) -> bool
fn is_test(&self) -> bool
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A single target (lib, bin, example, ...) provided by a crate

---

## TargetBuilder

`struct` · `cargo_metadata::TargetBuilder`

```rust
struct TargetBuilder
```

**Derives**: Default

**Methods** (10)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<Target, TargetBuilderError>
fn crate_types<VALUE: ::derive_builder::export::core::convert::Into<Vec<CrateType>>>(self, value: VALUE) -> Self
fn doc<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
fn doctest<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
fn edition<VALUE: ::derive_builder::export::core::convert::Into<Edition>>(self, value: VALUE) -> Self
fn kind<VALUE: ::derive_builder::export::core::convert::Into<Vec<TargetKind>>>(self, value: VALUE) -> Self
fn name<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn required_features<VALUE: ::derive_builder::export::core::convert::Into<Vec<String>>>(self, value: VALUE) -> Self
fn src_path<VALUE: ::derive_builder::export::core::convert::Into<Utf8PathBuf>>(self, value: VALUE) -> Self
fn test<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
```

Builder for [`Target`](struct.Target.html).

---

## WorkspaceDefaultMembers

`struct` · `cargo_metadata::WorkspaceDefaultMembers`

```rust
struct WorkspaceDefaultMembers
```

**Implements**: `core::ops::deref::Deref`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn is_available(&self) -> bool
fn is_missing(&self) -> bool
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A list of default workspace members.

See [`Metadata::workspace_default_members`].

It is only available if running a version of Cargo of 1.71 or newer.

# Panics

Dereferencing when running an older version of Cargo will panic.

---
