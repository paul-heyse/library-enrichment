# `arrow_arith::arity::binary_mut`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.binary_mut.json).

<a id="op-c01470dd82ba77667f5ee248"></a>
## binary_mut

`function` · `arrow_arith::arity::binary_mut` · arrow-arith 59.3.0

```rust
fn binary_mut<T, U, F>(a: PrimitiveArray<T>, b: &PrimitiveArray<U>, op: F) -> Result<Result<PrimitiveArray<T>, arrow_schema::ArrowError>, PrimitiveArray<T>> where T: ArrowPrimitiveType, U: ArrowPrimitiveType, F: Fn(T::Native, U::Native) -> T::Native
```

Source: `src/arity.rs:201`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Applies a binary and infallible function to values in two arrays, replacing
the values in the first array in place.

# Details

Given two arrays of length `len`, calls `op(a[i], b[i])` for `i` in
`0..len`, modifying the [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) `a` in place, if possible.

If any index is null in either `a` or `b`, the corresponding index in the
result will also be null.

# Buffer Reuse

If the underlying buffers in `a` are not shared with other arrays,  mutates
the underlying buffer in place, without allocating.

If the underlying buffer in `a` are shared, returns Err(self)

Like [`unary`](../operations/arrow_arith.arity.unary.md#op-53e0a88375dd8e9cf37488a9) the provided function is evaluated for every index, ignoring validity. This
is beneficial when the cost of the operation is low compared to the cost of branching, and
especially when the operation can be vectorised, however, requires `op` to be infallible
for all possible values of its inputs

# Errors

* If the arrays have different lengths
* If the array is not mutable (see "Buffer Reuse")

# See Also

* Documentation on [`PrimitiveArray::unary_mut`] for operating on [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1).

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

Unresolved upstream links (retained, not inferred): ``PrimitiveArray::unary_mut``.
