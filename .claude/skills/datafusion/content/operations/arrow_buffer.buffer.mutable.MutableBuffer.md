# `arrow_buffer::buffer::mutable::MutableBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.mutable.MutableBuffer.json).

<a id="op-c6098a137f542fbc611c6673"></a>
## MutableBuffer

`struct` · `arrow_buffer::buffer::mutable::MutableBuffer` · arrow-buffer 59.3.0

```rust
struct MutableBuffer
```

Source: `src/buffer/mutable.rs:99`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) is a wrapper over memory regions, used to build
[`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s out of items or slices of items.

[`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s created from [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) (via `into`) are guaranteed to be
aligned along cache lines and in multiples of 64 bytes.

Use [MutableBuffer::push](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-6cb8939ad5dae53225639f18) to insert an item, [MutableBuffer::extend_from_slice](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-1bae38d324b1bfa17cf919f8)
to insert many items, and `into` to convert it to [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b). For typed data,
it is often more efficient to use [`Vec`] and convert it to [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) rather
than using [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) (see examples below).

# See Also
* For a safe, strongly typed API consider using [`Vec`] and [`ScalarBuffer`](crate::ScalarBuffer)
* To apply bitwise operations, see [`apply_bitwise_binary_op`] and [`apply_bitwise_unary_op`]

[`apply_bitwise_binary_op`]: crate::bit_util::apply_bitwise_binary_op
[`apply_bitwise_unary_op`]: crate::bit_util::apply_bitwise_unary_op

# Example: Creating a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) from a [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673)
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
let mut buffer = MutableBuffer::new(0);
buffer.push(256u32);
buffer.extend_from_slice(&[1u32]);
let buffer = Buffer::from(buffer);
assert_eq!(buffer.as_slice(), &[0u8, 1, 0, 0, 1, 0, 0, 0])
```

The same can be achieved more efficiently by using a `Vec<u32>`
```
# use arrow_buffer::buffer::Buffer;
let mut vec = Vec::new();
vec.push(256u32);
vec.extend_from_slice(&[1u32]);
let buffer = Buffer::from(vec);
assert_eq!(buffer.as_slice(), &[0u8, 1, 0, 0, 1, 0, 0, 0]);
```

# Example: Creating a [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) from a `Vec<T>`
```
# use arrow_buffer::buffer::MutableBuffer;
let vec = vec![1u32, 2, 3];
let mutable_buffer = MutableBuffer::from(vec); // reuses the allocation from vec
assert_eq!(mutable_buffer.len(), 12); // 3 * 4 bytes
```

# Example: Creating a [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) from a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
let buffer: Buffer = Buffer::from(&[1u8, 2, 3, 4][..]);
// Only possible to convert a Buffer into a MutableBuffer if uniquely owned
// (i.e., there are no other references to it).
let mut mutable_buffer = match buffer.into_mutable() {
   Ok(mutable) => mutable,
   Err(orig_buffer) => {
     panic!("buffer was not uniquely owned");
   }
};
mutable_buffer.push(5u8);
let buffer = Buffer::from(mutable_buffer);
assert_eq!(buffer.as_slice(), &[1u8, 2, 3, 4, 5])
```

Unresolved upstream links (retained, not inferred): ``Vec``.

<a id="op-888e47afbfc08bac0d7bf9f2"></a>
## Target

`assoc_type` · `arrow_buffer::buffer::mutable::MutableBuffer::Target` · arrow-buffer 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 1], "end": [1081, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/mutable.rs:1076`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a59c8912645405299e49e35"></a>
## as_mut_ptr

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::as_mut_ptr` · arrow-buffer 59.3.0

```rust
fn as_mut_ptr(&mut self) -> *mut u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:532`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a mutable raw pointer to this buffer's internal memory
This pointer is guaranteed to be aligned along cache-lines.

<a id="op-1536c27dfd6ea46f27de6665"></a>
## as_ptr

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::as_ptr` · arrow-buffer 59.3.0

```rust
const fn as_ptr(&self) -> *const u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:525`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a raw pointer to this buffer's internal memory
This pointer is guaranteed to be aligned along cache-lines.

<a id="op-6a3d543f1cba6ff705d14390"></a>
## as_slice

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::as_slice` · arrow-buffer 59.3.0

```rust
fn as_slice(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:513`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the data stored in this buffer as a slice.

<a id="op-7ec815665c1fdb2c9c90aece"></a>
## as_slice_mut

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::as_slice_mut` · arrow-buffer 59.3.0

```rust
fn as_slice_mut(&mut self) -> &mut [u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:518`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the data stored in this buffer as a mutable slice.

<a id="op-afcb557e0852d146b38f94e1"></a>
## capacity

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::capacity` · arrow-buffer 59.3.0

```rust
const fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:497`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the total capacity in this buffer, in bytes.

The invariant `buffer.len() <= buffer.capacity()` is always upheld.

<a id="op-8f6d5215132ef4e817ba6464"></a>
## claim

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::claim` · arrow-buffer 59.3.0

```rust
fn claim(&self, pool: &dyn MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:847`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Register this [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) with the provided [`MemoryPool`](../operations/arrow_buffer.pool.MemoryPool.md#op-ed45519ea8493e8aed943b0a)

This claims the memory used by this buffer in the pool, allowing for
accurate accounting of memory usage. Any prior reservation will be
released so this works well when the buffer is being shared among
multiple arrays.

<a id="op-4cd59d82e29a08de62a62c90"></a>
## clear

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::clear` · arrow-buffer 59.3.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:502`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Clear all existing data from this buffer.

<a id="op-9f53082ce994192253fc2416"></a>
## collect_bool

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::collect_bool` · arrow-buffer 59.3.0

```rust
fn collect_bool<F: FnMut(usize) -> bool>(len: usize, f: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:675`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Invokes `f` with values `0..len` collecting the boolean results into a new `MutableBuffer`

This is similar to `from_trusted_len_iter_bool`, however, can be significantly faster
as it eliminates the conditional `Iterator::next`

<a id="op-0ed4dc0be69cab288dee1853"></a>
## default

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::default` · arrow-buffer 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 1], "end": [1073, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/buffer/mutable.rs:1070`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83b64cbd2d0742e22383103c"></a>
## deref

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::deref` · arrow-buffer 59.3.0

```rust
fn deref(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 1], "end": [1081, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/mutable.rs:1078`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a5386c79926c80155c6b380"></a>
## deref_mut

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::deref_mut` · arrow-buffer 59.3.0

```rust
fn deref_mut(&mut self) -> &mut [u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1083, 1], "end": [1087, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": null, "id": "core::ops::deref::DerefMut", "path": "DerefMut"}, "trait_path": "core::ops::deref::DerefMut"}`

Source: `src/buffer/mutable.rs:1084`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a32383c739aad4b0b4fdd721"></a>
## drop

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::drop` · arrow-buffer 59.3.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 1], "end": [1102, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/buffer/mutable.rs:1096`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed9bf8f8fb517306c8995726"></a>
## eq

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &MutableBuffer) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1104, 1], "end": [1114, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/buffer/mutable.rs:1105`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-177d224ffb14d28630099297"></a>
## extend

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::extend` · arrow-buffer 59.3.0

```rust
fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [871, 1], "end": [877, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "A"}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/buffer/mutable.rs:873`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db6ea15f12a090e6d6e86875"></a>
## extend_bool_trusted_len

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::extend_bool_trusted_len` · arrow-buffer 59.3.0

```rust
unsafe fn extend_bool_trusted_len<I: Iterator<Item = bool>>(&mut self, iter: I, offset: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:721`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Extends this buffer with boolean values.

This requires `iter` to report an exact size via `size_hint`.
`offset` indicates the starting offset in bits in this buffer to begin writing to
and must be less than or equal to the current length of this buffer.
All bits not written to (but readable due to byte alignment) will be zeroed out.

# Panics

Panics if `iter` does not report an exact size via `size_hint`, or if it yields fewer
items than reported, or if extending the buffer requires reserving a capacity that fails
for the same reasons as [`MutableBuffer::reserve`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-18a37c4ca2f74d77b509cc10).

# Safety
Callers must ensure that `iter` reports an exact size via `size_hint`.

<a id="op-1bae38d324b1bfa17cf919f8"></a>
## extend_from_slice

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::extend_from_slice` · arrow-buffer 59.3.0

```rust
fn extend_from_slice<T: ArrowNativeType>(&mut self, items: &[T])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:592`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Extends this buffer from a slice of items that can be represented in bytes, increasing its capacity if needed.
# Example
```
# use arrow_buffer::buffer::MutableBuffer;
let mut buffer = MutableBuffer::new(0);
buffer.extend_from_slice(&[2u32, 0]);
assert_eq!(buffer.len(), 8) // u32 has 4 bytes
```

# Panics

Panics if extending the buffer requires reserving a capacity that fails for the same
reasons as [`MutableBuffer::reserve`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-18a37c4ca2f74d77b509cc10).

<a id="op-6efed1b1c08e8ae4ec0d20fe"></a>
## extend_zeros

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::extend_zeros` · arrow-buffer 59.3.0

```rust
fn extend_zeros(&mut self, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:650`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Extends the buffer by `additional` bytes equal to `0u8`, incrementing its capacity if needed.

# Panics

Panics if `self.len + additional` overflows `usize`, or if growing the buffer requires
reserving a capacity that fails for the same reasons as [`MutableBuffer::reserve`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-18a37c4ca2f74d77b509cc10).

<a id="op-44398c7394c94ff95ad88932"></a>
## fmt

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 10], "end": [98, 15], "filename": "src/buffer/mutable.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer/mutable.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e50fed50cbac31115b5f10e"></a>
## from

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: Vec<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [879, 1], "end": [898, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/mutable.rs:880`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-341e336c2cbd563deb63e279"></a>
## from_iter

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item = bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1142, 1], "end": [1194, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/buffer/mutable.rs:1143`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66d4e7268f837adf5766b385"></a>
## from_iter

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1196, 1], "end": [1202, 2], "filename": "src/buffer/mutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/buffer/mutable.rs:1197`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6554b5089720e400767dded"></a>
## from_len_zeroed

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::from_len_zeroed` · arrow-buffer 59.3.0

```rust
fn from_len_zeroed(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:167`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Allocates a new [MutableBuffer](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) with `len` and capacity to be at least `len` where
all bytes are guaranteed to be `0u8`.
# Example
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
let mut buffer = MutableBuffer::from_len_zeroed(127);
assert_eq!(buffer.len(), 127);
assert!(buffer.capacity() >= 127);
let data = buffer.as_slice_mut();
assert_eq!(data[126], 0u8);
```

# Panics

Panics if `len` is too large to construct a valid allocation [`Layout`]

Unresolved upstream links (retained, not inferred): ``Layout``.

<a id="op-80c2c56890ecf1eedaa66f47"></a>
## from_trusted_len_iter

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::from_trusted_len_iter` · arrow-buffer 59.3.0

```rust
unsafe fn from_trusted_len_iter<T: ArrowNativeType, I: Iterator<Item = T>>(iterator: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 1], "end": [1067, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:958`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) from an [`Iterator`] with a trusted (upper) length.
Prefer this to `collect` whenever possible, as it is faster ~60% faster.
# Example
```
# use arrow_buffer::buffer::MutableBuffer;
let v = vec![1u32];
let iter = v.iter().map(|x| x * 2);
let buffer = unsafe { MutableBuffer::from_trusted_len_iter(iter) };
assert_eq!(buffer.len(), 4) // u32 has 4 bytes
```

# Panics

Panics if the iterator does not report an upper bound via `size_hint`, or if the
reported length does not match the number of items produced, or if allocating the
required buffer fails for the same reasons as [`MutableBuffer::new`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-f5811bd4323fb6c5ed369c24).

# Safety
This method assumes that the iterator's size is correct and is undefined behavior
to use it on an iterator that reports an incorrect length.

Unresolved upstream links (retained, not inferred): ``Iterator``.

<a id="op-09ff5e524424bf68a48fb8f9"></a>
## from_trusted_len_iter_bool

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::from_trusted_len_iter_bool` · arrow-buffer 59.3.0

```rust
unsafe fn from_trusted_len_iter_bool<I: Iterator<Item = bool>>(iterator: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 1], "end": [1067, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:1008`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) from a boolean [`Iterator`] with a trusted (upper) length.
# use arrow_buffer::buffer::MutableBuffer;
# Example
```
# use arrow_buffer::buffer::MutableBuffer;
let v = vec![false, true, false];
let iter = v.iter().map(|x| *x || true);
let buffer = unsafe { MutableBuffer::from_trusted_len_iter_bool(iter) };
assert_eq!(buffer.len(), 1) // 3 booleans have 1 byte
```

# Panics

Panics if the iterator does not report an upper bound via `size_hint`, or if it yields
fewer items than reported.

# Safety
This method assumes that the iterator's size is correct and is undefined behavior
to use it on an iterator that reports an incorrect length.

Unresolved upstream links (retained, not inferred): ``Iterator``.

<a id="op-fb444a4424d17a5bd80dc95b"></a>
## is_empty

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::is_empty` · arrow-buffer 59.3.0

```rust
const fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:482`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns whether this buffer is empty or not.

<a id="op-380ce5b10d2d1caf5e031284"></a>
## len

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::len` · arrow-buffer 59.3.0

```rust
const fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:489`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the length (the number of bytes written) in this buffer.
The invariant `buffer.len() <= buffer.capacity()` is always upheld.

<a id="op-f5811bd4323fb6c5ed369c24"></a>
## new

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::new` · arrow-buffer 59.3.0

```rust
fn new(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Allocate a new [MutableBuffer](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) with initial capacity to be at least `capacity`.

See [`MutableBuffer::with_capacity`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-3a7b4a5c65c7b3ed5f3c4942).

# Panics

See [`MutableBuffer::with_capacity`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-3a7b4a5c65c7b3ed5f3c4942).

<a id="op-3eac369752237659ad18ed17"></a>
## new_null

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::new_null` · arrow-buffer 59.3.0

```rust
fn new_null(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

creates a new [MutableBuffer](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) with capacity and length capable of holding `len` bits.
This is useful to create a buffer for packed bitmaps.

# Panics

See [`MutableBuffer::from_len_zeroed`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-d6554b5089720e400767dded).

<a id="op-6cb8939ad5dae53225639f18"></a>
## push

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::push` · arrow-buffer 59.3.0

```rust
fn push<T: ToByteSlice>(&mut self, item: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:620`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Extends the buffer with a new item, increasing its capacity if needed.
# Example
```
# use arrow_buffer::buffer::MutableBuffer;
let mut buffer = MutableBuffer::new(0);
buffer.push(256u32);
assert_eq!(buffer.len(), 4) // u32 has 4 bytes
```

# Panics

Panics if extending the buffer requires reserving a capacity that fails for the same
reasons as [`MutableBuffer::reserve`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-18a37c4ca2f74d77b509cc10).

<a id="op-8cbfacfd8e4e575feba3f267"></a>
## push_unchecked

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::push_unchecked` · arrow-buffer 59.3.0

```rust
unsafe fn push_unchecked<T: ToByteSlice>(&mut self, item: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:635`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Extends the buffer with a new item, without checking for sufficient capacity
# Safety
Caller must ensure that the capacity()-len()>=`size_of<T>`()

<a id="op-3f6b9e306fee01491a76ec62"></a>
## repeat_slice_n_times

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::repeat_slice_n_times` · arrow-buffer 59.3.0

```rust
fn repeat_slice_n_times<T: ArrowNativeType>(&mut self, slice_to_repeat: &[T], repeat_count: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:311`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Adding to this mutable buffer `slice_to_repeat` repeated `repeat_count` times.

# Example

## Repeat the same string bytes multiple times
```
# use arrow_buffer::buffer::MutableBuffer;
let mut buffer = MutableBuffer::new(0);
let bytes_to_repeat = b"ab";
buffer.repeat_slice_n_times(bytes_to_repeat, 3);
assert_eq!(buffer.as_slice(), b"ababab");
```

# Panics

Panics if the repeated slice byte length overflows `usize`, if the resulting buffer
length overflows `usize`, or if reserving the required capacity fails for the same
reasons as [`MutableBuffer::reserve`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-18a37c4ca2f74d77b509cc10).

<a id="op-18a37c4ca2f74d77b509cc10"></a>
## reserve

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::reserve` · arrow-buffer 59.3.0

```rust
fn reserve(&mut self, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Ensures that this buffer has at least `self.len + additional` bytes. This re-allocates iff
`self.len + additional > capacity`.
# Example
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
let mut buffer = MutableBuffer::new(0);
buffer.reserve(253); // allocates for the first time
(0..253u8).for_each(|i| buffer.push(i)); // no reallocation
let buffer: Buffer = buffer.into();
assert_eq!(buffer.len(), 253);
```

# Panics

Panics if `self.len + additional` overflows `usize`, or if the required capacity is too
large to round up to the next 64-byte boundary and construct a valid allocation layout.

<a id="op-54e5aef3d0c0441e7d081e8a"></a>
## resize

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::resize` · arrow-buffer 59.3.0

```rust
fn resize(&mut self, new_len: usize, value: u8)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:436`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Resizes the buffer, either truncating its contents (with no change in capacity), or
growing it (potentially reallocating it) and writing `value` in the newly available bytes.
# Example
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
let mut buffer = MutableBuffer::new(0);
buffer.resize(253, 2); // allocates for the first time
assert_eq!(buffer.as_slice()[252], 2u8);
```

# Panics

Panics if growing the buffer requires reserving a capacity that fails for the same
reasons as [`MutableBuffer::reserve`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-18a37c4ca2f74d77b509cc10).

<a id="op-efe3de972a2df35f08aee960"></a>
## set_len

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::set_len` · arrow-buffer 59.3.0

```rust
unsafe fn set_len(&mut self, len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:665`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

# Safety
The caller must ensure that the buffer was properly initialized up to `len`.

# Panics

Panics if `len` exceeds the buffer capacity.

<a id="op-d07a7d41c04ac800036baf0c"></a>
## set_null_bits

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::set_null_bits` · arrow-buffer 59.3.0

```rust
fn set_null_bits(&mut self, start: usize, count: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:248`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Ensure that `count` bytes from `start` contain zero bits

This is used to initialize the bits in a buffer, however, it has no impact on the
`len` of the buffer and so can be used to initialize the memory region from
`len` to `capacity`.

# Panics

Panics if the byte range `start..start + count` exceeds the buffer capacity.

<a id="op-c3db2f9fbcca75abefe06175"></a>
## shrink_to_fit

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::shrink_to_fit` · arrow-buffer 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:473`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Shrinks the capacity of the buffer as much as possible.
The new capacity will aligned to the nearest 64 bit alignment.

# Example
```
# use arrow_buffer::buffer::{Buffer, MutableBuffer};
// 2 cache lines
let mut buffer = MutableBuffer::new(128);
assert_eq!(buffer.capacity(), 128);
buffer.push(1);
buffer.push(2);

buffer.shrink_to_fit();
assert!(buffer.capacity() >= 64 && buffer.capacity() < 128);
```

# Panics

Panics if the current length is too large to round up to the next 64-byte boundary and
construct a valid allocation layout.

<a id="op-8ddc062666e8c5f13c4c14d3"></a>
## truncate

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::truncate` · arrow-buffer 59.3.0

```rust
fn truncate(&mut self, len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:406`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Truncates this buffer to `len` bytes

If `len` is greater than the buffer's current length, this has no effect

<a id="op-9dd9f9cc31007da5d8292572"></a>
## try_from_trusted_len_iter

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::try_from_trusted_len_iter` · arrow-buffer 59.3.0

```rust
unsafe fn try_from_trusted_len_iter<E, T: ArrowNativeType, I: Iterator<Item = Result<T, E>>>(iterator: I) -> Result<Self, E>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [900, 1], "end": [1067, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:1030`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) from an [`Iterator`] with a trusted (upper) length or errors
if any of the items of the iterator is an error.
Prefer this to `collect` whenever possible, as it is faster ~60% faster.

# Panics

Panics if the iterator does not report an upper bound via `size_hint`, or if the
reported length does not match the number of items produced before an error-free finish,
or if allocating the required buffer fails for the same reasons as
[`MutableBuffer::new`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-f5811bd4323fb6c5ed369c24).

# Safety
This method assumes that the iterator's size is correct and is undefined behavior
to use it on an iterator that reports an incorrect length.

Unresolved upstream links (retained, not inferred): ``Iterator``.

<a id="op-e37013c4d072f16d8eed0b5e"></a>
## typed_data

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::typed_data` · arrow-buffer 59.3.0

```rust
fn typed_data<T: ArrowNativeType>(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:569`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

View buffer as a immutable slice of a specific type.

# Panics

This function panics if the underlying buffer is not aligned correctly for type `T`, or
if its length is not a multiple of `size_of::<T>()`.

<a id="op-dacbfb9541c99f877bf561d6"></a>
## typed_data_mut

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::typed_data_mut` · arrow-buffer 59.3.0

```rust
fn typed_data_mut<T: ArrowNativeType>(&mut self) -> &mut [T]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:554`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

View this buffer as a mutable slice of a specific type.

# Panics

This function panics if the underlying buffer is not aligned correctly for type `T`, or
if its length is not a multiple of `size_of::<T>()`.

<a id="op-211baba3a63e106a4d1ac880"></a>
## with_bitset

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::with_bitset` · arrow-buffer 59.3.0

```rust
fn with_bitset(self, end: usize, val: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Set the bits in the range of `[0, end)` to 0 (if `val` is false), or 1 (if `val`
is true). Also extend the length of this buffer to be `end`.

This is useful when one wants to clear (or set) the bits and then manipulate
the buffer directly (e.g., modifying the buffer by holding a mutable reference
from `data_mut()`).

# Panics

Panics if `end` exceeds the buffer capacity.

<a id="op-3a7b4a5c65c7b3ed5f3c4942"></a>
## with_capacity

`function` · `arrow_buffer::buffer::mutable::MutableBuffer::with_capacity` · arrow-buffer 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [850, 2], "filename": "src/buffer/mutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/mutable.rs:131`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Allocate a new [MutableBuffer](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) with initial capacity to be at least `capacity`.

# Panics

If `capacity`, when rounded up to the nearest multiple of [`ALIGNMENT`](../operations/arrow_buffer.alloc.alignment.ALIGNMENT.md#op-8f396fad8ebb4580d9689e9f), is greater
then `isize::MAX`, then this function will panic.
