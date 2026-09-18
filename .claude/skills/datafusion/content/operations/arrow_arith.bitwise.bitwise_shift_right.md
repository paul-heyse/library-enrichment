# `arrow_arith::bitwise::bitwise_shift_right`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.bitwise.bitwise_shift_right.json).

<a id="op-5c555e5e0328e3d20f66b0f3"></a>
## bitwise_shift_right

`function` · `arrow_arith::bitwise::bitwise_shift_right` · arrow-arith 59.3.0

```rust
fn bitwise_shift_right<T>(left: &PrimitiveArray<T>, right: &PrimitiveArray<T>) -> Result<PrimitiveArray<T>, arrow_schema::ArrowError> where T: ArrowNumericType, T::Native: WrappingShr<Output = T::Native>
```

Source: `src/bitwise.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Perform bitwise `left >> right` operation on two arrays. If either left or right value is null
then the result is also null.
