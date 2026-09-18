# `buoyant_kernel::history_manager::error::LogHistoryError::InvalidTimestampRange`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.error.LogHistoryError.InvalidTimestampRange.json).

<a id="op-ebe2f7efcc0d6390be251f68"></a>
## end_timestamp

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::InvalidTimestampRange::end_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
end_timestamp: super::Timestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L54).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:54`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The end timestamp that was less than the start timestamp.

<a id="op-1b10f085430ab157ac414359"></a>
## start_timestamp

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::InvalidTimestampRange::start_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start_timestamp: super::Timestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L52).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The start timestamp that was greater than the end timestamp.
