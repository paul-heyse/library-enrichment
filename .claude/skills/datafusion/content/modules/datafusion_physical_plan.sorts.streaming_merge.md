# `datafusion_physical_plan::sorts::streaming_merge`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.streaming_merge.json).

<a id="op-3caa3351d34c9f11cc6b0550"></a>
## streaming_merge

`module` · `datafusion_physical_plan::sorts::streaming_merge` · datafusion-physical-plan 55.1.0

```rust
mod streaming_merge
```

Source: `src/sorts/streaming_merge.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Merge that deals with an arbitrary size of streaming inputs.
This is an order-preserving merge.
