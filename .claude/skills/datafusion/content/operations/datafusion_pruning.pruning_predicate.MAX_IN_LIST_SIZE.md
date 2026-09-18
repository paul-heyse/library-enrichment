# `datafusion_pruning::pruning_predicate::MAX_IN_LIST_SIZE`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_pruning.pruning_predicate.MAX_IN_LIST_SIZE.json).

<a id="op-6ffaf54ce6196eab4ef02c82"></a>
## MAX_IN_LIST_SIZE

`constant` · `datafusion_pruning::pruning_predicate::MAX_IN_LIST_SIZE` · datafusion-pruning 55.1.0

```rust
const MAX_IN_LIST_SIZE: usize = 20
```

Source: `src/pruning_predicate.rs:1467`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Default maximum number of entries in an `IN (...)` list that will be
rewritten into a chain of per-value min/max checks by
`build_predicate_expression`. Callers threading a [`PredicateRewriter`](../operations/datafusion_pruning.pruning_predicate.PredicateRewriter.md#op-580f45ac4a696442fc6d93be)
can override this via [`PredicateRewriter::with_max_in_list_size`](../operations/datafusion_pruning.pruning_predicate.PredicateRewriter.md#op-b022a1e85f97d9c752a69e3f), and
query engines can wire it from the
`datafusion.execution.parquet.max_in_list_size` config option.
