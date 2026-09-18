# `datafusion_expr_common::type_coercion::aggregates`

Crate `datafusion-expr-common` · 3 public items · structured records in [`model/datafusion_expr_common.type_coercion.aggregates.json`](../model/datafusion_expr_common.type_coercion.aggregates.json)

## check_arg_count

`function` · `datafusion_expr_common::type_coercion::aggregates::check_arg_count`

Also reachable as `datafusion_expr::type_coercion::aggregates::check_arg_count`

```rust
fn check_arg_count(func_name: &str, input_fields: &[arrow::datatypes::FieldRef], signature: &signature::TypeSignature) -> datafusion_common::Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.type_coercion.aggregates.check_arg_count.md).


Validate the length of `input_fields` matches the `signature` for `agg_fun`.

This method DOES NOT validate the argument fields - only that (at least one,
in the case of [`TypeSignature::OneOf`]) signature matches the desired
number of input types.

---

## INTEGERS

`static` · `datafusion_expr_common::type_coercion::aggregates::INTEGERS`

> **Deprecated** — since 54.0.0: Use functions signatures

Also reachable as `datafusion_expr::type_coercion::aggregates::INTEGERS`

```rust
static INTEGERS: &[arrow::datatypes::DataType]
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.type_coercion.aggregates.INTEGERS.md).


---

## NUMERICS

`static` · `datafusion_expr_common::type_coercion::aggregates::NUMERICS`

> **Deprecated** — since 54.0.0: Use functions signatures

Also reachable as `datafusion_expr::type_coercion::aggregates::NUMERICS`

```rust
static NUMERICS: &[arrow::datatypes::DataType]
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr_common.type_coercion.aggregates.NUMERICS.md).


---
