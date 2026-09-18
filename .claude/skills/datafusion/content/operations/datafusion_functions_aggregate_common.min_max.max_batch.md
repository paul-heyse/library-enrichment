# `datafusion_functions_aggregate_common::min_max::max_batch`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.min_max.max_batch.json).

<a id="op-d21c2119a9cb909974fd0aa8"></a>
## max_batch

`function` · `datafusion_functions_aggregate_common::min_max::max_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn max_batch(values: &arrow::array::ArrayRef) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

Source: `src/min_max.rs:764`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

dynamically-typed max(array) -> ScalarValue
