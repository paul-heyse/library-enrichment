# `arrow_array::record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.record_batch.json).

<a id="op-4641f570cd1f55102adb1f0f"></a>
## record_batch

`macro` · `arrow_array::record_batch` · arrow-array 59.3.0

```rust
macro_rules! record_batch
```

Source: `src/record_batch.rs:181`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a record batch from literal slice of values, suitable for rapid
testing and development.

Example:

```rust
use arrow_array::record_batch;
use arrow_schema;

let batch = record_batch!(
    ("a", Int32, [1, 2, 3]),
    ("b", Float64, [Some(4.0), None, Some(5.0)]),
    ("c", Utf8, ["alpha", "beta", "gamma"])
);
```

Variables and expressions are also supported:

```rust
use arrow_array::record_batch;

let values = vec![1, 2, 3];
let batch = record_batch!(
    ("a", Int32, values),
    ("b", Float64, vec![Some(4.0), None, Some(5.0)])
);
```
Due to limitation of [`create_array!`](../operations/arrow_array.create_array.md#op-44a423b9f37b4517fb89d1cc) macro, support for limited data types is available.
