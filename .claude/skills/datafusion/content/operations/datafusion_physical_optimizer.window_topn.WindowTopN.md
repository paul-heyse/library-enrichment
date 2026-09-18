# `datafusion_physical_optimizer::window_topn::WindowTopN`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.window_topn.WindowTopN.json).

<a id="op-d3e707f45dfad8feac4ca8f0"></a>
## WindowTopN

`struct` · `datafusion_physical_optimizer::window_topn::WindowTopN` · datafusion-physical-optimizer 55.1.0

```rust
struct WindowTopN
```

Source: `src/window_topn.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Physical optimizer rule that converts per-partition `ROW_NUMBER` and
`RANK` top-K queries into a more efficient plan using
[`PartitionedTopKExec`].

# Pattern Detected

```text
FilterExec(<ranking fn output> <= K)
  [optional ProjectionExec]
    BoundedWindowAggExec(<ranking fn> PARTITION BY ... ORDER BY ...)
```

# Replacement

```text
[optional ProjectionExec]
  BoundedWindowAggExec(<ranking fn> PARTITION BY ... ORDER BY ...)
    PartitionedTopKExec(fn=<row_number|rank>, partition_keys, order_keys, fetch=K)
```

The `FilterExec` is removed entirely. The child of `BoundedWindowAggExec` is now
`PartitionedTopKExec`, which maintains a per-partition top-K heap (and,
for `RANK`, a sibling ties `Vec`) instead of sorting the whole dataset.

# Supported Predicates

- `rn <= K` → fetch = K
- `rn < K` → fetch = K - 1
- `K >= rn` (flipped) → fetch = K
- `K > rn` (flipped) → fetch = K - 1

# When the Rule Fires

All of the following must be true:
- Config flag `enable_window_topn` is `true`
- The plan matches `FilterExec → [ProjectionExec] → BoundedWindowAggExec`
- The window function is `ROW_NUMBER` or `RANK` (not `DENSE_RANK`)
- The window function has a `PARTITION BY` clause (global top-K is
  already handled by `SortExec` with `fetch`)
- For `RANK`: a non-empty `ORDER BY` clause (otherwise all rows tie
  at rank 1 — the optimization is useless and the boundary-tie storage
  would be unbounded)
- The filter predicate compares the window output column to an integer
  literal using `<=`, `<`, `>=`, or `>`

[`PartitionedTopKExec`]: datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec

<a id="op-49a89e17df35a52976d523a9"></a>
## clone

`function` · `datafusion_physical_optimizer::window_topn::WindowTopN::clone` · datafusion-physical-optimizer 55.1.0

```rust
fn clone(&self) -> WindowTopN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::window_topn::WindowTopN", "path": "WindowTopN"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 19], "end": [118, 24], "filename": "src/window_topn.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_topn.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a732fdbf03a74e34b8b8511d"></a>
## default

`function` · `datafusion_physical_optimizer::window_topn::WindowTopN::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> WindowTopN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::window_topn::WindowTopN", "path": "WindowTopN"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 10], "end": [118, 17], "filename": "src/window_topn.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/window_topn.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-534cb8c726a756b24df2a3e1"></a>
## fmt

`function` · `datafusion_physical_optimizer::window_topn::WindowTopN::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::window_topn::WindowTopN", "path": "WindowTopN"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 26], "end": [118, 31], "filename": "src/window_topn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_topn.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae28d69d3f4a514f4b847b63"></a>
## name

`function` · `datafusion_physical_optimizer::window_topn::WindowTopN::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::window_topn::WindowTopN", "path": "WindowTopN"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [238, 2], "filename": "src/window_topn.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/window_topn.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd5d7c628a631512765027cb"></a>
## new

`function` · `datafusion_physical_optimizer::window_topn::WindowTopN::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::window_topn::WindowTopN", "path": "WindowTopN"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [207, 2], "filename": "src/window_topn.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_topn.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d9754832f73cdf2d08a777d"></a>
## optimize

`function` · `datafusion_physical_optimizer::window_topn::WindowTopN::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::window_topn::WindowTopN", "path": "WindowTopN"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [238, 2], "filename": "src/window_topn.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/window_topn.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca97b4793cfa6beed3a87fd1"></a>
## schema_check

`function` · `datafusion_physical_optimizer::window_topn::WindowTopN::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::window_topn::WindowTopN", "path": "WindowTopN"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 1], "end": [238, 2], "filename": "src/window_topn.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/window_topn.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
