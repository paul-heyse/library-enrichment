# `arrow_arith::aggregate::max_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max_array.json).

<a id="op-23cc4c5633950651e7e69f1c"></a>
## max_array

`function` · `arrow_arith::aggregate::max_array` · arrow-arith 59.3.0

```rust
fn max_array<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Option<T::Native>
```

Source: `src/aggregate.rs:752`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the max of values in the array of `ArrowNumericType` type, or dictionary
array with value of `ArrowNumericType` type.
