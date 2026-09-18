# `datafusion_expr::type_coercion::functions::UDFCoercionExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.type_coercion.functions.UDFCoercionExt.json).

<a id="op-130334a83ad11ff7269deef8"></a>
## UDFCoercionExt

`trait` · `datafusion_expr::type_coercion::functions::UDFCoercionExt` · datafusion-expr 55.1.0

```rust
trait UDFCoercionExt
```

Source: `src/type_coercion/functions.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Extension trait to unify common functionality between [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0), [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)
and [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65) for use by signature coercion functions.

<a id="op-5a5e3e7c8b2d112871e3b9a1"></a>
## coerce_types

`function` · `datafusion_expr::type_coercion::functions::UDFCoercionExt::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Source: `src/type_coercion/functions.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Should delegate to [`ScalarUDF::coerce_types`](../operations/datafusion_expr.udf.ScalarUDF.md#op-375dff5a7c82c6790615845a), [`AggregateUDF::coerce_types`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-ec074b52a75a156da628c7ba)
or [`WindowUDF::coerce_types`](../operations/datafusion_expr.udwf.WindowUDF.md#op-101f79c460b30f0971589100).

<a id="op-b6ad940415f304bff7c139bb"></a>
## name

`function` · `datafusion_expr::type_coercion::functions::UDFCoercionExt::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/type_coercion/functions.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Should delegate to [`ScalarUDF::name`](../operations/datafusion_expr.udf.ScalarUDF.md#op-9636aad62898c4abb7c9c555), [`AggregateUDF::name`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-929a1ba07ae51fc17bcb0452) or [`WindowUDF::name`](../operations/datafusion_expr.udwf.WindowUDF.md#op-8a912b48a52ae4ecfc63e727).

<a id="op-a343019cb9318f60dc706349"></a>
## signature

`function` · `datafusion_expr::type_coercion::functions::UDFCoercionExt::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Source: `src/type_coercion/functions.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Should delegate to [`ScalarUDF::signature`](../operations/datafusion_expr.udf.ScalarUDF.md#op-a26d546cca40671f7d56d593), [`AggregateUDF::signature`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-64e9934ca38899b047046aa3)
or [`WindowUDF::signature`](../operations/datafusion_expr.udwf.WindowUDF.md#op-8e20f6387020a1ce6981d77d).
