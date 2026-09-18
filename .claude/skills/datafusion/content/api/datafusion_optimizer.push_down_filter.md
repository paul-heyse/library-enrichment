# `datafusion_optimizer::push_down_filter`

Crate `datafusion-optimizer` · 3 public items · structured records in [`model/datafusion_optimizer.push_down_filter.json`](../model/datafusion_optimizer.push_down_filter.json)

## make_filter

`function` · `datafusion_optimizer::push_down_filter::make_filter`

> **Deprecated** — deprecated

```rust
fn make_filter(predicate: datafusion_expr::Expr, input: std::sync::Arc<datafusion_expr::logical_plan::LogicalPlan>) -> datafusion_common::Result<datafusion_expr::logical_plan::LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.push_down_filter.make_filter.md).


Creates a new LogicalPlan::Filter node.

Deprecated: use [`Filter::try_new`] directly.

---

## replace_cols_by_name

`function` · `datafusion_optimizer::push_down_filter::replace_cols_by_name`

```rust
fn replace_cols_by_name(e: datafusion_expr::Expr, replace_map: &std::collections::HashMap<String, impl AsRef<datafusion_expr::Expr>>) -> datafusion_common::Result<datafusion_expr::Expr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.push_down_filter.replace_cols_by_name.md).


replaces columns by its name on the projection.

---

## PushDownFilter

`struct` · `datafusion_optimizer::push_down_filter::PushDownFilter`

```rust
struct PushDownFilter
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

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.push_down_filter.PushDownFilter.md).


Optimizer rule for pushing (moving) filter expressions down in a plan so
they are applied as early as possible.

# Introduction

The goal of this rule is to improve query performance by eliminating
redundant work.

For example, given a plan that sorts all values where `a > 10`:

```text
 Filter (a > 10)
   Sort (a, b)
```

A better plan is to filter the data *before* the Sort, which sorts fewer
rows and therefore does less work overall:

```text
 Sort (a, b)
   Filter (a > 10)  <-- Filter is moved before the sort
```

However it is not always possible to push filters down. For example, given a
plan that finds the top 3 values and then keeps only those that are greater
than 10, if the filter is pushed below the limit it would produce a
different result.

```text
 Filter (a > 10)   <-- cannot move this Filter before the limit
   Limit (fetch=3)
     Sort (a, b)
```


More formally, a filter-commutative operation is an operation `op` that
satisfies `filter(op(data)) = op(filter(data))`.

The filter-commutative property is plan and column-specific. A filter on `a`
can be pushed through a `Aggregate(group_by = [a], agg=[sum(b)])`. However, a
filter on `sum(b)` cannot be pushed through the same aggregate.

# Handling Conjunctions

It is possible to only push down **part** of a filter expression if it is
connected with `AND`s (more formally if it is a "conjunction").

For example, given the following plan:

```text
Filter(a > 10 AND sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b)])
```

The `a > 10` is commutative with the `Aggregate` but `sum(b) < 5` is not.
Therefore it is possible to only push down part of the expression, resulting in:

```text
Filter(sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b)])
    Filter(a > 10)
```

# Handling Column Aliases

This optimizer must sometimes handle rewriting filter expressions when they are
pushed. For example, consider a projection that aliases `a+1` to `"b"`:

```text
Filter (b > 10)
    Projection: [a+1 AS "b"]  <-- changes the name of `a+1` to `b`
```

To push this filter below the `Projection`, all references to `b` must be
rewritten to `a+1`:

```text
Projection: [a+1 AS "b"]
    Filter: (a+1 > 10)  <--- changed from b to a+1
```
# Implementation Notes

This implementation performs a single pass through the plan, "pushing" down
filters. When it passes through a filter, it stores that filter, and when it
reaches a plan node that does not commute with that filter, it adds the
filter to that place. When it passes through a projection, it re-writes the
filter's expression taking into account that projection.

---
