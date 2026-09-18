# `buoyant_kernel::history_manager::error::LogHistoryError::EmptyTimestampRange`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.error.LogHistoryError.EmptyTimestampRange.json).

<a id="op-ce8ea3b02f9594bdd854ad90"></a>
## between_version

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::EmptyTimestampRange::between_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
between_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version immediately before the timestamp range (the next version is
`between_version + 1`).

<a id="op-02b378999a04636d141eb5b4"></a>
## end_timestamp

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::EmptyTimestampRange::end_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
end_timestamp: super::Timestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L67).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:67`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The end of the empty timestamp range.

<a id="op-1fd6f2a547f339fcf6888c3c"></a>
## start_timestamp

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::EmptyTimestampRange::start_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
start_timestamp: super::Timestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L65).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:65`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The start of the empty timestamp range.
