# `datafusion_physical_optimizer::topk_repartition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.topk_repartition.json).

<a id="op-92b1d6caa1748f076e3bd317"></a>
## topk_repartition

`module` · `datafusion_physical_optimizer::topk_repartition` · datafusion-physical-optimizer 55.1.0

```rust
mod topk_repartition
```

Source: `src/topk_repartition.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Push TopK (Sort with fetch) past Hash Repartition

When a `SortExec` with a fetch limit (TopK) sits above a
`RepartitionExec(Hash)`, and the hash partition expressions are a prefix
of the sort expressions, this rule inserts a copy of the TopK below
the repartition to reduce the volume of data flowing through the shuffle.

This is correct because the hash partition key being a prefix of the sort
key guarantees that all rows with the same partition key end up in the same
output partition. Therefore, rows that survive the final TopK after
repartitioning will always survive the pre-repartition TopK as well.

## Example

Before:
```text
SortExec: TopK(fetch=3), expr=[a ASC, b ASC]
  RepartitionExec: Hash([a], 4)
    DataSourceExec
```

After:
```text
SortExec: TopK(fetch=3), expr=[a ASC, b ASC]
  RepartitionExec: Hash([a], 4)
    SortExec: TopK(fetch=3), expr=[a ASC, b ASC]
      DataSourceExec
```
