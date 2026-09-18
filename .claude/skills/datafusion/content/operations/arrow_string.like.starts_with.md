# `arrow_string::like::starts_with`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.starts_with.json).

<a id="op-aa0551ac5d219073c426d280"></a>
## starts_with

`function` · `arrow_string::like::starts_with` · arrow-string 59.3.0

```rust
fn starts_with(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:138`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform SQL `STARTSWITH(left, right)`

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
# use arrow_string::like::starts_with;
let strings = StringArray::from(vec!["arrow-rs", "arrow-rs", "arrow-rs", "Parquet"]);
let patterns = StringArray::from(vec!["arr", "arrow", "arrow-cpp", "p"]);

let result = starts_with(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, true, false, false]));
```
