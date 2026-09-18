# `arrow_buffer::util::bit_util::get_bit_raw`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_util.get_bit_raw.json).

<a id="op-624d335e9c5ffa440abb79a0"></a>
## get_bit_raw

`function` · `arrow_buffer::util::bit_util::get_bit_raw` · arrow-buffer 59.3.0

```rust
unsafe fn get_bit_raw(data: *const u8, i: usize) -> bool
```

Source: `src/util/bit_util.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns whether bit at position `i` in `data` is set or not.

# Safety

Note this doesn't do any bound checking, for performance reason. The caller is
responsible to guarantee that `i` is within bounds.
