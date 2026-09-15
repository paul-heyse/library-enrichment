# `arrow_schema::extension::canonical::bool8`

Crate `arrow-schema` · 1 public items · structured records in [`model/arrow_schema.extension.canonical.bool8.json`](../model/arrow_schema.extension.canonical.bool8.json)

## Bool8

`struct` · `arrow_schema::extension::canonical::bool8::Bool8`

```rust
struct Bool8
```

**Implements**: `arrow_schema::extension::ExtensionType`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

**via `arrow_schema::extension::ExtensionType`**

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
fn metadata(&self) -> &Self::Metadata
fn serialize_metadata(&self) -> Option<String>
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
fn try_new(data_type: &DataType, _metadata: Self::Metadata) -> Result<Self, ArrowError>
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

The extension type for `8-bit Boolean`.

Extension name: `arrow.bool8`.

The storage type of the extension is `Int8` where:
- false is denoted by the value 0.
- true can be specified using any non-zero value. Preferably 1.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#bit-boolean>

---
