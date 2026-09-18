# `arrow_arith::aggregate::max_string`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.max_string.json).

<a id="op-3202f8c5c4a92b37405c7619"></a>
## max_string

`function` · `arrow_arith::aggregate::max_string` · arrow-arith 59.3.0

```rust
fn max_string<T: OffsetSizeTrait>(array: &GenericStringArray<T>) -> Option<&str>
```

Source: `src/aggregate.rs:551`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the maximum value in the string array, according to the natural order.
