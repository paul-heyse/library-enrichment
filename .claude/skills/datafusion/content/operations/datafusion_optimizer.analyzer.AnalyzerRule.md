# `datafusion_optimizer::analyzer::AnalyzerRule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.analyzer.AnalyzerRule.json).

<a id="op-cff859d3fc5687654af4ac1d"></a>
## AnalyzerRule

`trait` · `datafusion_optimizer::analyzer::AnalyzerRule` · datafusion-optimizer 55.1.0

```rust
trait AnalyzerRule: Debug
```

Source: `src/analyzer/mod.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

[`AnalyzerRule`](../operations/datafusion_optimizer.analyzer.AnalyzerRule.md#op-cff859d3fc5687654af4ac1d)s transform [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s in some way to make
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

<a id="op-f4d3ee9c95ee852590b1adbc"></a>
## analyze

`function` · `datafusion_optimizer::analyzer::AnalyzerRule::analyze` · datafusion-optimizer 55.1.0

```rust
fn analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>
```

Source: `src/analyzer/mod.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Rewrite `plan`

<a id="op-39d0cba2eed5b93811a42f62"></a>
## name

`function` · `datafusion_optimizer::analyzer::AnalyzerRule::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/analyzer/mod.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

A human readable name for this analyzer rule
