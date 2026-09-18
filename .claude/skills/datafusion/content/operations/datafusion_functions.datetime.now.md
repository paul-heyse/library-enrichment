# `datafusion_functions::datetime::now`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.now.json).

<a id="op-1f0caa4188fb242322587fc3"></a>
## now

`function` · `datafusion_functions::datetime::now` · datafusion-functions 55.1.0

```rust
fn now(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

Source: `src/datetime/mod.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of now
