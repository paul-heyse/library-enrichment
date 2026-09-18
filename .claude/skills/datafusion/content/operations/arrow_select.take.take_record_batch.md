# `arrow_select::take::take_record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.take.take_record_batch.json).

<a id="op-929c00ffe22d86d208c23db7"></a>
## take_record_batch

`function` · `arrow_select::take::take_record_batch` · arrow-select 59.3.0

```rust
fn take_record_batch(record_batch: &RecordBatch, indices: &dyn Array) -> Result<RecordBatch, arrow_schema::ArrowError>
```

Source: `src/take.rs:1121`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Take rows by index from [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) and returns a new [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) from those indexes.

This function will call [`take`](../operations/arrow_select.take.take.md#op-4197d454d308f4ceadb20600) on each array of the [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) and assemble a new [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

# Example
```
# use std::sync::Arc;
# use arrow_array::{StringArray, Int32Array, UInt32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};
# use arrow_select::take::take_record_batch;
let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Utf8, true),
]));
let batch = RecordBatch::try_new(
    schema.clone(),
    vec![
        Arc::new(Int32Array::from_iter_values(0..20)),
        Arc::new(StringArray::from_iter_values(
            (0..20).map(|i| format!("str-{}", i)),
        )),
    ],
)
.unwrap();

let indices = UInt32Array::from(vec![1, 5, 10]);
let taken = take_record_batch(&batch, &indices).unwrap();

let expected = RecordBatch::try_new(
    schema,
    vec![
        Arc::new(Int32Array::from(vec![1, 5, 10])),
        Arc::new(StringArray::from(vec!["str-1", "str-5", "str-10"])),
    ],
)
.unwrap();
assert_eq!(taken, expected);
```
