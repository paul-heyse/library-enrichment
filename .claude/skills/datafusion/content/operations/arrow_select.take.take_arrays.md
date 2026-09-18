# `arrow_select::take::take_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.take.take_arrays.json).

<a id="op-b8024b1f5c1204e51b646dba"></a>
## take_arrays

`function` · `arrow_select::take::take_arrays` · arrow-select 59.3.0

```rust
fn take_arrays(arrays: &[ArrayRef], indices: &dyn Array, options: Option<TakeOptions>) -> Result<Vec<ArrayRef>, arrow_schema::ArrowError>
```

Source: `src/take.rs:154`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

For each [ArrayRef](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) in the [`Vec<ArrayRef>`], take elements by index and create a new
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

Unresolved upstream links (retained, not inferred): ``Vec<ArrayRef>``.
