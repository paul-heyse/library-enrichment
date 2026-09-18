# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::assign_initial_requirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.assign_initial_requirements.json).

<a id="op-44648d7c53337c816ba9bd80"></a>
## assign_initial_requirements

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::assign_initial_requirements` · datafusion-physical-optimizer 55.1.0

```rust
fn assign_initial_requirements(sort_push_down: &mut SortPushDown)
```

Source: `src/ensure_requirements/enforce_sorting/sort_pushdown.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Assigns the ordering requirement of the root node to the its children.
