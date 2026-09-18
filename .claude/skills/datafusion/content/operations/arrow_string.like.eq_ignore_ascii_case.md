# `arrow_string::like::eq_ignore_ascii_case`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.like.eq_ignore_ascii_case.json).

<a id="op-b792e1f5df9795072dc90f91"></a>
## eq_ignore_ascii_case

`function` · `arrow_string::like::eq_ignore_ascii_case` · arrow-string 59.3.0

```rust
fn eq_ignore_ascii_case(left: &dyn Datum, right: &dyn Datum) -> Result<BooleanArray, ArrowError>
```

Source: `src/like.rs:211`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Perform equality check on two arrays using an ASCII case-insensitive match.

`left` and `right` must be the same type, and one of
- Utf8
- LargeUtf8
- Utf8View

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::like::eq_ignore_ascii_case;
let strings = StringArray::from(vec!["arrow", "rs", "arrow-rS", "Parquet"]);
let patterns = StringArray::from(vec!["ARROW", "rS", "ARROW-rs", "arrow"]);

let result = eq_ignore_ascii_case(&strings, &patterns).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, true, true, false]));
```
