# `datafusion_optimizer::analyzer::function_rewrite`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.analyzer.function_rewrite.json`](../model/datafusion_optimizer.analyzer.function_rewrite.json)

## ApplyFunctionRewrites

`struct` · `datafusion_optimizer::analyzer::function_rewrite::ApplyFunctionRewrites`

```rust
struct ApplyFunctionRewrites
```

**Implements**: `datafusion_optimizer::analyzer::AnalyzerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new(function_rewrites: Vec<Arc<dyn FunctionRewrite + Send + Sync>>) -> Self
```

**via `datafusion_optimizer::analyzer::AnalyzerRule`**

```rust
fn analyze(&self, plan: LogicalPlan, options: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.analyzer.function_rewrite.ApplyFunctionRewrites.md).


Analyzer rule that invokes [`FunctionRewrite`]s on expressions

---
