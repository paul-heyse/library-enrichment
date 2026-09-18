# `buoyant_kernel::metrics::precounted_metrics_iterator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.precounted_metrics_iterator.json).

<a id="op-f2ca795fef5635a61d11e01f"></a>
## precounted_metrics_iterator

`module` · `buoyant_kernel::metrics::precounted_metrics_iterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod precounted_metrics_iterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/precounted_metrics_iterator.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/precounted_metrics_iterator.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Iterator wrapper that emits a metric with pre-computed `(num_files, bytes_read)`
exactly once -- when the inner iterator is exhausted or the wrapper is dropped.

Used by handlers that know their counts up-front from the input `&[FileMeta]` --
today, JSON and Parquet `read_*_files` -- so there's no need to count items as
they flow. Pair with [`MetricsIterator`] (count-as-you-stream) for handlers whose
counts aren't known until the work happens.

[`MetricsIterator`]: crate::metrics::MetricsIterator

Unresolved upstream links (retained, not inferred): `crate::metrics::MetricsIterator`.
