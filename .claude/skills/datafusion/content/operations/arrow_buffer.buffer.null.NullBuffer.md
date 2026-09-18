# `arrow_buffer::buffer::null::NullBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.null.NullBuffer.json).

<a id="op-d8209d291b86c5deb0050e48"></a>
## NullBuffer

`struct` · `arrow_buffer::buffer::null::NullBuffer` · arrow-buffer 59.3.0

```rust
struct NullBuffer
```

Source: `src/buffer/null.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) used to encode validity (null values) for Arrow arrays

In the [Arrow specification], array validity is encoded in a packed bitmask with a
`true` value indicating the corresponding slot is not null, and `false` indicating
that it is null.

# See also
* [`NullBufferBuilder`] for creating `NullBuffer`s

[Arrow specification]: https://arrow.apache.org/docs/format/Columnar.html#validity-bitmaps
[`NullBufferBuilder`]: crate::NullBufferBuilder

<a id="op-65d338d227353f725c43fd17"></a>
## buffer

`function` · `arrow_buffer::buffer::null::NullBuffer::buffer` · arrow-buffer 59.3.0

```rust
fn buffer(&self) -> &Buffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:243`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the underlying [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)

<a id="op-08efef37c9cedb3222779a49"></a>
## claim

`function` · `arrow_buffer::buffer::null::NullBuffer::claim` · arrow-buffer 59.3.0

```rust
fn claim(&self, pool: &dyn MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:258`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Claim memory used by this null buffer in the provided memory pool.

<a id="op-2a90d2c33e4a2872ec618ee5"></a>
## clone

`function` · `arrow_buffer::buffer::null::NullBuffer::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> NullBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 22], "filename": "src/buffer/null.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/buffer/null.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-051ba94777607a0c694dde69"></a>
## contains

`function` · `arrow_buffer::buffer::null::NullBuffer::contains` · arrow-buffer 59.3.0

```rust
fn contains(&self, other: &NullBuffer) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if all nulls in `other` also exist in self

<a id="op-023ac9934ecd31040a5d80ab"></a>
## eq

`function` · `arrow_buffer::buffer::null::NullBuffer::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &NullBuffer) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 28], "end": [33, 37], "filename": "src/buffer/null.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/buffer/null.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f027e2b99882f5cb785c0524"></a>
## expand

`function` · `arrow_buffer::buffer::null::NullBuffer::expand` · arrow-buffer 59.3.0

```rust
fn expand(&self, count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a new [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) where each bit in the current null buffer
is repeated `count` times. This is useful for masking the nulls of
the child of a FixedSizeListArray based on its parent

<a id="op-78440db50eebe27ea2fbb60d"></a>
## fmt

`function` · `arrow_buffer::buffer::null::NullBuffer::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/buffer/null.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer/null.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ecd7035cfb45e71c8040a8a"></a>
## from

`function` · `arrow_buffer::buffer::null::NullBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: BooleanBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [277, 2], "filename": "src/buffer/null.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::boolean::BooleanBuffer", "path": "BooleanBuffer"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/null.rs:274`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-492a63429f82b5ec37679096"></a>
## from

`function` · `arrow_buffer::buffer::null::NullBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: &[bool]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [283, 2], "filename": "src/buffer/null.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "bool"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/null.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7962fee1fc2266375b86f4f8"></a>
## from

`function` · `arrow_buffer::buffer::null::NullBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: &[bool; N]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [{"kind": {"const": {"default": null, "type": {"primitive": "usize"}}}, "name": "N"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [285, 1], "end": [289, 2], "filename": "src/buffer/null.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"array": {"len": "N", "type": {"primitive": "bool"}}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/null.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d90255b3a8d0a7b387f7d39"></a>
## from

`function` · `arrow_buffer::buffer::null::NullBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: Vec<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [295, 2], "filename": "src/buffer/null.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/null.rs:292`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d745f2160c1ef28a5e41c3d0"></a>
## from

`function` · `arrow_buffer::buffer::null::NullBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(builder: BooleanBufferBuilder) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "crate::NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [365, 2], "filename": "src/builder/boolean.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/builder/boolean.rs:361`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e67b47e53686f67f992ce6d9"></a>
## from_iter

`function` · `arrow_buffer::buffer::null::NullBuffer::from_iter` · arrow-buffer 59.3.0

```rust
fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [297, 1], "end": [301, 2], "filename": "src/buffer/null.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/buffer/null.rs:298`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4ca6332548929000ee5dead"></a>
## from_unsliced_buffer

`function` · `arrow_buffer::buffer::null::NullBuffer::from_unsliced_buffer` · arrow-buffer 59.3.0

```rust
fn from_unsliced_buffer(buffer: impl Into<Buffer>, len: usize) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:250`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) from an *unsliced* validity bitmap (`offset = 0` **bits**) of length `len`.

Returns `None` if there are no nulls (all values valid).

<a id="op-db53eb1e29ecb9db2b63bf4b"></a>
## inner

`function` · `arrow_buffer::buffer::null::NullBuffer::inner` · arrow-buffer 59.3.0

```rust
fn inner(&self) -> &BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

<a id="op-fc9be8c4ae2e5698298e16d9"></a>
## into_inner

`function` · `arrow_buffer::buffer::null::NullBuffer::into_inner` · arrow-buffer 59.3.0

```rust
fn into_inner(self) -> BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:237`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

<a id="op-f6a06437506fa0857a7ad321"></a>
## is_empty

`function` · `arrow_buffer::buffer::null::NullBuffer::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:155`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) is empty

<a id="op-9b61cf641d0b60d57a8bca7a"></a>
## is_null

`function` · `arrow_buffer::buffer::null::NullBuffer::is_null` · arrow-buffer 59.3.0

```rust
fn is_null(&self, idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:178`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns `true` if the value at `idx` is null

<a id="op-9433be330e3807dfc5937a8a"></a>
## is_valid

`function` · `arrow_buffer::buffer::null::NullBuffer::is_valid` · arrow-buffer 59.3.0

```rust
fn is_valid(&self, idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:172`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns `true` if the value at `idx` is not null

<a id="op-5db878658ded638716992330"></a>
## iter

`function` · `arrow_buffer::buffer::null::NullBuffer::iter` · arrow-buffer 59.3.0

```rust
fn iter(&self) -> BitIterator<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns an iterator over the bits in this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48)

* `true` indicates that the corresponding value is not NULL
* `false` indicates that the corresponding value is NULL

Note: [`Self::valid_indices`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-7580d841bd9224f99ec5641e) will be significantly faster for most use-cases

<a id="op-4134c10a474e04ef31817b08"></a>
## len

`function` · `arrow_buffer::buffer::null::NullBuffer::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the length of this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) in bits

<a id="op-2d73dfcfb75e39904fab2a71"></a>
## new

`function` · `arrow_buffer::buffer::null::NullBuffer::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: BooleanBuffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) computing the null count

<a id="op-76efd66e814409d0285ffdf8"></a>
## new_null

`function` · `arrow_buffer::buffer::null::NullBuffer::new_null` · arrow-buffer 59.3.0

```rust
fn new_null(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) of length `len` where all values are null

<a id="op-36d842c7679ca904b86046c8"></a>
## new_unchecked

`function` · `arrow_buffer::buffer::null::NullBuffer::new_unchecked` · arrow-buffer 59.3.0

```rust
unsafe fn new_unchecked(buffer: BooleanBuffer, null_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) with the provided `buffer` and `null_count`

# Safety

`buffer` must contain `null_count` `0` bits

<a id="op-8a26b993b139f1fab39c4421"></a>
## new_valid

`function` · `arrow_buffer::buffer::null::NullBuffer::new_valid` · arrow-buffer 59.3.0

```rust
fn new_valid(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) of length `len` where all values are valid

Note: it is more efficient to not set the null buffer if it is known to
be all valid (aka all values are not null)

<a id="op-fe03d0144aa5595376385962"></a>
## null_count

`function` · `arrow_buffer::buffer::null::NullBuffer::null_count` · arrow-buffer 59.3.0

```rust
fn null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:166`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the null count for this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48)

<a id="op-51bfa1dfdb2a39293f18dec2"></a>
## offset

`function` · `arrow_buffer::buffer::null::NullBuffer::offset` · arrow-buffer 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the offset of this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) in bits

<a id="op-4ddc576f2ffb25b384c5c6c5"></a>
## shrink_to_fit

`function` · `arrow_buffer::buffer::null::NullBuffer::shrink_to_fit` · arrow-buffer 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Free up unused memory.

<a id="op-e0d33cec7c0340606091637b"></a>
## slice

`function` · `arrow_buffer::buffer::null::NullBuffer::slice` · arrow-buffer 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:189`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Slices this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) by the provided `offset` and `length`

<a id="op-f2c7b3fc7fb1cb6c171bf749"></a>
## try_for_each_valid_idx

`function` · `arrow_buffer::buffer::null::NullBuffer::try_for_each_valid_idx` · arrow-buffer 59.3.0

```rust
fn try_for_each_valid_idx<E, F: FnMut(usize) -> Result<(), E>>(&self, f: F) -> Result<(), E>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Calls the provided closure for each index in this null mask that is set

<a id="op-39e2ef9e40683d4b3905f71f"></a>
## union

`function` · `arrow_buffer::buffer::null::NullBuffer::union` · arrow-buffer 59.3.0

```rust
fn union(lhs: Option<&NullBuffer>, rhs: Option<&NullBuffer>) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Computes the union of the nulls in two optional [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48)

This is commonly used by binary operations where the result is NULL if either
of the input values is NULL. Handling the null mask separately in this way
can yield significant performance improvements over an iterator approach

<a id="op-d08ea030a4c48ac108ab7f1b"></a>
## union_many

`function` · `arrow_buffer::buffer::null::NullBuffer::union_many` · arrow-buffer 59.3.0

```rust
fn union_many<'a>(nulls: impl IntoIterator<Item = Option<&'a NullBuffer>>) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Computes the union of the nulls in multiple optional [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48)s

See [`union`](Self::union)

<a id="op-7580d841bd9224f99ec5641e"></a>
## valid_indices

`function` · `arrow_buffer::buffer::null::NullBuffer::valid_indices` · arrow-buffer 59.3.0

```rust
fn valid_indices(&self) -> BitIndexIterator<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:206`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a [`BitIndexIterator`](../operations/arrow_buffer.util.bit_iterator.BitIndexIterator.md#op-c2ca62d89850047c97347f85) over the valid indices in this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48)

Valid indices indicate the corresponding value is not NULL

<a id="op-ac4b049e7b8df8754ca1d963"></a>
## valid_slices

`function` · `arrow_buffer::buffer::null::NullBuffer::valid_slices` · arrow-buffer 59.3.0

```rust
fn valid_slices(&self) -> BitSliceIterator<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:213`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a [`BitSliceIterator`](../operations/arrow_buffer.util.bit_iterator.BitSliceIterator.md#op-c825a8467c523296ef7280a9) yielding contiguous ranges of valid indices

Valid indices indicate the corresponding value is not NULL

<a id="op-82ca4732196c04b7a3070985"></a>
## validity

`function` · `arrow_buffer::buffer::null::NullBuffer::validity` · arrow-buffer 59.3.0

```rust
fn validity(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::buffer::null::NullBuffer", "path": "NullBuffer"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [262, 2], "filename": "src/buffer/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/null.rs:184`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the packed validity of this [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) not including any offset
