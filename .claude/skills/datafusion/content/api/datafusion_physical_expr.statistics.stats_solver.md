# `datafusion_physical_expr::statistics::stats_solver`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.statistics.stats_solver.json`](../model/datafusion_physical_expr.statistics.stats_solver.json)

## ExprStatisticsGraph

`struct` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraph`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

```rust
struct ExprStatisticsGraph
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn assign_statistics(&mut self, assignments: &[(usize, Distribution)])
fn evaluate_statistics(&mut self) -> Result<&Distribution>
fn propagate_statistics(&mut self, given_stats: Distribution) -> Result<PropagationResult>
fn try_new(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
```

This object implements a directed acyclic expression graph (DAEG) that
is used to compute statistics/distributions for expressions hierarchically.

---

## ExprStatisticsGraphNode

`struct` · `datafusion_physical_expr::statistics::stats_solver::ExprStatisticsGraphNode`

> **Deprecated** — since 54.0.0: Part of the unused Statistics V2 framework; see https://github.com/apache/datafusion/pull/22071

```rust
struct ExprStatisticsGraphNode
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn distribution(&self) -> &Distribution
fn make_node(node: &ExprTreeNode<NodeIndex>, schema: &Schema) -> Result<Self>
```

This is a node in the DAEG; it encapsulates a reference to the actual
[`PhysicalExpr`] as well as its statistics/distribution.

---
