# `datafusion_spark::register_all`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.register_all.json).

<a id="op-fd7e242dfb35c47d60f716a0"></a>
## register_all

`function` · `datafusion_spark::register_all` · datafusion-spark 55.1.0

```rust
fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()>
```

Source: `src/lib.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Registers all enabled packages with a [`FunctionRegistry`](../operations/datafusion_expr.registry.FunctionRegistry.md#op-3a0de62f03e60cdeb963c7b3), overriding any existing
functions if there is a name clash.
