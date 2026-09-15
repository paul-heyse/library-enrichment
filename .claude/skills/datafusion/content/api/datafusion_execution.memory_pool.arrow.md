# `datafusion_execution::memory_pool::arrow`

Crate `datafusion-execution` · 1 public items · structured records in [`model/datafusion_execution.memory_pool.arrow.json`](../model/datafusion_execution.memory_pool.arrow.json)

## ArrowMemoryPool

`struct` · `datafusion_execution::memory_pool::arrow::ArrowMemoryPool`

```rust
struct ArrowMemoryPool
```

**Implements**: `arrow_buffer::pool::MemoryPool`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn MemoryPool>, consumer: MemoryConsumer) -> Self
```

**via `arrow_buffer::pool::MemoryPool`**

```rust
fn available(&self) -> isize
fn capacity(&self) -> usize
fn reserve(&self, size: usize) -> Box<dyn arrow_buffer::MemoryReservation>
fn used(&self) -> usize
```

An adapter that implements Arrow's [`arrow_buffer::MemoryPool`] trait
by wrapping a DataFusion [`MemoryPool`].

This allows DataFusion's memory management system to be used with Arrow's
memory allocation APIs. Each reservation made through this pool will be
tracked using the provided [`MemoryConsumer`], enabling DataFusion to
monitor and limit memory usage across Arrow operations.

This is useful when you want Arrow operations (such as array builders
or compute kernels) to participate in DataFusion's memory management
and respect the same memory limits as DataFusion operators.

---
