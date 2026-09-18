# `arrow_arith::arity::try_binary_mut`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.arity.try_binary_mut.json).

<a id="op-4e93319270ec6b664f8bcf50"></a>
## try_binary_mut

`function` · `arrow_arith::arity::try_binary_mut` · arrow-arith 59.3.0

```rust
fn try_binary_mut<T, F>(a: PrimitiveArray<T>, b: &PrimitiveArray<T>, op: F) -> Result<Result<PrimitiveArray<T>, arrow_schema::ArrowError>, PrimitiveArray<T>> where T: ArrowPrimitiveType, F: Fn(T::Native, T::Native) -> Result<T::Native, arrow_schema::ArrowError>
```

Source: `src/arity.rs:305`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Applies the provided fallible binary operation across `a` and `b` by mutating the mutable
[`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) `a` with the results.

Returns any error encountered, or collects the results into a [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) as return
value. If any index is null in either `a` or `b`, the corresponding index in the result will
also be null.

Like [`try_unary`](../operations/arrow_arith.arity.try_unary.md#op-57aa76381324ccfd6a6d1c1a) the function is only evaluated for non-null indices.

See [`binary_mut`](../operations/arrow_arith.arity.binary_mut.md#op-c01470dd82ba77667f5ee248) for errors and buffer reuse information.
