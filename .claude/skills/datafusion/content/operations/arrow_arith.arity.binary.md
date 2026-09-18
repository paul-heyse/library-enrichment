# `arrow_arith::arity::binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.binary.json).

<a id="op-fd2506f633b6b4f9b95ff797"></a>
## binary

`function` · `arrow_arith::arity::binary` · arrow-arith 59.3.0

```rust
fn binary<A, B, F, O>(a: &PrimitiveArray<A>, b: &PrimitiveArray<B>, op: F) -> Result<PrimitiveArray<O>, arrow_schema::ArrowError> where A: ArrowPrimitiveType, B: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(A::Native, B::Native) -> O::Native
```

Source: `src/arity.rs:104`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Allies a binary infallable function to two [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814)s,
producing a new [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814)

# Details

Given two arrays of length `len`, calls `op(a[i], b[i])` for `i` in `0..len`, collecting
the results in a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814).

If any index is null in either `a` or `b`, the
corresponding index in the result will also be null

Like [`unary`](../operations/arrow_arith.arity.unary.md#op-53e0a88375dd8e9cf37488a9), the `op` is evaluated for every element in the two arrays,
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
