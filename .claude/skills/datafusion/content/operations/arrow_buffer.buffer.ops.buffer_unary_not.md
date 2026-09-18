# `arrow_buffer::buffer::ops::buffer_unary_not`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.ops.buffer_unary_not.json).

<a id="op-91054b79915e3886ef059572"></a>
## buffer_unary_not

`function` · `arrow_buffer::buffer::ops::buffer_unary_not` · arrow-buffer 59.3.0

```rust
fn buffer_unary_not(left: &super::Buffer, offset_in_bits: usize, len_in_bits: usize) -> super::Buffer
```

Source: `src/buffer/ops.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Apply a bitwise not to one input and return the result as a Buffer.
The input is treated as a bitmap, meaning that offset and length are specified in number of bits.

# See Also
* [`BooleanBuffer::from_bitwise_unary_op`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-a684517dfe8bfbdb1184fefc) for creating `BooleanBuffer`s directly
