# `arrow_buffer::buffer::ops::bitwise_quaternary_op_helper`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.ops.bitwise_quaternary_op_helper.json).

<a id="op-3b65b4df1447af9b3b099c9e"></a>
## bitwise_quaternary_op_helper

`function` · `arrow_buffer::buffer::ops::bitwise_quaternary_op_helper` · arrow-buffer 59.3.0

```rust
fn bitwise_quaternary_op_helper<F>(buffers: [&super::Buffer; 4], offsets: [usize; 4], len_in_bits: usize, op: F) -> super::Buffer where F: Fn(u64, u64, u64, u64) -> u64
```

Source: `src/buffer/ops.rs:29`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Apply a bitwise operation `op` to four inputs and return the result as a Buffer.

The inputs are treated as bitmaps, meaning that offsets and length are
specified in number of bits.

NOTE: The operation `op` is applied to chunks of 64 bits (u64) and any bits
outside the offsets and len are set to zero out before calling `op`.
