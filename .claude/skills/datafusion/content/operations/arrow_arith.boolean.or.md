# `arrow_arith::boolean::or`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.or.json).

<a id="op-9ef96457a2d7ecbfab405246"></a>
## or

`function` · `arrow_arith::boolean::or` · arrow-arith 59.3.0

```rust
fn or(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:273`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Performs `OR` operation on two arrays. If either left or right value is null then the
result is also null.
# Error
This function errors when the arrays have different lengths.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::or;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let or_ab = or(&a, &b).unwrap();
assert_eq!(or_ab, BooleanArray::from(vec![Some(true), Some(true), None]));
```
