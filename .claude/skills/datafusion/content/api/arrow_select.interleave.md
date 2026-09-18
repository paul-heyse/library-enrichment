# `arrow_select::interleave`

Crate `arrow-select` · 2 public items · structured records in [`model/arrow_select.interleave.json`](../model/arrow_select.interleave.json)

## interleave

`function` · `arrow_select::interleave::interleave`

Also reachable as `arrow::compute::interleave`, `arrow::compute::kernels::interleave::interleave`

```rust
fn interleave(values: &[&dyn Array], indices: &[(usize, usize)]) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_select.interleave.interleave.md).



Takes elements by index from a list of [`Array`], creating a new [`Array`] from those values.

Each element in `indices` is a pair of `usize` with the first identifying the index
of the [`Array`] in `values`, and the second the index of the value within that [`Array`]

```text
┌─────────────────┐      ┌─────────┐                                  ┌─────────────────┐
│        A        │      │ (0, 0)  │        interleave(               │        A        │
├─────────────────┤      ├─────────┤          [values0, values1],     ├─────────────────┤
│        D        │      │ (1, 0)  │          indices                 │        B        │
└─────────────────┘      ├─────────┤        )                         ├─────────────────┤
  values array 0         │ (1, 1)  │      ─────────────────────────▶  │        C        │
                         ├─────────┤                                  ├─────────────────┤
                         │ (0, 1)  │                                  │        D        │
                         └─────────┘                                  └─────────────────┘
┌─────────────────┐       indices
│        B        │        array
├─────────────────┤                                                    result
│        C        │
├─────────────────┤
│        E        │
└─────────────────┘
  values array 1
```

For selecting values by index from a single array see [`crate::take`]

---

## interleave_record_batch

`function` · `arrow_select::interleave::interleave_record_batch`

Also reachable as `arrow::compute::interleave_record_batch`, `arrow::compute::kernels::interleave::interleave_record_batch`

```rust
fn interleave_record_batch(record_batches: &[&RecordBatch], indices: &[(usize, usize)]) -> Result<RecordBatch, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_select.interleave.interleave_record_batch.md).


Interleave rows by index from multiple [`RecordBatch`] instances and return a new [`RecordBatch`].

This function will call [`interleave`] on each array of the [`RecordBatch`] instances and assemble a new [`RecordBatch`].

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

---
