# `arrow_arith::numeric::div`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.div.json).

<a id="op-02d0efd797f57738078e1e85"></a>
## div

`function` · `arrow_arith::numeric::div` · arrow-arith 59.3.0

```rust
fn div(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs / rhs`

Overflow or division by zero will result in an error, with exception to
floating point numbers, which instead follow the IEEE 754 rules
