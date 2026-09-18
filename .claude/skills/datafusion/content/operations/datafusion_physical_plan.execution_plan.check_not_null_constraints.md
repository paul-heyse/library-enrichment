# `datafusion_physical_plan::execution_plan::check_not_null_constraints`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.check_not_null_constraints.json).

<a id="op-3404751a4160ce875c46325f"></a>
## check_not_null_constraints

`function` · `datafusion_physical_plan::execution_plan::check_not_null_constraints` · datafusion-physical-plan 55.1.0

```rust
fn check_not_null_constraints(batch: arrow::array::RecordBatch, column_indices: &Vec<usize>) -> datafusion_common::Result<arrow::array::RecordBatch>
```

Source: `src/execution_plan.rs:1930`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Checks a `RecordBatch` for `not null` constraints on specified columns.

# Arguments

* `batch` - The `RecordBatch` to be checked
* `column_indices` - A vector of column indices that should be checked for
  `not null` constraints.

# Returns

* `Result<RecordBatch>` - The original `RecordBatch` if all constraints are met

This function iterates over the specified column indices and ensures that none
of the columns contain null values. If any column contains null values, an error
is returned.
