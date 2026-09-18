# `arrow_buffer::buffer::offset::OffsetBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.offset.OffsetBuffer.json).

<a id="op-a91a6a87515fb5ae378e13fc"></a>
## OffsetBuffer

`struct` · `arrow_buffer::buffer::offset::OffsetBuffer` · arrow-buffer 59.3.0

```rust
struct OffsetBuffer<O: ArrowNativeType>
```

Source: `src/buffer/offset.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A non-empty buffer of monotonically increasing, positive integers.

[`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) are used to represent ranges of offsets. An
`OffsetBuffer` of `N+1` items contains `N` such ranges. The start
offset for element `i` is `offsets[i]` and the end offset is
`offsets[i+1]`. Equal offsets represent an empty range.

# Example

This example shows how 5 distinct ranges, are represented using a
6 entry `OffsetBuffer`. The first entry `(0, 3)` represents the
three offsets `0, 1, 2`. The entry `(3,3)` represent no offsets
(e.g. an empty list).

```text
  ┌───────┐                ┌───┐
  │ (0,3) │                │ 0 │
  ├───────┤                ├───┤
  │ (3,3) │                │ 3 │
  ├───────┤                ├───┤
  │ (3,4) │                │ 3 │
  ├───────┤                ├───┤
  │ (4,5) │                │ 4 │
  ├───────┤                ├───┤
  │ (5,7) │                │ 5 │
  └───────┘                ├───┤
                           │ 7 │
                           └───┘

                       Offsets Buffer
   Logical
   Offsets

 (offsets[i],
  offsets[i+1])
```

<a id="op-ab9ea7dd53fabc5f3a4ccabe"></a>
## Target

`assoc_type` · `arrow_buffer::buffer::offset::OffsetBuffer::Target` · arrow-buffer 59.3.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [401, 2], "filename": "src/buffer/offset.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/offset.rs:395`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e393f5b4c6622d19b4d31f0"></a>
## as_ref

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::as_ref` · arrow-buffer 59.3.0

```rust
fn as_ref(&self) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [403, 1], "end": [408, 2], "filename": "src/buffer/offset.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"generic": "T"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/buffer/offset.rs:405`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d2cb9cc6834c613a7688620"></a>
## claim

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::claim` · arrow-buffer 59.3.0

```rust
fn claim(&self, pool: &dyn MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Claim memory used by this buffer in the provided memory pool.

<a id="op-f1c7589d99105a6ea330f635"></a>
## clone

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> OffsetBuffer<O>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 17], "end": [58, 22], "filename": "src/buffer/offset.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/buffer/offset.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a31925b6d4fbd673427c2d4"></a>
## default

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::default` · arrow-buffer 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [416, 1], "end": [420, 2], "filename": "src/buffer/offset.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/buffer/offset.rs:417`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a7ae44853d89142d91a6b56"></a>
## deref

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::deref` · arrow-buffer 59.3.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [394, 1], "end": [401, 2], "filename": "src/buffer/offset.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/buffer/offset.rs:398`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68a3fa3621f2a2f2766dfe0c"></a>
## eq

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &OffsetBuffer<O>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 24], "end": [58, 33], "filename": "src/buffer/offset.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/buffer/offset.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9870f6b3bbf455c6159f6d82"></a>
## fmt

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/buffer/offset.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer/offset.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7ec619c9829ce200a96faaf"></a>
## from

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::from` · arrow-buffer 59.3.0

```rust
fn from(value: OffsetBufferBuilder<O>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [410, 1], "end": [414, 2], "filename": "src/buffer/offset.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::builder::offset::OffsetBufferBuilder", "path": "OffsetBufferBuilder"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/buffer/offset.rs:411`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5760d0f503b26092e0308693"></a>
## from_lengths

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::from_lengths` · arrow-buffer 59.3.0

```rust
fn from_lengths<I>(lengths: I) -> Self where I: IntoIterator<Item = usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:121`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) from the iterator of slice lengths

```
# use arrow_buffer::OffsetBuffer;
let offsets = OffsetBuffer::<i32>::from_lengths([1, 3, 5]);
assert_eq!(offsets.as_ref(), &[0, 1, 4, 9]);
```

If you want to create an [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) where all lengths are the same,
consider using the faster [`OffsetBuffer::from_repeated_length`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-daa862f460a34296d55459d0) instead.

# Panics

Panics on overflow

<a id="op-daa862f460a34296d55459d0"></a>
## from_repeated_length

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::from_repeated_length` · arrow-buffer 59.3.0

```rust
fn from_repeated_length(length: usize, n: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) where each slice has the same length
`length`, repeated `n` times.


Example
```
# use arrow_buffer::OffsetBuffer;
let offsets = OffsetBuffer::<i32>::from_repeated_length(4, 3);
assert_eq!(offsets.as_ref(), &[0, 4, 8, 12]);
```

# Panics

Panics on overflow

<a id="op-4e17f092d36ccde60c2df460"></a>
## has_non_empty_nulls

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::has_non_empty_nulls` · arrow-buffer 59.3.0

```rust
fn has_non_empty_nulls(&self, null_buffer: Option<&NullBuffer>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Check if any null positions in the `null_buffer` correspond to
non-empty ranges in this [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc).

In variable-length array types (e.g., `StringArray`, `ListArray`),
null entries may or may not have empty offset ranges. This method
detects cases where a null entry has a non-empty range
(i.e., `offsets[i] != offsets[i+1]`), which means the underlying
data buffer contains data behind nulls.

This matters because unwrapping (flattening) a list array exposes
the child values, including those behind null entries. If null
entries point to non-empty ranges, the unwrapped values will
contain data that may not be meaningful to operate on and could
cause errors (e.g., division by zero in the child values).

Returns `false` if `null_buffer` is `None` or contains no nulls.

# Example

```
# use arrow_buffer::{OffsetBuffer, ScalarBuffer, NullBuffer};
// Offsets where null at index 1 has an empty range (3..3)
let offsets = OffsetBuffer::new(ScalarBuffer::<i32>::from(vec![0, 3, 3, 6]));
let nulls = NullBuffer::from(vec![true, false, true]);
assert!(!offsets.has_non_empty_nulls(Some(&nulls)));

// Offsets where null at index 1 has a non-empty range (3..7)
let offsets = OffsetBuffer::new(ScalarBuffer::<i32>::from(vec![0, 3, 7, 10]));
let nulls = NullBuffer::from(vec![true, false, true]);
assert!(offsets.has_non_empty_nulls(Some(&nulls)));
```

# Panics

Panics if the length of the `null_buffer` does not equal `self.len() - 1`.

<a id="op-1dbde088d841096c589245e0"></a>
## inner

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::inner` · arrow-buffer 59.3.0

```rust
fn inner(&self) -> &ScalarBuffer<O>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:214`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247)

<a id="op-974b6dc77f6e550c9c6b6538"></a>
## into_inner

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::into_inner` · arrow-buffer 59.3.0

```rust
fn into_inner(self) -> ScalarBuffer<O>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247), consuming self

<a id="op-067520535f59d3cd137880cc"></a>
## lengths

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::lengths` · arrow-buffer 59.3.0

```rust
fn lengths(&self) -> impl ExactSizeIterator<Item = usize> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Get an Iterator over the lengths of this [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc)

```
# use arrow_buffer::{OffsetBuffer, ScalarBuffer};
let offsets = OffsetBuffer::<_>::new(ScalarBuffer::<i32>::from(vec![0, 1, 4, 9]));
assert_eq!(offsets.lengths().collect::<Vec<usize>>(), vec![1, 3, 5]);
```

Empty [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) will return an empty iterator
```
# use arrow_buffer::OffsetBuffer;
let offsets = OffsetBuffer::<i32>::new_empty();
assert_eq!(offsets.lengths().count(), 0);
```

This can be used to merge multiple [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc)s to one
```
# use arrow_buffer::{OffsetBuffer, ScalarBuffer};

let buffer1 = OffsetBuffer::<i32>::from_lengths([2, 6, 3, 7, 2]);
let buffer2 = OffsetBuffer::<i32>::from_lengths([1, 3, 5, 7, 9]);

let merged = OffsetBuffer::<i32>::from_lengths(
    vec![buffer1, buffer2].iter().flat_map(|x| x.lengths())
);

assert_eq!(merged.lengths().collect::<Vec<_>>(), &[2, 6, 3, 7, 2, 1, 3, 5, 7, 9]);
```

<a id="op-193f8950b56c9e61af59d3e1"></a>
## new

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: ScalarBuffer<O>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) from the provided [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247)

# Panics

Panics if `buffer` is not a non-empty buffer containing
monotonically increasing values greater than or equal to zero

<a id="op-47481544c19eef1b39861506"></a>
## new_empty

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::new_empty` · arrow-buffer 59.3.0

```rust
fn new_empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) containing a single 0 value

<a id="op-3e5da89f279726f5682ad3bd"></a>
## new_unchecked

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::new_unchecked` · arrow-buffer 59.3.0

```rust
unsafe fn new_unchecked(buffer: ScalarBuffer<O>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) from the provided [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247)

# Safety

`buffer` must be a non-empty buffer containing monotonically increasing
values greater than or equal to zero

<a id="op-6763861b4f9d7723c1335331"></a>
## new_zeroed

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::new_zeroed` · arrow-buffer 59.3.0

```rust
fn new_zeroed(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) containing `len + 1` `0` values

<a id="op-431e1a45f8db8983518c67aa"></a>
## ptr_eq

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::ptr_eq` · arrow-buffer 59.3.0

```rust
fn ptr_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:238`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if this [`OffsetBuffer`](../operations/arrow_buffer.buffer.offset.OffsetBuffer.md#op-a91a6a87515fb5ae378e13fc) is equal to `other`, using pointer comparisons
to determine buffer equality. This is cheaper than `PartialEq::eq` but may
return false when the arrays are logically equal

<a id="op-c3c46f18dc52a1b375dfd123"></a>
## shrink_to_fit

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::shrink_to_fit` · arrow-buffer 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Free up unused memory.

<a id="op-32c48fb03724df8913828d7d"></a>
## slice

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::slice` · arrow-buffer 59.3.0

```rust
fn slice(&self, offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:230`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns a zero-copy slice of this buffer with length `len` and starting at `offset`

<a id="op-cbd03e240166ff405c7efea6"></a>
## subtract

`function` · `arrow_buffer::buffer::offset::OffsetBuffer::subtract` · arrow-buffer 59.3.0

```rust
fn subtract(self, rhs: O) -> Self where O: std::ops::Sub<Output = O> + std::cmp::PartialOrd + num_traits::CheckedSub
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "arrow_buffer::buffer::offset::OffsetBuffer", "path": "OffsetBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [392, 2], "filename": "src/buffer/offset.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/offset.rs:349`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Subtract `rhs` from all offsets
This will try to reuse the existing allocation as much as possible

Panics: this will panic if `rhs` > the first offset or if `rhs` will lead to overflow (when `rhs` is negative)

# Example

```
# use arrow_buffer::OffsetBuffer;
let offsets = OffsetBuffer::<i32>::from_lengths(vec![4, 1, 5, 6]);
assert_eq!(offsets.as_ref(), &[0, 4, 5, 10, 16]);

let sliced_offsets = offsets.slice(1, 2);
assert_eq!(sliced_offsets.as_ref(), &[4, 5, 10]);

let shifted_offsets = sliced_offsets.subtract(4);
assert_eq!(shifted_offsets.as_ref(), &[0, 1, 6]);
```

