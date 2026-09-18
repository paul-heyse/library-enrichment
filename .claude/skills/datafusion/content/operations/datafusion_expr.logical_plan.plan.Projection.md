# `datafusion_expr::logical_plan::plan::Projection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Projection.json).

<a id="op-88d0abf949aefc5ba694af3b"></a>
## Projection

`struct` · `datafusion_expr::logical_plan::plan::Projection` · datafusion-expr 55.1.0

```rust
struct Projection
```

Source: `src/logical_plan/plan.rs:2447`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Evaluates an arbitrary list of expressions (essentially a
SELECT with an expression list) on its input.

<a id="op-769467e971363ed90e87c136"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Projection::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Projection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2444, 10], "end": [2444, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17c0c57f01815b799983bfee"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Projection::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Projection) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2444, 17], "end": [2444, 26], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09b45daffab222e868d977af"></a>
## expr

`struct_field` · `datafusion_expr::logical_plan::plan::Projection::expr` · datafusion-expr 55.1.0

```rust
expr: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:2449`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The list of expressions

<a id="op-bee0078a72128a69a7342ffa"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Projection::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2444, 38], "end": [2444, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbb928bd4c68b5f46d280bcb"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Projection::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2444, 32], "end": [2444, 36], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:2444`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06f55138e008faa5df316b5a"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Projection::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:2451`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-19036a6282626bbf333665f9"></a>
## new_from_schema

`function` · `datafusion_expr::logical_plan::plan::Projection::new_from_schema` · datafusion-expr 55.1.0

```rust
fn new_from_schema(input: Arc<LogicalPlan>, schema: DFSchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2468, 1], "end": [2507, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2499`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Projection using the specified output schema

<a id="op-6c5a7613ec5cd70cb8669f51"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Projection::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2457, 1], "end": [2466, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2458`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4e78de68b430393b81a9852"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Projection::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:2453`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema description of the output

<a id="op-71af97a1e7c0599f91e4a745"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::Projection::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(expr: Vec<Expr>, input: Arc<LogicalPlan>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2468, 1], "end": [2507, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2470`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Projection

<a id="op-3f982eb351da043f2bb73f0f"></a>
## try_new_with_schema

`function` · `datafusion_expr::logical_plan::plan::Projection::try_new_with_schema` · datafusion-expr 55.1.0

```rust
fn try_new_with_schema(expr: Vec<Expr>, input: Arc<LogicalPlan>, schema: DFSchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Projection", "path": "Projection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2468, 1], "end": [2507, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2476`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new Projection using the specified output schema
