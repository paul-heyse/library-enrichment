# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::plan_with_order_preserving_variants`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.plan_with_order_preserving_variants.json).

<a id="op-46af66fb1ed03a9108eecb38"></a>
## plan_with_order_preserving_variants

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::plan_with_order_preserving_variants` · datafusion-physical-optimizer 55.1.0

```rust
fn plan_with_order_preserving_variants(sort_input: OrderPreservationContext, is_spr_better: bool, is_spm_better: bool, fetch: Option<usize>) -> datafusion_common::Result<OrderPreservationContext>
```

Source: `src/ensure_requirements/enforce_sorting/replace_with_order_preserving_variants.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Calculates the updated plan by replacing operators that lose ordering
inside `sort_input` with their order-preserving variants. This will
generate an alternative plan, which will be accepted or rejected later on
depending on whether it helps us remove a `SortExec`.
