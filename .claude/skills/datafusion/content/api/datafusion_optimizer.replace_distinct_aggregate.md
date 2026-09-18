# `datafusion_optimizer::replace_distinct_aggregate`

Crate `datafusion-optimizer` · 1 public items · structured records in [`model/datafusion_optimizer.replace_distinct_aggregate.json`](../model/datafusion_optimizer.replace_distinct_aggregate.json)

## ReplaceDistinctWithAggregate

`struct` · `datafusion_optimizer::replace_distinct_aggregate::ReplaceDistinctWithAggregate`

```rust
struct ReplaceDistinctWithAggregate
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
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.replace_distinct_aggregate.ReplaceDistinctWithAggregate.md).


Optimizer that replaces logical [[Distinct]] with a logical [[Aggregate]]

```text
SELECT DISTINCT a, b FROM tab
```

Into
```text
SELECT a, b FROM tab GROUP BY a, b
```

On the other hand, for a `DISTINCT ON` query the replacement is
a bit more involved and effectively converts
```text
SELECT DISTINCT ON (a) b FROM tab ORDER BY a DESC, c
```

into
```text
SELECT b FROM (
    SELECT a, FIRST_VALUE(b ORDER BY a DESC, c) AS b
    FROM tab
    GROUP BY a
)
ORDER BY a DESC
```

In case there are no columns, the [[Distinct]] is replaced by a [[Limit]]

```text
SELECT DISTINCT * FROM empty_table
```

Into
```text
SELECT * FROM empty_table LIMIT 1
```

---
