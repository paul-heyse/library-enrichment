# `datafusion_proto::bytes::logical_plan_from_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_from_bytes.json).

<a id="op-ceb439fdd50c6f5801cbb397"></a>
## logical_plan_from_bytes

`function` · `datafusion_proto::bytes::logical_plan_from_bytes` · datafusion-proto 55.1.0

```rust
fn logical_plan_from_bytes(bytes: &[u8], ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<datafusion_expr::LogicalPlan>
```

Source: `src/bytes/mod.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a LogicalPlan from bytes
