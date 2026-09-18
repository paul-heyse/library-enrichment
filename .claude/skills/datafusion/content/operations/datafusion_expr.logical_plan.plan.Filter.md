# `datafusion_expr::logical_plan::plan::Filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Filter.json).

<a id="op-46bb7ed0568ba578d8668f2d"></a>
## Filter

`struct` · `datafusion_expr::logical_plan::plan::Filter` · datafusion-expr 55.1.0

```rust
struct Filter
```

Source: `src/logical_plan/plan.rs:2637`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Filters rows from its input that do not match an
expression (essentially a WHERE clause with a predicate
expression).

Semantically, `<predicate>` is evaluated for each row of the input;
If the value of `<predicate>` is true, the input row is passed to
the output. If the value of `<predicate>` is false, the row is
discarded.

Filter should not be created directly but instead use `try_new()`
and that these fields are only pub to support pattern matching

<a id="op-55d82034cbd4fc1ad58716de"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Filter::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Filter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Filter", "path": "Filter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2635, 17], "end": [2635, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2635`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4852ef2350e024c94640dcd9"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Filter::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Filter) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Filter", "path": "Filter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2635, 24], "end": [2635, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2635`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02694824d68d5d57f172d0be"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Filter::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Filter", "path": "Filter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2635, 10], "end": [2635, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2635`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d54cbb2bf753d9a1d3de866e"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Filter::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Filter", "path": "Filter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2635, 51], "end": [2635, 55], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:2635`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7302a4f1e5e2aa269925e186"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::plan::Filter::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/plan.rs:2641`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The incoming logical plan

<a id="op-78c8c816518ebfda57d137c7"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Filter::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Filter) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Filter", "path": "Filter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2635, 39], "end": [2635, 49], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2635`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49a74a1b834e0f0a8e5f607c"></a>
## predicate

`struct_field` · `datafusion_expr::logical_plan::plan::Filter::predicate` · datafusion-expr 55.1.0

```rust
predicate: Expr
```

Source: `src/logical_plan/plan.rs:2639`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The predicate expression, which must have Boolean type.

<a id="op-5b67191dbdb495018daf431e"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::Filter::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(predicate: Expr, input: Arc<LogicalPlan>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Filter", "path": "Filter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2644, 1], "end": [2762, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:2662`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new filter operator.

Notes: as Aliases have no effect on the output of a filter operator,
they are removed from the predicate expression.
