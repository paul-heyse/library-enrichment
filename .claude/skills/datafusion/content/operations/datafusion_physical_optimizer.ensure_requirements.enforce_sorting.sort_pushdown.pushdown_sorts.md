# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::pushdown_sorts`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.pushdown_sorts.json).

<a id="op-37bce4490c79d4f27b1cd4ed"></a>
## pushdown_sorts

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::pushdown_sorts` · datafusion-physical-optimizer 55.1.0

```rust
fn pushdown_sorts(sort_push_down: SortPushDown) -> datafusion_common::Result<SortPushDown>
```

Source: `src/ensure_requirements/enforce_sorting/sort_pushdown.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Tries to push down the sort requirements as far as possible, if decides a `SortExec` is unnecessary removes it.
