# `buoyant_kernel::table_features::TABLE_FEATURES_MIN_WRITER_VERSION`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.TABLE_FEATURES_MIN_WRITER_VERSION.json).

<a id="op-078fd4a98101bf89c93f7b3b"></a>
## TABLE_FEATURES_MIN_WRITER_VERSION

`constant` · `buoyant_kernel::table_features::TABLE_FEATURES_MIN_WRITER_VERSION` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const TABLE_FEATURES_MIN_WRITER_VERSION: i32 = 7
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/mod.rs#L56).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/mod.rs:56`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Minimum writer version for tables that use table features.
When set to 7, the protocol requires an explicit `writerFeatures` array.
