# `datafusion_physical_plan::filter_pushdown::PushedDownPredicate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter_pushdown.PushedDownPredicate.json).

<a id="op-3bebd35156d8a2c066f3e8da"></a>
## PushedDownPredicate

`struct` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate` · datafusion-physical-plan 55.1.0

```rust
struct PushedDownPredicate
```

Source: `src/filter_pushdown.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The result of a plan for pushing down a filter into a child node.
This contains references to filters so that nodes can mutate a filter
before pushing it down to a child node (e.g. to adjust a projection)
or can directly take ownership of filters that their children
could not handle.

<a id="op-362fc820795806062810bd03"></a>
## clone

`function` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PushedDownPredicate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDownPredicate", "path": "PushedDownPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 17], "end": [95, 22], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter_pushdown.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4155aba45dbee226c1e7905"></a>
## discriminant

`struct_field` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate::discriminant` · datafusion-physical-plan 55.1.0

```rust
discriminant: PushedDown
```

Source: `src/filter_pushdown.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd05f8934fe3af08085dd5d1"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDownPredicate", "path": "PushedDownPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 15], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_pushdown.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69730d71e6074d5174fec3c9"></a>
## into_inner

`function` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate::into_inner` · datafusion-physical-plan 55.1.0

```rust
fn into_inner(self) -> Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDownPredicate", "path": "PushedDownPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [122, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the wrapped [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7), discarding whether it is supported or unsupported.

<a id="op-d303ed0c59104f230d33d0c9"></a>
## predicate

`struct_field` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate::predicate` · datafusion-physical-plan 55.1.0

```rust
predicate: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/filter_pushdown.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-974787b779e5752c49224a84"></a>
## supported

`function` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate::supported` · datafusion-physical-plan 55.1.0

```rust
fn supported(predicate: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDownPredicate", "path": "PushedDownPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [122, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`PushedDownPredicate`](../operations/datafusion_physical_plan.filter_pushdown.PushedDownPredicate.md#op-3bebd35156d8a2c066f3e8da) with supported pushdown.

<a id="op-07436a39e245df6ed1470c56"></a>
## unsupported

`function` · `datafusion_physical_plan::filter_pushdown::PushedDownPredicate::unsupported` · datafusion-physical-plan 55.1.0

```rust
fn unsupported(predicate: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::PushedDownPredicate", "path": "PushedDownPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [122, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`PushedDownPredicate`](../operations/datafusion_physical_plan.filter_pushdown.PushedDownPredicate.md#op-3bebd35156d8a2c066f3e8da) with unsupported pushdown.
