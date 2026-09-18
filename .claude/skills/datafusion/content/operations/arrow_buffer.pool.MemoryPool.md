# `arrow_buffer::pool::MemoryPool`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.pool.MemoryPool.json).

<a id="op-ed45519ea8493e8aed943b0a"></a>
## MemoryPool

`trait` · `arrow_buffer::pool::MemoryPool` · arrow-buffer 59.3.0

```rust
trait MemoryPool: Debug + Send + Sync
```

Source: `src/pool.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

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

<a id="op-2963a1a9d8173b44d87bd2dc"></a>
## available

`function` · `arrow_buffer::pool::MemoryPool::available` · arrow-buffer 59.3.0

```rust
fn available(&self) -> isize
```

Source: `src/pool.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the current available memory in the pool.

The pool may be overfilled, so this method might return a negative value.

<a id="op-23765a02138c9c9633edcdda"></a>
## capacity

`function` · `arrow_buffer::pool::MemoryPool::capacity` · arrow-buffer 59.3.0

```rust
fn capacity(&self) -> usize
```

Source: `src/pool.rs:88`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the maximum memory that can be reserved from the pool.

<a id="op-27167cc6c765f4c7ae938288"></a>
## reserve

`function` · `arrow_buffer::pool::MemoryPool::reserve` · arrow-buffer 59.3.0

```rust
fn reserve(&self, size: usize) -> Box<dyn MemoryReservation>
```

Source: `src/pool.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Reserves memory from the pool. Infallible.

Returns a reservation of the requested size.

<a id="op-0a1bdecf439405631944d47e"></a>
## used

`function` · `arrow_buffer::pool::MemoryPool::used` · arrow-buffer 59.3.0

```rust
fn used(&self) -> usize
```

Source: `src/pool.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the current used memory from the pool.
