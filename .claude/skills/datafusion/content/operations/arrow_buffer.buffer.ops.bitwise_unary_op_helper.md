# `arrow_buffer::buffer::ops::bitwise_unary_op_helper`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.ops.bitwise_unary_op_helper.json).

<a id="op-9da6b66374dcbfc9b80e0dde"></a>
## bitwise_unary_op_helper

`function` · `arrow_buffer::buffer::ops::bitwise_unary_op_helper` · arrow-buffer 59.3.0

```rust
fn bitwise_unary_op_helper<F>(left: &super::Buffer, offset_in_bits: usize, len_in_bits: usize, op: F) -> super::Buffer where F: FnMut(u64) -> u64
```

Source: `src/buffer/ops.rs:112`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Apply a bitwise operation `op` to one input and return the result as a Buffer.

The input is treated as a bitmap, meaning that offset and length are
specified in number of bits.

NOTE: The operation `op` is applied to chunks of 64 bits (u64) and any bits
outside the offsets and len are set to zero out before calling `op`.
