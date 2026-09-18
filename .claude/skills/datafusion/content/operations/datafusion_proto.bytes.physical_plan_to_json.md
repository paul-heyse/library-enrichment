# `datafusion_proto::bytes::physical_plan_to_json`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_to_json.json).

<a id="op-63eddff51ae2bd9b67dd2d5b"></a>
## physical_plan_to_json

`function` · `datafusion_proto::bytes::physical_plan_to_json` · datafusion-proto 55.1.0

```rust
fn physical_plan_to_json(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<String>
```

Source: `src/bytes/mod.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a PhysicalPlan as JSON
