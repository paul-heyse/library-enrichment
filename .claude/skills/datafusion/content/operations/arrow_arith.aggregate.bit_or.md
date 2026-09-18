# `arrow_arith::aggregate::bit_or`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.bit_or.json).

<a id="op-605589d4ddccdf6deaf05927"></a>
## bit_or

`function` · `arrow_arith::aggregate::bit_or` · arrow-arith 59.3.0

```rust
fn bit_or<T>(array: &PrimitiveArray<T>) -> Option<T::Native> where T: ArrowNumericType, T::Native: BitOr<Output = T::Native> + ArrowNativeTypeOp
```

Source: `src/aggregate.rs:862`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the bitwise or of all non-null input values.

Returns `None` if the array is empty or only contains null values.
