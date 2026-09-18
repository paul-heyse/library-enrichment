# `arrow_select::take::take`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.take.take.json).

<a id="op-4197d454d308f4ceadb20600"></a>
## take

`function` · `arrow_select::take::take` · arrow-select 59.3.0

```rust
fn take(values: &dyn Array, indices: &dyn Array, options: Option<TakeOptions>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/take.rs:88`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Take elements by index from [Array](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21), creating a new [Array](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21) from those indexes.

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

For selecting values by index from multiple arrays see [`crate::interleave`](../modules/arrow_select.interleave.md#op-0c061b9c668fa0e5cddfc971)

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
* [`BatchCoalescer`]: to filter multiple [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) and coalesce
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
