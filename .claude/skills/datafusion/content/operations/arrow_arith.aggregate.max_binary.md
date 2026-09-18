# `arrow_arith::aggregate::max_binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max_binary.json).

<a id="op-3e5b30d4508619139a575f3b"></a>
## max_binary

`function` · `arrow_arith::aggregate::max_binary` · arrow-arith 59.3.0

```rust
fn max_binary<T: OffsetSizeTrait>(array: &GenericBinaryArray<T>) -> Option<&[u8]>
```

Source: `src/aggregate.rs:521`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the maximum value in the binary array, according to the natural order.
