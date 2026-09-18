# `datafusion_physical_plan::streaming::PartitionStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.streaming.PartitionStream.json).

<a id="op-477eb3d02543add98f425b63"></a>
## PartitionStream

`trait` · `datafusion_physical_plan::streaming::PartitionStream` · datafusion-physical-plan 55.1.0

```rust
trait PartitionStream: Debug + Send + Sync
```

Source: `src/streaming.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A partition that can be converted into a [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6)

Combined with [`StreamingTableExec`](../operations/datafusion_physical_plan.streaming.StreamingTableExec.md#op-318fccba34a8cefdd69316b3), you can use this trait to implement
[`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) for a custom source with less boiler plate than
implementing `ExecutionPlan` directly for many use cases.

<a id="op-2e9e4e3702c04b550fd37a39"></a>
## execute

`function` · `datafusion_physical_plan::streaming::PartitionStream::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, ctx: Arc<TaskContext>) -> SendableRecordBatchStream
```

Source: `src/streaming.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a stream yielding this partitions values

<a id="op-931fd91c88a49a1607d5cc50"></a>
## schema

`function` · `datafusion_physical_plan::streaming::PartitionStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Source: `src/streaming.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the schema of this partition
