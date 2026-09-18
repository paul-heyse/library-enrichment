# `arrow_arith::numeric::mul`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.mul.json).

<a id="op-ca1f1a8fa665842dfbfd9d72"></a>
## mul

`function` · `arrow_arith::numeric::mul` · arrow-arith 59.3.0

```rust
fn mul(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs * rhs`, returning an error on overflow
