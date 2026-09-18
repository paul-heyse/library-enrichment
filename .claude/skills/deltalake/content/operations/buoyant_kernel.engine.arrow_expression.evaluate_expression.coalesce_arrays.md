# `buoyant_kernel::engine::arrow_expression::evaluate_expression::coalesce_arrays`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_expression.evaluate_expression.coalesce_arrays.json).

<a id="op-87fe522dfa7834b456aa7424"></a>
## coalesce_arrays

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::coalesce_arrays` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn coalesce_arrays(arrays: &[arrow::array::ArrayRef], result_type: Option<&schema::DataType>) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/evaluate_expression.rs#L836).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/evaluate_expression.rs:836`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Coalesce multiple arrays into one by selecting the first non-null value from each row.

This function implements SQL COALESCE semantics: for each row, it iterates through
the input arrays from left to right and returns the first non-null value found. If all values
are null for a given row, the result will be null for that row.

# Parameters
- `arrays`: Slice of Arrow arrays to coalesce. Must not be empty and all arrays must have the
  same data type.
- `result_type`: Optional expected result type. If provided, must match the arrays' data type.

# Returns
An `ArrayRef` containing the coalesced values with the same number of rows as the input arrays.

# Errors
This function returns an `ArrowError` in the following cases:
- **Empty input**: The default engine currently does not support empty COALESCE statements.
- **Mismatched row counts**: Not all arrays have the same number of rows.
- **Mismatched data types**: Not all arrays have exactly the same data type.
- **Invalid result type**: If `result_type` is provided but doesn't match the arrays' data type.
