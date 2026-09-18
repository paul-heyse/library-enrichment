# `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.partitioned_topk.WindowFnKind.json).

<a id="op-4da708ba7c2adb8dcf339cf3"></a>
## WindowFnKind

`enum` · `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind` · datafusion-physical-plan 55.1.0

```rust
enum WindowFnKind
```

Source: `src/sorts/partitioned_topk.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Which window function `PartitionedTopKExec` is optimizing.

Different ranking functions have different per-partition retention rules:
- [`RowNumber`](Self::RowNumber): exactly K rows per partition.
- [`Rank`](Self::Rank): K rows plus any rows tied at the boundary
  ORDER BY value (RANK semantics — `WHERE rk <= K` may keep more
  than K rows when ties straddle the boundary).

<a id="op-61ea1931e400e25a7990c3d4"></a>
## Rank

`variant` · `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind::Rank` · datafusion-physical-plan 55.1.0

```rust
Rank
```

Source: `src/sorts/partitioned_topk.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`RANK()` — keep K rows plus any rows tied at the boundary.

<a id="op-99bfffea8644338484d99182"></a>
## RowNumber

`variant` · `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind::RowNumber` · datafusion-physical-plan 55.1.0

```rust
RowNumber
```

Source: `src/sorts/partitioned_topk.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`ROW_NUMBER()` — keep exactly K rows per partition.

<a id="op-45ab6d5d72a0c6ed7b25ee8b"></a>
## clone

`function` · `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> WindowFnKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind", "path": "WindowFnKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 17], "end": [64, 22], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sorts/partitioned_topk.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f99879173e04dfc1f5d8b877"></a>
## eq

`function` · `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &WindowFnKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind", "path": "WindowFnKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 30], "end": [64, 39], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sorts/partitioned_topk.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c78176c096a424894a639821"></a>
## fmt

`function` · `datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::partitioned_topk::WindowFnKind", "path": "WindowFnKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "src/sorts/partitioned_topk.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sorts/partitioned_topk.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
