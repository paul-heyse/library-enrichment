# `arrow_string::regexp::regexp_is_match_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.regexp.regexp_is_match_scalar.json).

<a id="op-2d89cb8af1a8d4ad52e20356"></a>
## regexp_is_match_scalar

`function` · `arrow_string::regexp::regexp_is_match_scalar` · arrow-string 59.3.0

```rust
fn regexp_is_match_scalar<'a, S>(array: &'a S, regex: &str, flag: Option<&str>) -> Result<BooleanArray, arrow_schema::ArrowError> where &'a S: StringArrayType<'a>
```

Source: `src/regexp.rs:175`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Return BooleanArray indicating which strings in an array match a single regular expression.

This is equivalent to the SQL `array ~ regex_array`, supporting
[`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945) / [`LargeStringArray`](../operations/arrow_array.array.string_array.LargeStringArray.md#op-829f12b9a45fbfe752a7ee52) / [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0) and a scalar.

See the documentation on [`regexp_is_match`](../operations/arrow_string.regexp.regexp_is_match.md#op-0bbc5531b193f8f07f05b357) for more details on arguments

# See Also
* [`regexp_is_match`](../operations/arrow_string.regexp.regexp_is_match.md#op-0bbc5531b193f8f07f05b357) for matching an array of regular expression against an array of strings
* [`regexp_match`](../operations/arrow_string.regexp.regexp_match.md#op-368b8c3d1636909236c9f0ff) for extracting groups from a string array based on a regular expression

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::regexp::regexp_is_match_scalar;
// array of strings to match
let array = StringArray::from(vec!["Foo", "Bar", "FooBar", "Baz"]);
let regexp = "^Foo"; // regular expression to match against
let flags: Option<&str> = None;  // flags can control the matching behavior
// The result is a BooleanArray indicating when each string in `array`
// matches the regular expression `regexp`
let result = regexp_is_match_scalar(&array, regexp, None).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, true, false]));
```
