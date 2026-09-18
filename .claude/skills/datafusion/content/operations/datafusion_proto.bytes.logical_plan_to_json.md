# `datafusion_proto::bytes::logical_plan_to_json`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_to_json.json).

<a id="op-8695b3fd1e7f7c7bde41f8d0"></a>
## logical_plan_to_json

`function` · `datafusion_proto::bytes::logical_plan_to_json` · datafusion-proto 55.1.0

```rust
fn logical_plan_to_json(plan: &datafusion_expr::LogicalPlan) -> datafusion_common::Result<String>
```

Source: `src/bytes/mod.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a LogicalPlan as JSON
