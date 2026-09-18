# `arrow_arith::numeric::mul_wrapping`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.mul_wrapping.json).

<a id="op-76c7a183848d609b7cc913f9"></a>
## mul_wrapping

`function` · `arrow_arith::numeric::mul_wrapping` · arrow-arith 59.3.0

```rust
fn mul_wrapping(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs * rhs`, wrapping on overflow for [`DataType::is_integer`]

Unresolved upstream links (retained, not inferred): ``DataType::is_integer``.
