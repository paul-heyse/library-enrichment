# `arrow_buffer::buffer::ops`

Crate `arrow-buffer` · 8 public items · structured records in [`model/arrow_buffer.buffer.ops.json`](../model/arrow_buffer.buffer.ops.json)

## bitwise_bin_op_helper

`function` · `arrow_buffer::buffer::ops::bitwise_bin_op_helper`

```rust
fn bitwise_bin_op_helper<F>(left: &super::Buffer, left_offset_in_bits: usize, right: &super::Buffer, right_offset_in_bits: usize, len_in_bits: usize, op: F) -> super::Buffer where F: FnMut(u64, u64) -> u64
```

Apply a bitwise operation `op` to two inputs and return the result as a Buffer.

The inputs are treated as bitmaps, meaning that offsets and length are
specified in number of bits.

NOTE: The operation `op` is applied to chunks of 64 bits (u64) and any bits
outside the offsets and len are set to zero out before calling `op`.

---

## bitwise_quaternary_op_helper

`function` · `arrow_buffer::buffer::ops::bitwise_quaternary_op_helper`

```rust
fn bitwise_quaternary_op_helper<F>(buffers: [&super::Buffer; 4], offsets: [usize; 4], len_in_bits: usize, op: F) -> super::Buffer where F: Fn(u64, u64, u64, u64) -> u64
```

Apply a bitwise operation `op` to four inputs and return the result as a Buffer.

The inputs are treated as bitmaps, meaning that offsets and length are
specified in number of bits.

NOTE: The operation `op` is applied to chunks of 64 bits (u64) and any bits
outside the offsets and len are set to zero out before calling `op`.

---

## bitwise_unary_op_helper

`function` · `arrow_buffer::buffer::ops::bitwise_unary_op_helper`

```rust
fn bitwise_unary_op_helper<F>(left: &super::Buffer, offset_in_bits: usize, len_in_bits: usize, op: F) -> super::Buffer where F: FnMut(u64) -> u64
```

Apply a bitwise operation `op` to one input and return the result as a Buffer.

The input is treated as a bitmap, meaning that offset and length are
specified in number of bits.

NOTE: The operation `op` is applied to chunks of 64 bits (u64) and any bits
outside the offsets and len are set to zero out before calling `op`.

---

## buffer_bin_and

`function` · `arrow_buffer::buffer::ops::buffer_bin_and`

```rust
fn buffer_bin_and(left: &super::Buffer, left_offset_in_bits: usize, right: &super::Buffer, right_offset_in_bits: usize, len_in_bits: usize) -> super::Buffer
```

Apply a bitwise and to two inputs and return the result as a Buffer.
The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

# See Also
* [`BooleanBuffer::from_bitwise_binary_op`] for creating `BooleanBuffer`s directly

---

## buffer_bin_and_not

`function` · `arrow_buffer::buffer::ops::buffer_bin_and_not`

```rust
fn buffer_bin_and_not(left: &super::Buffer, left_offset_in_bits: usize, right: &super::Buffer, right_offset_in_bits: usize, len_in_bits: usize) -> super::Buffer
```

Apply a bitwise and_not to two inputs and return the result as a Buffer.
The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

# See Also
* [`BooleanBuffer::from_bitwise_binary_op`] for creating `BooleanBuffer`s directly

---

## buffer_bin_or

`function` · `arrow_buffer::buffer::ops::buffer_bin_or`

```rust
fn buffer_bin_or(left: &super::Buffer, left_offset_in_bits: usize, right: &super::Buffer, right_offset_in_bits: usize, len_in_bits: usize) -> super::Buffer
```

Apply a bitwise or to two inputs and return the result as a Buffer.
The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

# See Also
* [`BooleanBuffer::from_bitwise_binary_op`] for creating `BooleanBuffer`s directly

---

## buffer_bin_xor

`function` · `arrow_buffer::buffer::ops::buffer_bin_xor`

```rust
fn buffer_bin_xor(left: &super::Buffer, left_offset_in_bits: usize, right: &super::Buffer, right_offset_in_bits: usize, len_in_bits: usize) -> super::Buffer
```

Apply a bitwise xor to two inputs and return the result as a Buffer.
The inputs are treated as bitmaps, meaning that offsets and length are specified in number of bits.

# See Also
* [`BooleanBuffer::from_bitwise_binary_op`] for creating `BooleanBuffer`s directly

---

## buffer_unary_not

`function` · `arrow_buffer::buffer::ops::buffer_unary_not`

```rust
fn buffer_unary_not(left: &super::Buffer, offset_in_bits: usize, len_in_bits: usize) -> super::Buffer
```

Apply a bitwise not to one input and return the result as a Buffer.
The input is treated as a bitmap, meaning that offset and length are specified in number of bits.

# See Also
* [`BooleanBuffer::from_bitwise_unary_op`] for creating `BooleanBuffer`s directly

---
