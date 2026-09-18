# `deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.datafusion_ext.DeltaDataWriterExt.json).

<a id="op-f59334d0f543607589982765"></a>
## DeltaDataWriterExt

`trait` · `deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait DeltaDataWriterExt
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L63).

Source: `crates/core/src/datafile/datafusion_ext.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

DataFusion extension to [`DeltaDataWriter`](../operations/deltalake_core.datafile.DeltaDataWriter.md#op-c1f5bba8aa5bb8d203fc6776): write the output of an execution plan.

<a id="op-4391ee4d84fca15804c03fe6"></a>
## write_plan

`function` · `deltalake_core::datafile::datafusion_ext::DeltaDataWriterExt::write_plan` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn write_plan(Box<self>, session: &dyn Session, plan: Arc<dyn ExecutionPlan>) -> DeltaResult<Vec<Add>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L66).

Source: `crates/core/src/datafile/datafusion_ext.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execute `plan` (already containing any validation/repartition/CDC nodes)
against `session` and write its output through the basic writer.
