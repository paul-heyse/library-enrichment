# `datafusion_proto::bytes::logical_plan_to_json_with_extension_codec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_to_json_with_extension_codec.json).

<a id="op-9a5cdfe586f51129472ebdbc"></a>
## logical_plan_to_json_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_to_json_with_extension_codec` · datafusion-proto 55.1.0

```rust
fn logical_plan_to_json_with_extension_codec(plan: &datafusion_expr::LogicalPlan, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<String>
```

Source: `src/bytes/mod.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a LogicalPlan as JSON using the provided extension codec
