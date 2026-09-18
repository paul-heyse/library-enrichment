# `arrow_string::like::contains`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.contains.json).

<a id="op-d1fb90ac78de354764603ab4"></a>
## contains

`function` · `arrow_string::like::contains` · arrow-string 59.3.0

```rust
fn contains(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:190`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform SQL `CONTAINS(left, right)`

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
# use arrow_string::like::contains;
let strings = StringArray::from(vec!["arrow-rs", "arrow-rs", "arrow-rs", "Parquet"]);
let patterns = StringArray::from(vec!["arr", "-rs", "arrow-cpp", "X"]);

let result = contains(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, true, false, false]));
```
