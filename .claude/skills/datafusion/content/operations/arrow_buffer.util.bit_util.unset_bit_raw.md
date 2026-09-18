# `arrow_buffer::util::bit_util::unset_bit_raw`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_util.unset_bit_raw.json).

<a id="op-b52ca6464bd4f73f844831c5"></a>
## unset_bit_raw

`function` · `arrow_buffer::util::bit_util::unset_bit_raw` · arrow-buffer 59.3.0

```rust
unsafe fn unset_bit_raw(data: *mut u8, i: usize)
```

Source: `src/util/bit_util.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Sets bit at position `i` for `data` to 0

# Safety

Note this doesn't do any bound checking, for performance reason. The caller is
responsible to guarantee that `i` is within bounds.
