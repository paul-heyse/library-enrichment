# `arrow_buffer::util::bit_util::set_bit_raw`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_util.set_bit_raw.json).

<a id="op-6c380012b69e03dcaf388987"></a>
## set_bit_raw

`function` · `arrow_buffer::util::bit_util::set_bit_raw` · arrow-buffer 59.3.0

```rust
unsafe fn set_bit_raw(data: *mut u8, i: usize)
```

Source: `src/util/bit_util.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Sets bit at position `i` for `data`

# Safety

Note this doesn't do any bound checking, for performance reason. The caller is
responsible to guarantee that `i` is within bounds.
