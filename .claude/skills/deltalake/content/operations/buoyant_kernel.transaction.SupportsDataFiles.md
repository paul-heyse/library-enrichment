# `buoyant_kernel::transaction::SupportsDataFiles`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.SupportsDataFiles.json).

<a id="op-aaa24a96fb4d2961e35193b9"></a>
## SupportsDataFiles

`trait` · `buoyant_kernel::transaction::SupportsDataFiles` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait SupportsDataFiles
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L175).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:175`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Marker trait for transaction states that support data file operations.

Only transaction types that implement this trait can access methods for adding, removing, or
updating data files. This prevents compile-time misuse by states like `AlterTable` that
only perform metadata-only commits.
