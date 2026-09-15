# `datafusion_common::types::extension`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.types.extension.json`](../model/datafusion_common.types.extension.json)

## DFExtensionType

`trait` · `datafusion_common::types::extension::DFExtensionType`

```rust
trait DFExtensionType: Debug + Send + Sync
```

**Implementors** (7)

- `datafusion_common::types::canonical_extensions::bool8::DFBool8`
- `datafusion_common::types::canonical_extensions::fixed_shape_tensor::DFFixedShapeTensor`
- `datafusion_common::types::canonical_extensions::json::DFJson`
- `datafusion_common::types::canonical_extensions::opaque::DFOpaque`
- `datafusion_common::types::canonical_extensions::timestamp_with_offset::DFTimestampWithOffset`
- `datafusion_common::types::canonical_extensions::uuid::DFUuid`
- `datafusion_common::types::canonical_extensions::variable_shape_tensor::DFVariableShapeTensor`

**Methods** (3)

```rust
fn create_array_formatter<'fmt>(&self, _array: &'fmt dyn Array, _options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
fn serialize_metadata(&self) -> Option<String>
fn storage_type(&self) -> DataType
```

Represents an implementation of a DataFusion extension type, including the storage [`DataType`].
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
needs. [`DFExtensionType`] is DataFusion's implementation of this extension type mechanism.

Furthermore, the current trait in arrow-rs is not dyn-compatible, which we need for implementing
extension type registries. In the future, the two implementations may increasingly converge.

Another difference is that [`DFExtensionType`] represents a fully resolved extension type that
has a fixed storage type (i.e., [`DataType`]). This is different from arrow-rs, which only
stores the extension type's metadata. For example, an instance of DataFusion's JSON extension
type fixes one of the three possible storage types: [`DataType::Utf8`],
[`DataType::LargeUtf8`], or [`DataType::Utf8View`]. This fixed storaga type is returned in
[`DFExtensionType::storage_type`]. This is not possible in arrow-rs' extension type instances.
This is the reason why we have different types in DataFusion that usually delegate the metadata
processing to the underlying arrow-rs extension type instance
(e.g., [`DFJson`](crate::types::DFJson) instead of [`Json`](arrow_schema::extension::Json)).

# Examples

Examples for using the extension type machinery can be found in the DataFusion examples
directory.

---

## DFExtensionTypeRef

`type_alias` · `datafusion_common::types::extension::DFExtensionTypeRef`

```rust
type DFExtensionTypeRef = std::sync::Arc<dyn DFExtensionType>
```

A cheaply cloneable pointer to a [`DFExtensionType`].

---
