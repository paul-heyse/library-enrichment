# `buoyant_kernel::partition::serialization`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.partition.serialization.json).

<a id="op-e018074bed24f759d81f8888"></a>
## serialization

`module` · `buoyant_kernel::partition::serialization` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod serialization
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/partition/serialization.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/partition/serialization.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Partition value serialization for the Delta log.

A partition value goes through several transformation steps before reaching the
Delta log (see the [`super`](../modules/buoyant_kernel.partition.md#op-c3f0fbf14b67d59e9094e867) module for the full pipeline and encoding tables).
This module converts typed [`Scalar`] values into the strings that appear in
`AddFile.partitionValues`.

```text
Step 2 (THIS MODULE):  Scalar::String("US/East")  ->  Some("US/East")  (partitionValues)
Step 3 (hive module):  "US/East"                  ->  "US%2FEast"      (directory name)
```

[`Scalar`]: crate::expressions::Scalar
