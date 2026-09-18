# `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.analyzer.type_coercion.TypeCoercionRewriter.json).

<a id="op-c7c84381bee05c1cf75f1a87"></a>
## TypeCoercionRewriter

`struct` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter` · datafusion-optimizer 55.1.0

```rust
struct TypeCoercionRewriter<'a>
```

Source: `src/analyzer/type_coercion.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Rewrite expressions to apply type coercion.

<a id="op-4d168977f9176992928d568b"></a>
## Node

`assoc_type` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter::Node` · datafusion-optimizer 55.1.0

```rust
Node
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter", "path": "TypeCoercionRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [547, 1], "end": [944, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/analyzer/type_coercion.rs:548`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94fbd7e0092ee58eefa00f14"></a>
## coerce_join

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter::coerce_join` · datafusion-optimizer 55.1.0

```rust
fn coerce_join(&mut self, join: Join) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter", "path": "TypeCoercionRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [545, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/type_coercion.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Coerce join equality expressions and join filter

Joins must be treated specially as their equality expressions are stored
as a parallel list of left and right expressions, rather than a single
equality expression

For example, on_exprs like `t1.a = t2.b AND t1.x = t2.y` will be stored
as a list of `(t1.a, t2.b), (t1.x, t2.y)`

<a id="op-f5e3018eb6a70a37476dfe32"></a>
## coerce_plan

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter::coerce_plan` · datafusion-optimizer 55.1.0

```rust
fn coerce_plan(&mut self, plan: LogicalPlan) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter", "path": "TypeCoercionRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [545, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/type_coercion.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Coerce the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

Refer to [`TypeCoercionRewriter::coerce_join`](../operations/datafusion_optimizer.analyzer.type_coercion.TypeCoercionRewriter.md#op-94fbd7e0092ee58eefa00f14) and [`TypeCoercionRewriter::coerce_union`](../operations/datafusion_optimizer.analyzer.type_coercion.TypeCoercionRewriter.md#op-a9955fcf98ebcbf9e6a13b46)
for type-coercion approach.

<a id="op-a9955fcf98ebcbf9e6a13b46"></a>
## coerce_union

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter::coerce_union` · datafusion-optimizer 55.1.0

```rust
fn coerce_union(union_plan: Union) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter", "path": "TypeCoercionRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [545, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/type_coercion.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Coerce the union’s inputs to a common schema compatible with all inputs.
This occurs after wildcard expansion and the coercion of the input expressions.

<a id="op-763cc7540d77c41a816243f5"></a>
## f_up

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter::f_up` · datafusion-optimizer 55.1.0

```rust
fn f_up(&mut self, expr: Expr) -> Result<Transformed<Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter", "path": "TypeCoercionRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [547, 1], "end": [944, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/analyzer/type_coercion.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d7dac90c38774720705c4f3"></a>
## new

`function` · `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter::new` · datafusion-optimizer 55.1.0

```rust
fn new(schema: &'a DFSchema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter", "path": "TypeCoercionRewriter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [180, 1], "end": [545, 2], "filename": "src/analyzer/type_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/analyzer/type_coercion.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a new [`TypeCoercionRewriter`](../operations/datafusion_optimizer.analyzer.type_coercion.TypeCoercionRewriter.md#op-c7c84381bee05c1cf75f1a87) with a provided schema
representing both the inputs and output of the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) node.
