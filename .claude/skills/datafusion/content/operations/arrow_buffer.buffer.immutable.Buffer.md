# `arrow_buffer::buffer::immutable::Buffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.immutable.Buffer.json).

<a id="op-f54755e677e7b3529b3edb8b"></a>
## Buffer

`struct` · `arrow_buffer::buffer::immutable::Buffer` · arrow-buffer 59.3.0

```rust
struct Buffer
```

Source: `src/buffer/immutable.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

 A contiguous memory region that can be shared with other buffers and across
 thread boundaries that stores Arrow data.

 `Buffer`s can be sliced and cloned without copying the underlying data and can
 be created from memory allocated by non-Rust sources such as C/C++.

 # Example: Create a `Buffer` from a `Vec` (without copying)
 ```
 # use arrow_buffer::Buffer;
 let vec: Vec<u32> = vec![1, 2, 3];
 let buffer = Buffer::from(vec);
 ```

 # Example: Convert a `Buffer` to a `Vec` (without copying)

 Use [`Self::into_vec`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-b8e45a0a04779bbbea258f48) to convert a `Buffer` back into a `Vec` if there are
 no other references and the types are aligned correctly.
 ```
 # use arrow_buffer::Buffer;
 # let vec: Vec<u32> = vec![1, 2, 3];
 # let buffer = Buffer::from(vec);
 // convert the buffer back into a Vec of u32
 // note this will fail if the buffer is shared or not aligned correctly
 let vec: Vec<u32> = buffer.into_vec().unwrap();
 ```

 # Example: Create a `Buffer` from a [`bytes::Bytes`] (without copying)

 [`bytes::Bytes`] is a common type in the Rust ecosystem for shared memory
 regions. You can create a buffer from a `Bytes` instance using the `From`
 implementation, also without copying.

 ```
 # use arrow_buffer::Buffer;
 let bytes = bytes::Bytes::from("hello");
 let buffer = Buffer::from(bytes);
```

Unresolved upstream links (retained, not inferred): ``bytes::Bytes``.

<a id="op-5bddde83f8959d806ac22235"></a>
## Target

`assoc_type` · `arrow_buffer::buffer::immutable::Buffer::Target` · arrow-buffer 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [547, 1], "end": [553, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/immutable.rs:548`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83cde9aa827dec7452908a49"></a>
## advance

`function` · `arrow_buffer::buffer::immutable::Buffer::advance` · arrow-buffer 59.3.0

```rust
fn advance(&mut self, offset: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:264`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Increases the offset of this buffer by `offset`

# Panics

Panics iff `offset` is larger than `len`.

<a id="op-787d906f2ca15bd141a52630"></a>
## as_ptr

`function` · `arrow_buffer::buffer::immutable::Buffer::as_ptr` · arrow-buffer 59.3.0

```rust
fn as_ptr(&self) -> *const u8
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:308`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a pointer to the start of this buffer.

Note that this should be used cautiously, and the returned pointer should not be
stored anywhere, to avoid dangling pointers.

<a id="op-3556d9c8864e509e308e477d"></a>
## as_slice

`function` · `arrow_buffer::buffer::immutable::Buffer::as_slice` · arrow-buffer 59.3.0

```rust
fn as_slice(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:236`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the byte slice stored in this buffer

<a id="op-0be42f5b9c8031f33a5aa5af"></a>
## bit_chunks

`function` · `arrow_buffer::buffer::immutable::Buffer::bit_chunks` · arrow-buffer 59.3.0

```rust
fn bit_chunks(&self, offset: usize, len: usize) -> BitChunks<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:351`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a `BitChunks` instance which can be used to iterate over this buffers bits
in larger chunks and starting at arbitrary bit offsets.
Note that both `offset` and `length` are measured in bits.

<a id="op-2532d8eb1c44b9281e815d26"></a>
## bit_slice

`function` · `arrow_buffer::buffer::immutable::Buffer::bit_slice` · arrow-buffer 59.3.0

```rust
fn bit_slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:330`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a slice of this buffer starting at a certain bit offset.
If the offset is byte-aligned the returned buffer is a shallow clone,
otherwise a new buffer is allocated and filled with a copy of the bits in the range.

<a id="op-8495b47a07ce65ac0dd98943"></a>
## capacity

`function` · `arrow_buffer::buffer::immutable::Buffer::capacity` · arrow-buffer 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:190`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the capacity of this buffer.
For externally owned buffers, this returns zero

<a id="op-3276c69adbc249530c026c94"></a>
## claim

`function` · `arrow_buffer::buffer::immutable::Buffer::claim` · arrow-buffer 59.3.0

```rust
fn claim(&self, pool: &dyn MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:472`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Register this [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) with the provided [`MemoryPool`](../operations/arrow_buffer.pool.MemoryPool.md#op-ed45519ea8493e8aed943b0a)

This claims the memory used by this buffer in the pool, allowing for
accurate accounting of memory usage. Any prior reservation will be
released so this works well when the buffer is being shared among
multiple arrays.

<a id="op-5e1c980124fd4269c000df2d"></a>
## clone

`function` · `arrow_buffer::buffer::immutable::Buffer::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 10], "end": [70, 15], "filename": "src/buffer/immutable.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/buffer/immutable.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f01810b7a5b13322211ef4f2"></a>
## count_set_bits_offset

`function` · `arrow_buffer::buffer::immutable::Buffer::count_set_bits_offset` · arrow-buffer 59.3.0

```rust
fn count_set_bits_offset(&self, offset: usize, len: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of 1-bits in this buffer, starting from `offset` with `length` bits
inspected. Note that both `offset` and `length` are measured in bits.

<a id="op-c16e73d66e5038f7ef8ed7d6"></a>
## data_ptr

`function` · `arrow_buffer::buffer::immutable::Buffer::data_ptr` · arrow-buffer 59.3.0

```rust
fn data_ptr(&self) -> NonNull<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the pointer to the start of the buffer without the offset.

<a id="op-223ab83d91c9503ccda0b760"></a>
## default

`function` · `arrow_buffer::buffer::immutable::Buffer::default` · arrow-buffer 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [92, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/buffer/immutable.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f506563377db26f86d3cc4a4"></a>
## deref

`function` · `arrow_buffer::buffer::immutable::Buffer::deref` · arrow-buffer 59.3.0

```rust
fn deref(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [547, 1], "end": [553, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/immutable.rs:550`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28b2b0119cc3299baea3944f"></a>
## eq

`function` · `arrow_buffer::buffer::immutable::Buffer::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [98, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/buffer/immutable.rs:95`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-957dbc2c9b9b9fe9d22b387d"></a>
## fmt

`function` · `arrow_buffer::buffer::immutable::Buffer::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 17], "end": [70, 22], "filename": "src/buffer/immutable.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer/immutable.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1acff22cc4259f14ea19b144"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(p: &[u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [485, 1], "end": [489, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:486`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b354992411d19e45e284fc5"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(bytes: bytes::Bytes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [530, 1], "end": [535, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:531`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-336b867ca012fb69501bcb0b"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: BufferBuilder<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [568, 1], "end": [572, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:569`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5066ab9bf57b149aa65a62be"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(buffer: MutableBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [561, 1], "end": [566, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:563`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-555002ee82049116a52d1b2f"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(builder: BooleanBufferBuilder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "crate::Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [350, 2], "filename": "src/builder/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/builder/boolean.rs:347`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ce5272f8e13198a6e336586"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: ScalarBuffer<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [509, 1], "end": [513, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:510`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fb0a25b9186bb8962ed4772"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(p: &[u8; N]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [501, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"array": {"len": "N", "type": {"primitive": "u8"}}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:498`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f0a2a30acf020597ac5c78b"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: Vec<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [507, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c48958fb73c62664da5de12"></a>
## from

`function` · `arrow_buffer::buffer::immutable::Buffer::from` · arrow-buffer 59.3.0

```rust
fn from(p: [u8; N]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [491, 1], "end": [495, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"array": {"len": "N", "type": {"primitive": "u8"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/immutable.rs:492`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50bd01bdcd0c36f67c52c7a3"></a>
## from_custom_allocation

`function` · `arrow_buffer::buffer::immutable::Buffer::from_custom_allocation` · arrow-buffer 59.3.0

```rust
unsafe fn from_custom_allocation(ptr: NonNull<u8>, len: usize, owner: Arc<dyn Allocation>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:158`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a buffer from an existing memory region.

Ownership of the memory is tracked via reference counting
and the memory will be freed using the `drop` method of
[crate::alloc::Allocation](../operations/arrow_buffer.alloc.Allocation.md#op-4d4e5ee0400b139a1788a1a4) when the reference count reaches zero.

# Arguments

* `ptr` - Pointer to raw parts
* `len` - Length of raw parts in **bytes**
* `owner` - A [crate::alloc::Allocation](../operations/arrow_buffer.alloc.Allocation.md#op-4d4e5ee0400b139a1788a1a4) which is responsible for freeing that data

# Safety

This function is unsafe as there is no guarantee that the given pointer is valid for `len` bytes

<a id="op-62834f635a18e105b1b9d3fc"></a>
## from_iter

`function` · `arrow_buffer::buffer::immutable::Buffer::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item = bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [545, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/buffer/immutable.rs:539`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cfc33591e26a24ab5391fc5"></a>
## from_iter

`function` · `arrow_buffer::buffer::immutable::Buffer::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [619, 1], "end": [624, 2], "filename": "src/buffer/immutable.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/buffer/immutable.rs:620`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-384bff5c2034a169c48b9619"></a>
## from_slice_ref

`function` · `arrow_buffer::buffer::immutable::Buffer::from_slice_ref` · arrow-buffer 59.3.0

```rust
fn from_slice_ref<U: ArrowNativeType, T: AsRef<[U]>>(items: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Initializes a [Buffer](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) from a slice of items.

<a id="op-f8e283486e39f39bde89ee39"></a>
## from_trusted_len_iter

`function` · `arrow_buffer::buffer::immutable::Buffer::from_trusted_len_iter` · arrow-buffer 59.3.0

```rust
unsafe fn from_trusted_len_iter<T: ArrowNativeType, I: Iterator<Item = T>>(iterator: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [574, 1], "end": [617, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:595`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) from an [`Iterator`] with a trusted (upper) length.

Prefer this to `collect` whenever possible, as it is ~60% faster.

# Example
```
# use arrow_buffer::buffer::Buffer;
let v = vec![1u32];
let iter = v.iter().map(|x| x * 2);
let buffer = unsafe { Buffer::from_trusted_len_iter(iter) };
assert_eq!(buffer.len(), 4) // u32 has 4 bytes
```
# Safety
This method assumes that the iterator's size is correct and is undefined behavior
to use it on an iterator that reports an incorrect length.

Unresolved upstream links (retained, not inferred): ``Iterator``.

<a id="op-c46590d5e0cbbb8c02a5d493"></a>
## from_vec

`function` · `arrow_buffer::buffer::immutable::Buffer::from_vec` · arrow-buffer 59.3.0

```rust
fn from_vec<T: ArrowNativeType>(vec: Vec<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) from the provided [`Vec`] without copying

Unresolved upstream links (retained, not inferred): ``Vec``.

<a id="op-b40a9aec389ac5ed0f9b3d96"></a>
## into_mutable

`function` · `arrow_buffer::buffer::immutable::Buffer::into_mutable` · arrow-buffer 59.3.0

```rust
fn into_mutable(self) -> Result<MutableBuffer, Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:383`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns `MutableBuffer` for mutating the buffer if this buffer is not shared or sliced.
Returns `Err` if this is shared or the [`Self::ptr_offset`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-5f15004632d08f07b518c8fc) is greater than 0 or its allocation is from an external source or
it is not allocated with alignment [`ALIGNMENT`]

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

[`ALIGNMENT`]: crate::alloc::ALIGNMENT

<a id="op-b8e45a0a04779bbbea258f48"></a>
## into_vec

`function` · `arrow_buffer::buffer::immutable::Buffer::into_vec` · arrow-buffer 59.3.0

```rust
fn into_vec<T: ArrowNativeType>(self) -> Result<Vec<T>, Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:422`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Converts self into a `Vec`, if possible.

This can be used to reuse / mutate the underlying data.

# Errors

Returns `Err(self)` if
1. The buffer does not have the same [`Layout`] as the destination Vec
2. The buffer contains a non-zero offset
3. The buffer is shared

Unresolved upstream links (retained, not inferred): ``Layout``.

<a id="op-27b39925f86494358fab2d5a"></a>
## is_empty

`function` · `arrow_buffer::buffer::immutable::Buffer::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if the buffer is empty.

<a id="op-bf70478301e98db1dea7fb67"></a>
## len

`function` · `arrow_buffer::buffer::immutable::Buffer::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:183`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of bytes in the buffer

<a id="op-4b4c7531fb9d6ace7fb9d16f"></a>
## ptr_eq

`function` · `arrow_buffer::buffer::immutable::Buffer::ptr_eq` · arrow-buffer 59.3.0

```rust
fn ptr_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:461`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if this [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) is equal to `other`, using pointer comparisons
to determine buffer equality. This is cheaper than `PartialEq::eq` but may
return false when the arrays are logically equal

<a id="op-5f15004632d08f07b518c8fc"></a>
## ptr_offset

`function` · `arrow_buffer::buffer::immutable::Buffer::ptr_offset` · arrow-buffer 59.3.0

```rust
fn ptr_offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the offset, in bytes, of `Self::ptr` to `Self::data`

self.ptr and self.data can be different after slicing or advancing the buffer.

<a id="op-f544ca949daf3bea0e76e63f"></a>
## shrink_to_fit

`function` · `arrow_buffer::buffer::immutable::Buffer::shrink_to_fit` · arrow-buffer 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:203`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Tries to shrink the capacity of the buffer as much as possible, freeing unused memory.

If the buffer is shared, this is a no-op.

If the memory was allocated with a custom allocator, this is a no-op.

If the capacity is already less than or equal to the desired capacity, this is a no-op.

The memory region will be reallocated using `std::alloc::realloc`.

<a id="op-5880bd863ef97c2ab6ec61eb"></a>
## slice

`function` · `arrow_buffer::buffer::immutable::Buffer::slice` · arrow-buffer 59.3.0

```rust
fn slice(&self, offset: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:252`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a new [Buffer](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) that is a slice of this buffer starting at `offset`.

This function is `O(1)` and does not copy any data, allowing the
same memory region to be shared between buffers.

# Panics

Panics iff `offset` is larger than `len`.

<a id="op-8f4e7d2a4f545e81280045ed"></a>
## slice_with_length

`function` · `arrow_buffer::buffer::immutable::Buffer::slice_with_length` · arrow-buffer 59.3.0

```rust
fn slice_with_length(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:287`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a new [Buffer](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) that is a slice of this buffer starting at `offset`,
with `length` bytes.

This function is `O(1)` and does not copy any data, allowing the same
memory region to be shared between buffers.

# Panics
Panics iff `(offset + length)` is larger than the existing length.

<a id="op-ad098e77bf32b9903b989d22"></a>
## strong_count

`function` · `arrow_buffer::buffer::immutable::Buffer::strong_count` · arrow-buffer 59.3.0

```rust
fn strong_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:124`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of strong references to the buffer.

This method is safe but if the buffer is shared across multiple threads
the underlying value could change between calling this method and using
the result.

<a id="op-7c141251c5eace73db145aa0"></a>
## try_from_trusted_len_iter

`function` · `arrow_buffer::buffer::immutable::Buffer::try_from_trusted_len_iter` · arrow-buffer 59.3.0

```rust
unsafe fn try_from_trusted_len_iter<E, T: ArrowNativeType, I: Iterator<Item = Result<T, E>>>(iterator: I) -> Result<Self, E>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [574, 1], "end": [617, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:608`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) from an [`Iterator`] with a trusted (upper) length or errors
if any of the items of the iterator is an error.
Prefer this to `collect` whenever possible, as it is ~60% faster.
# Safety
This method assumes that the iterator's size is correct and is undefined behavior
to use it on an iterator that reports an incorrect length.

Unresolved upstream links (retained, not inferred): ``Iterator``.

<a id="op-b7547a4422f011559df26aec"></a>
## typed_data

`function` · `arrow_buffer::buffer::immutable::Buffer::typed_data` · arrow-buffer 59.3.0

```rust
fn typed_data<T: ArrowNativeType>(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [475, 2], "filename": "src/buffer/immutable.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/immutable.rs:318`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

View buffer as a slice of a specific type.

# Panics

This function panics if the underlying buffer is not aligned
correctly for type `T`.
