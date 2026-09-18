# `deltalake_core::kernel::models::actions::new_metadata`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.new_metadata.json).

<a id="op-5a2a4c7b66e83972066412c4"></a>
## new_metadata

`function` · `deltalake_core::kernel::models::actions::new_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_metadata(schema: &kernel::StructType, partition_columns: impl IntoIterator<Item = impl ToString>, configuration: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> kernel::DeltaResult<Metadata>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L21).

Source: `crates/core/src/kernel/models/actions.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Please don't use, this API will be leaving shortly!

Since the adoption of delta-kernel-rs we lost the direct ability to create [Metadata](../operations/buoyant_kernel.actions.Metadata.md#op-1843fecb3566e0e5ad63d7aa) actions
which is required for some use-cases.

Upstream tracked here: <https://github.com/delta-io/delta-kernel-rs/issues/1055>
