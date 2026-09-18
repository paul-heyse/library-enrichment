# `buoyant_kernel::table_features::iceberg_compat`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.iceberg_compat.json).

<a id="op-65fdc08a10bd8513e8d4a530"></a>
## iceberg_compat

`module` · `buoyant_kernel::table_features::iceberg_compat` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod iceberg_compat
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/iceberg_compat/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/iceberg_compat/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

IcebergCompat invariant checks shared across versions.

Each `delta.enableIcebergCompatV{N}` version owns a submodule (currently only
[`v3`](../modules/buoyant_kernel.table_features.iceberg_compat.v3.md#op-228acd7f49bd1da61a0078a6)), and exposes a single
`pub(crate) const V{N}_VALIDATOR: IcebergCompatValidator`. Callers feed that
constant to [`validate_iceberg_compat_if_needed`].

Unresolved upstream links (retained, not inferred): ``validate_iceberg_compat_if_needed``.
