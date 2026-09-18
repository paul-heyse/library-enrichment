# `arrow_cast::cast::union::union_extract_by_type`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.union.union_extract_by_type.json).

<a id="op-3fed8edf85f7f1c575996d4d"></a>
## union_extract_by_type

`function` · `arrow_cast::cast::union::union_extract_by_type` · arrow-cast 59.3.0

```rust
fn union_extract_by_type(union_array: &arrow_array::UnionArray, target_type: &arrow_schema::DataType, cast_options: &super::CastOptions<'_>) -> Result<arrow_array::ArrayRef, arrow_schema::ArrowError>
```

Source: `src/cast/union.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Extracts the best-matching child array from a [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac) for a given target type,
and casts it to that type.

Rows where a different child array is active become NULL.
If no child array matches, returns an error.

# Example

```
# use std::sync::Arc;
# use arrow_schema::{DataType, Field, UnionFields};
# use arrow_array::{UnionArray, StringArray, Int32Array, Array};
# use arrow_cast::cast::union_extract_by_type;
# use arrow_cast::CastOptions;
let fields = UnionFields::try_new(
    [0, 1],
    [
        Field::new("int", DataType::Int32, true),
        Field::new("str", DataType::Utf8, true),
    ],
).unwrap();

let union = UnionArray::try_new(
    fields,
    vec![0, 1, 0].into(),
    None,
    vec![
        Arc::new(Int32Array::from(vec![Some(42), None, Some(99)])),
        Arc::new(StringArray::from(vec![None, Some("hello"), None])),
    ],
)
.unwrap();

// extract the Utf8 child array and cast to Utf8View
let result = union_extract_by_type(&union, &DataType::Utf8View, &CastOptions::default()).unwrap();
assert_eq!(result.data_type(), &DataType::Utf8View);
assert!(result.is_null(0));   // Int32 row -> NULL
assert!(!result.is_null(1));  // Utf8 row -> "hello"
assert!(result.is_null(2));   // Int32 row -> NULL
```
