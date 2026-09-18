# `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracker`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracker.json).

<a id="op-383ca0700391d73c8cc3426b"></a>
## DynamicFilterTracker

`struct` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracker` · datafusion-physical-expr 55.1.0

```rust
struct DynamicFilterTracker
```

Source: `src/expressions/dynamic_filters/tracker.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Watches every still-incomplete [`DynamicFilterPhysicalExpr`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-2f06013c01568d705f497f61) reachable from a
predicate and reports, cheaply, whether any of them has been updated since
the last check.

Obtain one from [`DynamicFilterTracking::classify`](../operations/datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracking.md#op-abeb7ffb274ab858a84357c0) via
[`DynamicFilterTracking::watcher`](../operations/datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracking.md#op-523e0432a864db5a6fae6d33); the `Watching` variant carries it only
when there is at least one dynamic filter that can still change.

<a id="op-751d75bb944e80962df650ea"></a>
## changed

`function` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracker::changed` · datafusion-physical-expr 55.1.0

```rust
fn changed(&mut self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracker", "path": "DynamicFilterTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [145, 2], "filename": "src/expressions/dynamic_filters/tracker.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/tracker.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns `true` if any watched filter's expression has advanced since the
previous call.

Filters that have completed are dropped from the watch set as they are
observed; once every filter has completed this is a no-op that always
returns `false`.

<a id="op-244d1943ea991b9c0f652cb9"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracker::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracker", "path": "DynamicFilterTracker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 15], "filename": "src/expressions/dynamic_filters/tracker.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/dynamic_filters/tracker.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
