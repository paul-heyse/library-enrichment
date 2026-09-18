# `arrow_array::array::run_array::AnyRunEndArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.run_array.AnyRunEndArray.json).

<a id="op-66f5298fc7c34c1bec0ad508"></a>
## AnyRunEndArray

`trait` · `arrow_array::array::run_array::AnyRunEndArray` · arrow-array 59.3.0

```rust
trait AnyRunEndArray: Array
```

Source: `src/array/run_array.rs:795`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An array that can be downcast to a [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) of any run end type and any value type.

This can be used to efficiently implement kernels for all possible run end
types without needing to create specialized implementations for each key type.

<a id="op-43772147475dbd0e652c6bc2"></a>
## values

`function` · `arrow_array::array::run_array::AnyRunEndArray::values` · arrow-array 59.3.0

```rust
fn values(&self) -> &Arc<dyn Array>
```

Source: `src/array/run_array.rs:797`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the values of this array.

<a id="op-e1c788819df324bbf5ae60ed"></a>
## with_values

`function` · `arrow_array::array::run_array::AnyRunEndArray::with_values` · arrow-array 59.3.0

```rust
fn with_values(&self, values: ArrayRef) -> ArrayRef
```

Source: `src/array/run_array.rs:801`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a new run-end encoded array with the given values, preserving the
existing run ends.
