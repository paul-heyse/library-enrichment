# `buoyant_kernel::metrics::metered_parquet`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_parquet.json).

<a id="op-ff2e7c05f83416eb7076d19e"></a>
## metered_parquet

`module` · `buoyant_kernel::metrics::metered_parquet` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod metered_parquet
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_parquet.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_parquet.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`MeteredParquetHandler`](../operations/buoyant_kernel.metrics.metered_parquet.MeteredParquetHandler.md#op-0ff1413b5221a5a8440c7dda) wraps any [`ParquetHandler`](../operations/buoyant_kernel.ParquetHandler.md#op-935e1b04f902c9a95af7c376) so its `read_parquet_files`
emits the kernel's standard `ParquetReadCompleted` span, carrying
`(num_files, bytes_read)` exactly once when the returned iterator is exhausted or
dropped. `read_parquet_footer` and `write_parquet_file` pass through.
