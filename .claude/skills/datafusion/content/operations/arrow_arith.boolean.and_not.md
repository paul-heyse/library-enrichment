# `arrow_arith::boolean::and_not`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.and_not.json).

<a id="op-a1ee122c40fb6d7ece5f59fb"></a>
## and_not

`function` · `arrow_arith::boolean::and_not` · arrow-arith 59.3.0

```rust
fn and_not(left: &BooleanArray, right: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:291`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Performs `AND_NOT` operation on two arrays. If either left or right value is null then the
result is also null.
# Error
This function errors when the arrays have different lengths.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::{and, not, and_not};
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let b = BooleanArray::from(vec![Some(true), Some(true), Some(false)]);
let andn_ab = and_not(&a, &b).unwrap();
assert_eq!(andn_ab, BooleanArray::from(vec![Some(false), Some(false), None]));
// It's equal to and(left, not(right))
assert_eq!(andn_ab, and(&a, &not(&b).unwrap()).unwrap());
