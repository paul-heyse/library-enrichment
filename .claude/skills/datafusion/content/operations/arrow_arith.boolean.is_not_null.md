# `arrow_arith::boolean::is_not_null`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.is_not_null.json).

<a id="op-31315188ddad394ae8448fc1"></a>
## is_not_null

`function` · `arrow_arith::boolean::is_not_null` · arrow-arith 59.3.0

```rust
fn is_not_null(input: &dyn Array) -> Result<BooleanArray, arrow_schema::ArrowError>
```

Source: `src/boolean.rs:347`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns a non-null [BooleanArray](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) with whether each value of the array is not null.
# Error
This function never errors.
# Example
```rust
# use arrow_array::BooleanArray;
# use arrow_arith::boolean::is_not_null;
let a = BooleanArray::from(vec![Some(false), Some(true), None]);
let a_is_not_null = is_not_null(&a).unwrap();
assert_eq!(a_is_not_null, BooleanArray::from(vec![true, true, false]));
```
