# `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracking.json).

<a id="op-1fa5e4ba84971a91a36d7142"></a>
## DynamicFilterTracking

`enum` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking` · datafusion-physical-expr 55.1.0

```rust
enum DynamicFilterTracking
```

Source: `src/expressions/dynamic_filters/tracker.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Classification of a predicate according to the dynamic filters it contains.

Produced by [`DynamicFilterTracking::classify`](../operations/datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracking.md#op-abeb7ffb274ab858a84357c0) with a single tree walk so
callers can answer both "is it worth pruning at all?" and "do I need to keep
watching?" without traversing the predicate twice.

<a id="op-c641cb1fa6c4b285dc25fbee"></a>
## AllComplete

`variant` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking::AllComplete` · datafusion-physical-expr 55.1.0

```rust
AllComplete
```

Source: `src/expressions/dynamic_filters/tracker.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The predicate contains one or more dynamic filters, but all of them have
already been marked complete. Their *current* values may differ from
what was known at planning time (so a one-shot prune is still
worthwhile), but they will not change again — there is nothing to watch.

<a id="op-fe097b5560c20640054ef137"></a>
## Static

`variant` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking::Static` · datafusion-physical-expr 55.1.0

```rust
Static
```

Source: `src/expressions/dynamic_filters/tracker.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The predicate contains no [`DynamicFilterPhysicalExpr`](../operations/datafusion_physical_expr.expressions.dynamic_filters.DynamicFilterPhysicalExpr.md#op-2f06013c01568d705f497f61) at all. It is
fully static and will never change.

<a id="op-f3735d5afb0782da265cf7b7"></a>
## Watching

`variant` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking::Watching` · datafusion-physical-expr 55.1.0

```rust
Watching
```

Source: `src/expressions/dynamic_filters/tracker.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The predicate contains at least one dynamic filter that can still change.
The embedded [`DynamicFilterTracker`](../operations/datafusion_physical_expr.expressions.dynamic_filters.tracker.DynamicFilterTracker.md#op-383ca0700391d73c8cc3426b) should be polled to detect updates.

<a id="op-abeb7ffb274ab858a84357c0"></a>
## classify

`function` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking::classify` · datafusion-physical-expr 55.1.0

```rust
fn classify(predicate: &Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking", "path": "DynamicFilterTracking"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [112, 2], "filename": "src/expressions/dynamic_filters/tracker.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/tracker.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Walk `predicate` once and classify its dynamic-filter content,
subscribing to every filter that is not yet complete.

<a id="op-9ae27bd376232e8c9a4d7cad"></a>
## contains_dynamic_filter

`function` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking::contains_dynamic_filter` · datafusion-physical-expr 55.1.0

```rust
fn contains_dynamic_filter(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking", "path": "DynamicFilterTracking"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [112, 2], "filename": "src/expressions/dynamic_filters/tracker.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/tracker.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

`true` if the predicate contains any dynamic filter (complete or not),
i.e. its value may differ from what was known at planning time and is
therefore worth re-evaluating at least once.

<a id="op-4f34b6c876e83772618af1bd"></a>
## fmt

`function` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking", "path": "DynamicFilterTracking"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 10], "end": [53, 15], "filename": "src/expressions/dynamic_filters/tracker.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expressions/dynamic_filters/tracker.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-523e0432a864db5a6fae6d33"></a>
## watcher

`function` · `datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking::watcher` · datafusion-physical-expr 55.1.0

```rust
fn watcher(&mut self) -> Option<&mut DynamicFilterTracker>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::expressions::dynamic_filters::tracker::DynamicFilterTracking", "path": "DynamicFilterTracking"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [112, 2], "filename": "src/expressions/dynamic_filters/tracker.rs"}, "trait": null, "trait_path": null}`

Source: `src/expressions/dynamic_filters/tracker.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Mutable access to the underlying tracker when there is still something to
watch.
