# `arrow_arith::boolean::not`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.not.json).

<a id="op-cf212d91dbe199b0f03c41c0"></a>
## not

`function` · `arrow_arith::boolean::not` · arrow-arith 59.3.0

```rust
fn not(left: &BooleanArray) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Performs unary `NOT` operation on an arrays. If value is null then the result is also
null.
# Error
This function never errors. It returns an error for consistency.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::not;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let not_a = not(&a).unwrap();
assert_eq!(not_a, BooleanArray::from(vec![Some(true), Some(false), None]));
```
