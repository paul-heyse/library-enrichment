# `datafusion_functions::unicode::substr::enable_ascii_fast_path`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.substr.enable_ascii_fast_path.json).

<a id="op-fb9532667fc75a97fc443b08"></a>
## enable_ascii_fast_path

`function` · `datafusion_functions::unicode::substr::enable_ascii_fast_path` · datafusion-functions 55.1.0

```rust
fn enable_ascii_fast_path<'a, V: StringArrayType<'a>>(string_array: &V, start: &arrow::array::Int64Array, count: Option<&arrow::array::Int64Array>) -> bool
```

Source: `src/unicode/substr.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
