# `arrow_schema::extension::canonical::opaque`

Crate `arrow-schema` · 2 public items · structured records in [`model/arrow_schema.extension.canonical.opaque.json`](../model/arrow_schema.extension.canonical.opaque.json)

## Opaque

`struct` · `arrow_schema::extension::canonical::opaque::Opaque`

```rust
struct Opaque
```

**Implements**: `arrow_schema::extension::ExtensionType`, `core::convert::From`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(type_name: impl Into<String>, vendor_name: impl Into<String>) -> Self
fn type_name(&self) -> &str
fn vendor_name(&self) -> &str
```

**via `arrow_schema::extension::ExtensionType`**

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
fn metadata(&self) -> &Self::Metadata
fn serialize_metadata(&self) -> Option<String>
fn supports_data_type(&self, _data_type: &DataType) -> Result<(), ArrowError>
fn try_new(_data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
fn validate(_data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

**via `core::convert::From`**

```rust
fn from(value: OpaqueMetadata) -> Self
```

The extension type for `Opaque`.

Extension name: `arrow.opaque`.

Opaque represents a type that an Arrow-based system received from an
external (often non-Arrow) system, but that it cannot interpret. In this
case, it can pass on Opaque to its clients to at least show that a field
exists and preserve metadata about the type from the other system.

The storage type of this extension is any type. If there is no underlying
data, the storage type should be Null.

---

## OpaqueMetadata

`struct` · `arrow_schema::extension::canonical::opaque::OpaqueMetadata`

```rust
struct OpaqueMetadata
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(type_name: impl Into<String>, vendor_name: impl Into<String>) -> Self
fn type_name(&self) -> &str
fn vendor_name(&self) -> &str
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

Extension type metadata for [`Opaque`].

---
