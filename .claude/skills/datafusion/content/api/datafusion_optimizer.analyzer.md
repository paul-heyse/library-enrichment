# `datafusion_optimizer::analyzer`

Crate `datafusion-optimizer` · 2 public items · structured records in [`model/datafusion_optimizer.analyzer.json`](../model/datafusion_optimizer.analyzer.json)

## Analyzer

`struct` · `datafusion_optimizer::analyzer::Analyzer`

Also reachable as `datafusion::optimizer::Analyzer`, `datafusion_optimizer::Analyzer`

```rust
struct Analyzer
```

**Fields**: `function_rewrites`, `rules`

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn add_function_rewrite(&mut self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>)
fn execute_and_check<F>(&self, plan: LogicalPlan, config: &ConfigOptions, observer: F) -> Result<LogicalPlan> where F: FnMut(&LogicalPlan, &dyn AnalyzerRule)
fn function_rewrites(&self) -> &[Arc<dyn FunctionRewrite + Send + Sync>]
fn new() -> Self
fn with_rules(rules: Vec<Arc<dyn AnalyzerRule + Send + Sync>>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.analyzer.Analyzer.md).


Rule-based Analyzer.

Applies [`FunctionRewrite`]s and [`AnalyzerRule`]s to transform a
[`LogicalPlan`] in preparation for execution.

For example, the `Analyzer` applies type coercion to ensure the types of
operands match the types required by functions.

---

## AnalyzerRule

`trait` · `datafusion_optimizer::analyzer::AnalyzerRule`

Also reachable as `datafusion::optimizer::AnalyzerRule`, `datafusion_optimizer::AnalyzerRule`

```rust
trait AnalyzerRule: Debug
```

**Implementors** (3)

- `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites`
- `datafusion_optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction`
- `datafusion_optimizer::analyzer::type_coercion::TypeCoercion`

**Methods** (2)

```rust
fn analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.analyzer.AnalyzerRule.md).


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

---
