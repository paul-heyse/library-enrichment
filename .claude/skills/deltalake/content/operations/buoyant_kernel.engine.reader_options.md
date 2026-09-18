# `buoyant_kernel::engine::reader_options`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.reader_options.json).

<a id="op-3f990aef8071b67fa4575e76"></a>
## reader_options

`function` · `buoyant_kernel::engine::reader_options` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn reader_options() -> parquet::arrow::arrow_reader::ArrowReaderOptions
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/mod.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/mod.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the standard [`ArrowReaderOptions`] for all default engine parquet reads.

Skipping the embedded Arrow IPC schema avoids dependence on Arrow-specific metadata and
ensures that type resolution is driven by the kernel schema rather than the file's schema.

Unresolved upstream links (retained, not inferred): ``ArrowReaderOptions``.
