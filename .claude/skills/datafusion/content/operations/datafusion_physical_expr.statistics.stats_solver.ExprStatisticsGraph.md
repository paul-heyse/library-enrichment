# `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.statistics.stats_solver.ExprStatisticsGraph.json).

<a id="op-b193a14d1c01ca6b74a757c9"></a>
## ExprStatisticsGraph

`struct` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph` · datafusion-physical-expr 55.1.0

```rust
struct ExprStatisticsGraph
```

Source: `src/statistics/stats_solver.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This object implements a directed acyclic expression graph (DAEG) that
is used to compute statistics/distributions for expressions hierarchically.

<a id="op-094af02d0c63f2584f34a13e"></a>
## assign_statistics

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph::assign_statistics` · datafusion-physical-expr 55.1.0

```rust
fn assign_statistics(&mut self, assignments: &[(usize, Distribution)])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph", "path": "ExprStatisticsGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [217, 2], "filename": "src/statistics/stats_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics/stats_solver.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function assigns given distributions to expressions in the DAEG.
The argument `assignments` associates indices of sought expressions
with their corresponding new distributions.

<a id="op-826e7e609bfe9d75c128195d"></a>
## clone

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ExprStatisticsGraph
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph", "path": "ExprStatisticsGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "src/statistics/stats_solver.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics/stats_solver.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-731e4995b8be3221cfa34935"></a>
## evaluate_statistics

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph::evaluate_statistics` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_statistics(&mut self) -> Result<&Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph", "path": "ExprStatisticsGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [217, 2], "filename": "src/statistics/stats_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics/stats_solver.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Computes statistics/distributions for an expression via a bottom-up
traversal.

<a id="op-20482f0ea18d7cc3dbabe545"></a>
## fmt

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph", "path": "ExprStatisticsGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 22], "filename": "src/statistics/stats_solver.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics/stats_solver.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b6a004f4bb85402bcf92b9a"></a>
## propagate_statistics

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph::propagate_statistics` · datafusion-physical-expr 55.1.0

```rust
fn propagate_statistics(&mut self, given_stats: Distribution) -> Result<PropagationResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph", "path": "ExprStatisticsGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [217, 2], "filename": "src/statistics/stats_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics/stats_solver.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Runs a propagation mechanism in a top-down manner to update statistics
of leaf nodes.

<a id="op-3f5f8f30b1ff4e7ee35148f8"></a>
## try_new

`function` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph", "path": "ExprStatisticsGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [217, 2], "filename": "src/statistics/stats_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics/stats_solver.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
