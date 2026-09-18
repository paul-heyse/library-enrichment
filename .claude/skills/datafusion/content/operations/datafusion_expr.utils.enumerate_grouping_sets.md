# `datafusion_expr::utils::enumerate_grouping_sets`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.enumerate_grouping_sets.json).

<a id="op-ddcbcb8cac420da36b90e371"></a>
## enumerate_grouping_sets

`function` · `datafusion_expr::utils::enumerate_grouping_sets` · datafusion-expr 55.1.0

```rust
fn enumerate_grouping_sets(group_expr: Vec<Expr>) -> datafusion_common::Result<Vec<Expr>>
```

Source: `src/utils.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Convert multiple grouping expressions into one [`GroupingSet::GroupingSets`](../operations/datafusion_expr.expr.GroupingSet.md#op-e323280bb1ad09dec715a72a),\
if the grouping expression does not contain [`Expr::GroupingSet`](../operations/datafusion_expr.expr.Expr.md#op-9aea79504686cdb750abed59) or only has one expression,\
no conversion will be performed.

e.g.

person.id,\
GROUPING SETS ((person.age, person.salary),(person.age)),\
ROLLUP(person.state, person.birth_date)

=>

GROUPING SETS (\
  (person.id, person.age, person.salary),\
  (person.id, person.age, person.salary, person.state),\
  (person.id, person.age, person.salary, person.state, person.birth_date),\
  (person.id, person.age),\
  (person.id, person.age, person.state),\
  (person.id, person.age, person.state, person.birth_date)\
)
