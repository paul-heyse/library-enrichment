# `datafusion_common::assert_batches_eq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.assert_batches_eq.json).

<a id="op-d069d2a820f5c18534d0e6a5"></a>
## assert_batches_eq

`macro` · `datafusion_common::assert_batches_eq` · datafusion-common 55.1.0

```rust
macro_rules! assert_batches_eq
```

Source: `src/test_util.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compares formatted output of a record batch with an expected
vector of strings, with the result of pretty formatting record
batches. This is a macro so errors appear on the correct line

Designed so that failure output can be directly copy/pasted
into the test code as expected results.

Expects to be called about like this:

`assert_batches_eq!(expected_lines: &[&str], batches: &[RecordBatch])`

# Example
```
# use std::sync::Arc;
# use arrow::record_batch::RecordBatch;
# use arrow::array::{ArrayRef, Int32Array};
# use datafusion_common::assert_batches_eq;
let col: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
let batch = RecordBatch::try_from_iter([("column", col)]).unwrap();
// Expected output is a vec of strings
let expected = vec![
    "+--------+",
    "| column |",
    "+--------+",
    "| 1      |",
    "| 2      |",
    "+--------+",
];
// compare the formatted output of the record batch with the expected output
assert_batches_eq!(expected, &[batch]);
```
