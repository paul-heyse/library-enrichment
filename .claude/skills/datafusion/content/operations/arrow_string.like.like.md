# `arrow_string::like::like`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.like.json).

<a id="op-4f5e15a014f56ccae12c54a6"></a>
## like

`function` · `arrow_string::like::like` · arrow-string 59.3.0

```rust
fn like(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform SQL `left LIKE right`

# Supported DataTypes

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View

There are two wildcards supported with the LIKE operator:

1. `%` - The percent sign represents zero, one, or multiple characters
2. `_` - The underscore represents a single character

Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::like;
let strings = StringArray::from(vec!["Arrow", "Arrow", "Arrow", "Ar"]);
let patterns = StringArray::from(vec!["A%", "B%", "A.", "A_"]);

let result = like(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, false, true]));
```
