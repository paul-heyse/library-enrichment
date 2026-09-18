# `arrow_schema::extension::canonical::json`

Crate `arrow-schema` · 2 public items · structured records in [`model/arrow_schema.extension.canonical.json.json`](../model/arrow_schema.extension.canonical.json.json)

## Json

`struct` · `arrow_schema::extension::canonical::json::Json`

```rust
struct Json
```

**Implements**: `arrow_schema::extension::ExtensionType`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `arrow_schema::extension::ExtensionType`**

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
fn metadata(&self) -> &Self::Metadata
fn serialize_metadata(&self) -> Option<String>
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.canonical.json.Json.md).


The extension type for `JSON`.

Extension name: `arrow.json`.

The storage type of this extension is `String` or `LargeString` or
`StringView`. Only UTF-8 encoded JSON as specified in [rfc8259](https://datatracker.ietf.org/doc/html/rfc8259)
is supported.

This type does not have any parameters.

Metadata is either an empty string or a JSON string with an empty
object. In the future, additional fields may be added, but they are not
required to interpret the array.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#json>

---

## JsonMetadata

`struct` · `arrow_schema::extension::canonical::json::JsonMetadata`

```rust
struct JsonMetadata
```

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.canonical.json.JsonMetadata.md).


Extension type metadata for [`Json`].

---
