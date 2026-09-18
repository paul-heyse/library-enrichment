# `arrow_arith::numeric::rem`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.rem.json).

<a id="op-5a686bf79f3939d742ea7bbd"></a>
## rem

`function` · `arrow_arith::numeric::rem` · arrow-arith 59.3.0

```rust
fn rem(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs % rhs`

Division by zero will result in an error, with exception to
floating point numbers, which instead follow the IEEE 754 rules

`signed_integer::MIN % -1` will not result in an error but return 0
