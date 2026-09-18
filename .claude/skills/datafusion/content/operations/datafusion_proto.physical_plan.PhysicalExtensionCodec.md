# `datafusion_proto::physical_plan::PhysicalExtensionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.PhysicalExtensionCodec.json).

<a id="op-850a12f382d8c2d858dedae2"></a>
## PhysicalExtensionCodec

`trait` · `datafusion_proto::physical_plan::PhysicalExtensionCodec` · datafusion-proto 55.1.0

```rust
trait PhysicalExtensionCodec: Debug + Send + Sync + Any
```

Source: `src/physical_plan/mod.rs:1535`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-868d965d547d5346aa43ffd3"></a>
## try_decode

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], ctx: &TaskContext, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_plan/mod.rs:1536`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02dc6012c6efc2e24ed580ae"></a>
## try_decode_expr

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_decode_expr` · datafusion-proto 55.1.0

```rust
fn try_decode_expr(&self, _buf: &[u8], _inputs: &[Arc<dyn PhysicalExpr>], _ctx: &PhysicalExprDecodeCtx<'_>) -> Result<Arc<dyn PhysicalExpr>>
```

Source: `src/physical_plan/mod.rs:1591`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Decode a custom extension expression from `buf`.

`inputs` holds the already-decoded children carried in the
`PhysicalExtensionExprNode.inputs` field. If the codec instead embeds
nested `PhysicalExprNode`s *inside* `buf`, decode them through
`ctx.decode(..)` (equivalently [`PhysicalExprDecodeCtx::decode`]) rather
than the free [`parse_physical_expr`] function: `ctx` carries the active
schema and task context (so UDF/column references resolve against the
real registry) and routes through any active `DeduplicatingDeserializer`,
so a shared inner expression (e.g. a `DynamicFilterPhysicalExpr`
referenced both from a `SortExec.filter` and from inside this blob)
cache-hits on its `expr_id` and re-shares one `Arc<dyn PhysicalExpr>`.

[`parse_physical_expr`]: crate::physical_plan::from_proto::parse_physical_expr

Unresolved upstream links (retained, not inferred): ``PhysicalExprDecodeCtx::decode``.

<a id="op-cb2cf1e837ae99f5a7686bc7"></a>
## try_decode_higher_order_function

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_decode_higher_order_function` · datafusion-proto 55.1.0

```rust
fn try_decode_higher_order_function(&self, name: &str, _buf: &[u8]) -> Result<Arc<HigherOrderUDF>>
```

Source: `src/physical_plan/mod.rs:1559`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-024a698f530df3071300b8d4"></a>
## try_decode_udaf

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_decode_udaf` · datafusion-proto 55.1.0

```rust
fn try_decode_udaf(&self, name: &str, _buf: &[u8]) -> Result<Arc<AggregateUDF>>
```

Source: `src/physical_plan/mod.rs:1619`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a6b0fa7a7c18cac9cccecc4"></a>
## try_decode_udf

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_decode_udf` · datafusion-proto 55.1.0

```rust
fn try_decode_udf(&self, name: &str, _buf: &[u8]) -> Result<Arc<ScalarUDF>>
```

Source: `src/physical_plan/mod.rs:1551`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4db03dfebeb323e0061c8b51"></a>
## try_decode_udwf

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_decode_udwf` · datafusion-proto 55.1.0

```rust
fn try_decode_udwf(&self, name: &str, _buf: &[u8]) -> Result<Arc<WindowUDF>>
```

Source: `src/physical_plan/mod.rs:1629`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad673b43f6a4127dce747d0b"></a>
## try_encode

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, proto_converter: &dyn PhysicalProtoConverterExtension) -> Result<()>
```

Source: `src/physical_plan/mod.rs:1544`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-706f49cfb479e1eb22cb4e47"></a>
## try_encode_expr

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_encode_expr` · datafusion-proto 55.1.0

```rust
fn try_encode_expr(&self, _node: &Arc<dyn PhysicalExpr>, _buf: &mut Vec<u8>, _ctx: &PhysicalExprEncodeCtx<'_>) -> Result<()>
```

Source: `src/physical_plan/mod.rs:1610`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Encode a custom extension expression into `buf`.

If the codec embeds nested `PhysicalExprNode`s inside `buf`, encode them
through `ctx.encode_child(..)` (equivalently
[`PhysicalExprEncodeCtx::encode_child`]) rather than the free
[`serialize_physical_expr`] function, so an active
`DeduplicatingProtoConverter` stamps matching `expr_id`s for shared
inner expressions. See [`Self::try_decode_expr`](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md#op-02dc6012c6efc2e24ed580ae).

[`serialize_physical_expr`]: crate::physical_plan::to_proto::serialize_physical_expr

Unresolved upstream links (retained, not inferred): ``PhysicalExprEncodeCtx::encode_child``.

<a id="op-86c8e90db6fc9fd960b69fb1"></a>
## try_encode_higher_order_function

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_encode_higher_order_function` · datafusion-proto 55.1.0

```rust
fn try_encode_higher_order_function(&self, _node: &HigherOrderUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/physical_plan/mod.rs:1569`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aa5fb86fc7a7369db74a44c"></a>
## try_encode_udaf

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_encode_udaf` · datafusion-proto 55.1.0

```rust
fn try_encode_udaf(&self, _node: &AggregateUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/physical_plan/mod.rs:1625`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a736e58f5dfc7fe25d5e9e61"></a>
## try_encode_udf

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_encode_udf` · datafusion-proto 55.1.0

```rust
fn try_encode_udf(&self, _node: &ScalarUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/physical_plan/mod.rs:1555`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-303e2110881f5709bd3957e4"></a>
## try_encode_udwf

`function` · `datafusion_proto::physical_plan::PhysicalExtensionCodec::try_encode_udwf` · datafusion-proto 55.1.0

```rust
fn try_encode_udwf(&self, _node: &WindowUDF, _buf: &mut Vec<u8>) -> Result<()>
```

Source: `src/physical_plan/mod.rs:1633`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
