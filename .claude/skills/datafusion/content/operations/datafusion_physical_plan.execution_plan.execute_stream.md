# `datafusion_physical_plan::execution_plan::execute_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.execute_stream.json).

<a id="op-536cfbe7789a3c57b9fc1a2e"></a>
## execute_stream

`function` · `datafusion_physical_plan::execution_plan::execute_stream` · datafusion-physical-plan 55.1.0

```rust
fn execute_stream(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream>
```

Source: `src/execution_plan.rs:1772`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execute the [ExecutionPlan](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) and return a single stream of `RecordBatch`es.

See [collect](../operations/datafusion_physical_plan.execution_plan.collect.md#op-037eade84bbecff723ca3134) to buffer the `RecordBatch`es in memory.

# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources
