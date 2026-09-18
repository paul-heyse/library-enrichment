# `buoyant_kernel::engine::arrow_conversion::scalar::extract_primitive_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_conversion.scalar.extract_primitive_scalar.json).

<a id="op-0d4020279d213d52d5f73aee"></a>
## extract_primitive_scalar

`function` · `buoyant_kernel::engine::arrow_conversion::scalar::extract_primitive_scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn extract_primitive_scalar(array: &dyn Array, row_idx: usize) -> DeltaResult<expressions::Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/scalar.rs#L43).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/scalar.rs:43`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extracts a primitive kernel [`Scalar`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-8dd45baeed3da91aa357441a) from the given row of an Arrow array.

This is useful for connectors that partition data using Arrow arrays and need typed
partition values for the write path.

Returns `Scalar::Null(data_type)` if the value at `row_idx` is null.

# Errors

Returns an error if:
- `row_idx` is out of bounds for the array
- The Arrow data type is not a supported primitive type (e.g., Struct, List, Map)
- The Arrow data type is a `Timestamp` with a non-microsecond time unit
- The decimal precision/scale is invalid
