# `buoyant_kernel::engine`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.engine.json`](../model/buoyant_kernel.engine.json)

## reader_options

`function` · `buoyant_kernel::engine::reader_options`

Also reachable as `delta_kernel::engine::reader_options`, `deltalake::kernel::engine::reader_options`, `deltalake_core::kernel::engine::reader_options`

```rust
fn reader_options() -> parquet::arrow::arrow_reader::ArrowReaderOptions
```

Returns the standard [`ArrowReaderOptions`] for all default engine parquet reads.

Skipping the embedded Arrow IPC schema avoids dependence on Arrow-specific metadata and
ensures that type resolution is driven by the kernel schema rather than the file's schema.

---

## writer_options

`function` · `buoyant_kernel::engine::writer_options`

Also reachable as `delta_kernel::engine::writer_options`, `deltalake::kernel::engine::writer_options`, `deltalake_core::kernel::engine::writer_options`

```rust
fn writer_options() -> parquet::arrow::arrow_writer::ArrowWriterOptions
```

Returns the standard [`ArrowWriterOptions`] for all kernel parquet writes.

Omitting the Arrow IPC schema from the file metadata keeps Delta files interoperable with
non-Arrow readers and avoids encoding Arrow-specific type information.

---
