# `datafusion_physical_expr::utils`

Crate `datafusion-physical-expr` · 11 public items · structured records in [`model/datafusion_physical_expr.utils.json`](../model/datafusion_physical_expr.utils.json)

## build_dag

`function` · `datafusion_physical_expr::utils::build_dag`

```rust
fn build_dag<T, F>(expr: std::sync::Arc<dyn PhysicalExpr>, constructor: &F) -> datafusion_common::Result<(petgraph::graph::NodeIndex, petgraph::stable_graph::StableGraph<T, usize>)> where F: Fn(&ExprTreeNode<petgraph::graph::NodeIndex>) -> datafusion_common::Result<T>
```

---

## collect_columns

`function` · `datafusion_physical_expr::utils::collect_columns`

```rust
fn collect_columns(expr: &std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::HashSet<expressions::Column>
```

Recursively extract referenced [`Column`]s within a [`PhysicalExpr`].

---

## conjunction

`function` · `datafusion_physical_expr::utils::conjunction`

Also reachable as `datafusion::physical_expr::conjunction`, `datafusion_physical_expr::conjunction`

```rust
fn conjunction(predicates: impl IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>) -> std::sync::Arc<dyn PhysicalExpr>
```

Create a conjunction of the given predicates.
If the input is empty, return a literal true.
If the input contains a single predicate, return the predicate.
Otherwise, return a conjunction of the predicates (e.g. `a AND b AND c`).

---

## conjunction_opt

`function` · `datafusion_physical_expr::utils::conjunction_opt`

Also reachable as `datafusion::physical_expr::conjunction_opt`, `datafusion_physical_expr::conjunction_opt`

```rust
fn conjunction_opt(predicates: impl IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>) -> Option<std::sync::Arc<dyn PhysicalExpr>>
```

Create a conjunction of the given predicates.
If the input is empty or the return None.
If the input contains a single predicate, return Some(predicate).
Otherwise, return a Some(..) of a conjunction of the predicates (e.g. `Some(a AND b AND c)`).

---

## convert_to_expr

`function` · `datafusion_physical_expr::utils::convert_to_expr`

```rust
fn convert_to_expr<T: Borrow<PhysicalSortExpr>>(sequence: impl IntoIterator<Item = T>) -> Vec<std::sync::Arc<dyn PhysicalExpr>>
```

This function returns all `Arc<dyn PhysicalExpr>`s inside the given
`PhysicalSortExpr` sequence.

---

## get_indices_of_exprs_strict

`function` · `datafusion_physical_expr::utils::get_indices_of_exprs_strict`

```rust
fn get_indices_of_exprs_strict<T: Borrow<std::sync::Arc<dyn PhysicalExpr>>>(targets: impl IntoIterator<Item = T>, items: &[std::sync::Arc<dyn PhysicalExpr>]) -> Vec<usize>
```

This function finds the indices of `targets` within `items` using strict
equality.

---

## map_columns_before_projection

`function` · `datafusion_physical_expr::utils::map_columns_before_projection`

```rust
fn map_columns_before_projection(parent_required: &[std::sync::Arc<dyn PhysicalExpr>], proj_exprs: &[(std::sync::Arc<dyn PhysicalExpr>, String)]) -> Vec<std::sync::Arc<dyn PhysicalExpr>>
```

This function maps back requirement after ProjectionExec
to the Executor for its input.

---

## reassign_expr_columns

`function` · `datafusion_physical_expr::utils::reassign_expr_columns`

```rust
fn reassign_expr_columns(expr: std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Re-assign indices of [`Column`]s within the given [`PhysicalExpr`] according to
the provided [`Schema`].

This can be useful when attempting to map an expression onto a different schema.

# Errors

This function will return an error if any column in the expression cannot be found
in the provided schema.

---

## split_conjunction

`function` · `datafusion_physical_expr::utils::split_conjunction`

Also reachable as `datafusion::physical_expr::split_conjunction`, `datafusion_physical_expr::split_conjunction`

```rust
fn split_conjunction(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> Vec<&std::sync::Arc<dyn PhysicalExpr>>
```

Assume the predicate is in the form of CNF, split the predicate to a Vec of PhysicalExprs.

For example, split "a1 = a2 AND b1 <= b2 AND c1 != c2" into ["a1 = a2", "b1 <= b2", "c1 != c2"]

---

## split_disjunction

`function` · `datafusion_physical_expr::utils::split_disjunction`

```rust
fn split_disjunction(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> Vec<&std::sync::Arc<dyn PhysicalExpr>>
```

Assume the predicate is in the form of DNF, split the predicate to a Vec of PhysicalExprs.

For example, split "a1 = a2 OR b1 <= b2 OR c1 != c2" into ["a1 = a2", "b1 <= b2", "c1 != c2"]

---

## ExprTreeNode

`type_alias` · `datafusion_physical_expr::utils::ExprTreeNode`

```rust
type ExprTreeNode<T> = tree_node::ExprContext<Option<T>>
```

---
