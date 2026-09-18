# `datafusion_physical_plan::execution_plan::execute_stream_partitioned`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.execute_stream_partitioned.json).

<a id="op-6c3330c12f8fc5d2eece1b49"></a>
## execute_stream_partitioned

`function` · `datafusion_physical_plan::execution_plan::execute_stream_partitioned` · datafusion-physical-plan 55.1.0

```rust
fn execute_stream_partitioned(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<SendableRecordBatchStream>>
```

Source: `src/execution_plan.rs:1847`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Execute the [ExecutionPlan](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) and return a vec with one stream per output
partition

# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources
