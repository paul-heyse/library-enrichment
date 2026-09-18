# `arrow_select::interleave::interleave_record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.interleave.interleave_record_batch.json).

<a id="op-d1e4a8d9bfc4e50f8c16783e"></a>
## interleave_record_batch

`function` · `arrow_select::interleave::interleave_record_batch` · arrow-select 59.3.0

```rust
fn interleave_record_batch(record_batches: &[&RecordBatch], indices: &[(usize, usize)]) -> Result<RecordBatch, arrow_schema::ArrowError>
```

Source: `src/interleave.rs:912`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Interleave rows by index from multiple [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) instances and return a new [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

This function will call [`interleave`](../operations/arrow_select.interleave.interleave.md#op-be8ca59e89328880f4e23523) on each array of the [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) instances and assemble a new [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

# Example
```
# use std::sync::Arc;
# use arrow_array::{StringArray, Int32Array, RecordBatch, UInt32Array};
# use arrow_schema::{DataType, Field, Schema};
# use arrow_select::interleave::interleave_record_batch;

let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Utf8, true),
]));

let batch1 = RecordBatch::try_new(
    schema.clone(),
    vec![
        Arc::new(Int32Array::from(vec![0, 1, 2])),
        Arc::new(StringArray::from(vec!["a", "b", "c"])),
    ],
).unwrap();

let batch2 = RecordBatch::try_new(
    schema.clone(),
    vec![
        Arc::new(Int32Array::from(vec![3, 4, 5])),
        Arc::new(StringArray::from(vec!["d", "e", "f"])),
    ],
).unwrap();

let indices = vec![(0, 1), (1, 2), (0, 0), (1, 1)];
let interleaved = interleave_record_batch(&[&batch1, &batch2], &indices).unwrap();

let expected = RecordBatch::try_new(
    schema,
    vec![
        Arc::new(Int32Array::from(vec![1, 5, 0, 4])),
        Arc::new(StringArray::from(vec!["b", "f", "a", "e"])),
    ],
).unwrap();
assert_eq!(interleaved, expected);
```
