# `datafusion_physical_optimizer::join_selection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.join_selection.json).

<a id="op-7731ad4887cc74f3772d3f27"></a>
## join_selection

`module` · `datafusion_physical_optimizer::join_selection` · datafusion-physical-optimizer 55.1.0

```rust
mod join_selection
```

Source: `src/join_selection.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

The [`JoinSelection`](../operations/datafusion_physical_optimizer.join_selection.JoinSelection.md#op-33eb2e7b16aaea008290f269) rule tries to modify a given plan so that it can
accommodate infinite sources and utilize statistical information (if there
is any) to obtain more performant plans. To achieve the first goal, it
tries to transform a non-runnable query (with the given infinite sources)
into a runnable query by replacing pipeline-breaking join operations with
pipeline-friendly ones. To achieve the second goal, it selects the proper
`PartitionMode` and the build side using the available statistics for hash joins.
