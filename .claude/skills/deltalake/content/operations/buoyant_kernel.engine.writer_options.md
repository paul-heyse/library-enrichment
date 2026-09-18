# `buoyant_kernel::engine::writer_options`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.writer_options.json).

<a id="op-d3bf17089e6b335197b991fb"></a>
## writer_options

`function` · `buoyant_kernel::engine::writer_options` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn writer_options() -> parquet::arrow::arrow_writer::ArrowWriterOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/mod.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/mod.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the standard [`ArrowWriterOptions`] for all kernel parquet writes.

Omitting the Arrow IPC schema from the file metadata keeps Delta files interoperable with
non-Arrow readers and avoids encoding Arrow-specific type information.

Unresolved upstream links (retained, not inferred): ``ArrowWriterOptions``.
