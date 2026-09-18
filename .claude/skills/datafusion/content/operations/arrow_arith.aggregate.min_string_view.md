# `arrow_arith::aggregate::min_string_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.aggregate.min_string_view.json).

<a id="op-0e93027c08d9888f6731d282"></a>
## min_string_view

`function` · `arrow_arith::aggregate::min_string_view` · arrow-arith 59.3.0

```rust
fn min_string_view(array: &StringViewArray) -> Option<&str>
```

Source: `src/aggregate.rs:566`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Returns the minimum value in the string view array, according to the natural order.
