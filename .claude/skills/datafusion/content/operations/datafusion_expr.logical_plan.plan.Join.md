# `datafusion_expr::logical_plan::plan::Join`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Join.json).

<a id="op-4605ce35c4fc82deb44cd317"></a>
## Join

`struct` · `datafusion_expr::logical_plan::plan::Join` · datafusion-expr 55.1.0

```rust
struct Join
```

Source: `src/logical_plan/plan.rs:4231`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Join two logical plans on one or more join columns

<a id="op-3320c2c5058e2e3dca59efaa"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Join::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Join
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4230, 17], "end": [4230, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:4230`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-462697fbbadce2eb2fbcde95"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Join::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Join) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4230, 24], "end": [4230, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:4230`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ad7ca83f61dc09fe87501cd"></a>
## filter

`struct_field` · `datafusion_expr::logical_plan::plan::Join::filter` · datafusion-expr 55.1.0

```rust
filter: Option<Expr>
```

Source: `src/logical_plan/plan.rs:4239`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Filters applied during join (non-equi conditions)

<a id="op-c4e384954c7c3f6bc6ef4bba"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Join::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4230, 10], "end": [4230, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:4230`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9557e0a3e2722edcd1f8d19"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Join::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4230, 39], "end": [4230, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:4230`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce0b42506dbcada2181a53dc"></a>
## join_constraint

`struct_field` · `datafusion_expr::logical_plan::plan::Join::join_constraint` · datafusion-expr 55.1.0

```rust
join_constraint: JoinConstraint
```

Source: `src/logical_plan/plan.rs:4243`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Join constraint

<a id="op-27d3b662266107a641fc4224"></a>
## join_type

`struct_field` · `datafusion_expr::logical_plan::plan::Join::join_type` · datafusion-expr 55.1.0

```rust
join_type: JoinType
```

Source: `src/logical_plan/plan.rs:4241`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Join type

<a id="op-4a51f43e6f80587c633a6649"></a>
## left

`struct_field` · `datafusion_expr::logical_plan::plan::Join::left` · datafusion-expr 55.1.0

```rust
left: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:4233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Left input

<a id="op-4405a01298c038c1ec088930"></a>
## null_aware

`struct_field` · `datafusion_expr::logical_plan::plan::Join::null_aware` · datafusion-expr 55.1.0

```rust
null_aware: bool
```

Source: `src/logical_plan/plan.rs:4255`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether this is a null-aware anti join (for NOT IN semantics).

Only applies to LeftAnti joins. When true, implements SQL NOT IN semantics where:
- If the right side (subquery) contains any NULL in join keys, no rows are output
- Left side rows with NULL in join keys are not output

This is required for correct NOT IN subquery behavior with three-valued logic.

<a id="op-0bff712b2b0550b072bcbfb9"></a>
## null_equality

`struct_field` · `datafusion_expr::logical_plan::plan::Join::null_equality` · datafusion-expr 55.1.0

```rust
null_equality: datafusion_common::NullEquality
```

Source: `src/logical_plan/plan.rs:4247`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Defines the null equality for the join.

<a id="op-228d974f78b51e4c0fa65a88"></a>
## on

`struct_field` · `datafusion_expr::logical_plan::plan::Join::on` · datafusion-expr 55.1.0

```rust
on: Vec<(Expr, Expr)>
```

Source: `src/logical_plan/plan.rs:4237`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Equijoin clause expressed as pairs of (left, right) join expressions

<a id="op-caa0a4257c243ca20f4272ca"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Join::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4364, 1], "end": [4406, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:4365`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b0ecf9d92efa08550c884a2"></a>
## right

`struct_field` · `datafusion_expr::logical_plan::plan::Join::right` · datafusion-expr 55.1.0

```rust
right: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:4235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Right input

<a id="op-49c6db80988d8d57b40f233e"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Join::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:4245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The output schema, containing fields from the left and right inputs

<a id="op-e822c6f4b153a36481ff0e7e"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::Join::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(left: Arc<LogicalPlan>, right: Arc<LogicalPlan>, on: Vec<(Expr, Expr)>, filter: Option<Expr>, join_type: JoinType, join_constraint: JoinConstraint, null_equality: NullEquality, null_aware: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4258, 1], "end": [4361, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4279`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new Join operator with automatically computed schema.

This constructor computes the schema based on the join type and inputs,
removing the need to manually specify the schema or call `recompute_schema`.

# Arguments

* `left` - Left input plan
* `right` - Right input plan
* `on` - Join condition as a vector of (left_expr, right_expr) pairs
* `filter` - Optional filter expression (for non-equijoin conditions)
* `join_type` - Type of join (Inner, Left, Right, etc.)
* `join_constraint` - Join constraint (On, Using)
* `null_equality` - How to handle nulls in join comparisons
* `null_aware` - Whether this is a null-aware anti join (for NOT IN semantics)

# Returns

A new Join operator with the computed schema

<a id="op-2371b0360267edad0af44025"></a>
## try_new_with_project_input

`function` · `datafusion_expr::logical_plan::plan::Join::try_new_with_project_input` · datafusion-expr 55.1.0

```rust
fn try_new_with_project_input(original: &LogicalPlan, left: Arc<LogicalPlan>, right: Arc<LogicalPlan>, column_on: (Vec<Column>, Vec<Column>)) -> Result<(Self, bool)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4258, 1], "end": [4361, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:4306`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create Join with input which wrapped with projection, this method is used in physical planning only to help
create the physical join.
