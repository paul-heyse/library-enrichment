# `buoyant_kernel::engine::arrow_conversion::TryIntoKernel`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_conversion.TryIntoKernel.json).

<a id="op-6e10c11c45601e477d85eb7b"></a>
## TryIntoKernel

`trait` · `buoyant_kernel::engine::arrow_conversion::TryIntoKernel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TryIntoKernel<KernelType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L140).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:140`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convert an arrow type into a kernel type (automatically implemented for all types that
implement [`TryFromArrow`](../operations/buoyant_kernel.engine.arrow_conversion.TryFromArrow.md#op-5cc3880c91de985c46801d4a))

<a id="op-4e6d33212c1e0b1131f6fa1a"></a>
## try_into_kernel

`function` · `buoyant_kernel::engine::arrow_conversion::TryIntoKernel::try_into_kernel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_into_kernel(self) -> Result<KernelType, ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L141).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:141`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
