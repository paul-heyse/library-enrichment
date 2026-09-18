# `arrow_arith::arity::unary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.unary.json).

<a id="op-53e0a88375dd8e9cf37488a9"></a>
## unary

`function` · `arrow_arith::arity::unary` · arrow-arith 59.3.0

```rust
fn unary<I, F, O>(array: &PrimitiveArray<I>, op: F) -> PrimitiveArray<O> where I: ArrowPrimitiveType, O: ArrowPrimitiveType, F: Fn(I::Native) -> O::Native
```

Source: `src/arity.rs:29`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

See [`PrimitiveArray::unary`]

Unresolved upstream links (retained, not inferred): ``PrimitiveArray::unary``.
