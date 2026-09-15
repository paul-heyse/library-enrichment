# `datafusion_optimizer::analyzer::resolve_grouping_function`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.analyzer.resolve_grouping_function.json`](../model/datafusion_optimizer.analyzer.resolve_grouping_function.json)

## ResolveGroupingFunction

`struct` · `datafusion_optimizer::analyzer::resolve_grouping_function::ResolveGroupingFunction`

```rust
struct ResolveGroupingFunction
```

**Implements**: `datafusion_optimizer::analyzer::AnalyzerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::analyzer::AnalyzerRule`**

```rust
fn analyze(&self, plan: LogicalPlan, _: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

Replaces grouping aggregation function with value derived from internal grouping id

---
