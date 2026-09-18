# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.replace_with_order_preserving_variants.json).

<a id="op-ea161cce1b9cb3bd0d659a7c"></a>
## replace_with_order_preserving_variants

`module` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::replace_with_order_preserving_variants` · datafusion-physical-optimizer 55.1.0

```rust
mod replace_with_order_preserving_variants
```

Source: `src/ensure_requirements/enforce_sorting/replace_with_order_preserving_variants.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Optimizer rule that replaces executors that lose ordering with their
order-preserving variants when it is helpful; either in terms of
performance or to accommodate unbounded streams by fixing the pipeline.
