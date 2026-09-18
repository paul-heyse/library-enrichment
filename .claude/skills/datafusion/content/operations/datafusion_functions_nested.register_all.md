# `datafusion_functions_nested::register_all`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.register_all.json).

<a id="op-bda0c05dc7e85e7cddb0627c"></a>
## register_all

`function` · `datafusion_functions_nested::register_all` · datafusion-functions-nested 55.1.0

```rust
fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()>
```

Source: `src/lib.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

Registers all enabled packages with a [`FunctionRegistry`](../operations/datafusion_expr.registry.FunctionRegistry.md#op-3a0de62f03e60cdeb963c7b3)
