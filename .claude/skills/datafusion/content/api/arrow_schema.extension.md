# `arrow_schema::extension`

Crate `arrow-schema` · 3 public items · structured records in [`model/arrow_schema.extension.json`](../model/arrow_schema.extension.json)

## EXTENSION_TYPE_METADATA_KEY

`constant` · `arrow_schema::extension::EXTENSION_TYPE_METADATA_KEY`

```rust
const EXTENSION_TYPE_METADATA_KEY: &str = "ARROW:extension:metadata"
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.md).


The metadata key for a serialized representation of the [`ExtensionType`]
necessary to reconstruct the custom type.

---

## EXTENSION_TYPE_NAME_KEY

`constant` · `arrow_schema::extension::EXTENSION_TYPE_NAME_KEY`

```rust
const EXTENSION_TYPE_NAME_KEY: &str = "ARROW:extension:name"
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.EXTENSION_TYPE_NAME_KEY.md).


The metadata key for the string name identifying an [`ExtensionType`].

---

## ExtensionType

`trait` · `arrow_schema::extension::ExtensionType`

```rust
trait ExtensionType: Sized
```

**Implementors** (8)

- `arrow_schema::extension::canonical::bool8::Bool8`
- `arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor`
- `arrow_schema::extension::canonical::json::Json`
- `arrow_schema::extension::canonical::opaque::Opaque`
- `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset`
- `arrow_schema::extension::canonical::uuid::Uuid`
- `arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor`
- `parquet_variant_compute::variant_array::VariantType`

**Methods** (7)

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
fn metadata(&self) -> &Self::Metadata
fn serialize_metadata(&self) -> Option<String>
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
fn try_new_from_field_metadata(data_type: &DataType, metadata: &HashMap<String, String>) -> Result<Self, ArrowError>
fn validate(data_type: &DataType, metadata: Self::Metadata) -> Result<(), ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_schema.extension.ExtensionType.md).


Extension types.

User-defined “extension” types can be defined setting certain key value
pairs in the [`Field`] metadata structure. These extension keys are:
- [`EXTENSION_TYPE_NAME_KEY`]
- [`EXTENSION_TYPE_METADATA_KEY`]

Canonical extension types support in this crate requires the
`canonical_extension_types` feature.

Extension types may or may not use the [`EXTENSION_TYPE_METADATA_KEY`]
field.

# Example

The example below demonstrates how to implement this trait for a `Uuid`
type. Note this is not the canonical extension type for `Uuid`, which does
not include information about the `Uuid` version.

```
# use arrow_schema::ArrowError;
# fn main() -> Result<(), ArrowError> {
use arrow_schema::{DataType, extension::ExtensionType, Field};
use std::{fmt, str::FromStr};

/// The different Uuid versions.
#[derive(Clone, Copy, Debug, PartialEq)]
enum UuidVersion {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
}

// We'll use `Display` to serialize.
impl fmt::Display for UuidVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V1 => "V1",
                Self::V2 => "V2",
                Self::V3 => "V3",
                Self::V4 => "V4",
                Self::V5 => "V5",
                Self::V6 => "V6",
                Self::V7 => "V7",
                Self::V8 => "V8",
            }
        )
    }
}

// And `FromStr` to deserialize.
impl FromStr for UuidVersion {
    type Err = ArrowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "V1" => Ok(Self::V1),
            "V2" => Ok(Self::V2),
            "V3" => Ok(Self::V3),
            "V4" => Ok(Self::V4),
            "V5" => Ok(Self::V5),
            "V6" => Ok(Self::V6),
            "V7" => Ok(Self::V7),
            "V8" => Ok(Self::V8),
            _ => Err(ArrowError::ParseError("Invalid UuidVersion".to_owned())),
        }
    }
}

/// This is the extension type, not the container for Uuid values. It
/// stores the Uuid version (this is the metadata of this extension type).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Uuid(UuidVersion);

impl ExtensionType for Uuid {
    // We use a namespace as suggested by the specification.
    const NAME: &'static str = "myorg.example.uuid";

    // The metadata type is the Uuid version.
    type Metadata = UuidVersion;

    // We just return a reference to the Uuid version.
    fn metadata(&self) -> &Self::Metadata {
        &self.0
    }

    // We use the `Display` implementation to serialize the Uuid
    // version.
    fn serialize_metadata(&self) -> Option<String> {
        Some(self.0.to_string())
    }

    // We use the `FromStr` implementation to deserialize the Uuid
    // version.
    fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError> {
        metadata.map_or_else(
            || {
                Err(ArrowError::InvalidArgumentError(
                    "Uuid extension type metadata missing".to_owned(),
                ))
            },
            str::parse,
        )
    }

    // The only supported data type is `FixedSizeBinary(16)`.
    fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError> {
        match data_type {
            DataType::FixedSizeBinary(16) => Ok(()),
            data_type => Err(ArrowError::InvalidArgumentError(format!(
                "Uuid data type mismatch, expected FixedSizeBinary(16), found {data_type}"
            ))),
        }
    }

    // We should always check if the data type is supported before
    // constructing the extension type.
    fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError> {
        let uuid = Self(metadata);
        uuid.supports_data_type(data_type)?;
        Ok(uuid)
    }
}

// We can now construct the extension type.
let uuid_v1 = Uuid(UuidVersion::V1);

// And add it to a field.
let mut field =
    Field::new("", DataType::FixedSizeBinary(16), false).with_extension_type(uuid_v1);

// And extract it from this field.
assert_eq!(field.try_extension_type::<Uuid>()?, uuid_v1);

// When we try to add this to a field with an unsupported data type we
// get an error.
let result = Field::new("", DataType::Null, false).try_with_extension_type(uuid_v1);
assert!(result.is_err());
# Ok(()) }
```

<https://arrow.apache.org/docs/format/Columnar.html#extension-types>

[`Field`]: crate::Field

---
