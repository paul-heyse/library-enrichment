# `arrow_buffer::util::bit_util::apply_bitwise_unary_op`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_util.apply_bitwise_unary_op.json).

<a id="op-61b160ab902fd849eb53361c"></a>
## apply_bitwise_unary_op

`function` · `arrow_buffer::util::bit_util::apply_bitwise_unary_op` · arrow-buffer 59.3.0

```rust
fn apply_bitwise_unary_op<F>(buffer: &mut [u8], offset_in_bits: usize, len_in_bits: usize, op: F) where F: FnMut(u64) -> u64
```

Source: `src/util/bit_util.rs:308`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Apply a bitwise operation to a mutable buffer, updating it in place.

Note: applies the operation 64-bits (u64) at a time.

# Arguments

* `offset_in_bits` - Starting bit offset for the current buffer
* `len_in_bits` - Number of bits to process
* `op` - Unary operation to apply (e.g., `|a| !a`). Applied a word at a time

# Example: Modify entire buffer
```
# use arrow_buffer::MutableBuffer;
# use arrow_buffer::bit_util::apply_bitwise_unary_op;
let mut buffer = MutableBuffer::new(2);
buffer.extend_from_slice(&[0b11110000u8, 0b00110011u8]);
// apply bitwise NOT to the buffer in place
apply_bitwise_unary_op(buffer.as_slice_mut(), 0, 16, |a| !a);
assert_eq!(buffer.as_slice(), &[0b00001111u8, 0b11001100u8]);
```

# Example: Modify buffer with offsets
```
# use arrow_buffer::MutableBuffer;
# use arrow_buffer::bit_util::apply_bitwise_unary_op;
let mut buffer = MutableBuffer::new(2);
buffer.extend_from_slice(&[0b00000000u8, 0b00000000u8]);
// apply bitwise NOT to 8 bits starting from bit offset 3
apply_bitwise_unary_op(buffer.as_slice_mut(), 3, 8, |a| !a);
assert_eq!(buffer.as_slice(), &[0b11111000u8, 0b00000111u8]);
```

# Panics

If the offset and length exceed the buffer size.
