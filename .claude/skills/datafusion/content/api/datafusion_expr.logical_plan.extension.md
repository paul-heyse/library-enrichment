# `datafusion_expr::logical_plan::extension`

Crate `datafusion-expr` · 2 public items · structured records in [`model/datafusion_expr.logical_plan.extension.json`](../model/datafusion_expr.logical_plan.extension.json)

## UserDefinedLogicalNode

`trait` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode`

Also reachable as `datafusion::logical_expr::UserDefinedLogicalNode`, `datafusion_expr::UserDefinedLogicalNode`, `datafusion_expr::logical_plan::UserDefinedLogicalNode`

```rust
trait UserDefinedLogicalNode: fmt::Debug + Send + Sync
```

**Methods** (14)

```rust
fn as_any(&self) -> &dyn Any
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
fn dyn_eq(&self, other: &dyn UserDefinedLogicalNode) -> bool
fn dyn_hash(&self, state: &mut dyn Hasher)
fn dyn_ord(&self, other: &dyn UserDefinedLogicalNode) -> Option<Ordering>
fn expressions(&self) -> Vec<Expr>
fn fmt_for_explain(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn inputs(&self) -> Vec<&LogicalPlan>
fn name(&self) -> &str
fn necessary_children_exprs(&self, _output_columns: &[usize]) -> Option<Vec<Vec<usize>>>
fn prevent_predicate_push_down_columns(&self) -> HashSet<String>
fn schema(&self) -> &DFSchemaRef
fn supports_limit_pushdown(&self) -> bool
fn with_exprs_and_inputs(&self, exprs: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<Arc<dyn UserDefinedLogicalNode>>
```

This defines the interface for [`LogicalPlan`] nodes that can be
used to extend DataFusion with custom relational operators.

The [`UserDefinedLogicalNodeCore`] trait is *the recommended way to implement*
this trait and avoids having implementing some required boiler plate code.

---

## UserDefinedLogicalNodeCore

`trait` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore`

Also reachable as `datafusion::logical_expr::UserDefinedLogicalNodeCore`, `datafusion_expr::UserDefinedLogicalNodeCore`, `datafusion_expr::logical_plan::UserDefinedLogicalNodeCore`

```rust
trait UserDefinedLogicalNodeCore: fmt::Debug + Eq + PartialOrd + Hash + Sized + Send + Sync + 'static
```

**Methods** (10)

```rust
fn check_invariants(&self, _check: InvariantLevel) -> Result<()>
fn expressions(&self) -> Vec<Expr>
fn fmt_for_explain(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn inputs(&self) -> Vec<&LogicalPlan>
fn name(&self) -> &str
fn necessary_children_exprs(&self, _output_columns: &[usize]) -> Option<Vec<Vec<usize>>>
fn prevent_predicate_push_down_columns(&self) -> HashSet<String>
fn schema(&self) -> &DFSchemaRef
fn supports_limit_pushdown(&self) -> bool
fn with_exprs_and_inputs(&self, exprs: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<Self>
```

This trait facilitates implementation of the [`UserDefinedLogicalNode`].

See the example in
[user_defined_plan.rs](https://github.com/apache/datafusion/blob/main/datafusion/core/tests/user_defined/user_defined_plan.rs)
file for an example of how to use this extension API.

---
