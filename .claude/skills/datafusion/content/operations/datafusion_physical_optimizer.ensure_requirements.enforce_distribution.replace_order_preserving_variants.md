# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::replace_order_preserving_variants`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.replace_order_preserving_variants.json).

<a id="op-4de914ae4c320a941fcd72d1"></a>
## replace_order_preserving_variants

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::replace_order_preserving_variants` · datafusion-physical-optimizer 55.1.0

```rust
fn replace_order_preserving_variants(context: DistributionContext) -> datafusion_common::error::Result<DistributionContext>
```

Source: `src/ensure_requirements/enforce_distribution.rs:891`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Updates the [`DistributionContext`](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_distribution.DistributionContext.md#op-fe24bd62f27dcde3fcf50e83) if preserving ordering while changing partitioning is not helpful or desirable.

Assume that following plan is given:
```text
"SortPreservingMergeExec: \[a@0 ASC]"
"  RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=10, preserve_order=true",
"    RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=2, preserve_order=true",
"      DataSourceExec: file_groups={2 groups: \[\[x], \[y]]}, projection=\[a, b, c, d, e], output_ordering=\[a@0 ASC], file_type=parquet",
```

This function converts plan above to the following:

```text
"CoalescePartitionsExec"
"  RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=10",
"    RepartitionExec: partitioning=RoundRobinBatch(10), input_partitions=2",
"      DataSourceExec: file_groups={2 groups: \[\[x], \[y]]}, projection=\[a, b, c, d, e], output_ordering=\[a@0 ASC], file_type=parquet",
```
