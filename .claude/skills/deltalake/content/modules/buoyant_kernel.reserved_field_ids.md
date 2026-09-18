# `buoyant_kernel::reserved_field_ids`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.reserved_field_ids.json).

<a id="op-eb0e7cae8954d0a8614cf9cb"></a>
## reserved_field_ids

`module` · `buoyant_kernel::reserved_field_ids` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod reserved_field_ids
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L724).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:724`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reserved field IDs for metadata columns in Delta tables.

These field IDs are reserved and should not be used for regular table columns.
They are used to provide file-level metadata as virtual columns during reads.
