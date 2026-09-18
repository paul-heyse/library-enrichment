# `arrow_buffer::buffer::ops::buffer_bin_and`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.ops.buffer_bin_and.json).

<a id="op-ee981796fcf2cc23104920ad"></a>
## buffer_bin_and

`function` · `arrow_buffer::buffer::ops::buffer_bin_and` · arrow-buffer 59.3.0

```rust
fn buffer_bin_and(left: &super::Buffer, left_offset_in_bits: usize, right: &super::Buffer, right_offset_in_bits: usize, len_in_bits: usize) -> super::Buffer
```

Source: `src/buffer/ops.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Apply a bitwise and to two inputs and return the result as a Buffer.
The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

# See Also
* [`BooleanBuffer::from_bitwise_binary_op`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-9333fbfb6fa56e36b57b1265) for creating `BooleanBuffer`s directly
