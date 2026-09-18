# `arrow_buffer::buffer::run`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.buffer.run.json`](../model/arrow_buffer.buffer.run.json)

## RunEndBuffer

`struct` · `arrow_buffer::buffer::run::RunEndBuffer`

```rust
struct RunEndBuffer<E: ArrowNativeType>
```

**Derives**: Clone, Debug

**Methods** (17)

```rust
fn claim(&self, pool: &dyn MemoryPool)
fn get_end_physical_index(&self) -> usize
fn get_physical_index(&self, logical_index: usize) -> usize
fn get_physical_indices<I>(&self, logical_indices: &[I]) -> Result<Vec<usize>, I> where I: ArrowNativeType
fn get_start_physical_index(&self) -> usize
fn inner(&self) -> &ScalarBuffer<E>
fn into_inner(self) -> ScalarBuffer<E>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn max_value(&self) -> usize
fn new(run_ends: ScalarBuffer<E>, logical_offset: usize, logical_length: usize) -> Self
unsafe fn new_unchecked(run_ends: ScalarBuffer<E>, logical_offset: usize, logical_length: usize) -> Self
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, logical_offset: usize, logical_length: usize) -> Self
fn sliced_values(&self) -> impl Iterator<Item = E> + '_
fn values(&self) -> &[E]
```

[Full member, field, variant and typed contracts](../operations/arrow_buffer.buffer.run.RunEndBuffer.md).


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

A [`RunEndBuffer`] is physically the buffer and offset with length on the left.
In this case, the offset and length represent the whole buffer, so it is essentially
unsliced. See the section below on slicing for more details on how this buffer
handles slicing.

This means that multiple logical values are represented in the same physical index,
and multiple logical indices map to the same physical index. The [`RunEndBuffer`]
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

The physical `run_ends` [`ScalarBuffer`] remains unchanged, in order to facilitate
zero-copy. However, we now offset into the **logical** representation with an
accompanying length. This allows us to represent values `[A, B, C]` using physical
indices `0, 1, 2` with the same underlying physical buffer, at the cost of two
extra `usize`s to represent the logical slice that was taken.

(A [`RunEndBuffer`] is considered unsliced when `logical_offset` is `0` and
`logical_length` is equal to the last value in `run_ends`)

[`RunArray`]: https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html
[Run-End encoded layout]: https://arrow.apache.org/docs/format/Columnar.html#run-end-encoded-layout

---
