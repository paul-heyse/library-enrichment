# UserDefinedLogicalNodeCore

`datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore`

```rust
trait UserDefinedLogicalNodeCore: fmt::Debug + Eq + PartialOrd + Hash + Sized + Send + Sync + 'static
```

Also reachable as `datafusion::logical_expr::UserDefinedLogicalNodeCore`, `datafusion_expr::UserDefinedLogicalNodeCore`, `datafusion_expr::logical_plan::UserDefinedLogicalNodeCore`

Prose: [`api/datafusion_expr.logical_plan.extension.md`](../api/datafusion_expr.logical_plan.extension.md#userdefinedlogicalnodecore) · records: [`model/datafusion_expr.logical_plan.extension.json`](../model/datafusion_expr.logical_plan.extension.json)

## Required

Every implementation must supply these.

```rust
fn expressions(&self) -> Vec<Expr>
fn fmt_for_explain(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn inputs(&self) -> Vec<&LogicalPlan>
fn name(&self) -> &str
fn schema(&self) -> &DFSchemaRef
fn with_exprs_and_inputs(&self, exprs: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<Self>
```

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn check_invariants(&self, _check: InvariantLevel) -> Result<()>
fn necessary_children_exprs(&self, _output_columns: &[usize]) -> Option<Vec<Vec<usize>>>
fn prevent_predicate_push_down_columns(&self) -> HashSet<String>
fn supports_limit_pushdown(&self) -> bool
```

## Demonstrated by 3 upstream example(s)

- [`corpus/examples/dataframe/cache_factory.rs`](../corpus/examples/dataframe/cache_factory.rs)
- [`corpus/examples/query_planning/plan_to_sql.rs`](../corpus/examples/query_planning/plan_to_sql.rs)
- [`corpus/examples/relation_planner/table_sample.rs`](../corpus/examples/relation_planner/table_sample.rs)

## Documentation

This trait facilitates implementation of the [`UserDefinedLogicalNode`].

See the example in
[user_defined_plan.rs](https://github.com/apache/datafusion/blob/main/datafusion/core/tests/user_defined/user_defined_plan.rs)
file for an example of how to use this extension API.
