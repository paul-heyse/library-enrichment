# `datafusion_optimizer::single_distinct_to_groupby`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.single_distinct_to_groupby.json).

<a id="op-7c42958eb017f62b43bd1629"></a>
## single_distinct_to_groupby

`module` · `datafusion_optimizer::single_distinct_to_groupby` · datafusion-optimizer 55.1.0

```rust
mod single_distinct_to_groupby
```

Source: `src/single_distinct_to_groupby.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

[`SingleDistinctToGroupBy`](../operations/datafusion_optimizer.single_distinct_to_groupby.SingleDistinctToGroupBy.md#op-924b1ec5018a1724c5f02bed) replaces `AGG(DISTINCT ..)` with `AGG(..) GROUP BY ..`
