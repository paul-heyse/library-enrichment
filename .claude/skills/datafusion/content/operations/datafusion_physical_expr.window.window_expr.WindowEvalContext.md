# `datafusion_physical_expr::window::window_expr::WindowEvalContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.window_expr.WindowEvalContext.json).

<a id="op-d3b1685bd5f4a21c388c2b2b"></a>
## WindowEvalContext

`struct` · `datafusion_physical_expr::window::window_expr::WindowEvalContext` · datafusion-physical-expr 55.1.0

```rust
struct WindowEvalContext<'a>
```

Source: `src/window/window_expr.rs:634`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Stream-level context passed to [`WindowExpr::evaluate_stateful`](../operations/datafusion_physical_expr.window.window_expr.WindowExpr.md#op-c57bbb50bf0f0a07a0cb171c).

This carries information that spans all partitions of the input, as
opposed to the per-partition state in [`PartitionBatches`](../operations/datafusion_physical_expr.window.window_expr.PartitionBatches.md#op-81d8459c793f5c56479bc66c) and
[`PartitionWindowAggStates`](../operations/datafusion_physical_expr.window.window_expr.PartitionWindowAggStates.md#op-3cc59dc09b9b92f0693b1c47). It is `non_exhaustive` so that fields can
be added without breaking implementors; construct it with
[`Default::default`] and the `with_*` builder methods.

Unresolved upstream links (retained, not inferred): ``Default::default``.

<a id="op-4a79908cdd24e0f323829d30"></a>
## clone

`function` · `datafusion_physical_expr::window::window_expr::WindowEvalContext::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> WindowEvalContext<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::window::window_expr::WindowEvalContext", "path": "WindowEvalContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [632, 17], "end": [632, 22], "filename": "src/window/window_expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window/window_expr.rs:632`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db48e57ca76c4e87d7bb324d"></a>
## default

`function` · `datafusion_physical_expr::window::window_expr::WindowEvalContext::default` · datafusion-physical-expr 55.1.0

```rust
fn default() -> WindowEvalContext<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::window::window_expr::WindowEvalContext", "path": "WindowEvalContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [632, 30], "end": [632, 37], "filename": "src/window/window_expr.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/window/window_expr.rs:632`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b929e1f4a011398608d4cd0"></a>
## fmt

`function` · `datafusion_physical_expr::window::window_expr::WindowEvalContext::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::window::window_expr::WindowEvalContext", "path": "WindowEvalContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [632, 10], "end": [632, 15], "filename": "src/window/window_expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window/window_expr.rs:632`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-163d62b81c247ad923a12dab"></a>
## most_recent_row

`struct_field` · `datafusion_physical_expr::window::window_expr::WindowEvalContext::most_recent_row` · datafusion-physical-expr 55.1.0

```rust
most_recent_row: Option<&'a arrow::record_batch::RecordBatch>
```

Source: `src/window/window_expr.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A single-row batch containing the most recent input row, whichever
partition that row belongs to. It is `Some` only when the input is
ordered by the first ORDER BY column across partitions (`Linear`
mode), in which case no future input row -- in any partition -- can
precede it in that column; implementations can use this bound to
decide whether pending window frames can be finalized before their
partition receives more data.

<a id="op-aa786924c99f3b095b971133"></a>
## with_most_recent_row

`function` · `datafusion_physical_expr::window::window_expr::WindowEvalContext::with_most_recent_row` · datafusion-physical-expr 55.1.0

```rust
fn with_most_recent_row(self, batch: Option<&'a RecordBatch>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_expr::window::window_expr::WindowEvalContext", "path": "WindowEvalContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 1], "end": [651, 2], "filename": "src/window/window_expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/window/window_expr.rs:647`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Sets the most recent input row (see [`Self::most_recent_row`](../operations/datafusion_physical_expr.window.window_expr.WindowEvalContext.md#op-163d62b81c247ad923a12dab)).
