# `arrow_arith::aggregate::min_binary`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min_binary.json).

<a id="op-2714bd87b8daccac4d8d8a77"></a>
## min_binary

`function` · `arrow_arith::aggregate::min_binary` · arrow-arith 59.3.0

```rust
fn min_binary<T: OffsetSizeTrait>(array: &GenericBinaryArray<T>) -> Option<&[u8]>
```

Source: `src/aggregate.rs:536`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the minimum value in the binary array, according to the natural order.
