# `arrow_buffer::util::bit_util::round_upto_power_of_2`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_util.round_upto_power_of_2.json).

<a id="op-aa501705af01fc217e2f993b"></a>
## round_upto_power_of_2

`function` · `arrow_buffer::util::bit_util::round_upto_power_of_2` · arrow-buffer 59.3.0

```rust
fn round_upto_power_of_2(num: usize, factor: usize) -> usize
```

Source: `src/util/bit_util.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the nearest multiple of `factor` that is `>=` than `num`. Here `factor` must
be a power of 2.
