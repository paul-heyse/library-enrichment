# `arrow_string::regexp`

Crate `arrow-string` · 3 public items · structured records in [`model/arrow_string.regexp.json`](../model/arrow_string.regexp.json)

## regexp_is_match

`function` · `arrow_string::regexp::regexp_is_match`

Also reachable as `arrow::compute::kernels::comparison::regexp_is_match`, `arrow::compute::kernels::regexp::regexp_is_match`, `arrow::compute::regexp_is_match`

```rust
fn regexp_is_match<'a, S1, S2, S3>(array: &'a S1, regex_array: &'a S2, flags_array: Option<&'a S3>) -> Result<BooleanArray, arrow_schema::ArrowError> where &'a S1: StringArrayType<'a>, &'a S2: StringArrayType<'a>, &'a S3: StringArrayType<'a>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.regexp.regexp_is_match.md).


Return BooleanArray indicating which strings in an array match an array of
regular expressions.

This is equivalent to the SQL `array ~ regex_array`, supporting
[`StringArray`] / [`LargeStringArray`] / [`StringViewArray`].

If `regex_array` element has an empty value, the corresponding result value is always true.

`flags_array` are optional [`StringArray`] / [`LargeStringArray`] / [`StringViewArray`] flag,
which allow special search modes, such as case-insensitive and multi-line mode.
See the documentation [here](https://docs.rs/regex/1.5.4/regex/#grouping-and-flags)
for more information.

# See Also
* [`regexp_is_match_scalar`] for matching a single regular expression against an array of strings
* [`regexp_match`] for extracting groups from a string array based on a regular expression

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

---

## regexp_is_match_scalar

`function` · `arrow_string::regexp::regexp_is_match_scalar`

Also reachable as `arrow::compute::kernels::comparison::regexp_is_match_scalar`, `arrow::compute::kernels::regexp::regexp_is_match_scalar`, `arrow::compute::regexp_is_match_scalar`

```rust
fn regexp_is_match_scalar<'a, S>(array: &'a S, regex: &str, flag: Option<&str>) -> Result<BooleanArray, arrow_schema::ArrowError> where &'a S: StringArrayType<'a>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.regexp.regexp_is_match_scalar.md).


Return BooleanArray indicating which strings in an array match a single regular expression.

This is equivalent to the SQL `array ~ regex_array`, supporting
[`StringArray`] / [`LargeStringArray`] / [`StringViewArray`] and a scalar.

See the documentation on [`regexp_is_match`] for more details on arguments

# See Also
* [`regexp_is_match`] for matching an array of regular expression against an array of strings
* [`regexp_match`] for extracting groups from a string array based on a regular expression

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

---

## regexp_match

`function` · `arrow_string::regexp::regexp_match`

Also reachable as `arrow::compute::kernels::regexp::regexp_match`, `arrow::compute::regexp_match`

```rust
fn regexp_match(array: &dyn Array, regex_array: &dyn Datum, flags_array: Option<&dyn Datum>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_string.regexp.regexp_match.md).


Extract all groups matched by a regular expression for a given String array.

Modelled after the Postgres [regexp_match].

Returns a ListArray of [`GenericStringArray`] with each element containing the leftmost-first
match of the corresponding index in `regex_array` to string in `array`

If there is no match, the list element is NULL.

If a match is found, and the pattern contains no capturing parenthesized subexpressions,
then the list element is a single-element [`GenericStringArray`] containing the substring
matching the whole pattern.

If a match is found, and the pattern contains capturing parenthesized subexpressions, then the
list element is a [`GenericStringArray`] whose n'th element is the substring matching
the n'th capturing parenthesized subexpression of the pattern.

The flags parameter is an optional text string containing zero or more single-letter flags
that change the function's behavior.

# See Also
* [`regexp_is_match`] for matching (rather than extracting) a regular expression against an array of strings

[regexp_match]: https://www.postgresql.org/docs/current/functions-matching.html#FUNCTIONS-POSIX-REGEXP

---
