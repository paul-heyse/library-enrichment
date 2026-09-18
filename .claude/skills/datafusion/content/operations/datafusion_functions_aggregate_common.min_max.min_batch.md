# `datafusion_functions_aggregate_common::min_max::min_batch`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.min_max.min_batch.json).

<a id="op-46cba7f3b0dbd7b156ab1bb5"></a>
## min_batch

`function` · `datafusion_functions_aggregate_common::min_max::min_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn min_batch(values: &arrow::array::ArrayRef) -> datafusion_common::Result<datafusion_common::ScalarValue>
```

Source: `src/min_max.rs:686`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

dynamically-typed min(array) -> ScalarValue
