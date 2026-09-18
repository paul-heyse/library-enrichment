# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::ensure_distribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.ensure_distribution.json).

<a id="op-f51085b4b096f83e395fc264"></a>
## ensure_distribution

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution::ensure_distribution` · datafusion-physical-optimizer 55.1.0

```rust
fn ensure_distribution(dist_context: DistributionContext, config: &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<datafusion_common::tree_node::Transformed<DistributionContext>>
```

Source: `src/ensure_requirements/enforce_distribution.rs:1138`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This function checks whether we need to add additional data exchange
operators to satisfy distribution requirements. Since this function
takes care of such requirements, we should avoid manually adding data
exchange operators in other places.

This function is intended to be used in a bottom up traversal, as it
can first repartition (or newly partition) at the datasources -- these
source partitions may be later repartitioned with additional data exchange operators.
