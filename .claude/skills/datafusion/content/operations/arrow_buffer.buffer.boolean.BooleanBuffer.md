# `arrow_buffer::buffer::boolean::BooleanBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.boolean.BooleanBuffer.json).

<a id="op-3a838cdae998215374fcb4b5"></a>
## BooleanBuffer

`struct` · `arrow_buffer::buffer::boolean::BooleanBuffer` · arrow-buffer 59.3.0

```rust
struct BooleanBuffer
```

Source: `src/buffer/boolean.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A slice-able [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) containing bit-packed booleans

This structure represents a sequence of boolean values packed into a
byte-aligned [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b). Both the offset and length are represented in bits.

# Layout

The values are represented as little endian bit-packed values, where the
least significant bit of each byte represents the first boolean value and
then proceeding to the most significant bit.

For example, the 10 bit bitmask `0b0111001101` has length 10, and is
represented using 2 bytes with offset 0 like this:

```text
       ┌─────────────────────────────────┐    ┌─────────────────────────────────┐
       │┌───┬───┬───┬───┬───┬───┬───┬───┐│    │┌───┬───┬───┬───┬───┬───┬───┬───┐│
       ││ 1 │ 0 │ 1 │ 1 │ 0 │ 0 │ 1 │ 1 ││    ││ 1 │ 0 │ ? │ ? │ ? │ ? │ ? │ ? ││
       │└───┴───┴───┴───┴───┴───┴───┴───┘│    │└───┴───┴───┴───┴───┴───┴───┴───┘│
bit    └─────────────────────────────────┘    └─────────────────────────────────┘
offset  0             Byte 0             7    0              Byte 1            7

        length = 10 bits, offset = 0
```

The same bitmask with length 10 and offset 3 would be represented using 2
bytes like this:

```text
      ┌─────────────────────────────────┐    ┌─────────────────────────────────┐
      │┌───┬───┬───┬───┬───┬───┬───┬───┐│    │┌───┬───┬───┬───┬───┬───┬───┬───┐│
      ││ ? │ ? │ ? │ 1 │ 0 │ 1 │ 1 │ 0 ││    ││ 0 │ 1 │ 1 │ 1 │ 0 │ ? │ ? │ ? ││
      │└───┴───┴───┴───┴───┴───┴───┴───┘│    │└───┴───┴───┴───┴───┴───┴───┴───┘│
bit   └─────────────────────────────────┘    └─────────────────────────────────┘
offset 0             Byte 0             7    0              Byte 1            7

       length = 10 bits, offset = 3
```

Note that the bits marked `?` are not logically part of the mask and may
contain either `0` or `1`

# Bitwise Operations

`BooleanBuffer` implements the standard bitwise traits for creating a new
buffer ([`BitAnd`], [`BitOr`], [`BitXor`], [`Not`]) as well as the assign variants
for updating an existing buffer in place when possible ([`BitAndAssign`],
[`BitOrAssign`], [`BitXorAssign`]).

```
# use arrow_buffer::BooleanBuffer;
let mut left = BooleanBuffer::from(&[true, false, true, true] as &[bool]);
let right = BooleanBuffer::from(&[true, true, false, true] as &[bool]);

// Create a new buffer by applying bitwise AND
let anded = &left & &right;
assert_eq!(anded, BooleanBuffer::from(&[true, false, false, true] as &[bool]));

// Update `left` in place by applying bitwise AND in place
left &= &right;
assert_eq!(left, BooleanBuffer::from(&[true, false, false, true] as &[bool]));
```

# See Also
* [`BooleanBufferBuilder`](../operations/arrow_buffer.builder.boolean.BooleanBufferBuilder.md#op-3b603b0bd38714441734f47c) for building [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) instances
* [`NullBuffer`] for representing null values in Arrow arrays

[`NullBuffer`]: crate::NullBuffer

Unresolved upstream links (retained, not inferred): ``BitOr``, ``BitXor``, ``Not``, ``BitAnd``, ``BitAndAssign``, ``BitXorAssign``, ``BitOrAssign``.

<a id="op-00a276f3c27b8548dee98057"></a>
## bit_chunks

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::bit_chunks` · arrow-buffer 59.3.0

```rust
fn bit_chunks(&self) -> BitChunks<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:460`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a [`BitChunks`](../operations/arrow_buffer.util.bit_chunk_iterator.BitChunks.md#op-b8340de6eea8c71484dde9af) instance which can be used to iterate over
this buffer's bits in `u64` chunks

<a id="op-1e5bc4523a1ec5949e250a1c"></a>
## bitand_assign

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::bitand_assign` · arrow-buffer 59.3.0

```rust
fn bitand_assign(&mut self, rhs: &BooleanBuffer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [755, 1], "end": [759, 2], "filename": "src/buffer/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}}}}], "constraints": []}}, "id": "core::ops::bit::BitAndAssign", "path": "BitAndAssign"}, "trait_path": "core::ops::bit::BitAndAssign"}`

Source: `src/buffer/boolean.rs:756`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fba03443d15c07709a9c3067"></a>
## bitor_assign

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::bitor_assign` · arrow-buffer 59.3.0

```rust
fn bitor_assign(&mut self, rhs: &BooleanBuffer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [761, 1], "end": [765, 2], "filename": "src/buffer/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}}}}], "constraints": []}}, "id": "core::ops::bit::BitOrAssign", "path": "BitOrAssign"}, "trait_path": "core::ops::bit::BitOrAssign"}`

Source: `src/buffer/boolean.rs:762`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ff8d6f6f1e275e09227ce13"></a>
## bitxor_assign

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::bitxor_assign` · arrow-buffer 59.3.0

```rust
fn bitxor_assign(&mut self, rhs: &BooleanBuffer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [767, 1], "end": [771, 2], "filename": "src/buffer/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}}}}], "constraints": []}}, "id": "core::ops::bit::BitXorAssign", "path": "BitXorAssign"}, "trait_path": "core::ops::bit::BitXorAssign"}`

Source: `src/buffer/boolean.rs:768`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df11571fa397caabbc88d9a9"></a>
## claim

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::claim` · arrow-buffer 59.3.0

```rust
fn claim(&self, pool: &dyn MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Claim memory used by this buffer in the provided memory pool.

See [`Buffer::claim`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-3276c69adbc249530c026c94) for details.

<a id="op-b10a84b5c05fd84e25304bf0"></a>
## clone

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 17], "end": [96, 22], "filename": "src/buffer/boolean.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/buffer/boolean.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-138375e7b82b623a4cb86e6e"></a>
## collect_bool

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::collect_bool` · arrow-buffer 59.3.0

```rust
fn collect_bool<F: FnMut(usize) -> bool>(len: usize, f: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Invokes `f` with indexes `0..len` collecting the boolean results into a new `BooleanBuffer`

<a id="op-5b2be345703d4f67f27c838b"></a>
## count_set_bits

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::count_set_bits` · arrow-buffer 59.3.0

```rust
fn count_set_bits(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:438`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of set bits in this buffer

<a id="op-325e6dff591b69b94a4df25f"></a>
## eq

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [116, 2], "filename": "src/buffer/boolean.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/buffer/boolean.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-351273c1e2bff3a230ea8697"></a>
## find_nth_set_bit_position

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::find_nth_set_bit_position` · arrow-buffer 59.3.0

```rust
fn find_nth_set_bit_position(&self, start: usize, n: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:445`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Finds the position of the n-th set bit (1-based) starting from `start` index.
If fewer than `n` set bits are found, returns the length of the buffer.

<a id="op-9d9e34890142afa04e7c540a"></a>
## fmt

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 10], "end": [96, 15], "filename": "src/buffer/boolean.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer/boolean.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0c73f5a5e6b0a8d78111842"></a>
## from

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(builder: BooleanBufferBuilder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "crate::BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [352, 1], "end": [357, 2], "filename": "src/builder/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/builder/boolean.rs:354`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da03cb48032224e9d259b729"></a>
## from

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: &[bool]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [782, 1], "end": [788, 2], "filename": "src/buffer/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "bool"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/boolean.rs:783`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f577a706c0444169a38dada5"></a>
## from

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: Vec<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [790, 1], "end": [794, 2], "filename": "src/buffer/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/boolean.rs:791`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66055f235bdcc81b5959ba91"></a>
## from_bits

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::from_bits` · arrow-buffer 59.3.0

```rust
fn from_bits(src: impl AsRef<[u8]>, offset_in_bits: usize, len_in_bits: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) by copying the relevant bits from an
input buffer.

# Notes:
* The new `BooleanBuffer` may have non zero offset
  and/or padding bits outside the logical range.

# Example: Create a new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) copying a bit slice from in input slice
```
# use arrow_buffer::BooleanBuffer;
let input = [0b11001100u8, 0b10111010u8];
// // Copy bits 4..16 from input
let result = BooleanBuffer::from_bits(&input, 4, 12);
// output is 12 bits long starting from bit offset 4
assert_eq!(result.len(), 12);
assert_eq!(result.offset(), 4);
// the expected 12 bits are 0b101110101100 (bits 4..16 of the input)
let expected_bits = [false, false, true, true, false, true, false, true, true, true, false, true];
for (i, v) in expected_bits.into_iter().enumerate() {
   assert_eq!(result.value(i), v);
}
// However, underlying buffer has (ignored) bits set outside the requested range
assert_eq!(result.values(), &[0b11001100u8, 0b10111010, 0, 0, 0, 0, 0, 0]);

<a id="op-9333fbfb6fa56e36b57b1265"></a>
## from_bitwise_binary_op

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::from_bitwise_binary_op` · arrow-buffer 59.3.0

```rust
fn from_bitwise_binary_op<F>(left: impl AsRef<[u8]>, left_offset_in_bits: usize, right: impl AsRef<[u8]>, right_offset_in_bits: usize, len_in_bits: usize, op: F) -> Self where F: FnMut(u64, u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:332`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) by applying the bitwise operation `op` to
the relevant bits from two input buffers.

This function is faster than applying the operation bit by bit as
it processes input buffers in chunks of 64 bits (8 bytes) at a time

# Notes:
* `op` takes two `u64` inputs and produces one `u64` output.
* `op` must only apply bitwise operations
  on the relevant bits; the input `u64` values may contain irrelevant bits
  and may be processed differently on different endian architectures.
* `op` may be called with input bits outside the requested range.
* Returned `BooleanBuffer` may have non zero offset
* Returned `BooleanBuffer` may have bits set outside the requested range

# See Also
- [`BooleanBuffer::from_bitwise_unary_op`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-a684517dfe8bfbdb1184fefc) for unary operations on a single input buffer.
- [`apply_bitwise_binary_op`](bit_util::apply_bitwise_binary_op) for in-place binary bitwise operations

# Example: Create new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) from bitwise `AND` of two [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s
```
# use arrow_buffer::{Buffer, BooleanBuffer};
let left = Buffer::from(vec![0b11001100u8, 0b10111010u8]); // 2 bytes = 16 bits
let right = Buffer::from(vec![0b10101010u8, 0b11011100u8, 0b11110000u8]); // 3 bytes = 24 bits
// AND of the first 12 bits
let result = BooleanBuffer::from_bitwise_binary_op(
  &left, 0, &right, 0, 12, |a, b| a & b
);
assert_eq!(result.len(), 12);
for i in 0..12 {
    assert_eq!(result.value(i), left.as_slice()[i / 8] >> (i % 8) & 1 == 1
        && right.as_slice()[i / 8] >> (i % 8) & 1 == 1);
}
```

# Example: Create new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) from bitwise `OR` of two byte slices
```
# use arrow_buffer::{BooleanBuffer, bit_util};
let left = [0b11001100u8, 0b10111010u8];
let right = [0b10101010u8, 0b11011100u8];
// OR of bits 4..16 from left and bits 0..12 from right
let result = BooleanBuffer::from_bitwise_binary_op(
 &left, 4, &right, 0, 12, |a, b| a | b
);
assert_eq!(result.len(), 12);
for i in 0..12 {
    let l = bit_util::get_bit(&left, 4 + i);
    let r = bit_util::get_bit(&right, i);
    assert_eq!(result.value(i), l | r);
}
```

<a id="op-a684517dfe8bfbdb1184fefc"></a>
## from_bitwise_unary_op

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::from_bitwise_unary_op` · arrow-buffer 59.3.0

```rust
fn from_bitwise_unary_op<F>(src: impl AsRef<[u8]>, offset_in_bits: usize, len_in_bits: usize, op: F) -> Self where F: FnMut(u64) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:228`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) by applying the bitwise operation to `op`
to an input buffer.

This function is faster than applying the operation bit by bit as
it processes input buffers in chunks of 64 bits (8 bytes) at a time

# Notes:
* `op` takes a single `u64` inputs and produces one `u64` output.
* `op` must only apply bitwise operations
  on the relevant bits; the input `u64` may contain irrelevant bits
  and may be processed differently on different endian architectures.
* `op` may be called with input bits outside the requested range
* Returned `BooleanBuffer` may have non zero offset
* Returned `BooleanBuffer` may have bits set outside the requested range

# See Also
- [`BooleanBuffer::from_bitwise_binary_op`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-9333fbfb6fa56e36b57b1265) to create a new buffer from a binary operation
- [`apply_bitwise_unary_op`](bit_util::apply_bitwise_unary_op) for in-place unary bitwise operations

# Example: Create new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) from bitwise `NOT`
```
# use arrow_buffer::BooleanBuffer;
let input = [0b11001100u8, 0b10111010u8]; // 2 bytes = 16 bits
// NOT of bits 4..16
let result = BooleanBuffer::from_bitwise_unary_op(
 &input, 4, 12, |a| !a
);
// output is 12 bits long starting from bit offset 4
assert_eq!(result.len(), 12);
assert_eq!(result.offset(), 4);
// the expected 12 bits are 0b001100110101, (NOT of the requested bits)
let expected_bits = [true, true, false, false, true, false, true, false, false, false, true, false];
for (i, v) in expected_bits.into_iter().enumerate() {
    assert_eq!(result.value(i), v);
}
// However, underlying buffer has (ignored) bits set outside the requested range
let expected = [0b00110011u8, 0b01000101u8, 255, 255, 255, 255, 255, 255];
assert_eq!(result.values(), &expected);
```

<a id="op-9309118564b6a37b022138c2"></a>
## from_iter

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [796, 1], "end": [804, 2], "filename": "src/buffer/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/buffer/boolean.rs:797`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6630da893d64705641e473a9"></a>
## has_false

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::has_false` · arrow-buffer 59.3.0

```rust
fn has_false(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:660`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns whether there is at least one `false` value in this buffer.

This is more efficient than `len() > count_set_bits()` because it can short-circuit
as soon as a `false` value is found, without counting all set bits.

Returns `false` for empty buffer.

<a id="op-10df92dbd4047d492b3c8ede"></a>
## has_true

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::has_true` · arrow-buffer 59.3.0

```rust
fn has_true(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:645`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns whether there is at least one `true` value in this buffer.

This is more efficient than `count_set_bits() > 0` because it can short-circuit
as soon as a `true` value is found, without counting all set bits.

Returns `false` for empty buffer.

<a id="op-01865de3291f6ce36b609fc8"></a>
## inner

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::inner` · arrow-buffer 59.3.0

```rust
fn inner(&self) -> &Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:547`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)

Note: this does not account for offset and length of this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

<a id="op-b7beb5da8e30e546fd88402e"></a>
## into_inner

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::into_inner` · arrow-buffer 59.3.0

```rust
fn into_inner(self) -> Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:554`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b), consuming self

Note: this does not account for offset and length of this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

<a id="op-ea68a96cac33b942271ee7ec"></a>
## is_empty

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) is empty

<a id="op-578e9c3c4b063f01e33799d7"></a>
## iter

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::iter` · arrow-buffer 59.3.0

```rust
fn iter(&self) -> BitIterator<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:610`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns an iterator over the bits in this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

<a id="op-3247acad816f128d0d950d85"></a>
## len

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:472`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the length of this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) in bits (not bytes)

<a id="op-ed5d4cbddf64226a4050cc46"></a>
## new

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: Buffer, bit_offset: usize, bit_len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:124`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) from a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b), `bit_offset` offset and `bit_len` length

# Panics

This method will panic if `buffer` is not large enough

<a id="op-0440fc176cbee343fa69beee"></a>
## new_set

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::new_set` · arrow-buffer 59.3.0

```rust
fn new_set(length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) of `length` bits (not bytes) where all values are `true`

<a id="op-71d9bd4220c768bb06da6672"></a>
## new_unset

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::new_unset` · arrow-buffer 59.3.0

```rust
fn new_unset(length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) of `length` bits (not bytes) where all values are `false`

<a id="op-d5f2b1222a93ec6bf2c6fc41"></a>
## offset

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::offset` · arrow-buffer 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:466`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the offset of this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) in bits (not bytes)

<a id="op-13c70f00536b4a456c1b0e44"></a>
## ptr_eq

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::ptr_eq` · arrow-buffer 59.3.0

```rust
fn ptr_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:537`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) is equal to `other`, using pointer comparisons
to determine buffer equality. This is cheaper than `PartialEq::eq` but may
return false when the arrays are logically equal

<a id="op-876e53f813595f49d493348b"></a>
## set_indices

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::set_indices` · arrow-buffer 59.3.0

```rust
fn set_indices(&self) -> BitIndexIterator<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:620`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns an iterator over the set bit positions in this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

<a id="op-d0b54d0d7b4908d1ad6a84b8"></a>
## set_indices_u32

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::set_indices_u32` · arrow-buffer 59.3.0

```rust
fn set_indices_u32(&self) -> BitIndexU32Iterator<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a `u32` iterator over set bit positions without any usize->u32 conversion

<a id="op-548abec08d1016e37ef6686b"></a>
## set_slices

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::set_slices` · arrow-buffer 59.3.0

```rust
fn set_slices(&self) -> BitSliceIterator<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:630`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a [`BitSliceIterator`](../operations/arrow_buffer.util.bit_iterator.BitSliceIterator.md#op-c825a8467c523296ef7280a9) yielding contiguous ranges of set bits

<a id="op-2812668114fc891788fceb03"></a>
## shrink_to_fit

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::shrink_to_fit` · arrow-buffer 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:483`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Free up unused memory.

<a id="op-ae004a8e8e4aaae15a1dbab4"></a>
## slice

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::slice` · arrow-buffer 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:515`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Slices this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) by the provided `offset` and `length`

<a id="op-e3d35c51990b3812973a6267"></a>
## sliced

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::sliced` · arrow-buffer 59.3.0

```rust
fn sliced(&self) -> Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:530`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a new [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) containing the sliced contents of this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

Equivalent to `self.buffer.bit_slice(self.offset, self.len)`

<a id="op-5ed0dea58457b2bd4ab75e77"></a>
## value

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::value` · arrow-buffer 59.3.0

```rust
fn value(&self, idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:494`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the boolean value at index `i`.

# Panics

Panics if `i >= self.len()`

<a id="op-6af8de6634763294e8ad0b2b"></a>
## value_unchecked

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::value_unchecked` · arrow-buffer 59.3.0

```rust
unsafe fn value_unchecked(&self, i: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the boolean value at index `i`.

# Safety
This doesn't check bounds, the caller must ensure that index < self.len()

<a id="op-2790530427a59cfb76de9ae3"></a>
## values

`function` · `arrow_buffer::buffer::boolean::BooleanBuffer::values` · arrow-buffer 59.3.0

```rust
fn values(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [688, 2], "filename": "src/buffer/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/boolean.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the packed values of this [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) not including any offset
