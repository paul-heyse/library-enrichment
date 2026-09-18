# `datafusion_proto::physical_plan::AsExecutionPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.AsExecutionPlan.json).

<a id="op-0184d6079a300d2b8b15e1ea"></a>
## AsExecutionPlan

`trait` · `datafusion_proto::physical_plan::AsExecutionPlan` · datafusion-proto 55.1.0

```rust
trait AsExecutionPlan: Debug + Send + Sync + Clone
```

Source: `src/physical_plan/mod.rs:1510`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c004e2786016a9dc0d10f31"></a>
## try_decode

`function` · `datafusion_proto::physical_plan::AsExecutionPlan::try_decode` · datafusion-proto 55.1.0

```rust
fn try_decode(buf: &[u8]) -> Result<Self> where Self: Sized
```

Source: `src/physical_plan/mod.rs:1511`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fc067a1013f21060b585578"></a>
## try_encode

`function` · `datafusion_proto::physical_plan::AsExecutionPlan::try_encode` · datafusion-proto 55.1.0

```rust
fn try_encode<B>(&self, buf: &mut B) -> Result<()> where B: BufMut, Self: Sized
```

Source: `src/physical_plan/mod.rs:1515`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e31c9a16b61baf934edea97"></a>
## try_from_physical_plan

`function` · `datafusion_proto::physical_plan::AsExecutionPlan::try_from_physical_plan` · datafusion-proto 55.1.0

```rust
fn try_from_physical_plan(plan: Arc<dyn ExecutionPlan>, codec: &dyn PhysicalExtensionCodec) -> Result<Self> where Self: Sized
```

Source: `src/physical_plan/mod.rs:1527`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bda9d71f02e061dd41f3a446"></a>
## try_into_physical_plan

`function` · `datafusion_proto::physical_plan::AsExecutionPlan::try_into_physical_plan` · datafusion-proto 55.1.0

```rust
fn try_into_physical_plan(&self, ctx: &TaskContext, codec: &dyn PhysicalExtensionCodec) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_plan/mod.rs:1520`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
