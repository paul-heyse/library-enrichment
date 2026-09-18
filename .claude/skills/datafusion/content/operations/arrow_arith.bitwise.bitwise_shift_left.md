# `arrow_arith::bitwise::bitwise_shift_left`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_shift_left.json).

<a id="op-19b29e043de083595699db8e"></a>
## bitwise_shift_left

`function` · `arrow_arith::bitwise::bitwise_shift_left` · arrow-arith 59.3.0

```rust
fn bitwise_shift_left<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShl<Output = T::Native>
```

Source: `src/bitwise.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform bitwise `left << right` operation on two arrays. If either left or right value is null
then the result is also null.
