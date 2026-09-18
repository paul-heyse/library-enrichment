# `arrow_arith::arity::try_unary_mut`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.try_unary_mut.json).

<a id="op-7c2b0a13753c2be194fe7be0"></a>
## try_unary_mut

`function` · `arrow_arith::arity::try_unary_mut` · arrow-arith 59.3.0

```rust
fn try_unary_mut<I, F>(array: PrimitiveArray<I>, op: F) -> Result<Result<PrimitiveArray<I>, arrow_schema::ArrowError>, PrimitiveArray<I>> where I: ArrowPrimitiveType, F: Fn(I::Native) -> Result<I::Native, arrow_schema::ArrowError>
```

Source: `src/arity.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

See [`PrimitiveArray::try_unary_mut`]

Unresolved upstream links (retained, not inferred): ``PrimitiveArray::try_unary_mut``.
