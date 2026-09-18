# `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.limit_pushdown_past_window.LimitPushPastWindows.json).

<a id="op-4999b01cd92247da140f44c1"></a>
## LimitPushPastWindows

`struct` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows` · datafusion-physical-optimizer 55.1.0

```rust
struct LimitPushPastWindows
```

Source: `src/limit_pushdown_past_window.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This rule inspects [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673)'s attempting to find fetch limits that were not pushed
down by `LimitPushdown` because [BoundedWindowAggExec](../operations/datafusion_physical_plan.windows.bounded_window_agg_exec.BoundedWindowAggExec.md#op-4f455d56121cc7d9007e9454)s were "in the way". If the window is
bounded by [WindowFrameUnits::Rows](../operations/datafusion_expr.window_frame.WindowFrameUnits.md#op-ae14917a0baeee1f6b1224e9) then we calculate the adjustment needed to grow the limit
and continue pushdown.

<a id="op-08bd805620237f5b0544da17"></a>
## clone

`function` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows::clone` · datafusion-physical-optimizer 55.1.0

```rust
fn clone(&self) -> LimitPushPastWindows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows", "path": "LimitPushPastWindows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 19], "end": [41, 24], "filename": "src/limit_pushdown_past_window.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/limit_pushdown_past_window.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e67e77657c76bfe4806e4f92"></a>
## default

`function` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> LimitPushPastWindows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows", "path": "LimitPushPastWindows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 17], "filename": "src/limit_pushdown_past_window.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/limit_pushdown_past_window.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c5665033b9e4e504c4411aa"></a>
## fmt

`function` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows", "path": "LimitPushPastWindows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 26], "end": [41, 31], "filename": "src/limit_pushdown_past_window.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/limit_pushdown_past_window.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7749fc0ec9ce1347aa63e0e"></a>
## name

`function` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows", "path": "LimitPushPastWindows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [152, 2], "filename": "src/limit_pushdown_past_window.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limit_pushdown_past_window.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-650557ff9c1f8b8691ba551d"></a>
## new

`function` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows", "path": "LimitPushPastWindows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [48, 2], "filename": "src/limit_pushdown_past_window.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit_pushdown_past_window.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e85632385db47cbd6a7796f8"></a>
## optimize

`function` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, original: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows", "path": "LimitPushPastWindows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [152, 2], "filename": "src/limit_pushdown_past_window.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limit_pushdown_past_window.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7868aea0b1f3fe0e160617f2"></a>
## schema_check

`function` · `datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limit_pushdown_past_window::LimitPushPastWindows", "path": "LimitPushPastWindows"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [152, 2], "filename": "src/limit_pushdown_past_window.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limit_pushdown_past_window.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
