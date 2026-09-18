# `datafusion_expr_common::type_coercion::aggregates::check_arg_count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.type_coercion.aggregates.check_arg_count.json).

<a id="op-d42c46d494dd58c0f0e6c36d"></a>
## check_arg_count

`function` · `datafusion_expr_common::type_coercion::aggregates::check_arg_count` · datafusion-expr-common 55.1.0

```rust
fn check_arg_count(func_name: &str, input_fields: &[arrow::datatypes::FieldRef], signature: &signature::TypeSignature) -> datafusion_common::Result<()>
```

Source: `src/type_coercion/aggregates.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Validate the length of `input_fields` matches the `signature` for `agg_fun`.

This method DOES NOT validate the argument fields - only that (at least one,
in the case of [`TypeSignature::OneOf`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-dace4a06957ffe853cadab3c)) signature matches the desired
number of input types.
