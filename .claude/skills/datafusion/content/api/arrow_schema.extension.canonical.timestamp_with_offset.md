# `arrow_schema::extension::canonical::timestamp_with_offset`

Crate `arrow-schema` · 1 public items · structured records in [`model/arrow_schema.extension.canonical.timestamp_with_offset.json`](../model/arrow_schema.extension.canonical.timestamp_with_offset.json)

## TimestampWithOffset

`struct` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset`

```rust
struct TimestampWithOffset
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

The extension type for `TimestampWithOffset`.

Extension name: `arrow.timestamp_with_offset`.

This type represents a timestamp column that stores potentially different timezone offsets per
value. The timestamp is stored in UTC alongside the original timezone offset in minutes. This
extension type is intended to be compatible with ANSI SQL's `TIMESTAMP WITH TIME ZONE`, which
is supported by multiple database engines.

The storage type of the extension is a `Struct` with 2 fields, in order: - `timestamp`: a
non-nullable `Timestamp(time_unit, "UTC")`, where `time_unit` is any Arrow `TimeUnit` (s, ms,
us or ns). - `offset_minutes`: a non-nullable signed 16-bit integer (`Int16`) representing the
offset in minutes from the UTC timezone. Negative offsets represent time zones west of UTC,
while positive offsets represent east. Offsets normally range from -779 (-12:59) to +780
(+13:00).

This type has no type parameters.

Metadata is either empty or an empty string.

It is also *permissible* for the `offset_minutes` field to be dictionary-encoded with a
preferred (*but not required*) index type of `int8`, or run-end-encoded with a preferred (*but
not required*) runs type of `int8`.

It's worth noting that the data source needs to resolve timezone strings such as `UTC` or
`Americas/Los_Angeles` into an offset in minutes in order to construct a `TimestampWithOffset`.
This makes `TimestampWithOffset` type "lossy" in the sense that any original "unresolved"
timezone string gets lost in this conversion. It's a tradeoff for optimizing the row
representation and simplifying the client code, which does not need to know how to convert from
timezone string to its corresponding offset in minutes.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#timestamp-with-offset>

---
