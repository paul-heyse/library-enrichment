# `datafusion_optimizer::eliminate_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.eliminate_filter.json).

<a id="op-fa417d63792280ed46e3ea2b"></a>
## eliminate_filter

`module` · `datafusion_optimizer::eliminate_filter` · datafusion-optimizer 55.1.0

```rust
mod eliminate_filter
```

Source: `src/eliminate_filter.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

[`EliminateFilter`](../operations/datafusion_optimizer.eliminate_filter.EliminateFilter.md#op-bdb24402e21ac82bef3d5ee9) replaces `where false` or `where null` with an empty relation.
