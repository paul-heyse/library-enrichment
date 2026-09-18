# `arrow_arith::boolean::is_null`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.is_null.json).

<a id="op-3b18e2f683fac962887e8bff"></a>
## is_null

`function` · `arrow_arith::boolean::is_null` · arrow-arith 59.3.0

```rust
fn is_null(input: &dyn Array) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns a non-null [BooleanArray](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) with whether each value of the array is null.
# Error
This function never errors.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::is_null;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let a_is_null = is_null(&a).unwrap();
assert_eq!(a_is_null, BooleanArray::from(vec![false, false, true]));
```
