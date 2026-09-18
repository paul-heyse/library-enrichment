# `datafusion_expr::function::StateTypeFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.function.StateTypeFunction.json).

<a id="op-f91b2ecff8bcfdc470305b9e"></a>
## StateTypeFunction

`type_alias` · `datafusion_expr::function::StateTypeFunction` · datafusion-expr 55.1.0

```rust
type StateTypeFunction = std::sync::Arc<dyn Fn(&arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<Vec<arrow::datatypes::DataType>>> + Send + Sync>
```

Source: `src/function.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Factory that returns the types used by an aggregator to serialize
its state, given its return datatype.
