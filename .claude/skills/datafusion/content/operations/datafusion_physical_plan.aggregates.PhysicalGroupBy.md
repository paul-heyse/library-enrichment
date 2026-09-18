# `datafusion_physical_plan::aggregates::PhysicalGroupBy`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.PhysicalGroupBy.json).

<a id="op-6f1aa7f2cbf39dd2f1e47358"></a>
## PhysicalGroupBy

`struct` · `datafusion_physical_plan::aggregates::PhysicalGroupBy` · datafusion-physical-plan 55.1.0

```rust
struct PhysicalGroupBy
```

Source: `src/aggregates/mod.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Represents `GROUP BY` clause in the plan (including the more general GROUPING SET)
In the case of a simple `GROUP BY a, b` clause, this will contain the expression [a, b]
and a single group [false, false].
In the case of `GROUP BY GROUPING SETS/CUBE/ROLLUP` the planner will expand the expression
into multiple groups, using null expressions to align each group.
For example, with a group by clause `GROUP BY GROUPING SETS ((a,b),(a),(b))` the planner should
create a `PhysicalGroupBy` like
```text
PhysicalGroupBy {
    expr: [(col(a), a), (col(b), b)],
    null_expr: [(NULL, a), (NULL, b)],
    groups: [
        [false, false], // (a,b)
        [false, true],  // (a) <=> (a, NULL)
        [true, false]   // (b) <=> (NULL, b)
    ]
}
```

<a id="op-d14ad39a19f5194ac2b78b83"></a>
## as_final

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::as_final` · datafusion-physical-plan 55.1.0

```rust
fn as_final(&self) -> PhysicalGroupBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the `PhysicalGroupBy` for a final aggregation if `self` is used for a partial
aggregation.

<a id="op-e8b08b0a4970203f30dec066"></a>
## clone

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PhysicalGroupBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 10], "end": [414, 15], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregates/mod.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98b7b1f88bcd415fb9adf27f"></a>
## default

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> PhysicalGroupBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 24], "end": [414, 31], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregates/mod.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-967ac2933063e837120109a1"></a>
## eq

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &PhysicalGroupBy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 1], "end": [646, 2], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aggregates/mod.rs:630`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92f1f164a83e7a2dbd750a0e"></a>
## expr

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::expr` · datafusion-physical-plan 55.1.0

```rust
fn expr(&self) -> &[(Arc<dyn PhysicalExpr>, String)]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the group expressions

<a id="op-8467e5addd7e8b618ebd003a"></a>
## exprs_nullable

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::exprs_nullable` · datafusion-physical-plan 55.1.0

```rust
fn exprs_nullable(&self) -> Vec<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Calculate GROUP BY expressions nullable

<a id="op-71ff58a21385fef908aa26b5"></a>
## fmt

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 17], "end": [414, 22], "filename": "src/aggregates/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregates/mod.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dde918486de9c163532d642"></a>
## group_schema

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::group_schema` · datafusion-physical-plan 55.1.0

```rust
fn group_schema(&self, schema: &Schema) -> Result<SchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55d6a65ad176b0aff9965d70"></a>
## groups

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::groups` · datafusion-physical-plan 55.1.0

```rust
fn groups(&self) -> &[Vec<bool>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:487`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the group null masks

<a id="op-31351bb24d8172129add2115"></a>
## has_grouping_set

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::has_grouping_set` · datafusion-physical-plan 55.1.0

```rust
fn has_grouping_set(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if this grouping uses GROUPING SETS, CUBE or ROLLUP.

<a id="op-52286af8a6e66ff2a277bdb9"></a>
## input_exprs

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::input_exprs` · datafusion-physical-plan 55.1.0

```rust
fn input_exprs(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:508`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Calculate GROUP BY expressions according to input schema.

<a id="op-452434ed8ecef5167f84f4fe"></a>
## is_empty

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if this `PhysicalGroupBy` has no group expressions

<a id="op-c19513678920e758c70c4854"></a>
## is_single

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::is_single` · datafusion-physical-plan 55.1.0

```rust
fn is_single(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if this is a "simple" GROUP BY (not using GROUPING SETS/CUBE/ROLLUP).
This determines whether the `__grouping_id` column is included in the output schema.

<a id="op-7b528035ce8a1c71474dbd31"></a>
## is_true_no_grouping

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::is_true_no_grouping` · datafusion-physical-plan 55.1.0

```rust
fn is_true_no_grouping(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if this has no grouping at all (including no GROUPING SETS)

<a id="op-0f9204d52770b7fecb476534"></a>
## new

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::new` · datafusion-physical-plan 55.1.0

```rust
fn new(expr: Vec<(Arc<dyn PhysicalExpr>, String)>, null_expr: Vec<(Arc<dyn PhysicalExpr>, String)>, groups: Vec<Vec<bool>>, has_grouping_set: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new `PhysicalGroupBy`

<a id="op-24fb4fbde6627e54f5bef03b"></a>
## new_single

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::new_single` · datafusion-physical-plan 55.1.0

```rust
fn new_single(expr: Vec<(Arc<dyn PhysicalExpr>, String)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a GROUPING SET with only a single group. This is the "standard"
case when building a plan from an expression such as `GROUP BY a,b,c`

<a id="op-c3e264515a1fb5d11c0cc748"></a>
## null_expr

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::null_expr` · datafusion-physical-plan 55.1.0

```rust
fn null_expr(&self) -> &[(Arc<dyn PhysicalExpr>, String)]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the null expressions

<a id="op-3cfa80412324491fbb263156"></a>
## num_group_exprs

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::num_group_exprs` · datafusion-physical-plan 55.1.0

```rust
fn num_group_exprs(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:545`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number expression as grouping keys.

<a id="op-b94203852d9a2bfa707b7a2f"></a>
## output_exprs

`function` · `datafusion_physical_plan::aggregates::PhysicalGroupBy::output_exprs` · datafusion-physical-plan 55.1.0

```rust
fn output_exprs(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::aggregates::PhysicalGroupBy", "path": "PhysicalGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [430, 1], "end": [627, 2], "filename": "src/aggregates/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregates/mod.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return grouping expressions as they occur in the output schema.
