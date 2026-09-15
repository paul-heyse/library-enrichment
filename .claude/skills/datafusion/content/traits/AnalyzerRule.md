# AnalyzerRule

`datafusion_optimizer::analyzer::AnalyzerRule`

```rust
trait AnalyzerRule: Debug
```

Also reachable as `datafusion::optimizer::AnalyzerRule`, `datafusion_optimizer::AnalyzerRule`

Prose: [`api/datafusion_optimizer.analyzer.md`](../api/datafusion_optimizer.analyzer.md#analyzerrule) · records: [`model/datafusion_optimizer.analyzer.json`](../model/datafusion_optimizer.analyzer.json)

## Required

Every implementation must supply these.

```rust
fn analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

## Implementors (3)

Read one before writing your own.

- `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites`
- `datafusion_optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction`
- `datafusion_optimizer::analyzer::type_coercion::TypeCoercion`

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/query_planning/analyzer_rule.rs`](../corpus/examples/query_planning/analyzer_rule.rs)

## Documentation

[`AnalyzerRule`]s transform [`LogicalPlan`]s in some way to make
the plan valid prior to the rest of the DataFusion optimization process.

`AnalyzerRule`s are different than an [`OptimizerRule`](crate::OptimizerRule)s
which must preserve the semantics of the `LogicalPlan`, while computing
results in a more optimal way.

For example, an `AnalyzerRule` may resolve [`Expr`](datafusion_expr::Expr)s into more specific
forms such as a subquery reference, or do type coercion to ensure the types
of operands are correct.

Use [`SessionState::add_analyzer_rule`] to register additional
`AnalyzerRule`s.

[`SessionState::add_analyzer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_analyzer_rule
