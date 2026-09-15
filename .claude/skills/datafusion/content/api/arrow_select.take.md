# `arrow_select::take`

Crate `arrow-select` · 4 public items · structured records in [`model/arrow_select.take.json`](../model/arrow_select.take.json)

## take

`function` · `arrow_select::take::take`

Also reachable as `arrow::compute::kernels::take::take`, `arrow::compute::take`

```rust
fn take(values: &dyn Array, indices: &dyn Array, options: Option<TakeOptions>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Take elements by index from [Array], creating a new [Array] from those indexes.

```text
┌─────────────────┐      ┌─────────┐                              ┌─────────────────┐
│        A        │      │    0    │                              │        A        │
├─────────────────┤      ├─────────┤                              ├─────────────────┤
│        D        │      │    2    │                              │        B        │
├─────────────────┤      ├─────────┤   take(values, indices)      ├─────────────────┤
│        B        │      │    3    │ ─────────────────────────▶   │        C        │
├─────────────────┤      ├─────────┤                              ├─────────────────┤
│        C        │      │    1    │                              │        D        │
├─────────────────┤      └─────────┘                              └─────────────────┘
│        E        │
└─────────────────┘
   values array          indices array                              result
```

For selecting values by index from multiple arrays see [`crate::interleave`]

Note that this kernel, similar to other kernels in this crate,
will avoid allocating where not necessary. Consequently
the returned array may share buffers with the inputs

# Errors
This function errors whenever:
* An index cannot be casted to `usize` (typically 32 bit architectures)
* An index is out of bounds and `options` is set to check bounds.

# Safety

When `options` is not set to check bounds, taking indexes after `len` will panic.

# See also
* [`BatchCoalescer`]: to filter multiple [`RecordBatch`] and coalesce
  the results into a single array.

[`BatchCoalescer`]: crate::coalesce::BatchCoalescer

# Examples
```
# use arrow_array::{StringArray, UInt32Array, cast::AsArray};
# use arrow_select::take::take;
let values = StringArray::from(vec!["zero", "one", "two"]);

// Take items at index 2, and 1:
let indices = UInt32Array::from(vec![2, 1]);
let taken = take(&values, &indices, None).unwrap();
let taken = taken.as_string::<i32>();

assert_eq!(*taken, StringArray::from(vec!["two", "one"]));
```

---

## take_arrays

`function` · `arrow_select::take::take_arrays`

Also reachable as `arrow::compute::kernels::take::take_arrays`, `arrow::compute::take_arrays`

```rust
fn take_arrays(arrays: &[ArrayRef], indices: &dyn Array, options: Option<TakeOptions>) -> Result<Vec<ArrayRef>, arrow_schema::ArrowError>
```

For each [ArrayRef] in the [`Vec<ArrayRef>`], take elements by index and create a new
[`Vec<ArrayRef>`] from those indices.

```text
┌────────┬────────┐
│        │        │           ┌────────┐                                ┌────────┬────────┐
│   A    │   1    │           │        │                                │        │        │
├────────┼────────┤           │   0    │                                │   A    │   1    │
│        │        │           ├────────┤                                ├────────┼────────┤
│   D    │   4    │           │        │                                │        │        │
├────────┼────────┤           │   2    │  take_arrays(values,indices)   │   B    │   2    │
│        │        │           ├────────┤                                ├────────┼────────┤
│   B    │   2    │           │        │  ───────────────────────────►  │        │        │
├────────┼────────┤           │   3    │                                │   C    │   3    │
│        │        │           ├────────┤                                ├────────┼────────┤
│   C    │   3    │           │        │                                │        │        │
├────────┼────────┤           │   1    │                                │   D    │   4    │
│        │        │           └────────┘                                └────────┼────────┘
│   E    │   5    │
└────────┴────────┘
   values arrays             indices array                                      result
```

# Errors
This function errors whenever:
* An index cannot be casted to `usize` (typically 32 bit architectures)
* An index is out of bounds and `options` is set to check bounds.

# Safety

When `options` is not set to check bounds, taking indexes after `len` will panic.

# Examples
```
# use std::sync::Arc;
# use arrow_array::{StringArray, UInt32Array, cast::AsArray};
# use arrow_select::take::{take, take_arrays};
let string_values = Arc::new(StringArray::from(vec!["zero", "one", "two"]));
let values = Arc::new(UInt32Array::from(vec![0, 1, 2]));

// Take items at index 2, and 1:
let indices = UInt32Array::from(vec![2, 1]);
let taken_arrays = take_arrays(&[string_values, values], &indices, None).unwrap();
let taken_string = taken_arrays[0].as_string::<i32>();
assert_eq!(*taken_string, StringArray::from(vec!["two", "one"]));
let taken_values = taken_arrays[1].as_primitive();
assert_eq!(*taken_values, UInt32Array::from(vec![2, 1]));
```

---

## take_record_batch

`function` · `arrow_select::take::take_record_batch`

Also reachable as `arrow::compute::kernels::take::take_record_batch`, `arrow::compute::take_record_batch`

```rust
fn take_record_batch(record_batch: &RecordBatch, indices: &dyn Array) -> Result<RecordBatch, arrow_schema::ArrowError>
```

Take rows by index from [`RecordBatch`] and returns a new [`RecordBatch`] from those indexes.

This function will call [`take`] on each array of the [`RecordBatch`] and assemble a new [`RecordBatch`].

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

---

## TakeOptions

`struct` · `arrow_select::take::TakeOptions`

Also reachable as `arrow::compute::TakeOptions`, `arrow::compute::kernels::take::TakeOptions`

```rust
struct TakeOptions
```

**Fields**: `check_bounds`

**Derives**: Clone, Debug, Default

Options that define how `take` should behave

---
