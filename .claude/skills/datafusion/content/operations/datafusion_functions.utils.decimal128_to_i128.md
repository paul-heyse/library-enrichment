# `datafusion_functions::utils::decimal128_to_i128`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.utils.decimal128_to_i128.json).

<a id="op-575c4e15817a16dcf6cfb669"></a>
## decimal128_to_i128

`function` · `datafusion_functions::utils::decimal128_to_i128` · datafusion-functions 55.1.0

```rust
fn decimal128_to_i128(value: i128, scale: i8) -> datafusion_common::Result<i128, arrow::error::ArrowError>
```

Source: `src/utils.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Converts Decimal128 components (value and scale) to an unscaled i128
