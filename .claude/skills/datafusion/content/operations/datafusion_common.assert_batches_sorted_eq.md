# `datafusion_common::assert_batches_sorted_eq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.assert_batches_sorted_eq.json).

<a id="op-2627a165176928dd41981105"></a>
## assert_batches_sorted_eq

`macro` · `datafusion_common::assert_batches_sorted_eq` · datafusion-common 55.1.0

```rust
macro_rules! assert_batches_sorted_eq
```

Source: `src/test_util.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compares formatted output of a record batch with an expected
vector of strings in a way that order does not matter.
This is a macro so errors appear on the correct line

See [`assert_batches_eq`](../operations/datafusion_common.assert_batches_eq.md#op-d069d2a820f5c18534d0e6a5) for more details and example.

Expects to be called about like this:

`assert_batch_sorted_eq!(expected_lines: &[&str], batches: &[RecordBatch])`
