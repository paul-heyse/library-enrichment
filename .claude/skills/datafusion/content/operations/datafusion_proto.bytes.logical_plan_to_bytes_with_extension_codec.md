# `datafusion_proto::bytes::logical_plan_to_bytes_with_extension_codec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_to_bytes_with_extension_codec.json).

<a id="op-f19babac159b03c856e7b204"></a>
## logical_plan_to_bytes_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_to_bytes_with_extension_codec` · datafusion-proto 55.1.0

```rust
fn logical_plan_to_bytes_with_extension_codec(plan: &datafusion_expr::LogicalPlan, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<prost::bytes::Bytes>
```

Source: `src/bytes/mod.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a LogicalPlan as bytes, using the provided extension codec
