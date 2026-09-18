# `datafusion_common::types::canonical_extensions::opaque::DFOpaque`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.canonical_extensions.opaque.DFOpaque.json).

<a id="op-09d75bef61902c24680a7d77"></a>
## DFOpaque

`struct` · `datafusion_common::types::canonical_extensions::opaque::DFOpaque` · datafusion-common 55.1.0

```rust
struct DFOpaque
```

Source: `src/types/canonical_extensions/opaque.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Defines the extension type logic for the canonical `arrow.opaque` extension type. This extension
type represents types that DataFusion cannot interpret.

See [`DFExtensionType`](../operations/datafusion_common.types.extension.DFExtensionType.md#op-29e508bd346af7e06ec89300) for information on DataFusion's extension type mechanism. See also
[`Opaque`](../operations/arrow_schema.extension.canonical.opaque.Opaque.md#op-fb0ef27770fce0cdd79de96e) for the implementation of arrow-rs, which this type uses internally.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#opaque>

<a id="op-9e6b5ced7fd8558440a2f34c"></a>
## clone

`function` · `datafusion_common::types::canonical_extensions::opaque::DFOpaque::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFOpaque
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::opaque::DFOpaque", "path": "DFOpaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 22], "filename": "src/types/canonical_extensions/opaque.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/canonical_extensions/opaque.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-130f1c4e602274fb7800e31d"></a>
## fmt

`function` · `datafusion_common::types::canonical_extensions::opaque::DFOpaque::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::opaque::DFOpaque", "path": "DFOpaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/types/canonical_extensions/opaque.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/canonical_extensions/opaque.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af17ae2abc5c868fb94dbe48"></a>
## serialize_metadata

`function` · `datafusion_common::types::canonical_extensions::opaque::DFOpaque::serialize_metadata` · datafusion-common 55.1.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::opaque::DFOpaque", "path": "DFOpaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [58, 2], "filename": "src/types/canonical_extensions/opaque.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/opaque.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4335ba80477cfc98e36f64e1"></a>
## storage_type

`function` · `datafusion_common::types::canonical_extensions::opaque::DFOpaque::storage_type` · datafusion-common 55.1.0

```rust
fn storage_type(&self) -> DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::opaque::DFOpaque", "path": "DFOpaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [58, 2], "filename": "src/types/canonical_extensions/opaque.rs"}, "trait": {"args": null, "id": "datafusion_common::types::extension::DFExtensionType", "path": "DFExtensionType"}, "trait_path": "datafusion_common::types::extension::DFExtensionType"}`

Source: `src/types/canonical_extensions/opaque.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eb6551bde72a5a80f713c94"></a>
## try_new

`function` · `datafusion_common::types::canonical_extensions::opaque::DFOpaque::try_new` · datafusion-common 55.1.0

```rust
fn try_new(data_type: &DataType, metadata: <Opaque as ExtensionType>::Metadata) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::canonical_extensions::opaque::DFOpaque", "path": "DFOpaque"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [48, 2], "filename": "src/types/canonical_extensions/opaque.rs"}, "trait": null, "trait_path": null}`

Source: `src/types/canonical_extensions/opaque.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`DFOpaque`](../operations/datafusion_common.types.canonical_extensions.opaque.DFOpaque.md#op-09d75bef61902c24680a7d77), validating that the storage type is compatible with the
extension type.
