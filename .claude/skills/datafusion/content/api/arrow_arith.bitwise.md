# `arrow_arith::bitwise`

Crate `arrow-arith` · 12 public items · structured records in [`model/arrow_arith.bitwise.json`](../model/arrow_arith.bitwise.json)

## bitwise_and

`function` · `arrow_arith::bitwise::bitwise_and`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_and`

```rust
fn bitwise_and<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native>
```

Perform `left & right` operation on two arrays. If either left or right value is null
then the result is also null.

---

## bitwise_and_not

`function` · `arrow_arith::bitwise::bitwise_and_not`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_and_not`

```rust
fn bitwise_and_not<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native> + Not<Output = T::Native>
```

Perform `left & !right` operation on two arrays. If either left or right value is null
then the result is also null.

---

## bitwise_and_scalar

`function` · `arrow_arith::bitwise::bitwise_and_scalar`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_and_scalar`

```rust
fn bitwise_and_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitAnd<Output = T::Native>
```

Perform bitwise `and` every value in an array with the scalar. If any value in the array is null then the
result is also null.

---

## bitwise_not

`function` · `arrow_arith::bitwise::bitwise_not`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_not`

```rust
fn bitwise_not<T>(array: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: Not<Output = T::Native>
```

Perform `!array` operation on array. If array value is null
then the result is also null.

---

## bitwise_or

`function` · `arrow_arith::bitwise::bitwise_or`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_or`

```rust
fn bitwise_or<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitOr<Output = T::Native>
```

Perform `left | right` operation on two arrays. If either left or right value is null
then the result is also null.

---

## bitwise_or_scalar

`function` · `arrow_arith::bitwise::bitwise_or_scalar`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_or_scalar`

```rust
fn bitwise_or_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitOr<Output = T::Native>
```

Perform bitwise `or` every value in an array with the scalar. If any value in the array is null then the
result is also null.

---

## bitwise_shift_left

`function` · `arrow_arith::bitwise::bitwise_shift_left`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_shift_left`

```rust
fn bitwise_shift_left<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShl<Output = T::Native>
```

Perform bitwise `left << right` operation on two arrays. If either left or right value is null
then the result is also null.

---

## bitwise_shift_left_scalar

`function` · `arrow_arith::bitwise::bitwise_shift_left_scalar`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_shift_left_scalar`

```rust
fn bitwise_shift_left_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShl<Output = T::Native>
```

Perform bitwise `left << right` every value in an array with the scalar. If any value in the array is null then the
result is also null.

---

## bitwise_shift_right

`function` · `arrow_arith::bitwise::bitwise_shift_right`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_shift_right`

```rust
fn bitwise_shift_right<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShr<Output = T::Native>
```

Perform bitwise `left >> right` operation on two arrays. If either left or right value is null
then the result is also null.

---

## bitwise_shift_right_scalar

`function` · `arrow_arith::bitwise::bitwise_shift_right_scalar`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_shift_right_scalar`

```rust
fn bitwise_shift_right_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShr<Output = T::Native>
```

Perform bitwise `left >> right` every value in an array with the scalar. If any value in the array is null then the
result is also null.

---

## bitwise_xor

`function` · `arrow_arith::bitwise::bitwise_xor`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_xor`

```rust
fn bitwise_xor<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitXor<Output = T::Native>
```

Perform `left ^ right` operation on two arrays. If either left or right value is null
then the result is also null.

---

## bitwise_xor_scalar

`function` · `arrow_arith::bitwise::bitwise_xor_scalar`

Also reachable as `arrow::compute::kernels::bitwise::bitwise_xor_scalar`

```rust
fn bitwise_xor_scalar<T>(array: &PrimitiveArray<T>, scalar: T::Native) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: BitXor<Output = T::Native>
```

Perform bitwise `xor` every value in an array with the scalar. If any value in the array is null then the
result is also null.

---
