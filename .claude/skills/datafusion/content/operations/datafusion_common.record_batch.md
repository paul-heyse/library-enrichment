# `datafusion_common::record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.record_batch.json).

<a id="op-dae9006af6221760c8e1b6da"></a>
## record_batch

`macro` · `datafusion_common::record_batch` · datafusion-common 55.1.0

```rust
macro_rules! record_batch
```

Source: `src/test_util.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a record batch from literal slice of values, suitable for rapid
testing and development.

**Deprecated**: prefer the upstream macro from `arrow`,
[`arrow::array::record_batch`](../operations/arrow_array.record_batch.md#op-4641f570cd1f55102adb1f0f), which now supports both the literal slice
form shown below and a variable/expression form.

Example:
```
use arrow::array::record_batch;
let batch = record_batch!(
    ("a", Int32, vec![1, 2, 3]),
    ("b", Float64, vec![Some(4.0), None, Some(5.0)]),
    ("c", Utf8, vec!["alpha", "beta", "gamma"])
);
```
