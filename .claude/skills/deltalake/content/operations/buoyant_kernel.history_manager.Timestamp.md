# `buoyant_kernel::history_manager::Timestamp`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.Timestamp.json).

<a id="op-ef882b66bf7518874231d6c7"></a>
## Timestamp

`type_alias` · `buoyant_kernel::history_manager::Timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Timestamp = i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L48).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A timestamp representing milliseconds since the Unix epoch (1970-01-01 00:00:00 UTC).

This type is used throughout the history_manager module for timestamp-to-version conversion.
All timestamp values should be specified in milliseconds, not seconds or nanoseconds.
