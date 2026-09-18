# `parquet::arrow::async_writer::AsyncFileWriter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.async_writer.AsyncFileWriter.json).

<a id="op-d1acaf94f7f04028351c03f9"></a>
## AsyncFileWriter

`trait` · `parquet::arrow::async_writer::AsyncFileWriter` · parquet 59.3.0

```rust
trait AsyncFileWriter: Send
```

Source: `src/arrow/async_writer/mod.rs:85`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The asynchronous interface used by [`AsyncArrowWriter`](../operations/parquet.arrow.async_writer.AsyncArrowWriter.md#op-d9df4726e3133f2ae1d81ca9) to write parquet files.

<a id="op-7cf702f8c6d65e9da6d2fba2"></a>
## complete

`function` · `parquet::arrow::async_writer::AsyncFileWriter::complete` · parquet 59.3.0

```rust
fn complete(&mut self) -> BoxFuture<'_, Result<()>>
```

Source: `src/arrow/async_writer/mod.rs:97`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Flush any buffered data to the underlying writer and finish writing process.

After `complete` returns `Ok(())`, caller SHOULD not call write again.

<a id="op-931ace750589804aefd059c9"></a>
## write

`function` · `parquet::arrow::async_writer::AsyncFileWriter::write` · parquet 59.3.0

```rust
fn write(&mut self, bs: Bytes) -> BoxFuture<'_, Result<()>>
```

Source: `src/arrow/async_writer/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write the provided bytes to the underlying writer

The underlying writer CAN decide to buffer the data or write it immediately.
This design allows the writer implementer to control the buffering and I/O scheduling.

The underlying writer MAY implement retry logic to prevent breaking users write process.
