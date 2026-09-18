# `arrow_arith::arity::try_binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.try_binary.json).

<a id="op-094767f2d53db306632a0250"></a>
## try_binary

`function` · `arrow_arith::arity::try_binary` · arrow-arith 59.3.0

```rust
fn try_binary<A: ArrayAccessor, B: ArrayAccessor, F, O>(a: A, b: B, op: F) -> Result<PrimitiveArray<O>, arrow_schema::ArrowError> where O: ArrowPrimitiveType, F: Fn(A::Item, B::Item) -> Result<O::Native, arrow_schema::ArrowError>
```

Source: `src/arity.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Applies the provided fallible binary operation across `a` and `b`.

This will return any error encountered, or collect the results into
a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814). If any index is null in either `a`
or `b`, the corresponding index in the result will also be null

Like [`try_unary`](../operations/arrow_arith.arity.try_unary.md#op-57aa76381324ccfd6a6d1c1a) the function is only evaluated for non-null indices

# Error

Return an error if the arrays have different lengths or
the operation is under erroneous
