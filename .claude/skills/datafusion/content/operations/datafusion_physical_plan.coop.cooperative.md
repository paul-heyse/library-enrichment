# `datafusion_physical_plan::coop::cooperative`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coop.cooperative.json).

<a id="op-7cac1f0905838379990a4e69"></a>
## cooperative

`function` · `datafusion_physical_plan::coop::cooperative` · datafusion-physical-plan 55.1.0

```rust
fn cooperative<T>(stream: T) -> CooperativeStream<T> where T: RecordBatchStream + Unpin + Send + 'static
```

Source: `src/coop.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a [`CooperativeStream`](../operations/datafusion_physical_plan.coop.CooperativeStream.md#op-449130afa9608dcbf1fd3b4f) wrapper around the given [`RecordBatchStream`](../operations/datafusion_execution.stream.RecordBatchStream.md#op-6a21cda33374b9fc18902881).
This wrapper collaborates with the Tokio cooperative scheduler by consuming a unit of
scheduling budget for each returned record batch.
