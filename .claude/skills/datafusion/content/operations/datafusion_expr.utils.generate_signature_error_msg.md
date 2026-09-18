# `datafusion_expr::utils::generate_signature_error_msg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.generate_signature_error_msg.json).

<a id="op-1241493bcf0642a49b58e714"></a>
## generate_signature_error_msg

`function` · `datafusion_expr::utils::generate_signature_error_msg` · datafusion-expr 55.1.0

```rust
fn generate_signature_error_msg(func_name: &str, func_signature: datafusion_expr_common::signature::Signature, input_expr_types: &[arrow::datatypes::DataType]) -> String
```

Source: `src/utils.rs:1075`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a detailed error message for a function with wrong signature.

For example, a query like `select round(3.14, 1.1);` would yield:
```text
Error during planning: No function matches 'round(Float64, Float64)'. You might need to add explicit type casts.
    Candidate functions:
    round(Float64, Int64)
    round(Float32, Int64)
    round(Float64)
    round(Float32)
```
