# `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.statistics.stats_solver.ExprStatisticsGraphNode.json).

<a id="op-49d54d308df51e1fcb9a9d14"></a>
## ExprStatisticsGraphNode

`struct` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode` · datafusion-physical-expr 55.1.0

```rust
struct ExprStatisticsGraphNode
```

Source: `src/statistics/stats_solver.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This is a node in the DAEG; it encapsulates a reference to the actual
[`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) as well as its statistics/distribution.

<a id="op-bde92c2ffab341332b6ad373"></a>
## clone

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ExprStatisticsGraphNode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode", "path": "ExprStatisticsGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 10], "end": [61, 15], "filename": "src/statistics/stats_solver.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics/stats_solver.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab395b5d992c843d45238e82"></a>
## distribution

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode::distribution` · datafusion-physical-expr 55.1.0

```rust
fn distribution(&self) -> &Distribution
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode", "path": "ExprStatisticsGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [118, 2], "filename": "src/statistics/stats_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics/stats_solver.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the [`Distribution`](../operations/datafusion_expr_common.statistics.Distribution.md#op-01128278dc758957f40620ba) object representing the statistics of the
expression.

<a id="op-e6e8a754df7ac9b64d44cffa"></a>
## fmt

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode", "path": "ExprStatisticsGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 17], "end": [61, 22], "filename": "src/statistics/stats_solver.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics/stats_solver.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52af69329fbeca71eacf74f4"></a>
## make_node

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode::make_node` · datafusion-physical-expr 55.1.0

```rust
fn make_node(node: &ExprTreeNode<NodeIndex>, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode", "path": "ExprStatisticsGraphNode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [118, 2], "filename": "src/statistics/stats_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics/stats_solver.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function creates a DAEG node from DataFusion's [`ExprTreeNode`](../operations/datafusion_physical_expr.utils.ExprTreeNode.md#op-f3771825a1a9d4c62ed84d22)
object. Literals are created with `Uniform` distributions with a
definite, singleton interval. Expressions with a `Boolean` data type
result in a`Bernoulli` distribution with an unknown success probability.
Any other expression starts with an `Unknown` distribution with an
indefinite range (i.e. `[-∞, ∞]`).
