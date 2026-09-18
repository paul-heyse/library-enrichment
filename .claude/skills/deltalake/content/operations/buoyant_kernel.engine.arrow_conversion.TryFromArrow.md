# `buoyant_kernel::engine::arrow_conversion::TryFromArrow`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_conversion.TryFromArrow.json).

<a id="op-5cc3880c91de985c46801d4a"></a>
## TryFromArrow

`trait` · `buoyant_kernel::engine::arrow_conversion::TryFromArrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TryFromArrow<ArrowType>: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L134).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:134`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convert an arrow type into a kernel type (a similar [`TryIntoKernel`](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoKernel.md#op-6e10c11c45601e477d85eb7b) trait is automatically
implemented for all types that implement [`TryFromArrow`](../operations/buoyant_kernel.engine.arrow_conversion.TryFromArrow.md#op-5cc3880c91de985c46801d4a))

<a id="op-f63a977678aff11203a13f98"></a>
## try_from_arrow

`function` · `buoyant_kernel::engine::arrow_conversion::TryFromArrow::try_from_arrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from_arrow(t: ArrowType) -> Result<Self, ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L135).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:135`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
