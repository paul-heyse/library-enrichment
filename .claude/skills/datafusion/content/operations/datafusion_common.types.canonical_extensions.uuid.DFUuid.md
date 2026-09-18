# `datafusion_common::types::canonical_extensions::uuid::DFUuid`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.canonical_extensions.uuid.DFUuid.json).

<a id="op-6f13c0d44d9266e6606a9841"></a>
## DFUuid

`struct` · `datafusion_common::types::canonical_extensions::uuid::DFUuid` · datafusion-common 55.1.0

```rust
struct DFUuid
```

Source: `src/types/canonical_extensions/uuid.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the extension type logic for the canonical `arrow.uuid` extension type. This extension
type defines that a field should be interpreted as a
[UUID](https://de.wikipedia.org/wiki/Universally_Unique_Identifier).

See [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) for information on DataFusion's extension type mechanism. See also
[`Uuid`](../operations/arrow_schema.extension.canonical.uuid.Uuid.md#op-0e9bc650e20d322ee4024590) for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#uuid>

<a id="op-b933295bd6c93aa5b808c4a8"></a>
## clone

`function` · `datafusion_common::types::canonical_extensions::uuid::DFUuid::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFUuid
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::uuid::DFUuid", "path": "DFUuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/types/canonical_extensions/uuid.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/canonical_extensions/uuid.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d6f1b507e3bae316b46227d"></a>
## create_array_formatter

`function` · `datafusion_common::types::canonical_extensions::uuid::DFUuid::create_array_formatter` · datafusion-common 55.1.0

```rust
fn create_array_formatter<'fmt>(&self, array: &'fmt dyn Array, options: &FormatOptions<'fmt>) -> Result<Option<ArrayFormatter<'fmt>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::uuid::DFUuid", "path": "DFUuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/types/canonical_extensions/uuid.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/uuid.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d82f0bd2f1dce5ef8a2bf5f0"></a>
## fmt

`function` · `datafusion_common::types::canonical_extensions::uuid::DFUuid::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::uuid::DFUuid", "path": "DFUuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/types/canonical_extensions/uuid.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/canonical_extensions/uuid.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d8983f31e73a4478e072a2e"></a>
## serialize_metadata

`function` · `datafusion_common::types::canonical_extensions::uuid::DFUuid::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::uuid::DFUuid", "path": "DFUuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/types/canonical_extensions/uuid.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/uuid.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6900e6c3b56967d4d004a79e"></a>
## storage_type

`function` · `datafusion_common::types::canonical_extensions::uuid::DFUuid::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::uuid::DFUuid", "path": "DFUuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [77, 2], "filename": "src/types/canonical_extensions/uuid.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/uuid.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc4e8eb8d7be31c0af3a80ff"></a>
## try_new

`function` · `datafusion_common::types::canonical_extensions::uuid::DFUuid::try_new` · datafusion-common 55.1.0

```rust
fn try_new(data_type: &DataType, metadata: <Uuid as ExtensionType>::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::uuid::DFUuid", "path": "DFUuid"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [48, 2], "filename": "src/types/canonical_extensions/uuid.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/canonical_extensions/uuid.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`DFUuid`](../operations/datafusion_common.types.canonical_extensions.uuid.DFUuid.md#op-6f13c0d44d9266e6606a9841), validating that the storage type is compatible with the
extension type.
