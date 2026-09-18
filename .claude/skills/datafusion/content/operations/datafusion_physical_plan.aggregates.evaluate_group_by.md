# `datafusion_physical_plan::aggregates::evaluate_group_by`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.evaluate_group_by.json).

<a id="op-62b512cdabc9fbd6a8177032"></a>
## evaluate_group_by

`function` · `datafusion_physical_plan::aggregates::evaluate_group_by` · datafusion-physical-plan 55.1.0

```rust
fn evaluate_group_by(group_by: &PhysicalGroupBy, batch: &arrow::record_batch::RecordBatch) -> datafusion_common::Result<Vec<Vec<arrow::array::ArrayRef>>>
```

Source: `src/aggregates/mod.rs:3093`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Evaluate a group by expression against a `RecordBatch`

Arguments:
- `group_by`: the expression to evaluate
- `batch`: the `RecordBatch` to evaluate against

Returns: A Vec of Vecs of Array of results
The outer Vec appears to be for grouping sets
The inner Vec contains the results per expression
The inner-inner Array contains the results per row

For example, for `GROUP BY GROUPING SETS ((a, b), (a))` with input:

```text
a  b
1  1
1  2
2  1
```

The output is:

```text
[
  [
    a:           [1, 1, 2]
    b:           [1, 2, 1]
    grouping_id: [0, 0, 0]
  ],
  [
    a:           [1, 1, 2]
    b:           [NULL, NULL, NULL]
    grouping_id: [1, 1, 1]
  ]
]
```
