# `datafusion_optimizer::single_distinct_to_groupby`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.single_distinct_to_groupby.json`](../model/datafusion_optimizer.single_distinct_to_groupby.json)

## SingleDistinctToGroupBy

`struct` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy`

```rust
struct SingleDistinctToGroupBy
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn supports_rewrite(&self) -> bool
```

single distinct to group by optimizer rule
 ```text
   Before:
   SELECT a, count(DISTINCT b), sum(c)
   FROM t
   GROUP BY a

   After:
   SELECT a, count(alias1), sum(alias2)
   FROM (
     SELECT a, b as alias1, sum(c) as alias2
     FROM t
     GROUP BY a, b
   )
   GROUP BY a
 ```

---
