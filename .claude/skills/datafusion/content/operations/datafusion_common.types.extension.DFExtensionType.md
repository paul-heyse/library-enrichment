# `datafusion_common::types::extension::DFExtensionType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.extension.DFExtensionType.json).

<a id="op-29e508bd346af7e06ec89300"></a>
## DFExtensionType

`trait` · `datafusion_common::types::extension::DFExtensionType` · datafusion-common 55.1.0

```rust
trait DFExtensionType: Debug + Send + Sync
```

Source: `src/types/extension.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Represents an implementation of a DataFusion extension type, including the storage [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).
While, in general, an extension type can support several different storage types, a specific
instance of it is always locked into just one exact storage type and metadata pairing.

This trait allows users to customize the behavior of DataFusion for certain types. Having this
ability is necessary because extension types affect how columns should be treated by the query
engine. This effect includes which operations are possible on a column and what are the expected
results from these operations. The extension type mechanism allows users to define how these
operations apply to a particular extension type.

For example, adding two values of [`Int64`](arrow::datatypes::DataType::Int64) is a sensible
thing to do. However, if the same column is annotated with an extension type like `custom.id`,
the correct interpretation of a column changes. Adding together two `custom.id` values, even
though they are stored as integers, may no longer make sense.

Note that DataFusion's extension type support is still young and therefore might not cover all
relevant use cases. Currently, the following operations can be customized:
- Pretty-printing values in record batches

# Relation to Arrow's [`ExtensionType`](arrow_schema::extension::ExtensionType)

The purpose of Arrow's [`ExtensionType`](arrow_schema::extension::ExtensionType) trait, for the
time being, is to allow reading and writing extension type metadata in a type-safe manner. The
trait does not provide any customization options. Therefore, downstream users (such as
DataFusion) have the flexibility to implement the extension type mechanism according to their
needs. [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) is DataFusion's implementation of this extension type mechanism.

Furthermore, the current trait in arrow-rs is not dyn-compatible, which we need for implementing
extension type registries. In the future, the two implementations may increasingly converge.

Another difference is that [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) represents a fully resolved extension type that
has a fixed storage type (i.e., [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)). This is different from arrow-rs, which only
stores the extension type's metadata. For example, an instance of DataFusion's JSON extension
type fixes one of the three possible storage types: [`DataType::Utf8`](../operations/arrow_schema.datatype.DataType.md#op-4e52d4ade5cb975c8f4c452a),
[`DataType::LargeUtf8`](../operations/arrow_schema.datatype.DataType.md#op-d897e1ec91191af958fa69ca), or [`DataType::Utf8View`](../operations/arrow_schema.datatype.DataType.md#op-a3c8435fd0f9a132833a8fc0). This fixed storaga type is returned in
[`DFExtensionType::storage_type`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-30920c2386a57f70f8816259). This is not possible in arrow-rs' extension type instances.
This is the reason why we have different types in DataFusion that usually delegate the metadata
processing to the underlying arrow-rs extension type instance
(e.g., [`DFJson`](crate::types::DFJson) instead of [`Json`](arrow_schema::extension::Json)).

# Examples

Examples for using the extension type machinery can be found in the DataFusion examples
directory.

<a id="op-d5fe4a309046806d01806811"></a>
## create_array_formatter

`function` · `datafusion_common::types::extension::DFExtensionType::create_array_formatter` · datafusion-common 55.1.0

```rust
fn create_array_formatter<'fmt>(&self, _array: &'fmt dyn Array, _options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
```

Source: `src/types/extension.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns an [`ArrayFormatter`](../operations/arrow_cast.display.ArrayFormatter.md#op-c2a994be3b7cca3db9a55992) that can format values of this type.

If `Ok(None)` is returned, the default implementation will be used.
If an error is returned, there was an error creating the formatter.

<a id="op-ca86e9933ef3edca5402cf3c"></a>
## serialize_metadata

`function` · `datafusion_common::types::extension::DFExtensionType::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Source: `src/types/extension.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the serialized metadata.

<a id="op-30920c2386a57f70f8816259"></a>
## storage_type

`function` · `datafusion_common::types::extension::DFExtensionType::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Source: `src/types/extension.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the underlying storage type.
