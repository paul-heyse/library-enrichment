# `datafusion_optimizer::decorrelate`

Crate `datafusion-optimizer` · 3 public items · structured records in [`model/datafusion_optimizer.decorrelate.json`](../model/datafusion_optimizer.decorrelate.json)

## UN_MATCHED_ROW_INDICATOR

`constant` · `datafusion_optimizer::decorrelate::UN_MATCHED_ROW_INDICATOR`

```rust
const UN_MATCHED_ROW_INDICATOR: &str = "__always_true"
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.decorrelate.UN_MATCHED_ROW_INDICATOR.md).


Used to indicate the unmatched rows from the inner(subquery) table after the left out Join
This is used to handle [the Count bug]

[the Count bug]: https://github.com/apache/datafusion/issues/10553

---

## PullUpCorrelatedExpr

`struct` · `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr`

```rust
struct PullUpCorrelatedExpr
```

**Fields**: `join_filters`, `correlated_subquery_cols_map`, `in_predicate_opt`, `exists_sub_query`, `can_pull_up`, `need_handle_count_bug`, `collected_count_expr_map`, `pull_up_having_expr`, `pulled_up_scalar_agg`

**Implements**: `datafusion_common::tree_node::TreeNodeRewriter`

**Derives**: Debug, Default

**Methods** (4)

```rust
fn new() -> Self
fn with_exists_sub_query(self, exists_sub_query: bool) -> Self
fn with_in_predicate_opt(self, in_predicate_opt: Option<Expr>) -> Self
fn with_need_handle_count_bug(self, need_handle_count_bug: bool) -> Self
```

**via `datafusion_common::tree_node::TreeNodeRewriter`**

```rust
fn f_down(&mut self, plan: LogicalPlan) -> Result<Transformed<LogicalPlan>>
fn f_up(&mut self, plan: LogicalPlan) -> Result<Transformed<LogicalPlan>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.decorrelate.PullUpCorrelatedExpr.md).


This struct rewrite the sub query plan by pull up the correlated
expressions(contains outer reference columns) from the inner subquery's
'Filter'. It adds the inner reference columns to the 'Projection' or
'Aggregate' of the subquery if they are missing, so that they can be
evaluated by the parent operator as the join condition.

---

## ExprResultMap

`type_alias` · `datafusion_optimizer::decorrelate::ExprResultMap`

```rust
type ExprResultMap = datafusion_common::HashMap<String, datafusion_expr::Expr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_optimizer.decorrelate.ExprResultMap.md).


Mapping from expr display name to its evaluation result on empty record
batch (for example: 'count(*)' is 'ScalarValue(0)', 'count(*) + 2' is
'ScalarValue(2)')

---
