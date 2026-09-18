# `buoyant_kernel::engine::arrow_conversion::TryFromKernel`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_conversion.TryFromKernel.json).

<a id="op-1e593785ffff46db05df0313"></a>
## TryFromKernel

`trait` · `buoyant_kernel::engine::arrow_conversion::TryFromKernel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TryFromKernel<KernelType>: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L146).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:146`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convert a kernel type into an arrow type (a similar [`TryIntoArrow`](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoArrow.md#op-95d72eadcf91c464c844af81) trait is automatically
implemented for all types that implement [`TryFromKernel`](../operations/buoyant_kernel.engine.arrow_conversion.TryFromKernel.md#op-1e593785ffff46db05df0313))

<a id="op-e1f9a8da2c3b389273d89f54"></a>
## try_from_kernel

`function` · `buoyant_kernel::engine::arrow_conversion::TryFromKernel::try_from_kernel` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from_kernel(t: KernelType) -> Result<Self, ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L147).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:147`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
