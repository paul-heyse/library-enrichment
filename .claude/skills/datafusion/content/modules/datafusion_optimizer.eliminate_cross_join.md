# `datafusion_optimizer::eliminate_cross_join`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_cross_join.json).

<a id="op-911abe784d1f8825040efcb4"></a>
## eliminate_cross_join

`module` · `datafusion_optimizer::eliminate_cross_join` · datafusion-optimizer 55.1.0

```rust
mod eliminate_cross_join
```

Source: `src/eliminate_cross_join.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

[`EliminateCrossJoin`](../operations/datafusion_optimizer.eliminate_cross_join.EliminateCrossJoin.md#op-595298539f8047ce374a99b2) converts `CROSS JOIN` to `INNER JOIN` if join predicates are available.
