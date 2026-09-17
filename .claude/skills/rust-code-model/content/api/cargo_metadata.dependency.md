# `cargo_metadata::dependency`

Crate `cargo_metadata` · 4 public items · structured records in [`model/cargo_metadata.dependency.json`](../model/cargo_metadata.dependency.json)

## DependencyBuilderError

`enum` · `cargo_metadata::dependency::DependencyBuilderError`

```rust
enum DependencyBuilderError
```

**Variants**: `UninitializedField`, `ValidationError`

Error type for DependencyBuilder

---

## DependencyKind

`enum` · `cargo_metadata::dependency::DependencyKind`

Also reachable as `cargo_metadata::DependencyKind`

```rust
enum DependencyKind
```

**Variants**: `Normal`, `Development`, `Build`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

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

Dependencies can come in three kinds

---

## Dependency

`struct` · `cargo_metadata::dependency::Dependency`

Also reachable as `cargo_metadata::Dependency`

```rust
struct Dependency
```

**Fields**: `name`, `source`, `req`, `kind`, `optional`, `uses_default_features`, `features`, `target`, `rename`, `registry`, `path`

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

A dependency of the main crate

---

## DependencyBuilder

`struct` · `cargo_metadata::dependency::DependencyBuilder`

Also reachable as `cargo_metadata::DependencyBuilder`

```rust
struct DependencyBuilder
```

**Derives**: Default

**Methods** (12)

```rust
fn build(self) -> ::derive_builder::export::core::result::Result<Dependency, DependencyBuilderError>
fn features<VALUE: ::derive_builder::export::core::convert::Into<Vec<String>>>(self, value: VALUE) -> Self
fn kind<VALUE: ::derive_builder::export::core::convert::Into<DependencyKind>>(self, value: VALUE) -> Self
fn name<VALUE: ::derive_builder::export::core::convert::Into<String>>(self, value: VALUE) -> Self
fn optional<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
fn path<VALUE: ::derive_builder::export::core::convert::Into<Option<Utf8PathBuf>>>(self, value: VALUE) -> Self
fn registry<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn rename<VALUE: ::derive_builder::export::core::convert::Into<Option<String>>>(self, value: VALUE) -> Self
fn req<VALUE: ::derive_builder::export::core::convert::Into<VersionReq>>(self, value: VALUE) -> Self
fn source<VALUE: ::derive_builder::export::core::convert::Into<Option<Source>>>(self, value: VALUE) -> Self
fn target<VALUE: ::derive_builder::export::core::convert::Into<Option<Platform>>>(self, value: VALUE) -> Self
fn uses_default_features<VALUE: ::derive_builder::export::core::convert::Into<bool>>(self, value: VALUE) -> Self
```

Builder for [`Dependency`](struct.Dependency.html).

---
