# `datafusion_physical_plan::execution_plan::Boundedness::Unbounded`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.Boundedness.Unbounded.json).

<a id="op-14634c8d1e708c62d2cbd54a"></a>
## requires_infinite_memory

`struct_field` · `datafusion_physical_plan::execution_plan::Boundedness::Unbounded::requires_infinite_memory` · datafusion-physical-plan 55.1.0

```rust
requires_infinite_memory: bool
```

Source: `src/execution_plan.rs:1312`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Whether this operator requires infinite memory to process the unbounded stream.
If false, the operator can process an infinite stream with bounded memory.
If true, memory usage may grow unbounded while processing the stream.

For example, `Median` requires infinite memory to compute the median of an unbounded stream.
`Min/Max` requires infinite memory if the stream is unordered, but can be computed with bounded memory if the stream is ordered.
