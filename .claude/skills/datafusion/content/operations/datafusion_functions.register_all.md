# `datafusion_functions::register_all`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.register_all.json).

<a id="op-f9df861972813bbf8aa26db5"></a>
## register_all

`function` · `datafusion_functions::register_all` · datafusion-functions 55.1.0

```rust
fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()>
```

Source: `src/lib.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Registers all enabled packages with a [`FunctionRegistry`](../operations/datafusion_expr.registry.FunctionRegistry.md#op-3a0de62f03e60cdeb963c7b3)
