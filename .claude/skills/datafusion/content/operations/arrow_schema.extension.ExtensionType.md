# `arrow_schema::extension::ExtensionType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.ExtensionType.json).

<a id="op-efe5f6895e277f3bfd68ba9f"></a>
## ExtensionType

`trait` · `arrow_schema::extension::ExtensionType` · arrow-schema 59.3.0

```rust
trait ExtensionType: Sized
```

Source: `src/extension/mod.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Extension types.

User-defined “extension” types can be defined setting certain key value
pairs in the [`Field`] metadata structure. These extension keys are:
- [`EXTENSION_TYPE_NAME_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_NAME_KEY.md#op-1d143f6e8f659d207021ff6f)
- [`EXTENSION_TYPE_METADATA_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.md#op-c38aeda07fc1041ad3e5ea8f)

Canonical extension types support in this crate requires the
`canonical_extension_types` feature.

Extension types may or may not use the [`EXTENSION_TYPE_METADATA_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.md#op-c38aeda07fc1041ad3e5ea8f)
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

<a id="op-7f9576d5351b22ca3595d907"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::ExtensionType::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Source: `src/extension/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The metadata type of this extension type.

Implementations can use strongly or loosly typed data structures here
depending on the complexity of the metadata.

Implementations can also use `Self` here if the extension type can be
constructed directly from its metadata.

If an extension type defines no metadata it should use `()` to indicate
this.

<a id="op-fc4fcaa150f20d01bc853b71"></a>
## NAME

`assoc_const` · `arrow_schema::extension::ExtensionType::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Source: `src/extension/mod.rs:207`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The name identifying this extension type.

This is the string value that is used for the
[`EXTENSION_TYPE_NAME_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_NAME_KEY.md#op-1d143f6e8f659d207021ff6f) in the [`Field::metadata`] of a [`Field`]
to identify this extension type.

We recommend that you use a “namespace”-style prefix for extension
type names to minimize the possibility of conflicts with multiple Arrow
readers and writers in the same application. For example, use
`myorg.name_of_type` instead of simply `name_of_type`.

Extension names beginning with `arrow.` are reserved for canonical
extension types, they should not be used for third-party extension
types.

Extension names are case-sensitive.

[`Field`]: crate::Field
[`Field::metadata`]: crate::Field::metadata

<a id="op-8dc143b1119a7546255404f2"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::ExtensionType::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Source: `src/extension/mod.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Deserialize the metadata of this extension type from the serialized
representation of the metadata. An extension type that defines no
metadata should expect `None` for the serialized metadata and return
`Ok(())`.

This function should return an error when
- expected metadata is missing (for extensions types with non-optional
  metadata)
- unexpected metadata is set (for extension types without metadata)
- deserialization of metadata fails

<a id="op-8134ca5e175f9de2cf842438"></a>
## metadata

`function` · `arrow_schema::extension::ExtensionType::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Source: `src/extension/mod.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a reference to the metadata of this extension type, or `&()` if
if this extension type defines no metadata (`Self::Metadata=()`).

<a id="op-95048e2ed1972cc07160097a"></a>
## serialize_metadata

`function` · `arrow_schema::extension::ExtensionType::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Source: `src/extension/mod.rs:235`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns the serialized representation of the metadata of this extension
type, or `None` if this extension type defines no metadata
(`Self::Metadata=()`).

This is string value that is used for the
[`EXTENSION_TYPE_METADATA_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.md#op-c38aeda07fc1041ad3e5ea8f) in the [`Field::metadata`] of a
[`Field`].

[`Field`]: crate::Field
[`Field::metadata`]: crate::Field::metadata

<a id="op-3521ef845c8e02ad4e4d2450"></a>
## supports_data_type

`function` · `arrow_schema::extension::ExtensionType::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
```

Source: `src/extension/mod.rs:251`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns `Ok(())` iff the given data type is supported by this extension
type.

<a id="op-226955309e0587a6d6c9751b"></a>
## try_new

`function` · `arrow_schema::extension::ExtensionType::try_new` · arrow-schema 59.3.0

```rust
fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Source: `src/extension/mod.rs:258`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Construct this extension type for a field with the given data type and
metadata.

This should return an error if the given data type is not supported by
this extension type.

<a id="op-937e747629ba2188834f2a79"></a>
## try_new_from_field_metadata

`function` · `arrow_schema::extension::ExtensionType::try_new_from_field_metadata` · arrow-schema 59.3.0

```rust
fn try_new_from_field_metadata(data_type: &DataType, metadata: &HashMap<String, String>) -> Result<Self, ArrowError>
```

Source: `src/extension/mod.rs:284`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Construct this extension type from field metadata and data type.

This is a provided method that extracts extension type information from
metadata (using [`EXTENSION_TYPE_NAME_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_NAME_KEY.md#op-1d143f6e8f659d207021ff6f) and
[`EXTENSION_TYPE_METADATA_KEY`](../operations/arrow_schema.extension.EXTENSION_TYPE_METADATA_KEY.md#op-c38aeda07fc1041ad3e5ea8f)) and delegates to [`Self::try_new`](../operations/arrow_schema.extension.ExtensionType.md#op-226955309e0587a6d6c9751b).

Returns an error if:
- The extension type name is missing or doesn't match [`Self::NAME`](../operations/arrow_schema.extension.ExtensionType.md#op-fc4fcaa150f20d01bc853b71)
- Metadata deserialization fails
- The data type is not supported

This method enables extension type checking without requiring a full
[`Field`] instance, useful when only metadata and data type are available.

[`Field`]: crate::Field

<a id="op-e773d8931cb5b2cb39d2a6a0"></a>
## validate

`function` · `arrow_schema::extension::ExtensionType::validate` · arrow-schema 59.3.0

```rust
fn validate(data_type: &DataType, metadata: Self::Metadata) -> Result<(), ArrowError>
```

Source: `src/extension/mod.rs:265`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Validate this extension type for a field with the given data type and
metadata.

The default implementation delegates to [`Self::try_new`](../operations/arrow_schema.extension.ExtensionType.md#op-226955309e0587a6d6c9751b). Extension
types may override this to validate without constructing `Self`.
