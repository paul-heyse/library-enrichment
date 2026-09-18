# `arrow_arith::aggregate::min_string`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min_string.json).

<a id="op-97eede0602a559cf70f9ae3e"></a>
## min_string

`function` · `arrow_arith::aggregate::min_string` · arrow-arith 59.3.0

```rust
fn min_string<T: OffsetSizeTrait>(array: &GenericStringArray<T>) -> Option<&str>
```

Source: `src/aggregate.rs:561`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the minimum value in the string array, according to the natural order.
