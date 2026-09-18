# `datafusion_common::metadata::ScalarAndMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.metadata.ScalarAndMetadata.json).

<a id="op-c0a2d9bfdd3bd171f9807adf"></a>
## ScalarAndMetadata

`struct` · `datafusion_common::metadata::ScalarAndMetadata` · datafusion-common 55.1.0

```rust
struct ScalarAndMetadata
```

Source: `src/metadata.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) with optional [`FieldMetadata`](../operations/datafusion_common.metadata.FieldMetadata.md#op-8afcac4d754acb196812c528)

<a id="op-3494badbb48ac5f89e6ef8d7"></a>
## cast_storage_to

`function` · `datafusion_common::metadata::ScalarAndMetadata::cast_storage_to` · datafusion-common 55.1.0

```rust
fn cast_storage_to(&self, target_type: &DataType) -> Result<Self, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [65, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Cast this values's storage type

This operation assumes that if the underlying [ScalarValue](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) can be casted
to a given type that any extension type represented by the metadata is also
valid.

<a id="op-56c56f7ba9c6623a93c3e28d"></a>
## clone

`function` · `datafusion_common::metadata::ScalarAndMetadata::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ScalarAndMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 17], "end": [26, 22], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metadata.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a16d78925dd7f98b397e8e68"></a>
## fmt

`function` · `datafusion_common::metadata::ScalarAndMetadata::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metadata.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-209670b1928bcda3e8aa0a5d"></a>
## from

`function` · `datafusion_common::metadata::ScalarAndMetadata::from` · datafusion-common 55.1.0

```rust
fn from(value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [73, 2], "filename": "src/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::ScalarValue", "path": "ScalarValue"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metadata.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a407f652b5a91c9dc620d5c9"></a>
## into_inner

`function` · `datafusion_common::metadata::ScalarAndMetadata::into_inner` · datafusion-common 55.1.0

```rust
fn into_inner(self) -> (ScalarValue, Option<FieldMetadata>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [65, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Consume self and return components

<a id="op-05a67701d56f8bb9ba967fa6"></a>
## metadata

`function` · `datafusion_common::metadata::ScalarAndMetadata::metadata` · datafusion-common 55.1.0

```rust
fn metadata(&self) -> Option<&FieldMetadata>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [65, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Access the [FieldMetadata](../operations/datafusion_common.metadata.FieldMetadata.md#op-8afcac4d754acb196812c528) attached to this value, if any

<a id="op-6735182cda2e5bbb438176db"></a>
## metadata

`struct_field` · `datafusion_common::metadata::ScalarAndMetadata::metadata` · datafusion-common 55.1.0

```rust
metadata: Option<FieldMetadata>
```

Source: `src/metadata.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1e41837d273884a481b120c"></a>
## new

`function` · `datafusion_common::metadata::ScalarAndMetadata::new` · datafusion-common 55.1.0

```rust
fn new(value: ScalarValue, metadata: Option<FieldMetadata>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [65, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new Literal from a scalar value with optional [`FieldMetadata`](../operations/datafusion_common.metadata.FieldMetadata.md#op-8afcac4d754acb196812c528)

<a id="op-7cf0e878576f59179c25e175"></a>
## value

`function` · `datafusion_common::metadata::ScalarAndMetadata::value` · datafusion-common 55.1.0

```rust
fn value(&self) -> &ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::ScalarAndMetadata", "path": "ScalarAndMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [65, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Access the underlying [ScalarValue](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b) storage

<a id="op-ad180271f898c0d61d434b39"></a>
## value

`struct_field` · `datafusion_common::metadata::ScalarAndMetadata::value` · datafusion-common 55.1.0

```rust
value: ScalarValue
```

Source: `src/metadata.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
