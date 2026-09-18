# `arrow_buffer::buffer::ops::bitwise_bin_op_helper`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.ops.bitwise_bin_op_helper.json).

<a id="op-95052c0a6bccf85c57d5fddf"></a>
## bitwise_bin_op_helper

`function` · `arrow_buffer::buffer::ops::bitwise_bin_op_helper` · arrow-buffer 59.3.0

```rust
fn bitwise_bin_op_helper<F>(left: &super::Buffer, left_offset_in_bits: usize, right: &super::Buffer, right_offset_in_bits: usize, len_in_bits: usize, op: F) -> super::Buffer where F: FnMut(u64, u64) -> u64
```

Source: `src/buffer/ops.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Apply a bitwise operation `op` to two inputs and return the result as a Buffer.

The inputs are treated as bitmaps, meaning that offsets and length are
specified in number of bits.

NOTE: The operation `op` is applied to chunks of 64 bits (u64) and any bits
outside the offsets and len are set to zero out before calling `op`.
