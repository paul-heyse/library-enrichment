# `buoyant_kernel::engine::parquet_row_group_skipping`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.parquet_row_group_skipping.json).

<a id="op-18624d78839dee0ea66f2a29"></a>
## parquet_row_group_skipping

`module` · `buoyant_kernel::engine::parquet_row_group_skipping` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod parquet_row_group_skipping
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/parquet_row_group_skipping.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/parquet_row_group_skipping.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An implementation of parquet row group skipping using data skipping predicates over footer
stats.
