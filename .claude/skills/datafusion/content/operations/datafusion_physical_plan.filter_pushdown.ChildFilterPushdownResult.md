# `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter_pushdown.ChildFilterPushdownResult.json).

<a id="op-0489d22d5f69cdc339954c18"></a>
## ChildFilterPushdownResult

`struct` · `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult` · datafusion-physical-plan 55.1.0

```rust
struct ChildFilterPushdownResult
```

Source: `src/filter_pushdown.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The result of pushing down a single parent filter into all children.

<a id="op-7581eca6d4cfc872a42b3574"></a>
## all

`function` · `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult::all` · datafusion-physical-plan 55.1.0

```rust
fn all(&self) -> PushedDown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult", "path": "ChildFilterPushdownResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [194, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Combine all child results using AND logic.
Returns `Yes` if **all** children support the filter.
Returns `No` if **any** child rejects the filter or if there are no children.

<a id="op-47e85b0b009be02ca350d683"></a>
## any

`function` · `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult::any` · datafusion-physical-plan 55.1.0

```rust
fn any(&self) -> PushedDown
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult", "path": "ChildFilterPushdownResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [166, 1], "end": [194, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Combine all child results using OR logic.
Returns `Yes` if **any** child supports the filter.
Returns `No` if **all** children reject the filter or if there are no children.

<a id="op-345cd9c642f5fa633d075cfd"></a>
## child_results

`struct_field` · `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult::child_results` · datafusion-physical-plan 55.1.0

```rust
child_results: Vec<PushedDown>
```

Source: `src/filter_pushdown.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b09eacc23f41b70c9a0e95d3"></a>
## clone

`function` · `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ChildFilterPushdownResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult", "path": "ChildFilterPushdownResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 17], "end": [160, 22], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter_pushdown.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3d8ba33cd54effe13ed69c9"></a>
## filter

`struct_field` · `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult::filter` · datafusion-physical-plan 55.1.0

```rust
filter: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/filter_pushdown.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f85a0cc25ce13b22d5bbc1bf"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::ChildFilterPushdownResult", "path": "ChildFilterPushdownResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 10], "end": [160, 15], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_pushdown.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
