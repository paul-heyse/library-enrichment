# `buoyant_kernel::reserved_field_ids::FILE_NAME`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.reserved_field_ids.FILE_NAME.json).

<a id="op-a2b742a3c77ba475fda2c18b"></a>
## FILE_NAME

`constant` · `buoyant_kernel::reserved_field_ids::FILE_NAME` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const FILE_NAME: i64 = 2147483646
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L727).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:727`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reserved field ID for the file name metadata column (`_file`).
This column provides the name of the Parquet file that contains each row.
