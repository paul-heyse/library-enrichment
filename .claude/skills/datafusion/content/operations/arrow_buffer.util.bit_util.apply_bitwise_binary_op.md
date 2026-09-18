# `arrow_buffer::util::bit_util::apply_bitwise_binary_op`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_util.apply_bitwise_binary_op.json).

<a id="op-1e28964b08a51187f1e0918f"></a>
## apply_bitwise_binary_op

`function` · `arrow_buffer::util::bit_util::apply_bitwise_binary_op` · arrow-buffer 59.3.0

```rust
fn apply_bitwise_binary_op<F>(left: &mut [u8], left_offset_in_bits: usize, right: impl AsRef<[u8]>, right_offset_in_bits: usize, len_in_bits: usize, op: F) where F: FnMut(u64, u64) -> u64
```

Source: `src/util/bit_util.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Applies a bitwise operation relative to another bit-packed byte slice
(right) in place

Note: applies the operation 64-bits (u64) at a time.

# Arguments

* `left` - The mutable buffer to be modified in-place
* `offset_in_bits` - Starting bit offset in Self buffer
* `right` - slice of bit-packed bytes in LSB order
* `right_offset_in_bits` - Starting bit offset in the right buffer
* `len_in_bits` - Number of bits to process
* `op` - Binary operation to apply (e.g., `|a, b| a & b`). Applied a word at a time

# Example: Modify entire buffer
```
# use arrow_buffer::MutableBuffer;
# use arrow_buffer::bit_util::apply_bitwise_binary_op;
let mut left = MutableBuffer::new(2);
left.extend_from_slice(&[0b11110000u8, 0b00110011u8]);
let right = &[0b10101010u8, 0b10101010u8];
// apply bitwise AND between left and right buffers, updating left in place
apply_bitwise_binary_op(left.as_slice_mut(), 0, right, 0, 16, |a, b| a & b);
assert_eq!(left.as_slice(), &[0b10100000u8, 0b00100010u8]);
```

# Example: Modify buffer with offsets
```
# use arrow_buffer::MutableBuffer;
# use arrow_buffer::bit_util::apply_bitwise_binary_op;
let mut left = MutableBuffer::new(2);
left.extend_from_slice(&[0b00000000u8, 0b00000000u8]);
let right = &[0b10110011u8, 0b11111110u8];
// apply bitwise OR between left and right buffers,
// Apply only 8 bits starting from bit offset 3 in left and bit offset 2 in right
apply_bitwise_binary_op(left.as_slice_mut(), 3, right, 2, 8, |a, b| a | b);
assert_eq!(left.as_slice(), &[0b01100000, 0b00000101u8]);
```

# Panics

If the offset or lengths exceed the buffer or slice size.
