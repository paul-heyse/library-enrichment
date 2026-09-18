# `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.proto.logical_extension_codec.ForeignLogicalExtensionCodec.json).

<a id="op-67eabd1453b37f78cb5f4a0f"></a>
## ForeignLogicalExtensionCodec

`struct` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec` · datafusion-ffi 55.1.0

```rust
struct ForeignLogicalExtensionCodec
```

Source: `src/proto/logical_extension_codec.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_LogicalExtensionCodec to interact with the foreign table provider.

<a id="op-36ac04959077e755ea9ab377"></a>
## 0

`struct_field` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::0` · datafusion-ffi 55.1.0

```rust
0: FFI_LogicalExtensionCodec
```

Source: `src/proto/logical_extension_codec.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05beba4dfd9d97804a8871a0"></a>
## fmt

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 10], "end": [353, 15], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/proto/logical_extension_codec.rs:353`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40b770b509460596363a1294"></a>
## try_decode

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_decode` · datafusion-ffi 55.1.0

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[LogicalPlan], _ctx: &TaskContext) -> Result<Extension>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f4eaf8cc874e6218e7e4bf6"></a>
## try_decode_file_format

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_decode_file_format` · datafusion-ffi 55.1.0

```rust
fn try_decode_file_format(&self, _buf: &[u8], _ctx: &TaskContext) -> Result<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afdd6d7fc47e02f954a55608"></a>
## try_decode_table_provider

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_decode_table_provider` · datafusion-ffi 55.1.0

```rust
fn try_decode_table_provider(&self, buf: &[u8], table_ref: &TableReference, schema: SchemaRef, _ctx: &TaskContext) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-826330cd192970dc297c6c1d"></a>
## try_decode_udaf

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_decode_udaf` · datafusion-ffi 55.1.0

```rust
fn try_decode_udaf(&self, name: &str, buf: &[u8]) -> Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:464`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2b0721bff0a060464e6e982"></a>
## try_decode_udf

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_decode_udf` · datafusion-ffi 55.1.0

```rust
fn try_decode_udf(&self, name: &str, buf: &[u8]) -> Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:446`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cdb64da20d5df7bc32a1b17"></a>
## try_decode_udwf

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_decode_udwf` · datafusion-ffi 55.1.0

```rust
fn try_decode_udwf(&self, name: &str, buf: &[u8]) -> Result<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78dc5038b6fb9f9f99925cce"></a>
## try_encode

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_encode` · datafusion-ffi 55.1.0

```rust
fn try_encode(&self, _node: &Extension, _buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a840f148ddc7b7b939bcc2a"></a>
## try_encode_file_format

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_encode_file_format` · datafusion-ffi 55.1.0

```rust
fn try_encode_file_format(&self, _buf: &mut Vec<u8>, _node: Arc<dyn FileFormatFactory>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:438`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-020c096d8d8827bba1481740"></a>
## try_encode_table_provider

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_encode_table_provider` · datafusion-ffi 55.1.0

```rust
fn try_encode_table_provider(&self, table_ref: &TableReference, node: Arc<dyn TableProvider>, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da19e14c190e000db36f8ac1"></a>
## try_encode_udaf

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_encode_udaf` · datafusion-ffi 55.1.0

```rust
fn try_encode_udaf(&self, node: &AggregateUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82c7938589a115d990092283"></a>
## try_encode_udf

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_encode_udf` · datafusion-ffi 55.1.0

```rust
fn try_encode_udf(&self, node: &ScalarUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f64b652e76eda238f5c2136b"></a>
## try_encode_udwf

`function` · `datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec::try_encode_udwf` · datafusion-ffi 55.1.0

```rust
fn try_encode_udwf(&self, node: &WindowUDF, buf: &mut Vec<u8>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::proto::logical_extension_codec::ForeignLogicalExtensionCodec", "path": "ForeignLogicalExtensionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [375, 1], "end": [501, 2], "filename": "src/proto/logical_extension_codec.rs"}, "trait": {"args": null, "id": "datafusion_proto::logical_plan::LogicalExtensionCodec", "path": "LogicalExtensionCodec"}, "trait_path": "datafusion_proto::logical_plan::LogicalExtensionCodec"}`

Source: `src/proto/logical_extension_codec.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
