# `arrow_arith::aggregate::max_boolean`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max_boolean.json).

<a id="op-78fbb8f72f5f5bafb6430bc5"></a>
## max_boolean

`function` · `arrow_arith::aggregate::max_boolean` · arrow-arith 59.3.0

```rust
fn max_boolean(array: &BooleanArray) -> Option<bool>
```

Source: `src/aggregate.rs:430`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the maximum value in the boolean array

# Example
```
# use arrow_array::BooleanArray;
# use arrow_arith::aggregate::max_boolean;
let a = BooleanArray::from(vec![Some(true), None, Some(false)]);
assert_eq!(max_boolean(&a), Some(true))
```
