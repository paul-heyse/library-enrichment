# `arrow_string::regexp::regexp_match`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_string.regexp.regexp_match.json).

<a id="op-368b8c3d1636909236c9f0ff"></a>
## regexp_match

`function` · `arrow_string::regexp::regexp_match` · arrow-string 59.3.0

```rust
fn regexp_match(array: &dyn Array, regex_array: &dyn Datum, flags_array: Option<&dyn Datum>) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/regexp.rs:414`. [Exact documentation build](https://docs.rs/crate/arrow-string/59.3.0/json).

Extract all groups matched by a regular expression for a given String array.

Modelled after the Postgres [regexp_match].

Returns a ListArray of [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) with each element containing the leftmost-first
match of the corresponding index in `regex_array` to string in `array`

If there is no match, the list element is NULL.

If a match is found, and the pattern contains no capturing parenthesized subexpressions,
then the list element is a single-element [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) containing the substring
matching the whole pattern.

If a match is found, and the pattern contains capturing parenthesized subexpressions, then the
list element is a [`GenericStringArray`](../operations/arrow_array.array.string_array.GenericStringArray.md#op-86b79a80b65c0571602ced76) whose n'th element is the substring matching
the n'th capturing parenthesized subexpression of the pattern.

The flags parameter is an optional text string containing zero or more single-letter flags
that change the function's behavior.

# See Also
* [`regexp_is_match`](../operations/arrow_string.regexp.regexp_is_match.md#op-0bbc5531b193f8f07f05b357) for matching (rather than extracting) a regular expression against an array of strings

[regexp_match]: https://www.postgresql.org/docs/current/functions-matching.html#FUNCTIONS-POSIX-REGEXP
