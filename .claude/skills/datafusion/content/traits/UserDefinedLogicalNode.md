# UserDefinedLogicalNode

`datafusion_expr::logical_plan::extension::UserDefinedLogicalNode`

```rust
trait UserDefinedLogicalNode: fmt::Debug + Send + Sync
```

Also reachable as `datafusion::logical_expr::UserDefinedLogicalNode`, `datafusion_expr::UserDefinedLogicalNode`, `datafusion_expr::logical_plan::UserDefinedLogicalNode`

Prose: [`api/datafusion_expr.logical_plan.extension.md`](../api/datafusion_expr.logical_plan.extension.md#userdefinedlogicalnode) · records: [`model/datafusion_expr.logical_plan.extension.json`](../model/datafusion_expr.logical_plan.extension.json)

## Required

Every implementation must supply these.

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
fn schema(&self) -> &DFSchemaRef
fn with_exprs_and_inputs(&self, exprs: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<Arc<dyn UserDefinedLogicalNode>>
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn necessary_children_exprs(&self, _output_columns: &[usize]) -> Option<Vec<Vec<usize>>>
fn prevent_predicate_push_down_columns(&self) -> HashSet<String>
fn supports_limit_pushdown(&self) -> bool
```

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/relation_planner/match_recognize.rs`](../corpus/examples/relation_planner/match_recognize.rs)

## Documentation

This defines the interface for [`LogicalPlan`] nodes that can be
used to extend DataFusion with custom relational operators.

The [`UserDefinedLogicalNodeCore`] trait is *the recommended way to implement*
this trait and avoids having implementing some required boiler plate code.
