# `datafusion_physical_expr::projection::ProjectionTargets`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.ProjectionTargets.json).

<a id="op-1e228730ebca0d61676ef600"></a>
## ProjectionTargets

`struct` · `datafusion_physical_expr::projection::ProjectionTargets` · datafusion-physical-expr 55.1.0

```rust
struct ProjectionTargets
```

Source: `src/projection.rs:1162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Stores target expressions, along with their indices, that associate with a
source expression in a projection mapping.

<a id="op-ca1a30e1de2b44446de795b8"></a>
## Target

`assoc_type` · `datafusion_physical_expr::projection::ProjectionTargets::Target` · datafusion-physical-expr 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1182, 1], "end": [1188, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/projection.rs:1183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dac2baff49df9324ecefa1c1"></a>
## clone

`function` · `datafusion_physical_expr::projection::ProjectionTargets::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ProjectionTargets
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1161, 10], "end": [1161, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:1161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f80d29d5e15f874a708d0bd4"></a>
## default

`function` · `datafusion_physical_expr::projection::ProjectionTargets::default` · datafusion-physical-expr 55.1.0

```rust
fn default() -> ProjectionTargets
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1161, 24], "end": [1161, 31], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/projection.rs:1161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-352ca97536f690fe2b1a190f"></a>
## deref

`function` · `datafusion_physical_expr::projection::ProjectionTargets::deref` · datafusion-physical-expr 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1182, 1], "end": [1188, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/projection.rs:1185`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2efea93d7a22ddd42bebc686"></a>
## first

`function` · `datafusion_physical_expr::projection::ProjectionTargets::first` · datafusion-physical-expr 55.1.0

```rust
fn first(&self) -> &(Arc<dyn PhysicalExpr>, usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1169, 1], "end": [1180, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:1171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the first target expression and its index.

<a id="op-d39c872aa7fb00bf10105de2"></a>
## fmt

`function` · `datafusion_physical_expr::projection::ProjectionTargets::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1161, 17], "end": [1161, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:1161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e974a12b613801c13becedc"></a>
## from

`function` · `datafusion_physical_expr::projection::ProjectionTargets::from` · datafusion-physical-expr 55.1.0

```rust
fn from(exprs_indices: Vec<(Arc<dyn PhysicalExpr>, usize)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1190, 1], "end": [1194, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}, {"primitive": "usize"}]}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/projection.rs:1191`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca26126f0067c925833eb5cd"></a>
## push

`function` · `datafusion_physical_expr::projection::ProjectionTargets::push` · datafusion-physical-expr 55.1.0

```rust
fn push(&mut self, target: (Arc<dyn PhysicalExpr>, usize))
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1169, 1], "end": [1180, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:1177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Adds a target expression and its index to the list of targets.
