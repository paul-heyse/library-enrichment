# `arrow_string::regexp::regexp_is_match`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.regexp.regexp_is_match.json).

<a id="op-0bbc5531b193f8f07f05b357"></a>
## regexp_is_match

`function` · `arrow_string::regexp::regexp_is_match` · arrow-string 59.3.0

```rust
fn regexp_is_match<'a, S1, S2, S3>(array: &'a S1, regex_array: &'a S2, flags_array: Option<&'a S3>) -> Result<BooleanArray, arrow_schema::ArrowError> where &'a S1: StringArrayType<'a>, &'a S2: StringArrayType<'a>, &'a S3: StringArrayType<'a>
```

Source: `src/regexp.rs:69`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Return BooleanArray indicating which strings in an array match an array of
regular expressions.

This is equivalent to the SQL `array ~ regex_array`, supporting
[`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945) / [`LargeStringArray`](../operations/arrow_array.array.string_array.LargeStringArray.md#op-829f12b9a45fbfe752a7ee52) / [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0).

If `regex_array` element has an empty value, the corresponding result value is always true.

`flags_array` are optional [`StringArray`](../operations/arrow_array.array.string_array.StringArray.md#op-5d32f770159d415652a55945) / [`LargeStringArray`](../operations/arrow_array.array.string_array.LargeStringArray.md#op-829f12b9a45fbfe752a7ee52) / [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0) flag,
which allow special search modes, such as case-insensitive and multi-line mode.
See the documentation [here](https://docs.rs/regex/1.5.4/regex/#grouping-and-flags)
for more information.

# See Also
* [`regexp_is_match_scalar`](../operations/arrow_string.regexp.regexp_is_match_scalar.md#op-2d89cb8af1a8d4ad52e20356) for matching a single regular expression against an array of strings
* [`regexp_match`](../operations/arrow_string.regexp.regexp_match.md#op-368b8c3d1636909236c9f0ff) for extracting groups from a string array based on a regular expression

# Example
```
# use arrow_array::{StringArray, BooleanArray};
# use arrow_string::regexp::regexp_is_match;
// First array is the array of strings to match
let array = StringArray::from(vec!["Foo", "Bar", "FooBar", "Baz"]);
// Second array is the array of regular expressions to match against
let regex_array = StringArray::from(vec!["^Foo", "^Foo", "Bar$", "Baz"]);
// Third array is the array of flags to use for each regular expression, if desired
// (the type must be provided to satisfy type inference for the third parameter)
let flags_array: Option<&StringArray> = None;
// The result is a BooleanArray indicating when each string in `array`
// matches the corresponding regular expression in `regex_array`
let result = regexp_is_match(&array, &regex_array, flags_array).unwrap();
assert_eq!(result, BooleanArray::from(vec![true, false, true, true]));
```
