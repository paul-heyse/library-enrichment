# `arrow_arith::arity::try_unary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.try_unary.json).

<a id="op-57aa76381324ccfd6a6d1c1a"></a>
## try_unary

`function` · `arrow_arith::arity::try_unary` · arrow-arith 59.3.0

```rust
fn try_unary<I, F, O>(array: &PrimitiveArray<I>, op: F) -> Result<PrimitiveArray<O>, arrow_schema::ArrowError> where I: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(I::Native) -> Result<O::Native, arrow_schema::ArrowError>
```

Source: `src/arity.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

See [`PrimitiveArray::try_unary`]

Unresolved upstream links (retained, not inferred): ``PrimitiveArray::try_unary``.
