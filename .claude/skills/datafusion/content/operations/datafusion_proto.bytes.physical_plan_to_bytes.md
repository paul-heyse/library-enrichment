# `datafusion_proto::bytes::physical_plan_to_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_to_bytes.json).

<a id="op-d2996f50612b582ffca55613"></a>
## physical_plan_to_bytes

`function` · `datafusion_proto::bytes::physical_plan_to_bytes` · datafusion-proto 55.1.0

```rust
fn physical_plan_to_bytes(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<prost::bytes::Bytes>
```

Source: `src/bytes/mod.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Serialize a PhysicalPlan as bytes
