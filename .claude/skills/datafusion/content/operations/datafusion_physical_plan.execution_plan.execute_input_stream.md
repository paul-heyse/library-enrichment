# `datafusion_physical_plan::execution_plan::execute_input_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.execute_input_stream.json).

<a id="op-fd2d1d12cb772512aecea6b4"></a>
## execute_input_stream

`function` · `datafusion_physical_plan::execution_plan::execute_input_stream` · datafusion-physical-plan 55.1.0

```rust
fn execute_input_stream(input: std::sync::Arc<dyn ExecutionPlan>, sink_schema: arrow::datatypes::SchemaRef, partition: usize, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream>
```

Source: `src/execution_plan.rs:1882`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Executes an input stream and ensures that the resulting stream adheres to
the `not null` constraints specified in the `sink_schema`.

# Arguments

* `input` - An execution plan
* `sink_schema` - The schema to be applied to the output stream
* `partition` - The partition index to be executed
* `context` - The task context

# Returns

* `Result<SendableRecordBatchStream>` - A stream of `RecordBatch`es if successful

This function first executes the given input plan for the specified partition
and context. It then checks if there are any columns in the input that might
violate the `not null` constraints specified in the `sink_schema`. If there are
such columns, it wraps the resulting stream to enforce the `not null` constraints
by invoking the [`check_not_null_constraints`](../operations/datafusion_physical_plan.execution_plan.check_not_null_constraints.md#op-3404751a4160ce875c46325f) function on each batch of the stream.
