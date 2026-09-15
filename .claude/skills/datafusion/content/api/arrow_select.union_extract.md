# `arrow_select::union_extract`

Crate `arrow-select` · 2 public items · structured records in [`model/arrow_select.union_extract.json`](../model/arrow_select.union_extract.json)

## union_extract

`function` · `arrow_select::union_extract::union_extract`

Also reachable as `arrow::compute::kernels::union_extract::union_extract`, `arrow::compute::union_extract`

```rust
fn union_extract(union_array: &arrow_array::UnionArray, target: &str) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Returns the value of the target field when selected, or NULL otherwise.
```text
┌─────────────────┐                                   ┌─────────────────┐
│       A=1       │                                   │        1        │
├─────────────────┤                                   ├─────────────────┤
│      A=NULL     │                                   │       NULL      │
├─────────────────┤    union_extract(values, 'A')     ├─────────────────┤
│      B='t'      │  ────────────────────────────▶    │       NULL      │
├─────────────────┤                                   ├─────────────────┤
│       A=3       │                                   │        3        │
├─────────────────┤                                   ├─────────────────┤
│      B=NULL     │                                   │       NULL      │
└─────────────────┘                                   └─────────────────┘
   union array                                              result
```
# Errors

Returns error if target field is not found

# Examples
```
# use std::sync::Arc;
# use arrow_schema::{DataType, Field, UnionFields};
# use arrow_array::{UnionArray, StringArray, Int32Array};
# use arrow_select::union_extract::union_extract;
let fields = UnionFields::try_new(
    [1, 3],
    [
        Field::new("A", DataType::Int32, true),
        Field::new("B", DataType::Utf8, true)
    ]
).unwrap();

let union = UnionArray::try_new(
    fields,
    vec![1, 1, 3, 1, 3].into(),
    None,
    vec![
        Arc::new(Int32Array::from(vec![Some(1), None, None, Some(3), Some(0)])),
        Arc::new(StringArray::from(vec![None, None, Some("t"), Some("."), None]))
    ]
).unwrap();

// Extract field A
let extracted = union_extract(&union, "A").unwrap();

assert_eq!(*extracted, Int32Array::from(vec![Some(1), None, None, Some(3), None]));
```

---

## union_extract_by_id

`function` · `arrow_select::union_extract::union_extract_by_id`

Also reachable as `arrow::compute::kernels::union_extract::union_extract_by_id`, `arrow::compute::union_extract_by_id`

```rust
fn union_extract_by_id(union_array: &arrow_array::UnionArray, target_type_id: i8) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Like [`union_extract`], but selects the child by `type_id` rather than by
field name.

This avoids ambiguity when the union contains duplicate field names.

# Errors

Returns error if `target_type_id` does not correspond to a field in the union.

---
