# `datafusion_proto::logical_plan::LogicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.logical_plan.LogicalExtensionCodec.json).

<a id="op-4284dcb76d9545ce9708c7f6"></a>
## LogicalExtensionCodec

`trait` · `datafusion_proto::logical_plan::LogicalExtensionCodec` · datafusion-proto 55.1.0

```rust
trait LogicalExtensionCodec: Debug + Send + Sync + std::any::Any
```

Source: `src/logical_plan/mod.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2036901eeef5a2a84eb06e9"></a>
## try_decode

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, buf: &[u8], inputs: &[LogicalPlan], ctx: &TaskContext) -> Result<Extension>
```

Source: `src/logical_plan/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be355122a90c47469792ffb6"></a>
## try_decode_file_format

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_decode_file_format` · datafusion-proto 55.1.0

```rust
fn try_decode_file_format(&self, _buf: &[u8], _ctx: &TaskContext) -> Result<Arc<dyn FileFormatFactory>>
```

Source: `src/logical_plan/mod.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42e2c0415b68d7ce30014e92"></a>
## try_decode_higher_order_function

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_decode_higher_order_function` · datafusion-proto 55.1.0

```rust
fn try_decode_higher_order_function(&self, name: &str, _buf: &[u8]) -> Result<Arc<HigherOrderUDF>>
```

Source: `src/logical_plan/mod.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4a5c4a5fdfb4d52dd3d6aa8"></a>
## try_decode_table_provider

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_decode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_decode_table_provider(&self, buf: &[u8], table_ref: &TableReference, schema: SchemaRef, ctx: &TaskContext) -> Result<Arc<dyn TableProvider>>
```

Source: `src/logical_plan/mod.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-505e7a30670d5df77fc49236"></a>
## try_decode_udaf

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_decode_udaf` · datafusion-proto 55.1.0

```rust
fn try_decode_udaf(&self, name: &str, _buf: &[u8]) -> Result<Arc<AggregateUDF>>
```

Source: `src/logical_plan/mod.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb7facd216824c335b7b377f"></a>
## try_decode_udf

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_decode_udf` · datafusion-proto 55.1.0

```rust
fn try_decode_udf(&self, name: &str, _buf: &[u8]) -> Result<Arc<ScalarUDF>>
```

Source: `src/logical_plan/mod.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5e08a4cf410f9a8d3316ec5"></a>
## try_decode_udwf

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_decode_udwf` · datafusion-proto 55.1.0

```rust
fn try_decode_udwf(&self, name: &str, _buf: &[u8]) -> Result<Arc<WindowUDF>>
```

Source: `src/logical_plan/mod.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65dbb4d29d089208caa38e56"></a>
## try_encode

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, node: &Extension, buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/logical_plan/mod.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7623039942ad9ab24714a0f4"></a>
## try_encode_file_format

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_encode_file_format` · datafusion-proto 55.1.0

```rust
fn try_encode_file_format(&self, _buf: &mut Vec<u8>, _node: Arc<dyn FileFormatFactory>) -> Result<()>
```

Source: `src/logical_plan/mod.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-923706976e09d52668380287"></a>
## try_encode_higher_order_function

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_encode_higher_order_function` · datafusion-proto 55.1.0

```rust
fn try_encode_higher_order_function(&self, _node: &HigherOrderUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/logical_plan/mod.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-916aafb3717b689a38fb3dee"></a>
## try_encode_table_provider

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_encode_table_provider` · datafusion-proto 55.1.0

```rust
fn try_encode_table_provider(&self, table_ref: &TableReference, node: Arc<dyn TableProvider>, buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/logical_plan/mod.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8901abdfa3f5e5bbc5bedb69"></a>
## try_encode_udaf

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_encode_udaf` · datafusion-proto 55.1.0

```rust
fn try_encode_udaf(&self, _node: &AggregateUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/logical_plan/mod.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6e954f8be91746cb40c734d"></a>
## try_encode_udf

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_encode_udf` · datafusion-proto 55.1.0

```rust
fn try_encode_udf(&self, _node: &ScalarUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/logical_plan/mod.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-460ca6a85ac5cf023497794e"></a>
## try_encode_udwf

`function` · `datafusion_proto::logical_plan::LogicalExtensionCodec::try_encode_udwf` · datafusion-proto 55.1.0

```rust
fn try_encode_udwf(&self, _node: &WindowUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/logical_plan/mod.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
