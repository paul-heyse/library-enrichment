# `arrow_buffer::builder::offset`

Crate `arrow-buffer` · 1 public items · structured records in [`model/arrow_buffer.builder.offset.json`](../model/arrow_buffer.builder.offset.json)

## OffsetBufferBuilder

`struct` · `arrow_buffer::builder::offset::OffsetBufferBuilder`

```rust
struct OffsetBufferBuilder<O: ArrowNativeType>
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Debug

**Methods** (5)

```rust
fn finish(self) -> OffsetBuffer<O>
fn finish_cloned(&self) -> OffsetBuffer<O>
fn new(capacity: usize) -> Self
fn push_length(&mut self, length: usize)
fn reserve(&mut self, additional: usize)
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Builder of [`OffsetBuffer`]

---
