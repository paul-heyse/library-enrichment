# `buoyant_kernel::engine::arrow_conversion::scalar`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.engine.arrow_conversion.scalar.json`](../model/buoyant_kernel.engine.arrow_conversion.scalar.json)

## extract_primitive_scalar

`function` · `buoyant_kernel::engine::arrow_conversion::scalar::extract_primitive_scalar`

Also reachable as `delta_kernel::engine::arrow_conversion::scalar::extract_primitive_scalar`

```rust
fn extract_primitive_scalar(array: &dyn Array, row_idx: usize) -> DeltaResult<expressions::Scalar>
```

Extracts a primitive kernel [`Scalar`] from the given row of an Arrow array.

This is useful for connectors that partition data using Arrow arrays and need typed
partition values for the write path.

Returns `Scalar::Null(data_type)` if the value at `row_idx` is null.

# Errors

Returns an error if:
- `row_idx` is out of bounds for the array
- The Arrow data type is not a supported primitive type (e.g., Struct, List, Map)
- The Arrow data type is a `Timestamp` with a non-microsecond time unit
- The decimal precision/scale is invalid

---
