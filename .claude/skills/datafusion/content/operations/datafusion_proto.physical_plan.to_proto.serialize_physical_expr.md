# `datafusion_proto::physical_plan::to_proto::serialize_physical_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_physical_expr.json).

<a id="op-eb7db89a4be180e8e9336fc6"></a>
## serialize_physical_expr

`function` · `datafusion_proto::physical_plan::to_proto::serialize_physical_expr` · datafusion-proto 55.1.0

```rust
fn serialize_physical_expr(value: &std::sync::Arc<dyn PhysicalExpr>, codec: &dyn PhysicalExtensionCodec) -> datafusion_common::Result<protobuf::PhysicalExprNode>
```

Source: `src/physical_plan/to_proto.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a `PhysicalExpr` to default protobuf representation.

If required, a [`PhysicalExtensionCodec`](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md#op-850a12f382d8c2d858dedae2) can be provided which can handle
serialization of udfs requiring specialized serialization (see [`PhysicalExtensionCodec::try_encode_udf`](../operations/datafusion_proto.physical_plan.PhysicalExtensionCodec.md#op-a736e58f5dfc7fe25d5e9e61))
