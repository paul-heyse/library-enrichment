# `buoyant_kernel::history_manager::error::LogHistoryError::TimestampOutOfRange`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.error.LogHistoryError.TimestampOutOfRange.json).

<a id="op-322d541a4a4d729284cd059d"></a>
## nearest_timestamp

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::TimestampOutOfRange::nearest_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
nearest_timestamp: NearestTimestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L83).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:83`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The nearest retained commit's timestamp on the side of the search
bound. For example, given a retained range `[100, 500]` and a
`GreatestLower` search at timestamp `50`, this is
`NearestTimestamp::Earliest(100)`.

<a id="op-5943e389ea68f45962b3fd69"></a>
## reason

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::TimestampOutOfRange::reason` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
reason: &'static str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L78).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Description of why the timestamp is out of range.

<a id="op-422b76461adfeab20b77ef68"></a>
## timestamp

`struct_field` · `buoyant_kernel::history_manager::error::LogHistoryError::TimestampOutOfRange::timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
timestamp: super::Timestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/error.rs#L76).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/error.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The timestamp that was out of range.
