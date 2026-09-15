# `arrow_buffer::pool`

Crate `arrow-buffer` · 3 public items · structured records in [`model/arrow_buffer.pool.json`](../model/arrow_buffer.pool.json)

## TrackingMemoryPool

`struct` · `arrow_buffer::pool::TrackingMemoryPool`

```rust
struct TrackingMemoryPool
```

**Implements**: `arrow_buffer::pool::MemoryPool`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn allocated(&self) -> usize
```

**via `arrow_buffer::pool::MemoryPool`**

```rust
fn available(&self) -> isize
fn capacity(&self) -> usize
fn reserve(&self, size: usize) -> Box<dyn MemoryReservation>
fn used(&self) -> usize
```

A simple [`MemoryPool`] that reports the total memory usage

---

## MemoryPool

`trait` · `arrow_buffer::pool::MemoryPool`

```rust
trait MemoryPool: Debug + Send + Sync
```

**Implementors** (2)

- `arrow_buffer::pool::TrackingMemoryPool`
- `datafusion_execution::memory_pool::arrow::ArrowMemoryPool`

**Methods** (4)

```rust
fn available(&self) -> isize
fn capacity(&self) -> usize
fn reserve(&self, size: usize) -> Box<dyn MemoryReservation>
fn used(&self) -> usize
```

A pool of memory that can be reserved and released.

This is used to accurately track memory usage when buffers are shared
between multiple arrays or other data structures.

For example, assume we have two arrays that share underlying buffer.
It's hard to tell how much memory is used by them because we can't
tell if the buffer is shared or not.

```text
      Array A           Array B    
   ┌────────────┐    ┌────────────┐
   │ slices...  │    │ slices...  │
   │────────────│    │────────────│
   │ Arc<Bytes> │    │ Arc<Bytes> │ (shared buffer)
   └─────▲──────┘    └───────▲────┘
         │                   │     
         │       Bytes       │     
         │  ┌─────────────┐  │     
         │  │   data...   │  │     
         │  │─────────────│  │     
         └──│   Memory    │──┘   (tracked with a memory pool)  
            │ Reservation │        
            └─────────────┘        
```

With a memory pool, we can count the memory usage by the shared buffer
directly.

---

## MemoryReservation

`trait` · `arrow_buffer::pool::MemoryReservation`

```rust
trait MemoryReservation: Debug + Send + Sync
```

**Implementors** (1)

- `datafusion_execution::memory_pool::MemoryReservation`

**Methods** (2)

```rust
fn resize(&mut self, new_size: usize)
fn size(&self) -> usize
```

A memory reservation within a [`MemoryPool`] that is freed on drop

---
