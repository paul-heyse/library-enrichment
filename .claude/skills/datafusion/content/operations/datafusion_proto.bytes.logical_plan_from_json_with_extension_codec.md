# `datafusion_proto::bytes::logical_plan_from_json_with_extension_codec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_from_json_with_extension_codec.json).

<a id="op-2c6b1865e834491edfd6560b"></a>
## logical_plan_from_json_with_extension_codec

`function` · `datafusion_proto::bytes::logical_plan_from_json_with_extension_codec` · datafusion-proto 55.1.0

```rust
fn logical_plan_from_json_with_extension_codec(json: &str, ctx: &datafusion_execution::TaskContext, extension_codec: &dyn LogicalExtensionCodec) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

Source: `src/bytes/mod.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a LogicalPlan from JSON
