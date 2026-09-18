# `arrow_arith::arity`

Crate `arrow-arith` · 8 public items · structured records in [`model/arrow_arith.arity.json`](../model/arrow_arith.arity.json)

## binary

`function` · `arrow_arith::arity::binary`

Also reachable as `arrow::compute::binary`, `arrow::compute::kernels::arity::binary`

```rust
fn binary<A, B, F, O>(a: &PrimitiveArray<A>, b: &PrimitiveArray<B>, op: F) -> Result<PrimitiveArray<O>, arrow_schema::ArrowError> where A: ArrowPrimitiveType, B: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(A::Native, B::Native) -> O::Native
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.binary.md).


Allies a binary infallable function to two [`PrimitiveArray`]s,
producing a new [`PrimitiveArray`]

# Details

Given two arrays of length `len`, calls `op(a[i], b[i])` for `i` in `0..len`, collecting
the results in a [`PrimitiveArray`].

If any index is null in either `a` or `b`, the
corresponding index in the result will also be null

Like [`unary`], the `op` is evaluated for every element in the two arrays,
including those elements which are NULL. This is beneficial as the cost of
the operation is low compared to the cost of branching, and especially when
the operation can be vectorised, however, requires `op` to be infallible for
all possible values of its inputs

# Errors

* if the arrays have different lengths.

# Example
```
# use arrow_arith::arity::binary;
# use arrow_array::{Float32Array, Int32Array};
# use arrow_array::types::Int32Type;
let a = Float32Array::from(vec![Some(5.1f32), None, Some(6.8), Some(7.2)]);
let b = Int32Array::from(vec![1, 2, 4, 9]);
// compute int(a) + b for each element
let c = binary(&a, &b, |a, b| a as i32 + b).unwrap();
assert_eq!(c, Int32Array::from(vec![Some(6), None, Some(10), Some(16)]));
```

---

## binary_mut

`function` · `arrow_arith::arity::binary_mut`

Also reachable as `arrow::compute::binary_mut`, `arrow::compute::kernels::arity::binary_mut`

```rust
fn binary_mut<T, U, F>(a: PrimitiveArray<T>, b: &PrimitiveArray<U>, op: F) -> Result<Result<PrimitiveArray<T>, arrow_schema::ArrowError>, PrimitiveArray<T>> where T: ArrowPrimitiveType, U: ArrowPrimitiveType, F: Fn(T::Native, U::Native) -> T::Native
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.binary_mut.md).


Applies a binary and infallible function to values in two arrays, replacing
the values in the first array in place.

# Details

Given two arrays of length `len`, calls `op(a[i], b[i])` for `i` in
`0..len`, modifying the [`PrimitiveArray`] `a` in place, if possible.

If any index is null in either `a` or `b`, the corresponding index in the
result will also be null.

# Buffer Reuse

If the underlying buffers in `a` are not shared with other arrays,  mutates
the underlying buffer in place, without allocating.

If the underlying buffer in `a` are shared, returns Err(self)

Like [`unary`] the provided function is evaluated for every index, ignoring validity. This
is beneficial when the cost of the operation is low compared to the cost of branching, and
especially when the operation can be vectorised, however, requires `op` to be infallible
for all possible values of its inputs

# Errors

* If the arrays have different lengths
* If the array is not mutable (see "Buffer Reuse")

# See Also

* Documentation on [`PrimitiveArray::unary_mut`] for operating on [`ArrayRef`].

# Example
```
# use arrow_arith::arity::binary_mut;
# use arrow_array::{Float32Array, Int32Array};
# use arrow_array::types::Int32Type;
// compute a + b for each element
let a = Float32Array::from(vec![Some(5.1f32), None, Some(6.8)]);
let b = Int32Array::from(vec![Some(1), None, Some(2)]);
// compute a + b, updating the value in a in place if possible
let a = binary_mut(a, &b, |a, b| a + b as f32).unwrap().unwrap();
// a is updated in place
assert_eq!(a, Float32Array::from(vec![Some(6.1), None, Some(8.8)]));
```

# Example with shared buffers
```
# use arrow_arith::arity::binary_mut;
# use arrow_array::Float32Array;
# use arrow_array::types::Int32Type;
let a = Float32Array::from(vec![Some(5.1f32), None, Some(6.8)]);
let b = Float32Array::from(vec![Some(1.0f32), None, Some(2.0)]);
// a_clone shares the buffer with a
let a_cloned = a.clone();
// try to update a in place, but it is shared. Returns Err(a)
let a = binary_mut(a, &b, |a, b| a + b).unwrap_err();
assert_eq!(a_cloned, a);
// drop shared reference
drop(a_cloned);
// now a is not shared, so we can update it in place
let a = binary_mut(a, &b, |a, b| a + b).unwrap().unwrap();
assert_eq!(a, Float32Array::from(vec![Some(6.1), None, Some(8.8)]));
```

---

## try_binary

`function` · `arrow_arith::arity::try_binary`

Also reachable as `arrow::compute::kernels::arity::try_binary`, `arrow::compute::try_binary`

```rust
fn try_binary<A: ArrayAccessor, B: ArrayAccessor, F, O>(a: A, b: B, op: F) -> Result<PrimitiveArray<O>, arrow_schema::ArrowError> where O: ArrowPrimitiveType, F: Fn(A::Item, B::Item) -> Result<O::Native, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.try_binary.md).


Applies the provided fallible binary operation across `a` and `b`.

This will return any error encountered, or collect the results into
a [`PrimitiveArray`]. If any index is null in either `a`
or `b`, the corresponding index in the result will also be null

Like [`try_unary`] the function is only evaluated for non-null indices

# Error

Return an error if the arrays have different lengths or
the operation is under erroneous

---

## try_binary_mut

`function` · `arrow_arith::arity::try_binary_mut`

Also reachable as `arrow::compute::kernels::arity::try_binary_mut`, `arrow::compute::try_binary_mut`

```rust
fn try_binary_mut<T, F>(a: PrimitiveArray<T>, b: &PrimitiveArray<T>, op: F) -> Result<Result<PrimitiveArray<T>, arrow_schema::ArrowError>, PrimitiveArray<T>> where T: ArrowPrimitiveType, F: Fn(T::Native, T::Native) -> Result<T::Native, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.try_binary_mut.md).


Applies the provided fallible binary operation across `a` and `b` by mutating the mutable
[`PrimitiveArray`] `a` with the results.

Returns any error encountered, or collects the results into a [`PrimitiveArray`] as return
value. If any index is null in either `a` or `b`, the corresponding index in the result will
also be null.

Like [`try_unary`] the function is only evaluated for non-null indices.

See [`binary_mut`] for errors and buffer reuse information.

---

## try_unary

`function` · `arrow_arith::arity::try_unary`

Also reachable as `arrow::compute::kernels::arity::try_unary`, `arrow::compute::try_unary`

```rust
fn try_unary<I, F, O>(array: &PrimitiveArray<I>, op: F) -> Result<PrimitiveArray<O>, arrow_schema::ArrowError> where I: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(I::Native) -> Result<O::Native, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.try_unary.md).


See [`PrimitiveArray::try_unary`]

---

## try_unary_mut

`function` · `arrow_arith::arity::try_unary_mut`

Also reachable as `arrow::compute::kernels::arity::try_unary_mut`, `arrow::compute::try_unary_mut`

```rust
fn try_unary_mut<I, F>(array: PrimitiveArray<I>, op: F) -> Result<Result<PrimitiveArray<I>, arrow_schema::ArrowError>, PrimitiveArray<I>> where I: ArrowPrimitiveType, F: Fn(I::Native) -> Result<I::Native, arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.try_unary_mut.md).


See [`PrimitiveArray::try_unary_mut`]

---

## unary

`function` · `arrow_arith::arity::unary`

Also reachable as `arrow::compute::kernels::arity::unary`, `arrow::compute::unary`

```rust
fn unary<I, F, O>(array: &PrimitiveArray<I>, op: F) -> PrimitiveArray<O> where I: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(I::Native) -> O::Native
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.unary.md).


See [`PrimitiveArray::unary`]

---

## unary_mut

`function` · `arrow_arith::arity::unary_mut`

Also reachable as `arrow::compute::kernels::arity::unary_mut`, `arrow::compute::unary_mut`

```rust
fn unary_mut<I, F>(array: PrimitiveArray<I>, op: F) -> Result<PrimitiveArray<I>, PrimitiveArray<I>> where I: ArrowPrimitiveType, F: Fn(I::Native) -> I::Native
```

[Full member, field, variant and typed contracts](../operations/arrow_arith.arity.unary_mut.md).


See [`PrimitiveArray::unary_mut`]

---
