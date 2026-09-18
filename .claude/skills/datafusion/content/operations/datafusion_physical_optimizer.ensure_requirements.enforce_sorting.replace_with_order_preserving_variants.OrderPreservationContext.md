# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::OrderPreservationContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.OrderPreservationContext.json).

<a id="op-859523e31c557590e73505b5"></a>
## OrderPreservationContext

`type_alias` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants::OrderPreservationContext` · datafusion-physical-optimizer 55.1.0

```rust
type OrderPreservationContext = datafusion_physical_plan::tree_node::PlanContext<bool>
```

Source: `src/ensure_requirements/enforce_sorting/replace_with_order_preserving_variants.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

For a given `plan`, this object carries the information one needs from its
descendants to decide whether it is beneficial to replace order-losing (but
somewhat faster) variants of certain operators with their order-preserving
(but somewhat slower) cousins.
