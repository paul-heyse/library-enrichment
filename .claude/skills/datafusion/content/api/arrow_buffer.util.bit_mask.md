# `arrow_buffer::util::bit_mask`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.util.bit_mask.json`](../model/arrow_buffer.util.bit_mask.json)

## set_bits

`function` · `arrow_buffer::util::bit_mask::set_bits`

Also reachable as `arrow::util::bit_mask::set_bits`, `arrow_data::bit_mask::set_bits`

```rust
fn set_bits(write_data: &mut [u8], data: &[u8], offset_write: usize, offset_read: usize, len: usize) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_mask.set_bits.md).


Util function to set bits in a slice of bytes.

This will sets all bits on `write_data` in the range `[offset_write..offset_write+len]`
to be equal to the bits in `data` in the range `[offset_read..offset_read+len]`
returns the number of `0` bits `data[offset_read..offset_read+len]`
`offset_write`, `offset_read`, and `len` are in terms of bits

---
