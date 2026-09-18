# `datafusion_physical_optimizer::ensure_requirements::enforce_distribution`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_distribution.json).

<a id="op-443b3d2102cb4ffecd4b6bbf"></a>
## enforce_distribution

`module` · `datafusion_physical_optimizer::ensure_requirements::enforce_distribution` · datafusion-physical-optimizer 55.1.0

```rust
mod enforce_distribution
```

Source: `src/ensure_requirements/enforce_distribution.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Distribution enforcement helpers. The standalone `EnforceDistribution`
rule that previously lived here has been retired in favour of
`EnsureRequirements` (which composes distribution and sorting
enforcement into a single idempotent pass). The helpers in this
module — `adjust_input_keys_ordering`, `reorder_join_keys_to_inputs`,
`DistributionContext`, `ensure_distribution`, …  — are used directly
by `EnsureRequirements`.

These helpers inspect the physical plan with respect to distribution
requirements and add [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b)s to satisfy them when necessary.
If increasing parallelism is beneficial (and also desirable according to
configuration), they increase partition counts in the physical plan.
