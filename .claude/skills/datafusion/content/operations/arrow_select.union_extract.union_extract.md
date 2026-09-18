# `arrow_select::union_extract::union_extract`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.union_extract.union_extract.json).

<a id="op-d34c0ba57ba2ef4680673369"></a>
## union_extract

`function` · `arrow_select::union_extract::union_extract` · arrow-select 59.3.0

```rust
fn union_extract(union_array: &arrow_array::UnionArray, target: &str) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Source: `src/union_extract.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

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
