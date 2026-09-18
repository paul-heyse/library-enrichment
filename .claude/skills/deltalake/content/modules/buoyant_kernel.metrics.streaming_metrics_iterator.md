# `buoyant_kernel::metrics::streaming_metrics_iterator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.streaming_metrics_iterator.json).

<a id="op-e48a7583b4fb1c76e46e083b"></a>
## streaming_metrics_iterator

`module` · `buoyant_kernel::metrics::streaming_metrics_iterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod streaming_metrics_iterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/streaming_metrics_iterator.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/streaming_metrics_iterator.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Iterator wrapper that counts items as they stream and emits the kernel's standard
`"storage"` tracing span on drop. Used by [`MeteredStorageHandler`] for handlers
whose counts are only known as items flow (`list_from`, `read_files`).

The span fires on the thread that drops the iterator (the caller's thread),
avoiding background-thread emission where no tracing subscriber may be installed.

[`MeteredStorageHandler`]: crate::metrics::MeteredStorageHandler
