# `datafusion_optimizer::unions_to_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.unions_to_filter.json).

<a id="op-8a1f56e2f742edf088411b0f"></a>
## unions_to_filter

`module` · `datafusion_optimizer::unions_to_filter` · datafusion-optimizer 55.1.0

```rust
mod unions_to_filter
```

Source: `src/unions_to_filter.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Rewrites `UNION DISTINCT` branches that differ only by filter predicates
into a single filtered branch plus `DISTINCT`.
