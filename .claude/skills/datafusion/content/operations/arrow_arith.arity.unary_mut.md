# `arrow_arith::arity::unary_mut`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.unary_mut.json).

<a id="op-75bfe85f7d0e59c9375d7d02"></a>
## unary_mut

`function` · `arrow_arith::arity::unary_mut` · arrow-arith 59.3.0

```rust
fn unary_mut<I, F>(array: PrimitiveArray<I>, op: F) -> Result<PrimitiveArray<I>, PrimitiveArray<I>> where I: ArrowPrimitiveType, F: Fn(I::Native) -> I::Native
```

Source: `src/arity.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

See [`PrimitiveArray::unary_mut`]

Unresolved upstream links (retained, not inferred): ``PrimitiveArray::unary_mut``.
