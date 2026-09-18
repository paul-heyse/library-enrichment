# `datafusion_optimizer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.json).

<a id="op-d31764067ba192ffdae27b04"></a>
## datafusion_optimizer

`module` · `datafusion_optimizer` · datafusion-optimizer 55.1.0

```rust
mod datafusion_optimizer
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

# DataFusion Optimizer

Contains rules for rewriting [`LogicalPlan`]s

1. [`Analyzer`](../operations/datafusion_optimizer.analyzer.Analyzer.md#op-f305ce03fd0d8b4faab403e4) applies [`AnalyzerRule`](../operations/datafusion_optimizer.analyzer.AnalyzerRule.md#op-cff859d3fc5687654af4ac1d)s to transform `LogicalPlan`s
   to make the plan valid prior to the rest of the DataFusion optimization
   process (for example, [`TypeCoercion`]).

2. [`Optimizer`](../operations/datafusion_optimizer.optimizer.Optimizer.md#op-111f177db94ba9d05d0d598a) applies [`OptimizerRule`](../operations/datafusion_optimizer.optimizer.OptimizerRule.md#op-16265e807b19887d2b41f187)s to transform `LogicalPlan`s
   into equivalent, but more efficient plans.

[`LogicalPlan`]: datafusion_expr::LogicalPlan
[`TypeCoercion`]: analyzer::type_coercion::TypeCoercion
