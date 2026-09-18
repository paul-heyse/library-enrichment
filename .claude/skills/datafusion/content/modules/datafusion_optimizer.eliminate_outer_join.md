# `datafusion_optimizer::eliminate_outer_join`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_outer_join.json).

<a id="op-fdba3a812d67c302790766b1"></a>
## eliminate_outer_join

`module` · `datafusion_optimizer::eliminate_outer_join` · datafusion-optimizer 55.1.0

```rust
mod eliminate_outer_join
```

Source: `src/eliminate_outer_join.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

[`EliminateOuterJoin`](../operations/datafusion_optimizer.eliminate_outer_join.EliminateOuterJoin.md#op-c7b50fdf31b5ebcd77bf3f63) rewrites outer joins to simpler join types when
filters make the outer rows unnecessary (e.g. `LEFT`/`RIGHT` to `INNER`,
and `FULL` to `LEFT`/`RIGHT`/`INNER`).
