# `datafusion_functions::unicode::substr::get_true_start_end`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.substr.get_true_start_end.json).

<a id="op-fba347819c5d1281c440e854"></a>
## get_true_start_end

`function` · `datafusion_functions::unicode::substr::get_true_start_end` · datafusion-functions 55.1.0

```rust
fn get_true_start_end(input: &str, start: i64, count: Option<i64>, is_input_ascii_only: bool) -> datafusion_common::Result<(usize, usize)>
```

Source: `src/unicode/substr.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Convert the given `start` and `count` to valid byte indices within `input` string.

Input `start` and `count` are equivalent to PostgreSQL's `substr(s, start, count)`.
`start` is 1-based; if `count` is not provided, returns indices to the end of the string.
Input indices are character-based, and return values are byte indices.
The input bounds can be outside string bounds; this function will return
the intersection between input bounds and valid string bounds.
`is_input_ascii_only` is used to optimize this function if `input` is ASCII-only.

# Example
```text
'Hi🌏' in-mem (`[]` for one char, `x` for one byte): [x][x][xxxx]
get_true_start_end('Hi🌏', 1, None) -> Ok((0, 6))
get_true_start_end('Hi🌏', 1, Some(1)) -> Ok((0, 1))
get_true_start_end('Hi🌏', -10, Some(2)) -> Ok((0, 0))
```
