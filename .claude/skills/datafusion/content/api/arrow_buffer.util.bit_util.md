# `arrow_buffer::util::bit_util`

Crate `arrow-buffer` · 11 public items · structured records in [`model/arrow_buffer.util.bit_util.json`](../model/arrow_buffer.util.bit_util.json)

## apply_bitwise_binary_op

`function` · `arrow_buffer::util::bit_util::apply_bitwise_binary_op`

Also reachable as `arrow::util::bit_util::apply_bitwise_binary_op`

```rust
fn apply_bitwise_binary_op<F>(left: &mut [u8], left_offset_in_bits: usize, right: impl AsRef<[u8]>, right_offset_in_bits: usize, len_in_bits: usize, op: F) where F: FnMut(u64, u64) -> u64
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.apply_bitwise_binary_op.md).


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

---

## apply_bitwise_unary_op

`function` · `arrow_buffer::util::bit_util::apply_bitwise_unary_op`

Also reachable as `arrow::util::bit_util::apply_bitwise_unary_op`

```rust
fn apply_bitwise_unary_op<F>(buffer: &mut [u8], offset_in_bits: usize, len_in_bits: usize, op: F) where F: FnMut(u64) -> u64
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.apply_bitwise_unary_op.md).


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

---

## ceil

`function` · `arrow_buffer::util::bit_util::ceil`

Also reachable as `arrow::util::bit_util::ceil`

```rust
fn ceil(value: usize, divisor: usize) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.ceil.md).


Returns the ceil of `value`/`divisor`

---

## get_bit

`function` · `arrow_buffer::util::bit_util::get_bit`

Also reachable as `arrow::util::bit_util::get_bit`

```rust
fn get_bit(data: &[u8], i: usize) -> bool
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.get_bit.md).


Returns whether bit at position `i` in `data` is set or not

---

## get_bit_raw

`function` · `arrow_buffer::util::bit_util::get_bit_raw`

Also reachable as `arrow::util::bit_util::get_bit_raw`

```rust
unsafe fn get_bit_raw(data: *const u8, i: usize) -> bool
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.get_bit_raw.md).


Returns whether bit at position `i` in `data` is set or not.

# Safety

Note this doesn't do any bound checking, for performance reason. The caller is
responsible to guarantee that `i` is within bounds.

---

## round_upto_multiple_of_64

`function` · `arrow_buffer::util::bit_util::round_upto_multiple_of_64`

Also reachable as `arrow::util::bit_util::round_upto_multiple_of_64`

```rust
fn round_upto_multiple_of_64(num: usize) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.round_upto_multiple_of_64.md).


Returns the nearest number that is `>=` than `num` and is a multiple of 64

---

## round_upto_power_of_2

`function` · `arrow_buffer::util::bit_util::round_upto_power_of_2`

Also reachable as `arrow::util::bit_util::round_upto_power_of_2`

```rust
fn round_upto_power_of_2(num: usize, factor: usize) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.round_upto_power_of_2.md).


Returns the nearest multiple of `factor` that is `>=` than `num`. Here `factor` must
be a power of 2.

---

## set_bit

`function` · `arrow_buffer::util::bit_util::set_bit`

Also reachable as `arrow::util::bit_util::set_bit`

```rust
fn set_bit(data: &mut [u8], i: usize)
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.set_bit.md).


Sets bit at position `i` for `data` to 1

---

## set_bit_raw

`function` · `arrow_buffer::util::bit_util::set_bit_raw`

Also reachable as `arrow::util::bit_util::set_bit_raw`

```rust
unsafe fn set_bit_raw(data: *mut u8, i: usize)
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.set_bit_raw.md).


Sets bit at position `i` for `data`

# Safety

Note this doesn't do any bound checking, for performance reason. The caller is
responsible to guarantee that `i` is within bounds.

---

## unset_bit

`function` · `arrow_buffer::util::bit_util::unset_bit`

Also reachable as `arrow::util::bit_util::unset_bit`

```rust
fn unset_bit(data: &mut [u8], i: usize)
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.unset_bit.md).


Sets bit at position `i` for `data` to 0

---

## unset_bit_raw

`function` · `arrow_buffer::util::bit_util::unset_bit_raw`

Also reachable as `arrow::util::bit_util::unset_bit_raw`

```rust
unsafe fn unset_bit_raw(data: *mut u8, i: usize)
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.util.bit_util.unset_bit_raw.md).


Sets bit at position `i` for `data` to 0

# Safety

Note this doesn't do any bound checking, for performance reason. The caller is
responsible to guarantee that `i` is within bounds.

---
