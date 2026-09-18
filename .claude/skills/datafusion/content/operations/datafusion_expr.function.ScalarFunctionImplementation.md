# `datafusion_expr::function::ScalarFunctionImplementation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.function.ScalarFunctionImplementation.json).

<a id="op-74c2097919bf27a302d74d70"></a>
## ScalarFunctionImplementation

`type_alias` · `datafusion_expr::function::ScalarFunctionImplementation` · datafusion-expr 55.1.0

```rust
type ScalarFunctionImplementation = std::sync::Arc<dyn Fn(&[ColumnarValue]) -> datafusion_common::Result<ColumnarValue> + Send + Sync>
```

Source: `src/function.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Scalar function

The Fn param is the wrapped function but be aware that the function will
be passed with the slice / vec of columnar values (either scalar or array)
with the exception of zero param function, where a singular element vec
will be passed. In that case the single element is a null array to indicate
the batch's row count (so that the generative zero-argument function can know
the result array size).
