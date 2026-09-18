# `buoyant_kernel::engine::arrow_expression::evaluate_expression`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.engine.arrow_expression.evaluate_expression.json`](../model/buoyant_kernel.engine.arrow_expression.evaluate_expression.json)

## coalesce_arrays

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::coalesce_arrays`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_expression.evaluate_expression.coalesce_arrays.md)

Also reachable as `delta_kernel::engine::arrow_expression::evaluate_expression::coalesce_arrays`

```rust
fn coalesce_arrays(arrays: &[arrow::array::ArrayRef], result_type: Option<&schema::DataType>) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError>
```

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

---

## evaluate_expression

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::evaluate_expression`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_expression.evaluate_expression.evaluate_expression.md)

Also reachable as `delta_kernel::engine::arrow_expression::evaluate_expression::evaluate_expression`

```rust
fn evaluate_expression(expression: &expressions::Expression, batch: &arrow::array::RecordBatch, result_type: Option<&schema::DataType>) -> error::DeltaResult<arrow::array::ArrayRef>
```

Evaluates a kernel expression over a record batch

---

## evaluate_predicate

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::evaluate_predicate`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_expression.evaluate_expression.evaluate_predicate.md)

Also reachable as `delta_kernel::engine::arrow_expression::evaluate_expression::evaluate_predicate`

```rust
fn evaluate_predicate(predicate: &expressions::Predicate, batch: &arrow::array::RecordBatch, inverted: bool) -> error::DeltaResult<arrow::array::BooleanArray>
```

Evaluates a (possibly inverted) kernel predicate over a record batch

---

## to_json

`function` · `buoyant_kernel::engine::arrow_expression::evaluate_expression::to_json`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_expression.evaluate_expression.to_json.md)

Also reachable as `delta_kernel::engine::arrow_expression::evaluate_expression::to_json`

```rust
fn to_json(input: &dyn Datum) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError>
```

Converts a StructArray to JSON-encoded strings

---
