# `datafusion_proto::bytes::logical_plan_to_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.logical_plan_to_bytes.json).

<a id="op-bc47d399bd4f5a32572340b6"></a>
## logical_plan_to_bytes

`function` · `datafusion_proto::bytes::logical_plan_to_bytes` · datafusion-proto 55.1.0

```rust
fn logical_plan_to_bytes(plan: &datafusion_expr::LogicalPlan) -> datafusion_common::Result<prost::bytes::Bytes>
```

Source: `src/bytes/mod.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a LogicalPlan as bytes
