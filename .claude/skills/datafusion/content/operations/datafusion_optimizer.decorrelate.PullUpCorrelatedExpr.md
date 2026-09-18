# `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.decorrelate.PullUpCorrelatedExpr.json).

<a id="op-4924c5f66c889bdc72cefe02"></a>
## PullUpCorrelatedExpr

`struct` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr` · datafusion-optimizer 55.1.0

```rust
struct PullUpCorrelatedExpr
```

Source: `src/decorrelate.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

This struct rewrite the sub query plan by pull up the correlated
expressions(contains outer reference columns) from the inner subquery's
'Filter'. It adds the inner reference columns to the 'Projection' or
'Aggregate' of the subquery if they are missing, so that they can be
evaluated by the parent operator as the join condition.

<a id="op-a8faf7e7da73c1932f894372"></a>
## Node

`assoc_type` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::Node` · datafusion-optimizer 55.1.0

```rust
Node
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [392, 2], "filename": "src/decorrelate.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/decorrelate.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2d121646d604ff24c062099"></a>
## can_pull_up

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::can_pull_up` · datafusion-optimizer 55.1.0

```rust
can_pull_up: bool
```

Source: `src/decorrelate.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Can the correlated expressions be pulled up. Defaults to **TRUE**

<a id="op-73d5e9e0d7c372bb5e8bccfe"></a>
## collected_count_expr_map

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::collected_count_expr_map` · datafusion-optimizer 55.1.0

```rust
collected_count_expr_map: datafusion_common::HashMap<datafusion_expr::LogicalPlan, ExprResultMap>
```

Source: `src/decorrelate.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

mapping from the plan to its expressions' evaluation result on empty batch

<a id="op-8cebb4a52a08433f17804b8a"></a>
## correlated_subquery_cols_map

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::correlated_subquery_cols_map` · datafusion-optimizer 55.1.0

```rust
correlated_subquery_cols_map: datafusion_common::HashMap<datafusion_expr::LogicalPlan, std::collections::BTreeSet<datafusion_common::Column>>
```

Source: `src/decorrelate.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

mapping from the plan to its holding correlated columns

<a id="op-15a91a0e1838ce718e5fb492"></a>
## default

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [84, 2], "filename": "src/decorrelate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/decorrelate.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9cdde0b15feaa06c76bbc2e"></a>
## exists_sub_query

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::exists_sub_query` · datafusion-optimizer 55.1.0

```rust
exists_sub_query: bool
```

Source: `src/decorrelate.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Is this an Exists(Not Exists) SubQuery. Defaults to **FALSE**

<a id="op-178f5bee3a738051e41cd06d"></a>
## f_down

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::f_down` · datafusion-optimizer 55.1.0

```rust
fn f_down(&mut self, plan: LogicalPlan) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [392, 2], "filename": "src/decorrelate.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/decorrelate.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6719a3a3cfc13acfc1e2526c"></a>
## f_up

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::f_up` · datafusion-optimizer 55.1.0

```rust
fn f_up(&mut self, plan: LogicalPlan) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [392, 2], "filename": "src/decorrelate.rs"}, "trait": {"args": null, "id": "datafusion_common::tree_node::TreeNodeRewriter", "path": "TreeNodeRewriter"}, "trait_path": "datafusion_common::tree_node::TreeNodeRewriter"}`

Source: `src/decorrelate.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42a20a8adabb0899480b1ba7"></a>
## fmt

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/decorrelate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/decorrelate.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8836e725bcee3dc908f668"></a>
## in_predicate_opt

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::in_predicate_opt` · datafusion-optimizer 55.1.0

```rust
in_predicate_opt: Option<datafusion_expr::Expr>
```

Source: `src/decorrelate.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e8eb62911785be4059487d9"></a>
## join_filters

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::join_filters` · datafusion-optimizer 55.1.0

```rust
join_filters: Vec<datafusion_expr::Expr>
```

Source: `src/decorrelate.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a085392bc0caaca7eeb21195"></a>
## need_handle_count_bug

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::need_handle_count_bug` · datafusion-optimizer 55.1.0

```rust
need_handle_count_bug: bool
```

Source: `src/decorrelate.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Do we need to handle [the count bug] during the pull up process.

The "count bug" was described in [Optimization of Nested SQL
Queries Revisited](https://dl.acm.org/doi/pdf/10.1145/38714.38723). This bug is
not specific to the COUNT function, and it can occur with any aggregate function,
such as SUM, AVG, etc. The anomaly arises because aggregates fail to distinguish
between an empty set and null values when optimizing a correlated query as a join.
Here, we use "the count bug" to refer to all such cases.

[the count bug]: https://github.com/apache/datafusion/issues/10553

<a id="op-274be88b011b99aa40b74bc4"></a>
## new

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [121, 2], "filename": "src/decorrelate.rs"}, "trait": null, "trait_path": null}`

Source: `src/decorrelate.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f57804fb1a3c7a829b442f1"></a>
## pull_up_having_expr

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::pull_up_having_expr` · datafusion-optimizer 55.1.0

```rust
pull_up_having_expr: Option<datafusion_expr::Expr>
```

Source: `src/decorrelate.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

pull up having expr, which must be evaluated after the Join

<a id="op-cd8ef74aa4509d480489552b"></a>
## pulled_up_scalar_agg

`struct_field` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::pulled_up_scalar_agg` · datafusion-optimizer 55.1.0

```rust
pulled_up_scalar_agg: bool
```

Source: `src/decorrelate.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

whether we have converted a scalar aggregation into a group aggregation. When unnesting
lateral joins, we need to produce a left outer join in such cases.

<a id="op-d9292d9a6f9fbdf7248063cd"></a>
## with_exists_sub_query

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::with_exists_sub_query` · datafusion-optimizer 55.1.0

```rust
fn with_exists_sub_query(self, exists_sub_query: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [121, 2], "filename": "src/decorrelate.rs"}, "trait": null, "trait_path": null}`

Source: `src/decorrelate.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Set if this is an Exists(Not Exists) SubQuery

<a id="op-e84c65e613ba148ad4501ce3"></a>
## with_in_predicate_opt

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::with_in_predicate_opt` · datafusion-optimizer 55.1.0

```rust
fn with_in_predicate_opt(self, in_predicate_opt: Option<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [121, 2], "filename": "src/decorrelate.rs"}, "trait": null, "trait_path": null}`

Source: `src/decorrelate.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Set the in_predicate_opt

<a id="op-d0143d6127e35d15a3cd4a01"></a>
## with_need_handle_count_bug

`function` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr::with_need_handle_count_bug` · datafusion-optimizer 55.1.0

```rust
fn with_need_handle_count_bug(self, need_handle_count_bug: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::decorrelate::PullUpCorrelatedExpr", "path": "PullUpCorrelatedExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [121, 2], "filename": "src/decorrelate.rs"}, "trait": null, "trait_path": null}`

Source: `src/decorrelate.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Set if we need to handle [the count bug] during the pull up process

[the count bug]: https://github.com/apache/datafusion/issues/10553
