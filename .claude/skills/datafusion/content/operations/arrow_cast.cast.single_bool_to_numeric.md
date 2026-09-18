# `arrow_cast::cast::single_bool_to_numeric`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.single_bool_to_numeric.json).

<a id="op-a7b1aae3cef1c29d484ac0e9"></a>
## single_bool_to_numeric

`function` · `arrow_cast::cast::single_bool_to_numeric` · arrow-cast 59.3.0

```rust
fn single_bool_to_numeric<O>(value: bool) -> Option<O> where O: num_traits::NumCast + Default
```

Source: `src/cast/mod.rs:2681`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Cast single bool value to numeric value.
