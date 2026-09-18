# `arrow_buffer::buffer::scalar::ScalarBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.scalar.ScalarBuffer.json).

<a id="op-3edbfa8eafafb64c0414f247"></a>
## ScalarBuffer

`struct` · `arrow_buffer::buffer::scalar::ScalarBuffer` · arrow-buffer 59.3.0

```rust
struct ScalarBuffer<T: ArrowNativeType>
```

Source: `src/buffer/scalar.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A strongly-typed [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) supporting zero-copy cloning and slicing

The easiest way to think about `ScalarBuffer<T>` is being equivalent to a `Arc<Vec<T>>`,
with the following differences:

- slicing and cloning is O(1).
- support for external allocated memory (e.g. via FFI).

See [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) for more low-level memory management details.

# Example: Convert to/from Vec (without copies)

(See [`Buffer::from_vec`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-c46590d5e0cbbb8c02a5d493) and [`Buffer::into_vec`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-b8e45a0a04779bbbea258f48) for a lower level API)
```
# use arrow_buffer::ScalarBuffer;
// Zero-copy conversion from Vec
let buffer = ScalarBuffer::from(vec![1, 2, 3]);
assert_eq!(&buffer, &[1, 2, 3]);
// convert the buffer back to Vec without copy assuming:
// 1. the inner buffer is not sliced
// 2. the inner buffer uses standard allocation
// 3. there are no other references to the inner buffer
let vec: Vec<i32> = buffer.into();
assert_eq!(&vec, &[1, 2, 3]);
```

# Example: Zero copy slicing
```
# use arrow_buffer::ScalarBuffer;
let buffer = ScalarBuffer::from(vec![1, 2, 3]);
assert_eq!(&buffer, &[1, 2, 3]);
// Zero-copy slicing
let sliced = buffer.slice(1, 2);
assert_eq!(&sliced, &[2, 3]);
// Original buffer is unchanged
assert_eq!(&buffer, &[1, 2, 3]);
// converting the sliced buffer back to Vec incurs a copy
let vec: Vec<i32> = sliced.into();
```

<a id="op-6cc1aefba4f3bc7eaf6cc509"></a>
## Target

`assoc_type` · `arrow_buffer::buffer::scalar::ScalarBuffer::Target` · arrow-buffer 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [169, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/scalar.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c29e48650e75ff383f4581f3"></a>
## as_ref

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::as_ref` · arrow-buffer 59.3.0

```rust
fn as_ref(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [176, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"generic": "T"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/buffer/scalar.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-255c999891cae5839634ff7c"></a>
## claim

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::claim` · arrow-buffer 59.3.0

```rust
fn claim(&self, pool: &dyn MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:133`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Claim memory used by this buffer in the provided memory pool.

See [`Buffer::claim`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-3276c69adbc249530c026c94) for details.

<a id="op-a3e4d84831565ddc931739f5"></a>
## clone

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> ScalarBuffer<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/buffer/scalar.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/buffer/scalar.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-185992de61dab537e68c86af"></a>
## default

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::default` · arrow-buffer 59.3.0

```rust
fn default() -> ScalarBuffer<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "$crate::default::Default"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 17], "end": [65, 24], "filename": "src/buffer/scalar.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/buffer/scalar.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92fd7126ba14e1def4d3c068"></a>
## deref

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::deref` · arrow-buffer 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [169, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/scalar.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ede45090a18ba4e4c63877d7"></a>
## eq

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &S) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"generic": "T"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [258, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/buffer/scalar.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9669325b258f57029ea3daa0"></a>
## fmt

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [76, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer/scalar.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c8296691b591611b7bbbdd6"></a>
## from

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(buffer: Buffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [184, 1], "end": [205, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::immutable::Buffer", "path": "Buffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/scalar.rs:185`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20213e9a59ae9ae1c6cb596f"></a>
## from

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: BufferBuilder<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [231, 1], "end": [236, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::builder::BufferBuilder", "path": "BufferBuilder"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/scalar.rs:232`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a786be2d0a4d0e590660744"></a>
## from

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: OffsetBuffer<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 1], "end": [211, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/scalar.rs:208`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a36a4170b0b06d58fca5393"></a>
## from

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: MutableBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [182, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::mutable::MutableBuffer", "path": "MutableBuffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/scalar.rs:179`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88442092306e6922ccb1f406"></a>
## from

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: Vec<T>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 1], "end": [220, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/scalar.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff8d0cb828fd803dba140629"></a>
## from_iter

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [238, 1], "end": [243, 2], "filename": "src/buffer/scalar.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/buffer/scalar.rs:240`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5d5765a4c92d8d7a94c03c0"></a>
## inner

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::inner` · arrow-buffer 59.3.0

```rust
fn inner(&self) -> &Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)

<a id="op-8ece75e4a282d2e7464e343c"></a>
## into_inner

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::into_inner` · arrow-buffer 59.3.0

```rust
fn into_inner(self) -> Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:125`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b), consuming self

<a id="op-7576636826bcd432d0d8771c"></a>
## is_empty

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns if the buffer is empty

<a id="op-e537f8b5e8fbc059da75240a"></a>
## len

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:146`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of elements in the buffer

<a id="op-20aae51b804a82b051c1ab27"></a>
## new

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: Buffer, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:89`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247) from a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b), and an `offset`
and `length` in units of `T`

# Panics

This method will panic if

* `offset` or `len` would result in overflow
* `buffer` is not aligned to a multiple of `std::mem::align_of::<T>`
* `bytes` is not large enough for the requested slice

<a id="op-f7724814fe50c0e805034b8f"></a>
## new_unchecked

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::new_unchecked` · arrow-buffer 59.3.0

```rust
unsafe fn new_unchecked(buffer: Buffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:102`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Unsafe function to create a new [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247) from a [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b).
Only use for testing purpose.

# Safety

This function is unsafe because it does not check if the `buffer` is aligned

<a id="op-750682a68624f10e5736b491"></a>
## ptr_eq

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::ptr_eq` · arrow-buffer 59.3.0

```rust
fn ptr_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:141`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if this [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247) is equal to `other`, using pointer comparisons
to determine buffer equality. This is cheaper than `PartialEq::eq` but may
return false when the arrays are logically equal

<a id="op-a5f4682c5160a4a22d207270"></a>
## shrink_to_fit

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::shrink_to_fit` · arrow-buffer 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:110`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Free up unused memory.

<a id="op-74b1401d59d376fac24f2e27"></a>
## slice

`function` · `arrow_buffer::buffer::scalar::ScalarBuffer::slice` · arrow-buffer 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::scalar::ScalarBuffer", "path": "ScalarBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [154, 2], "filename": "src/buffer/scalar.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/scalar.rs:115`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a zero-copy slice of this buffer with length `len` and starting at `offset`
