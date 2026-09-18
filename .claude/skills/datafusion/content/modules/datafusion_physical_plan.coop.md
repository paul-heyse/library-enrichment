# `datafusion_physical_plan::coop`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coop.json).

<a id="op-25db457d3f0428d093a6aeb9"></a>
## coop

`module` · `datafusion_physical_plan::coop` · datafusion-physical-plan 55.1.0

```rust
mod coop
```

Source: `src/coop.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Utilities for improved cooperative scheduling.

# Cooperative scheduling

A single call to `poll_next` on a top-level [`Stream`] may potentially perform a lot of work
before it returns a `Poll::Pending`. Think for instance of calculating an aggregation over a
large dataset.

If a `Stream` runs for a long period of time without yielding back to the Tokio executor,
it can starve other tasks waiting on that executor to execute them.
Additionally, this prevents the query execution from being cancelled.

For more background, please also see the [Using Rust async for Query Execution and Cancelling Long-Running Queries blog]

[Using Rust async for Query Execution and Cancelling Long-Running Queries blog]: https://datafusion.apache.org/blog/2025/06/30/cancellation

To ensure that `Stream` implementations yield regularly, operators can insert explicit yield
points using the utilities in this module. For most operators this is **not** necessary. The
`Stream`s of the built-in DataFusion operators that generate (rather than manipulate)
`RecordBatch`es such as `DataSourceExec` and those that eagerly consume `RecordBatch`es
(for instance, `RepartitionExec`) contain yield points that will make most query `Stream`s yield
periodically.

There are a couple of types of operators that _should_ insert yield points:
- New source operators that do not make use of Tokio resources
- Exchange like operators that do not use Tokio's `Channel` implementation to pass data between
  tasks

## Adding yield points

Yield points can be inserted manually using the facilities provided by the
[Tokio coop module](https://docs.rs/tokio/latest/tokio/task/coop/index.html) such as
[`tokio::task::coop::consume_budget`](https://docs.rs/tokio/latest/tokio/task/coop/fn.consume_budget.html).

Another option is to use the wrapper `Stream` implementation provided by this module which will
consume a unit of task budget every time a `RecordBatch` is produced.
Wrapper `Stream`s can be created using the [`cooperative`](../operations/datafusion_physical_plan.coop.cooperative.md#op-7cac1f0905838379990a4e69) and [`make_cooperative`](../operations/datafusion_physical_plan.coop.make_cooperative.md#op-5ca0ae018ad6b781199ba404) functions.

[`cooperative`](../operations/datafusion_physical_plan.coop.cooperative.md#op-7cac1f0905838379990a4e69) is a generic function that takes ownership of the wrapped [`RecordBatchStream`](../operations/datafusion_execution.stream.RecordBatchStream.md#op-6a21cda33374b9fc18902881).
This function has the benefit of not requiring an additional heap allocation and can avoid
dynamic dispatch.

[`make_cooperative`](../operations/datafusion_physical_plan.coop.make_cooperative.md#op-5ca0ae018ad6b781199ba404) is a non-generic function that wraps a [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6). This
can be used to wrap dynamically typed, heap allocated [`RecordBatchStream`](../operations/datafusion_execution.stream.RecordBatchStream.md#op-6a21cda33374b9fc18902881)s.

## Automatic cooperation

The `EnsureCooperative` physical optimizer rule, which is included in the default set of
optimizer rules, inspects query plans for potential cooperative scheduling issues.
It injects the [`CooperativeExec`](../operations/datafusion_physical_plan.coop.CooperativeExec.md#op-1efab241141bdbd717c01df7) wrapper `ExecutionPlan` into the query plan where necessary.
This `ExecutionPlan` uses [`make_cooperative`](../operations/datafusion_physical_plan.coop.make_cooperative.md#op-5ca0ae018ad6b781199ba404) to wrap the `Stream` of its input.

The optimizer rule currently checks the plan for exchange-like operators and leave operators
that report [`SchedulingType::NonCooperative`](../operations/datafusion_physical_plan.execution_plan.SchedulingType.md#op-87aaf2d02bb118a5b3d7c4aa) in their [plan properties](ExecutionPlan::properties).

Unresolved upstream links (retained, not inferred): ``Stream``.
