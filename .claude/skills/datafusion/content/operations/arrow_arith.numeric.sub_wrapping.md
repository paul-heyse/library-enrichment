# `arrow_arith::numeric::sub_wrapping`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.sub_wrapping.json).

<a id="op-0fcbeee3aae128ae1e9981cd"></a>
## sub_wrapping

`function` · `arrow_arith::numeric::sub_wrapping` · arrow-arith 59.3.0

```rust
fn sub_wrapping(lhs: &dyn Datum, rhs: &dyn Datum) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform `lhs - rhs`, wrapping on overflow for [`DataType::is_integer`]

Unresolved upstream links (retained, not inferred): ``DataType::is_integer``.
