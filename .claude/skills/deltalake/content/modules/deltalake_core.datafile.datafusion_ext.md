# `deltalake_core::datafile::datafusion_ext`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.datafusion_ext.json).

<a id="op-f359eba9d0d6ea6ae21ed7fc"></a>
## datafusion_ext

`module` · `deltalake_core::datafile::datafusion_ext` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod datafusion_ext
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/datafusion_ext.rs#L1).

Source: `crates/core/src/datafile/datafusion_ext.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

DataFusion-backed extensions to the basic data-file traits: write an
`ExecutionPlan`'s output, and read through `DeltaScanNext` (pushdown,
deletion vectors, transforms). The writer extension late-materializes a plan
into the basic record-batch stream and delegates to [`DeltaDataWriter`](../operations/deltalake_core.datafile.DeltaDataWriter.md#op-c1f5bba8aa5bb8d203fc6776).
