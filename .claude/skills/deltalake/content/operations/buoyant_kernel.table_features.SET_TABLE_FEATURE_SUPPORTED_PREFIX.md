# `buoyant_kernel::table_features::SET_TABLE_FEATURE_SUPPORTED_PREFIX`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.SET_TABLE_FEATURE_SUPPORTED_PREFIX.json).

<a id="op-27c74947b6c557fd0935d31e"></a>
## SET_TABLE_FEATURE_SUPPORTED_PREFIX

`constant` · `buoyant_kernel::table_features::SET_TABLE_FEATURE_SUPPORTED_PREFIX` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const SET_TABLE_FEATURE_SUPPORTED_PREFIX: &str = "delta.feature."
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/mod.rs#L61).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/mod.rs:61`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Prefix for table feature override properties.
Properties with this prefix (e.g., `delta.feature.deletionVectors`) are used to
explicitly turn on support for the feature in the protocol.
