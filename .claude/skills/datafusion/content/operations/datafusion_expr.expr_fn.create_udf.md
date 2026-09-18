# `datafusion_expr::expr_fn::create_udf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.create_udf.json).

<a id="op-eb71a422c4c9ac3d72bb474c"></a>
## create_udf

`function` · `datafusion_expr::expr_fn::create_udf` · datafusion-expr 55.1.0

```rust
fn create_udf(name: &str, input_types: Vec<arrow::datatypes::DataType>, return_type: arrow::datatypes::DataType, volatility: Volatility, fun: ScalarFunctionImplementation) -> ScalarUDF
```

Source: `src/expr_fn.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convenience method to create a new user defined scalar function (UDF) with a
specific signature and specific return type.

Note this function does not expose all available features of [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0),
such as

* computing return types based on input types
* multiple [`Signature`](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e)s
* aliases

See [`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0) for details and examples on how to use the full
functionality.
