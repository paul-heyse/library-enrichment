# `datafusion_functions::string::ascii::ascii`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.ascii.ascii.json).

<a id="op-0face1ed9a3c1f0bb729d21c"></a>
## ascii

`function` · `datafusion_functions::string::ascii::ascii` · datafusion-functions 55.1.0

```rust
fn ascii(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
```

Source: `src/string/ascii.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns the numeric code of the first character of the argument.
