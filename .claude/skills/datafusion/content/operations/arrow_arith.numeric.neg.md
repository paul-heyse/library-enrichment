# `arrow_arith::numeric::neg`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.numeric.neg.json).

<a id="op-d4800dcaf774738a3e19ffac"></a>
## neg

`function` · `arrow_arith::numeric::neg` · arrow-arith 59.3.0

```rust
fn neg(array: &dyn Array) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/numeric.rs:101`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Negates each element of  `array`, returning an error on overflow

Note: negation of unsigned arrays is not supported and will return in an error,
for wrapping unsigned negation consider using [`neg_wrapping`][neg_wrapping()](../operations/arrow_arith.numeric.neg_wrapping.md#op-5817a0d9556eed66b1e5c456)
