# `datafusion_expr::function::ReturnTypeFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.function.ReturnTypeFunction.json).

<a id="op-a52503745e52d69be94bc9f2"></a>
## ReturnTypeFunction

`type_alias` · `datafusion_expr::function::ReturnTypeFunction` · datafusion-expr 55.1.0

```rust
type ReturnTypeFunction = std::sync::Arc<dyn Fn(&[arrow::datatypes::DataType]) -> datafusion_common::Result<std::sync::Arc<arrow::datatypes::DataType>> + Send + Sync>
```

Source: `src/function.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Factory that returns the functions's return type given the input argument types
