# `arrow_array::record_batch::RecordBatchWriter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.record_batch.RecordBatchWriter.json).

<a id="op-0b56d3b45292ad4ca3f57f49"></a>
## RecordBatchWriter

`trait` · `arrow_array::record_batch::RecordBatchWriter` · arrow-array 59.3.0

```rust
trait RecordBatchWriter
```

Source: `src/record_batch.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Trait for types that can write `RecordBatch`'s.

<a id="op-caa4fc839b0c9e6545cc0042"></a>
## close

`function` · `arrow_array::record_batch::RecordBatchWriter::close` · arrow-array 59.3.0

```rust
fn close(self) -> Result<(), ArrowError>
```

Source: `src/record_batch.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Write footer or termination data, then mark the writer as done.

<a id="op-252938c260e65f8d9277d9c3"></a>
## write

`function` · `arrow_array::record_batch::RecordBatchWriter::write` · arrow-array 59.3.0

```rust
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

Source: `src/record_batch.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Write a single batch to the writer.
