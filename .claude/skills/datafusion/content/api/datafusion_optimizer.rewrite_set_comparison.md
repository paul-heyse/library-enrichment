# `datafusion_optimizer::rewrite_set_comparison`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.rewrite_set_comparison.json`](../model/datafusion_optimizer.rewrite_set_comparison.json)

## RewriteSetComparison

`struct` · `datafusion_optimizer::rewrite_set_comparison::RewriteSetComparison`

```rust
struct RewriteSetComparison
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Rewrite `SetComparison` expressions to scalar subqueries that return the
correct boolean value (including SQL NULL semantics). After this rule
runs, later rules such as `ScalarSubqueryToJoin` can decorrelate and
remove the remaining subquery.

---
