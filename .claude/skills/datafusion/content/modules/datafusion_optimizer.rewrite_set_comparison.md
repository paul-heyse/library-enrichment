# `datafusion_optimizer::rewrite_set_comparison`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.rewrite_set_comparison.json).

<a id="op-6808a3c47612900a397d426c"></a>
## rewrite_set_comparison

`module` · `datafusion_optimizer::rewrite_set_comparison` · datafusion-optimizer 55.1.0

```rust
mod rewrite_set_comparison
```

Source: `src/rewrite_set_comparison.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule rewriting `SetComparison` subqueries (e.g. `= ANY`,
`> ALL`) into boolean expressions built from `EXISTS` subqueries
that capture SQL three-valued logic.
