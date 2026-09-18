# `buoyant_kernel::parallel::parallel_scan_metadata::AfterSequentialScanMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.parallel.parallel_scan_metadata.AfterSequentialScanMetadata.json).

<a id="op-aa9b99fd00295aad86e5c209"></a>
## AfterSequentialScanMetadata

`enum` · `buoyant_kernel::parallel::parallel_scan_metadata::AfterSequentialScanMetadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum AfterSequentialScanMetadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L22).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Result of sequential scan metadata processing.

This enum indicates whether distributed processing is needed:
- `Done`: All processing completed sequentially - no distributed phase needed.
- `Parallel`: Contains state and files for parallel processing.

<a id="op-18f84b9379bf54ea1f4bfea8"></a>
## Done

`variant` · `buoyant_kernel::parallel::parallel_scan_metadata::AfterSequentialScanMetadata::Done` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Done
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ab99677da686c7cc6c4cefd"></a>
## Parallel

`variant` · `buoyant_kernel::parallel::parallel_scan_metadata::AfterSequentialScanMetadata::Parallel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Parallel
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/parallel/parallel_scan_metadata.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/parallel/parallel_scan_metadata.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
