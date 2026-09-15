# `datafusion_functions::unicode::substr`

Crate `datafusion-functions` · 3 public items · structured records in [`model/datafusion_functions.unicode.substr.json`](../model/datafusion_functions.unicode.substr.json)

## enable_ascii_fast_path

`function` · `datafusion_functions::unicode::substr::enable_ascii_fast_path`

```rust
fn enable_ascii_fast_path<'a, V: StringArrayType<'a>>(string_array: &V, start: &arrow::array::Int64Array, count: Option<&arrow::array::Int64Array>) -> bool
```

---

## get_true_start_end

`function` · `datafusion_functions::unicode::substr::get_true_start_end`

```rust
fn get_true_start_end(input: &str, start: i64, count: Option<i64>, is_input_ascii_only: bool) -> datafusion_common::Result<(usize, usize)>
```

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

---

## SubstrFunc

`struct` · `datafusion_functions::unicode::substr::SubstrFunc`

```rust
struct SubstrFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

---
