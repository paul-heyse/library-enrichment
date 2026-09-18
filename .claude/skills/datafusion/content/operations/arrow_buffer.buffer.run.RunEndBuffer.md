# `arrow_buffer::buffer::run::RunEndBuffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.buffer.run.RunEndBuffer.json).

<a id="op-e35311e9ed6d50fb4e98aafc"></a>
## RunEndBuffer

`struct` · `arrow_buffer::buffer::run::RunEndBuffer` · arrow-buffer 59.3.0

```rust
struct RunEndBuffer<E: ArrowNativeType>
```

Source: `src/buffer/run.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A buffer of monotonically increasing, positive integers used to store run-ends.

Used to compactly represent runs of the same value. Values being represented
are stored in a separate buffer from this struct. See [`RunArray`] for an example
of how this is used with a companion array to represent the values.

# Logical vs Physical

Physically, each value in the `run_ends` buffer is the cumulative length of
all runs in the logical representation, up to that physical index. Consider
the following example:

```text
          physical                        logical
    ┌─────────┬─────────┐           ┌─────────┬─────────┐
    │    3    │    0    │ ◄──────┬─ │    A    │    0    │
    ├─────────┼─────────┤        │  ├─────────┼─────────┤
    │    4    │    1    │ ◄────┐ ├─ │    A    │    1    │
    ├─────────┼─────────┤      │ │  ├─────────┼─────────┤
    │    6    │    2    │ ◄──┐ │ └─ │    A    │    2    │
    └─────────┴─────────┘    │ │    ├─────────┼─────────┤
     run-ends    index       │ └─── │    B    │    3    │
                             │      ├─────────┼─────────┤
     logical_offset = 0      ├───── │    C    │    4    │
     logical_length = 6      │      ├─────────┼─────────┤
                             └───── │    C    │    5    │
                                    └─────────┴─────────┘
                                      values     index
```

A [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc) is physically the buffer and offset with length on the left.
In this case, the offset and length represent the whole buffer, so it is essentially
unsliced. See the section below on slicing for more details on how this buffer
handles slicing.

This means that multiple logical values are represented in the same physical index,
and multiple logical indices map to the same physical index. The [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc)
containing `[3, 4, 6]` is essentially the physical indices `[0, 0, 0, 1, 2, 2]`,
and having a separately stored buffer of values such as `[A, B, C]` can turn
this into a representation of `[A, A, A, B, C, C]`.

# Slicing

In order to provide zero-copy slicing, this struct stores a separate **logical**
offset and length. Consider the following example:

```text
          physical                        logical
    ┌─────────┬─────────┐           ┌ ─ ─ ─ ─ ┬ ─ ─ ─ ─ ┐
    │    3    │    0    │ ◄──────┐       A         0
    ├─────────┼─────────┤        │  ├── ─ ─ ─ ┼ ─ ─ ─ ─ ┤
    │    4    │    1    │ ◄────┐ │       A         1
    ├─────────┼─────────┤      │ │  ├─────────┼─────────┤
    │    6    │    2    │ ◄──┐ │ └─ │    A    │    2    │◄─── logical_offset
    └─────────┴─────────┘    │ │    ├─────────┼─────────┤
     run-ends    index       │ └─── │    B    │    3    │
                             │      ├─────────┼─────────┤
     logical_offset = 2      └───── │    C    │    4    │
     logical_length = 3             ├─────────┼─────────┤
                                         C         5     ◄─── logical_offset + logical_length
                                    └ ─ ─ ─ ─ ┴ ─ ─ ─ ─ ┘
                                      values     index
```

The physical `run_ends` [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247) remains unchanged, in order to facilitate
zero-copy. However, we now offset into the **logical** representation with an
accompanying length. This allows us to represent values `[A, B, C]` using physical
indices `0, 1, 2` with the same underlying physical buffer, at the cost of two
extra `usize`s to represent the logical slice that was taken.

(A [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc) is considered unsliced when `logical_offset` is `0` and
`logical_length` is equal to the last value in `run_ends`)

[`RunArray`]: https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html
[Run-End encoded layout]: https://arrow.apache.org/docs/format/Columnar.html#run-end-encoded-layout

<a id="op-6d61b17de95408418c344977"></a>
## claim

`function` · `arrow_buffer::buffer::run::RunEndBuffer::claim` · arrow-buffer 59.3.0

```rust
fn claim(&self, pool: &dyn MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Claim memory used by this buffer in the provided memory pool.

<a id="op-9cc5fd7170b345fbaa2dea11"></a>
## clone

`function` · `arrow_buffer::buffer::run::RunEndBuffer::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> RunEndBuffer<E>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 17], "end": [96, 22], "filename": "src/buffer/run.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/buffer/run.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff27a362aec69c83ce545cee"></a>
## fmt

`function` · `arrow_buffer::buffer::run::RunEndBuffer::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 10], "end": [96, 15], "filename": "src/buffer/run.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffer/run.rs:96`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cea9a0a852df37958433a680"></a>
## get_end_physical_index

`function` · `arrow_buffer::buffer::run::RunEndBuffer::get_end_physical_index` · arrow-buffer 59.3.0

```rust
fn get_end_physical_index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:258`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the physical index at which the logical array ends.

The same as calling `get_physical_index(length - 1)` but with a fast path
if the buffer is not logically sliced, in which case it returns `length - 1`.

<a id="op-19268163441fcc18e75c61e3"></a>
## get_physical_index

`function` · `arrow_buffer::buffer::run::RunEndBuffer::get_physical_index` · arrow-buffer 59.3.0

```rust
fn get_physical_index(&self, logical_index: usize) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:232`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs a binary search to find the physical index for the given logical
index.

Useful for extracting the corresponding physical `run_ends` when this buffer
is logically sliced.

The result is arbitrary if `logical_index >= self.len()`.

<a id="op-576b50df7e077cbc839efad3"></a>
## get_physical_indices

`function` · `arrow_buffer::buffer::run::RunEndBuffer::get_physical_indices` · arrow-buffer 59.3.0

```rust
fn get_physical_indices<I>(&self, logical_indices: &[I]) -> Result<Vec<usize>, I> where I: ArrowNativeType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the physical indices corresponding to the provided logical indices.

Given a slice of logical indices, this method returns a `Vec` containing the
corresponding physical indices into the run-ends buffer.

This method operates by iterating the logical indices in sorted order, instead of
finding the physical index for each logical index using binary search via
the function [`RunEndBuffer::get_physical_index`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-19268163441fcc18e75c61e3).

Running benchmarks on both approaches showed that the approach used here
scaled well for larger inputs.

See <https://github.com/apache/arrow-rs/pull/3622#issuecomment-1407753727> for more details.

# Errors

If any logical index is out of bounds (>= self.len()), returns an error containing the invalid index.

<a id="op-19f60c1d1c389969c310f5e1"></a>
## get_start_physical_index

`function` · `arrow_buffer::buffer::run::RunEndBuffer::get_start_physical_index` · arrow-buffer 59.3.0

```rust
fn get_start_physical_index(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:246`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the physical index at which the logical array starts.

The same as calling `get_physical_index(0)` but with a fast path if the
buffer is not logically sliced, in which case it always returns `0`.

<a id="op-cbee3762d735caa41a6b42df"></a>
## inner

`function` · `arrow_buffer::buffer::run::RunEndBuffer::inner` · arrow-buffer 59.3.0

```rust
fn inner(&self) -> &ScalarBuffer<E>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:288`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247).

<a id="op-329511e2bfc6e68b32feb8ec"></a>
## into_inner

`function` · `arrow_buffer::buffer::run::RunEndBuffer::into_inner` · arrow-buffer 59.3.0

```rust
fn into_inner(self) -> ScalarBuffer<E>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:293`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247), consuming self.

<a id="op-ede2cc1cac6d8d666b5680ef"></a>
## is_empty

`function` · `arrow_buffer::buffer::run::RunEndBuffer::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if this buffer is logically empty.

<a id="op-cafea2e0d147403b62417a4e"></a>
## len

`function` · `arrow_buffer::buffer::run::RunEndBuffer::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:167`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the logical length of the run-ends stored by this buffer.

<a id="op-b0bd0405bd0f11cb5742f118"></a>
## max_value

`function` · `arrow_buffer::buffer::run::RunEndBuffer::max_value` · arrow-buffer 59.3.0

```rust
fn max_value(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:221`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the maximum run-end encoded in the underlying buffer; that is, the
last physical run of the buffer. This does not take into account any logical
slicing that may have occurred.

<a id="op-1897344cc1def84efe4ec6bc"></a>
## new

`function` · `arrow_buffer::buffer::run::RunEndBuffer::new` · arrow-buffer 59.3.0

```rust
fn new(run_ends: ScalarBuffer<E>, logical_offset: usize, logical_length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc) from a [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247), `logical_offset`
and `logical_length`.

# Panics

- `run_ends` does not contain strictly increasing values greater than zero
- The last value of `run_ends` is less than `logical_offset + logical_length`

<a id="op-1fefae2fe3a10f808cb263f0"></a>
## new_unchecked

`function` · `arrow_buffer::buffer::run::RunEndBuffer::new_unchecked` · arrow-buffer 59.3.0

```rust
unsafe fn new_unchecked(run_ends: ScalarBuffer<E>, logical_offset: usize, logical_length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc) from a [`ScalarBuffer`](../operations/arrow_buffer.buffer.scalar.ScalarBuffer.md#op-3edbfa8eafafb64c0414f247), `logical_offset`
and `logical_length`.

# Safety

- `run_ends` must contain strictly increasing values greater than zero
- The last value of `run_ends` must be greater than or equal to `logical_offset + logical_len`

<a id="op-5f2aa8ad71d94f118a274ea6"></a>
## offset

`function` · `arrow_buffer::buffer::run::RunEndBuffer::offset` · arrow-buffer 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:161`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the logical offset into the run-ends stored by this buffer.

<a id="op-f6c354e415f6cf28551ad64f"></a>
## shrink_to_fit

`function` · `arrow_buffer::buffer::run::RunEndBuffer::shrink_to_fit` · arrow-buffer 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:178`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Free up unused memory.

<a id="op-4ad75ee226678e17458ed82a"></a>
## slice

`function` · `arrow_buffer::buffer::run::RunEndBuffer::slice` · arrow-buffer 59.3.0

```rust
fn slice(&self, logical_offset: usize, logical_length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:275`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Slices this [`RunEndBuffer`](../operations/arrow_buffer.buffer.run.RunEndBuffer.md#op-e35311e9ed6d50fb4e98aafc) by the provided `logical_offset` and `logical_length`.

# Panics

- Specified slice (`logical_offset` + `logical_length`) exceeds existing
  logical length

<a id="op-d2d1f870c61f92cbbf9411ab"></a>
## sliced_values

`function` · `arrow_buffer::buffer::run::RunEndBuffer::sliced_values` · arrow-buffer 59.3.0

```rust
fn sliced_values(&self) -> impl Iterator<Item = E> + '_
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:199`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns an iterator yielding run ends adjusted for the logical slice.

Each yielded value is subtracted by the [`logical_offset`] and capped
at the [`logical_length`].

[`logical_offset`]: Self::offset
[`logical_length`]: Self::len

<a id="op-3c80c59e052b954fd6d0cf98"></a>
## values

`function` · `arrow_buffer::buffer::run::RunEndBuffer::values` · arrow-buffer 59.3.0

```rust
fn values(&self) -> &[E]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "E"}}], "constraints": []}}, "id": "arrow_buffer::buffer::run::RunEndBuffer", "path": "RunEndBuffer"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}}}], "generic_params": [], "type": {"generic": "E"}}}]}, "is_negative": false, "span": {"begin": [103, 1], "end": [379, 2], "filename": "src/buffer/run.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffer/run.rs:188`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the physical (**unsliced**) run ends of this buffer.

Take care when operating on these values as it doesn't take into account
any logical slicing that may have occurred.
