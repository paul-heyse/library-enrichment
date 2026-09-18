# `arrow_arith::aggregate`

Crate `arrow-arith` · 27 public items · structured records in [`model/arrow_arith.aggregate.json`](../model/arrow_arith.aggregate.json)

## bit_and

`function` · `arrow_arith::aggregate::bit_and`

Also reachable as `arrow::compute::bit_and`, `arrow::compute::kernels::aggregate::bit_and`

```rust
fn bit_and<T>(array: &PrimitiveArray<T>) -> Option<T::Native> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native> + ArrowNativeTypeOp
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.bit_and.md).


Returns the bitwise and of all non-null input values.

Returns `None` if the array is empty or only contains null values.

---

## bit_or

`function` · `arrow_arith::aggregate::bit_or`

Also reachable as `arrow::compute::bit_or`, `arrow::compute::kernels::aggregate::bit_or`

```rust
fn bit_or<T>(array: &PrimitiveArray<T>) -> Option<T::Native> where T: ArrowNumericType, T::Native: BitOr<Output = T::Native> + ArrowNativeTypeOp
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.bit_or.md).


Returns the bitwise or of all non-null input values.

Returns `None` if the array is empty or only contains null values.

---

## bit_xor

`function` · `arrow_arith::aggregate::bit_xor`

Also reachable as `arrow::compute::bit_xor`, `arrow::compute::kernels::aggregate::bit_xor`

```rust
fn bit_xor<T>(array: &PrimitiveArray<T>) -> Option<T::Native> where T: ArrowNumericType, T::Native: BitXor<Output = T::Native> + ArrowNativeTypeOp
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.bit_xor.md).


Returns the bitwise xor of all non-null input values.

Returns `None` if the array is empty or only contains null values.

---

## bool_and

`function` · `arrow_arith::aggregate::bool_and`

Also reachable as `arrow::compute::bool_and`, `arrow::compute::kernels::aggregate::bool_and`

```rust
fn bool_and(array: &BooleanArray) -> Option<bool>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.bool_and.md).


Returns true if all non-null input values are true, otherwise false.

Returns `None` if the array is empty or only contains null values.

---

## bool_or

`function` · `arrow_arith::aggregate::bool_or`

Also reachable as `arrow::compute::bool_or`, `arrow::compute::kernels::aggregate::bool_or`

```rust
fn bool_or(array: &BooleanArray) -> Option<bool>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.bool_or.md).


Returns true if any non-null input value is true, otherwise false.

Returns `None` if the array is empty or only contains null values.

---

## max

`function` · `arrow_arith::aggregate::max`

Also reachable as `arrow::compute::kernels::aggregate::max`, `arrow::compute::max`

```rust
fn max<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max.md).


Returns the maximum value in the array, according to the natural order.
For floating point arrays any NaN values are considered to be greater than any other non-null value

# Example
```rust
# use arrow_array::Int32Array;
# use arrow_arith::aggregate::max;
let array = Int32Array::from(vec![4, 8, 2]);
let result = max(&array);
assert_eq!(result, Some(8));
```

---

## max_array

`function` · `arrow_arith::aggregate::max_array`

Also reachable as `arrow::compute::kernels::aggregate::max_array`, `arrow::compute::max_array`

```rust
fn max_array<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Option<T::Native>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max_array.md).


Returns the max of values in the array of `ArrowNumericType` type, or dictionary
array with value of `ArrowNumericType` type.

---

## max_binary

`function` · `arrow_arith::aggregate::max_binary`

Also reachable as `arrow::compute::kernels::aggregate::max_binary`, `arrow::compute::max_binary`

```rust
fn max_binary<T: OffsetSizeTrait>(array: &GenericBinaryArray<T>) -> Option<&[u8]>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max_binary.md).


Returns the maximum value in the binary array, according to the natural order.

---

## max_binary_view

`function` · `arrow_arith::aggregate::max_binary_view`

Also reachable as `arrow::compute::kernels::aggregate::max_binary_view`, `arrow::compute::max_binary_view`

```rust
fn max_binary_view(array: &BinaryViewArray) -> Option<&[u8]>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max_binary_view.md).


Returns the maximum value in the binary view array, according to the natural order.

---

## max_boolean

`function` · `arrow_arith::aggregate::max_boolean`

Also reachable as `arrow::compute::kernels::aggregate::max_boolean`, `arrow::compute::max_boolean`

```rust
fn max_boolean(array: &BooleanArray) -> Option<bool>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max_boolean.md).


Returns the maximum value in the boolean array

# Example
```
# use arrow_array::BooleanArray;
# use arrow_arith::aggregate::max_boolean;
let a = BooleanArray::from(vec![Some(true), None, Some(false)]);
assert_eq!(max_boolean(&a), Some(true))
```

---

## max_fixed_size_binary

`function` · `arrow_arith::aggregate::max_fixed_size_binary`

Also reachable as `arrow::compute::kernels::aggregate::max_fixed_size_binary`, `arrow::compute::max_fixed_size_binary`

```rust
fn max_fixed_size_binary(array: &FixedSizeBinaryArray) -> Option<&[u8]>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max_fixed_size_binary.md).


Returns the maximum value in the fixed size binary array, according to the natural order.

---

## max_string

`function` · `arrow_arith::aggregate::max_string`

Also reachable as `arrow::compute::kernels::aggregate::max_string`, `arrow::compute::max_string`

```rust
fn max_string<T: OffsetSizeTrait>(array: &GenericStringArray<T>) -> Option<&str>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max_string.md).


Returns the maximum value in the string array, according to the natural order.

---

## max_string_view

`function` · `arrow_arith::aggregate::max_string_view`

Also reachable as `arrow::compute::kernels::aggregate::max_string_view`, `arrow::compute::max_string_view`

```rust
fn max_string_view(array: &StringViewArray) -> Option<&str>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.max_string_view.md).


Returns the maximum value in the string view array, according to the natural order.

---

## min

`function` · `arrow_arith::aggregate::min`

Also reachable as `arrow::compute::kernels::aggregate::min`, `arrow::compute::min`

```rust
fn min<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min.md).


Returns the minimum value in the array, according to the natural order.
For floating point arrays any NaN values are considered to be greater than any other non-null value

# Example
```rust
# use arrow_array::Int32Array;
# use arrow_arith::aggregate::min;
let array = Int32Array::from(vec![8, 2, 4]);
let result = min(&array);
assert_eq!(result, Some(2));
```

---

## min_array

`function` · `arrow_arith::aggregate::min_array`

Also reachable as `arrow::compute::kernels::aggregate::min_array`, `arrow::compute::min_array`

```rust
fn min_array<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Option<T::Native>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min_array.md).


Returns the min of values in the array of `ArrowNumericType` type, or dictionary
array with value of `ArrowNumericType` type.

---

## min_binary

`function` · `arrow_arith::aggregate::min_binary`

Also reachable as `arrow::compute::kernels::aggregate::min_binary`, `arrow::compute::min_binary`

```rust
fn min_binary<T: OffsetSizeTrait>(array: &GenericBinaryArray<T>) -> Option<&[u8]>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min_binary.md).


Returns the minimum value in the binary array, according to the natural order.

---

## min_binary_view

`function` · `arrow_arith::aggregate::min_binary_view`

Also reachable as `arrow::compute::kernels::aggregate::min_binary_view`, `arrow::compute::min_binary_view`

```rust
fn min_binary_view(array: &BinaryViewArray) -> Option<&[u8]>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min_binary_view.md).


Returns the minimum value in the binary view array, according to the natural order.

---

## min_boolean

`function` · `arrow_arith::aggregate::min_boolean`

Also reachable as `arrow::compute::kernels::aggregate::min_boolean`, `arrow::compute::min_boolean`

```rust
fn min_boolean(array: &BooleanArray) -> Option<bool>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min_boolean.md).


Returns the minimum value in the boolean array.

# Example
```
# use arrow_array::BooleanArray;
# use arrow_arith::aggregate::min_boolean;
let a = BooleanArray::from(vec![Some(true), None, Some(false)]);
assert_eq!(min_boolean(&a), Some(false))
```

---

## min_fixed_size_binary

`function` · `arrow_arith::aggregate::min_fixed_size_binary`

Also reachable as `arrow::compute::kernels::aggregate::min_fixed_size_binary`, `arrow::compute::min_fixed_size_binary`

```rust
fn min_fixed_size_binary(array: &FixedSizeBinaryArray) -> Option<&[u8]>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min_fixed_size_binary.md).


Returns the minimum value in the fixed size binary array, according to the natural order.

---

## min_string

`function` · `arrow_arith::aggregate::min_string`

Also reachable as `arrow::compute::kernels::aggregate::min_string`, `arrow::compute::min_string`

```rust
fn min_string<T: OffsetSizeTrait>(array: &GenericStringArray<T>) -> Option<&str>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min_string.md).


Returns the minimum value in the string array, according to the natural order.

---

## min_string_view

`function` · `arrow_arith::aggregate::min_string_view`

Also reachable as `arrow::compute::kernels::aggregate::min_string_view`, `arrow::compute::min_string_view`

```rust
fn min_string_view(array: &StringViewArray) -> Option<&str>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.min_string_view.md).


Returns the minimum value in the string view array, according to the natural order.

---

## product

`function` · `arrow_arith::aggregate::product`

Also reachable as `arrow::compute::kernels::aggregate::product`, `arrow::compute::product`

```rust
fn product<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.product.md).


Returns the product of values in the primitive array.

Returns `None` if the array is empty or only contains null values.

This doesn't detect overflow in release mode by default. Once overflowing, the result will
wrap around. For an overflow-checking variant, use [`product_checked`] instead.

---

## product_checked

`function` · `arrow_arith::aggregate::product_checked`

Also reachable as `arrow::compute::kernels::aggregate::product_checked`, `arrow::compute::product_checked`

```rust
fn product_checked<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Result<Option<T::Native>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.product_checked.md).


Returns the product of values in the primitive array.

Returns `Ok(None)` if the array is empty or only contains null values.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant,
use [`product`] instead.

---

## sum

`function` · `arrow_arith::aggregate::sum`

Also reachable as `arrow::compute::kernels::aggregate::sum`, `arrow::compute::sum`

```rust
fn sum<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.sum.md).


Returns the sum of values in the primitive array.

Returns `None` if the array is empty or only contains null values.

This doesn't detect overflow in release mode by default. Once overflowing, the result will
wrap around. For an overflow-checking variant, use [`sum_checked`] instead.

---

## sum_array

`function` · `arrow_arith::aggregate::sum_array`

Also reachable as `arrow::compute::kernels::aggregate::sum_array`, `arrow::compute::sum_array`

```rust
fn sum_array<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Option<T::Native>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.sum_array.md).


Returns the sum of values in the array.

This doesn't detect overflow. Once overflowing, the result will wrap around.
For an overflow-checking variant, use [`sum_array_checked`] instead.

---

## sum_array_checked

`function` · `arrow_arith::aggregate::sum_array_checked`

Also reachable as `arrow::compute::kernels::aggregate::sum_array_checked`, `arrow::compute::sum_array_checked`

```rust
fn sum_array_checked<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Result<Option<T::Native>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.sum_array_checked.md).


Returns the sum of values in the array.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant,
use [`sum_array`] instead.
Additionally returns an `Err` on run-end-encoded arrays with a provided
values type parameter that is incorrect.

---

## sum_checked

`function` · `arrow_arith::aggregate::sum_checked`

Also reachable as `arrow::compute::kernels::aggregate::sum_checked`, `arrow::compute::sum_checked`

```rust
fn sum_checked<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Result<Option<T::Native>, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.aggregate.sum_checked.md).


Returns the sum of values in the primitive array.

Returns `Ok(None)` if the array is empty or only contains null values.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant,
use [`sum`] instead.

---
