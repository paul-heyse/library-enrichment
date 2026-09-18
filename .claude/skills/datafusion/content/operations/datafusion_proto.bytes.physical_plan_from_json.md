# `datafusion_proto::bytes::physical_plan_from_json`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_from_json.json).

<a id="op-2af294dfc67a6f88ef40adbe"></a>
## physical_plan_from_json

`function` · `datafusion_proto::bytes::physical_plan_from_json` · datafusion-proto 55.1.0

```rust
fn physical_plan_from_json(json: &str, ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/bytes/mod.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a PhysicalPlan from JSON
