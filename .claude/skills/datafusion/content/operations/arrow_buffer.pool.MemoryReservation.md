# `arrow_buffer::pool::MemoryReservation`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.pool.MemoryReservation.json).

<a id="op-0ce9017e02ec504c2f8051b0"></a>
## MemoryReservation

`trait` · `arrow_buffer::pool::MemoryReservation` · arrow-buffer 59.3.0

```rust
trait MemoryReservation: Debug + Send + Sync
```

Source: `src/pool.rs:37`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

A memory reservation within a [`MemoryPool`](../operations/arrow_buffer.pool.MemoryPool.md#op-ed45519ea8493e8aed943b0a) that is freed on drop

<a id="op-fced651f03b8af47685d1c6b"></a>
## resize

`function` · `arrow_buffer::pool::MemoryReservation::resize` · arrow-buffer 59.3.0

```rust
fn resize(&mut self, new_size: usize)
```

Source: `src/pool.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Resize this reservation to a new size in bytes.

<a id="op-c1b45fc6f45b569ba2ac99a5"></a>
## size

`function` · `arrow_buffer::pool::MemoryReservation::size` · arrow-buffer 59.3.0

```rust
fn size(&self) -> usize
```

Source: `src/pool.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the size of this reservation in bytes.
