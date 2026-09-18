# `buoyant_kernel::crc::file_size_histogram`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.file_size_histogram.json).

<a id="op-44841b4e52fc5eaac748a929"></a>
## file_size_histogram

`module` · `buoyant_kernel::crc::file_size_histogram` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod file_size_histogram
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`FileSizeHistogram`](../operations/buoyant_kernel.crc.file_size_histogram.FileSizeHistogram.md#op-c8fc21baeab039dee041e99d) tracks the distribution of file sizes across predefined bins.

Used in CRC (version checksum) files to record the size distribution of active files in a
table version. Supports incremental updates via [`insert`](FileSizeHistogram::insert) and
[`remove`](FileSizeHistogram::remove), and delta merging via
[`try_apply_delta`](FileSizeHistogram::try_apply_delta).

[FileSizeHistogram]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#file-size-histogram-schema

Unresolved upstream links (retained, not inferred): `FileSizeHistogram::try_apply_delta`, `FileSizeHistogram::insert`, `FileSizeHistogram::remove`.
