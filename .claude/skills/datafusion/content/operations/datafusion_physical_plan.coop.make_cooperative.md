# `datafusion_physical_plan::coop::make_cooperative`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coop.make_cooperative.json).

<a id="op-5ca0ae018ad6b781199ba404"></a>
## make_cooperative

`function` · `datafusion_physical_plan::coop::make_cooperative` · datafusion-physical-plan 55.1.0

```rust
fn make_cooperative(stream: SendableRecordBatchStream) -> SendableRecordBatchStream
```

Source: `src/coop.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Wraps a `SendableRecordBatchStream` inside a [`CooperativeStream`](../operations/datafusion_physical_plan.coop.CooperativeStream.md#op-449130afa9608dcbf1fd3b4f) to enable cooperative multitasking.
Since `SendableRecordBatchStream` is a `dyn RecordBatchStream` this requires the use of dynamic
method dispatch.
When the stream type is statically known, consider use the generic [`cooperative`](../operations/datafusion_physical_plan.coop.cooperative.md#op-7cac1f0905838379990a4e69) function
to allow static method dispatch.
