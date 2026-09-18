# `datafusion_spark::function::math::pow`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.pow.json).

<a id="op-14b6cbfa176243802056ede6"></a>
## pow

`module` · `datafusion_spark::function::math::pow` · datafusion-spark 55.1.0

```rust
mod pow
```

Source: `src/function/math/pow.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `pow` / `power` function.

Unlike the default DataFusion (PostgreSQL) implementation, Spark returns
`Infinity` for `pow(0, <negative>)` rather than raising an error.
