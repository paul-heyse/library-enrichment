# `arrow_schema::extension::canonical::uuid`

Crate `arrow-schema` · 1 public items · structured records in [`model/arrow_schema.extension.canonical.uuid.json`](../model/arrow_schema.extension.canonical.uuid.json)

## Uuid

`struct` · `arrow_schema::extension::canonical::uuid::Uuid`

```rust
struct Uuid
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

The extension type for `UUID`.

Extension name: `arrow.uuid`.

The storage type of the extension is `FixedSizeBinary` with a length of
16 bytes.

Note:
A specific UUID version is not required or guaranteed. This extension
represents UUIDs as `FixedSizeBinary(16)` with big-endian notation and
does not interpret the bytes in any way.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#uuid>

---
