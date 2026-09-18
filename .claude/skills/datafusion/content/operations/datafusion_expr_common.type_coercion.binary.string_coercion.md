# `datafusion_expr_common::type_coercion::binary::string_coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.binary.string_coercion.json).

<a id="op-cd8a91820c4014357134abed"></a>
## string_coercion

`function` · `datafusion_expr_common::type_coercion::binary::string_coercion` · datafusion-expr-common 55.1.0

```rust
fn string_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType>
```

Source: `src/type_coercion/binary.rs:1744`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coercion rules for string view types (Utf8/LargeUtf8/Utf8View):
If at least one argument is a string view, we coerce to string view
based on the observation that StringArray to StringViewArray is cheap but not vice versa.

Between Utf8 and LargeUtf8, we coerce to LargeUtf8.
