# `arrow_buffer::util::bit_mask::set_bits`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_mask.set_bits.json).

<a id="op-5ccb3bf0fc897d073a312caa"></a>
## set_bits

`function` · `arrow_buffer::util::bit_mask::set_bits` · arrow-buffer 59.3.0

```rust
fn set_bits(write_data: &mut [u8], data: &[u8], offset_write: usize, offset_read: usize, len: usize) -> usize
```

Source: `src/util/bit_mask.rs:28`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Util function to set bits in a slice of bytes.

This will sets all bits on `write_data` in the range `[offset_write..offset_write+len]`
to be equal to the bits in `data` in the range `[offset_read..offset_read+len]`
returns the number of `0` bits `data[offset_read..offset_read+len]`
`offset_write`, `offset_read`, and `len` are in terms of bits
