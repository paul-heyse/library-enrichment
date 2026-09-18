# `datafusion_optimizer::extract_leaf_expressions`

Crate `datafusion-optimizer` · 2 public items · structured records in [`model/datafusion_optimizer.extract_leaf_expressions.json`](../model/datafusion_optimizer.extract_leaf_expressions.json)

## ExtractLeafExpressions

`struct` · `datafusion_optimizer::extract_leaf_expressions::ExtractLeafExpressions`

```rust
struct ExtractLeafExpressions
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
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.extract_leaf_expressions.ExtractLeafExpressions.md).


Extracts `MoveTowardsLeafNodes` sub-expressions from non-projection nodes
into **extraction projections** (pass 1 of 2).

This handles Filter, Sort, Limit, Aggregate, and Join nodes. For Projection
nodes, extraction and pushdown are handled by [`PushDownLeafProjections`].

# Key Concepts

**Extraction projection**: a projection inserted *below* a node that
pre-computes a cheap expression and exposes it under an alias
(`__datafusion_extracted_N`). The parent node then references the alias
instead of the original expression.

**Recovery projection**: a projection inserted *above* a node to restore
the original output schema when extraction changes it.
Schema-preserving nodes (Filter, Sort, Limit) gain extra columns from
the extraction projection that bubble up; the recovery projection selects
only the original columns to hide the extras.

# Example

Given a filter with a struct field access:

```text
Filter: user['status'] = 'active'
  TableScan: t [id, user]
```

This rule:
1. Inserts an **extraction projection** below the filter:
2. Adds a **recovery projection** above to hide the extra column:

```text
Projection: id, user                                                        <-- recovery projection
  Filter: __datafusion_extracted_1 = 'active'
    Projection: user['status'] AS __datafusion_extracted_1, id, user         <-- extraction projection
      TableScan: t [id, user]
```

**Important:** The `PushDownFilter` rule is aware of projections created by this rule
and will not push filters through them. It uses `ExpressionPlacement` to detect
`MoveTowardsLeafNodes` expressions and skip filter pushdown past them.

---

## PushDownLeafProjections

`struct` · `datafusion_optimizer::extract_leaf_expressions::PushDownLeafProjections`

```rust
struct PushDownLeafProjections
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
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.extract_leaf_expressions.PushDownLeafProjections.md).


Pushes extraction projections down through schema-preserving nodes towards
leaf nodes (pass 2 of 2, after [`ExtractLeafExpressions`]).

Handles two types of projections:
- **Pure extraction projections** (all `__datafusion_extracted` aliases + columns):
  pushes through Filter/Sort/Limit, merges into existing projections, or routes
  into multi-input node inputs (Join, SubqueryAlias, etc.)
- **Mixed projections** (user projections containing `MoveTowardsLeafNodes`
  sub-expressions): splits into a recovery projection + extraction projection,
  then pushes the extraction projection down.

# Example: Pushing through a Filter

After pass 1, the extraction projection sits directly below the filter:
```text
Projection: id, user                                                              <-- recovery
  Filter: __datafusion_extracted_1 = 'active'
    Projection: user['status'] AS __datafusion_extracted_1, id, user               <-- extraction
      TableScan: t [id, user]
```

Pass 2 pushes the extraction projection through the recovery and filter,
and a subsequent `OptimizeProjections` pass removes the (now-redundant)
recovery projection:
```text
Filter: __datafusion_extracted_1 = 'active'
  Projection: user['status'] AS __datafusion_extracted_1, id, user                 <-- extraction (pushed down)
    TableScan: t [id, user]
```

---
