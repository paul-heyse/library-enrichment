# `arrow_arith::boolean::and`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.and.json).

<a id="op-6a51c3e5e53e5307d1fe3def"></a>
## and

`function` · `arrow_arith::boolean::and` · arrow-arith 59.3.0

```rust
fn and(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Performs `AND` operation on two arrays. If either left or right value is null then the
result is also null.
# Error
This function errors when the arrays have different lengths.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::and;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let and_ab = and(&a, &b).unwrap();
assert_eq!(and_ab, BooleanArray::from(vec![Some(false), Some(true), None]));
```
