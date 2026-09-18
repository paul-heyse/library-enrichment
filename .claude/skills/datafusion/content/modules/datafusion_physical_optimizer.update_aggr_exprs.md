# `datafusion_physical_optimizer::update_aggr_exprs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.update_aggr_exprs.json).

<a id="op-a7ea5fc18e43b03a9a800431"></a>
## update_aggr_exprs

`module` · `datafusion_physical_optimizer::update_aggr_exprs` · datafusion-physical-optimizer 55.1.0

```rust
mod update_aggr_exprs
```

Source: `src/update_aggr_exprs.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

An optimizer rule that checks ordering requirements of aggregate expressions
and modifies the expressions to work more efficiently if possible.
