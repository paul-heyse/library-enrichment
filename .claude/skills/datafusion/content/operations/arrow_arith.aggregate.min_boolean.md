# `arrow_arith::aggregate::min_boolean`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min_boolean.json).

<a id="op-3e852b93796e5d31801ed6a0"></a>
## min_boolean

`function` · `arrow_arith::aggregate::min_boolean` · arrow-arith 59.3.0

```rust
fn min_boolean(array: &BooleanArray) -> Option<bool>
```

Source: `src/aggregate.rs:372`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the minimum value in the boolean array.

# Example
```
# use arrow_array::BooleanArray;
# use arrow_arith::aggregate::min_boolean;
let a = BooleanArray::from(vec![Some(true), None, Some(false)]);
assert_eq!(min_boolean(&a), Some(false))
```
