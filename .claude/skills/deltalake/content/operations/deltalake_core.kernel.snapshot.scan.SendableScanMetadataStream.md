# `deltalake_core::kernel::snapshot::scan::SendableScanMetadataStream`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.snapshot.scan.SendableScanMetadataStream.json).

<a id="op-adb8fdd890478bd9539c841a"></a>
## SendableScanMetadataStream

`type_alias` · `deltalake_core::kernel::snapshot::scan::SendableScanMetadataStream` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type SendableScanMetadataStream = std::pin::Pin<Box<dyn Stream<Item = DeltaResult<delta_kernel::scan::ScanMetadata>> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/snapshot/scan.rs#L24).

Source: `crates/core/src/kernel/snapshot/scan.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A boxed, `Send`able stream of [`ScanMetadata`](../operations/buoyant_kernel.scan.ScanMetadata.md#op-291499a74f7b78c8790cd5b2) results produced while scanning a snapshot.
