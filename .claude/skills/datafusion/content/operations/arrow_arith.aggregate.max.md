# `arrow_arith::aggregate::max`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max.json).

<a id="op-72a0cab4ee59faf1d28ad9bc"></a>
## max

`function` · `arrow_arith::aggregate::max` · arrow-arith 59.3.0

```rust
fn max<T: ArrowNumericType>(array: &PrimitiveArray<T>) -> Option<T::Native>
```

Source: `src/aggregate.rs:1027`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

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
