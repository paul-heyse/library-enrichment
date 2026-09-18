# `arrow_arith::numeric::neg_wrapping`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.neg_wrapping.json).

<a id="op-5817a0d9556eed66b1e5c456"></a>
## neg_wrapping

`function` · `arrow_arith::numeric::neg_wrapping` · arrow-arith 59.3.0

```rust
fn neg_wrapping(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Negates each element of  `array`, wrapping on overflow for [`DataType::is_integer`]

Unresolved upstream links (retained, not inferred): ``DataType::is_integer``.
