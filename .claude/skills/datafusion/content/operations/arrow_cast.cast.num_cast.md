# `arrow_cast::cast::num_cast`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.num_cast.json).

<a id="op-0e92fdea3be20c6f49abfe49"></a>
## num_cast

`function` · `arrow_cast::cast::num_cast` · arrow-cast 59.3.0

```rust
fn num_cast<I, O>(value: I) -> Option<O> where I: NumCast, O: NumCast
```

Source: `src/cast/mod.rs:2552`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Natural cast between numeric types
Return None if the input `value` can't be casted to type `O`.
