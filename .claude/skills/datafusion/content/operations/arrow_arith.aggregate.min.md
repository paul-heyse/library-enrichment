# `arrow_arith::aggregate::min`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min.json).

<a id="op-6f2b0a8e99f73ce057334379"></a>
## min

`function` · `arrow_arith::aggregate::min` · arrow-arith 59.3.0

```rust
fn min<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

Source: `src/aggregate.rs:1012`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

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
