# `arrow_arith::numeric::add_wrapping`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.add_wrapping.json).

<a id="op-71752560a0f8c1f9a21d04e8"></a>
## add_wrapping

`function` · `arrow_arith::numeric::add_wrapping` · arrow-arith 59.3.0

```rust
fn add_wrapping(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs + rhs`, wrapping on overflow for [`DataType::is_integer`]

Unresolved upstream links (retained, not inferred): ``DataType::is_integer``.
