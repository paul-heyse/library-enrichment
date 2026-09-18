# `datafusion_optimizer::decorrelate_predicate_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.decorrelate_predicate_subquery.json).

<a id="op-071a5d434394b23bb1c218be"></a>
## decorrelate_predicate_subquery

`module` · `datafusion_optimizer::decorrelate_predicate_subquery` · datafusion-optimizer 55.1.0

```rust
mod decorrelate_predicate_subquery
```

Source: `src/decorrelate_predicate_subquery.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

[`DecorrelatePredicateSubquery`](../operations/datafusion_optimizer.decorrelate_predicate_subquery.DecorrelatePredicateSubquery.md#op-bf2e5b87a797226bba08be61) converts `IN`/`EXISTS` subquery predicates to `SEMI`/`ANTI` joins
