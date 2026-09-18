# `arrow_string::like::ends_with`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.ends_with.json).

<a id="op-531937e4c4fbe1abfebbbf3b"></a>
## ends_with

`function` · `arrow_string::like::ends_with` · arrow-string 59.3.0

```rust
fn ends_with(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:164`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform SQL `ENDSWITH(left, right)`

# Supported DataTypes

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View
- Binary
- LargeBinary
- BinaryView

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::ends_with;
let strings = StringArray::from(vec!["arrow-rs", "arrow-rs",  "Parquet"]);
let patterns = StringArray::from(vec!["arr", "-rs", "t"]);

let result = ends_with(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![false, true, true]));
```
