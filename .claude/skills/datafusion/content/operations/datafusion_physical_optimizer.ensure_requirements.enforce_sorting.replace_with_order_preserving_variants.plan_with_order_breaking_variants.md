# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::plan_with_order_breaking_variants`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.plan_with_order_breaking_variants.json).

<a id="op-e37390ad597a76a86ea36e7f"></a>
## plan_with_order_breaking_variants

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::plan_with_order_breaking_variants` · datafusion-physical-optimizer 55.1.0

```rust
fn plan_with_order_breaking_variants(sort_input: OrderPreservationContext) -> datafusion_common::Result<OrderPreservationContext>
```

Source: `src/ensure_requirements/enforce_sorting/replace_with_order_preserving_variants.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Calculates the updated plan by replacing operators that preserve ordering
inside `sort_input` with their order-breaking variants. This will restore
the original plan modified by [`plan_with_order_preserving_variants`](../operations/datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.plan_with_order_preserving_variants.md#op-46af66fb1ed03a9108eecb38).
