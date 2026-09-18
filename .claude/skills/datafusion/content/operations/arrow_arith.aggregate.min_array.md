# `arrow_arith::aggregate::min_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min_array.json).

<a id="op-3e8dad514180bad59f93736c"></a>
## min_array

`function` · `arrow_arith::aggregate::min_array` · arrow-arith 59.3.0

```rust
fn min_array<T: ArrowNumericType, A: ArrayAccessor<Item = T::Native>>(array: A) -> Option<T::Native>
```

Source: `src/aggregate.rs:744`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the min of values in the array of `ArrowNumericType` type, or dictionary
array with value of `ArrowNumericType` type.
