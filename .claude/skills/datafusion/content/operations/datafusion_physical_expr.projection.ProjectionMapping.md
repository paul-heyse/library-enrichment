# `datafusion_physical_expr::projection::ProjectionMapping`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.ProjectionMapping.json).

<a id="op-0051327cd0be522ecb3e03bb"></a>
## ProjectionMapping

`struct` · `datafusion_physical_expr::projection::ProjectionMapping` · datafusion-physical-expr 55.1.0

```rust
struct ProjectionMapping
```

Source: `src/projection.rs:1199`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Stores the mapping between source expressions and target expressions for a
projection.

<a id="op-6cee81cacb34a22bf07fef15"></a>
## Target

`assoc_type` · `datafusion_physical_expr::projection::ProjectionMapping::Target` · datafusion-physical-expr 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionMapping", "path": "ProjectionMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1325, 1], "end": [1331, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/projection.rs:1326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-750da24578966f4b11913954"></a>
## clone

`function` · `datafusion_physical_expr::projection::ProjectionMapping::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ProjectionMapping
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionMapping", "path": "ProjectionMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1198, 10], "end": [1198, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:1198`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-246fbe7fd026060744a253fa"></a>
## deref

`function` · `datafusion_physical_expr::projection::ProjectionMapping::deref` · datafusion-physical-expr 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionMapping", "path": "ProjectionMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1325, 1], "end": [1331, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/projection.rs:1328`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a686b568bd741667ca293846"></a>
## fmt

`function` · `datafusion_physical_expr::projection::ProjectionMapping::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionMapping", "path": "ProjectionMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1198, 17], "end": [1198, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:1198`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12b881d707c102d23a2b4bdc"></a>
## from_indices

`function` · `datafusion_physical_expr::projection::ProjectionMapping::from_indices` · datafusion-physical-expr 55.1.0

```rust
fn from_indices(indices: &[usize], schema: &SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionMapping", "path": "ProjectionMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 1], "end": [1323, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:1315`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Constructs a subset mapping using the provided indices.

This is used when the output is a subset of the input without any
other transformations. The indices are for columns in the schema.

<a id="op-cb23053b1bc1b2aeb9cb798c"></a>
## from_iter

`function` · `datafusion_physical_expr::projection::ProjectionMapping::from_iter` · datafusion-physical-expr 55.1.0

```rust
fn from_iter<T: IntoIterator<Item = (Arc<dyn PhysicalExpr>, ProjectionTargets)>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionMapping", "path": "ProjectionMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1333, 1], "end": [1341, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}, {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionTargets", "path": "ProjectionTargets"}}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/projection.rs:1334`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14888fda3e36e6f850dc0b13"></a>
## try_new

`function` · `datafusion_physical_expr::projection::ProjectionMapping::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(expr: impl IntoIterator<Item = (Arc<dyn PhysicalExpr>, String)>, input_schema: &SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionMapping", "path": "ProjectionMapping"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 1], "end": [1323, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:1219`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Constructs the mapping between a projection's input and output
expressions.

For example, given the input projection expressions (`a + b`, `c + d`)
and an output schema with two columns `"c + d"` and `"a + b"`, the
projection mapping would be:

```text
 [0]: (c + d, [(col("c + d"), 0)])
 [1]: (a + b, [(col("a + b"), 1)])
```

where `col("c + d")` means the column named `"c + d"`.
