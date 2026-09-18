# `buoyant_kernel::table_properties::DEFAULT_NUM_INDEXED_COLS`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_properties.DEFAULT_NUM_INDEXED_COLS.json).

<a id="op-cba545b8578e55d72de8178b"></a>
## DEFAULT_NUM_INDEXED_COLS

`constant` · `buoyant_kernel::table_properties::DEFAULT_NUM_INDEXED_COLS` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const DEFAULT_NUM_INDEXED_COLS: u64 = 32
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L291).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:291`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Default number of leaf columns to collect statistics on when `dataSkippingNumIndexedCols`
is not specified.
