# `datafusion_proto::bytes::physical_plan_from_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.bytes.physical_plan_from_bytes.json).

<a id="op-4dff93058cf68a4a269c47cb"></a>
## physical_plan_from_bytes

`function` · `datafusion_proto::bytes::physical_plan_from_bytes` · datafusion-proto 55.1.0

```rust
fn physical_plan_from_bytes(bytes: &[u8], ctx: &datafusion_execution::TaskContext) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/bytes/mod.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Deserialize a PhysicalPlan from bytes
