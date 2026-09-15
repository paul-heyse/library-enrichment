# `datafusion_optimizer::eliminate_outer_join`

Crate `datafusion-optimizer` · 2 public items · structured records in [`model/datafusion_optimizer.eliminate_outer_join.json`](../model/datafusion_optimizer.eliminate_outer_join.json)

## eliminate_outer

`function` · `datafusion_optimizer::eliminate_outer_join::eliminate_outer`

```rust
fn eliminate_outer(join_type: datafusion_expr::logical_plan::JoinType, left_non_nullable: bool, right_non_nullable: bool) -> datafusion_expr::logical_plan::JoinType
```

---

## EliminateOuterJoin

`struct` · `datafusion_optimizer::eliminate_outer_join::EliminateOuterJoin`

```rust
struct EliminateOuterJoin
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
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
fn supports_rewrite(&self) -> bool
```

Attempt to simplify outer joins when filters make their null-padded
rows impossible to observe.

Outer joins are generally more expensive than inner joins and can block
predicate pushdown and other optimizations. When a filter above an outer
join removes every row the join would add for unmatched input rows, the
join can be changed to a cheaper join type.

For example:

```sql
SELECT ...
FROM a LEFT JOIN b ON ...
WHERE b.xx = 100
```

For unmatched rows from `a`, the LEFT JOIN would produce a row with
`b.xx` set to NULL. The predicate `b.xx = 100` does not pass for those
rows, so the query does not need the LEFT JOIN's null-padded output and
the join can be rewritten as an inner join.

The same reasoning can also simplify FULL joins to LEFT, RIGHT, or INNER
joins when filters remove the rows padded on one or both sides.

This rule looks for a filter above an outer join:

```text
Filter(predicate)
  Join(LEFT/RIGHT/FULL)
```

It also handles plan shapes where projection pruning has inserted one or
more Projection nodes between the filter and join:

```text
Filter(predicate over projection output)
  Projection(...)
    ...
      Join(LEFT/RIGHT/FULL)
```

In the projection case, the rule rewrites a copy of the predicate through
each Projection so it can analyze the predicate against the Join inputs.
The original filter predicate and Projection nodes are preserved when the
plan is rebuilt.

---
