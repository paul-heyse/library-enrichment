# `datafusion_proto::bytes::logical_plan_from_json`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_from_json.json).

<a id="op-9167cd686159bb8b45a4047e"></a>
## logical_plan_from_json

`function` · `datafusion_proto::bytes::logical_plan_from_json` · datafusion-proto 55.1.0

```rust
fn logical_plan_from_json(json: &str, ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

Source: `src/bytes/mod.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a LogicalPlan from JSON
