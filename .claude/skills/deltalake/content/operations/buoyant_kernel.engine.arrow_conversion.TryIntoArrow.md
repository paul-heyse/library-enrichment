# `buoyant_kernel::engine::arrow_conversion::TryIntoArrow`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_conversion.TryIntoArrow.json).

<a id="op-95d72eadcf91c464c844af81"></a>
## TryIntoArrow

`trait` · `buoyant_kernel::engine::arrow_conversion::TryIntoArrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TryIntoArrow<ArrowType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L128).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convert a kernel type into an arrow type (automatically implemented for all types that
implement [`TryFromKernel`](../operations/buoyant_kernel.engine.arrow_conversion.TryFromKernel.md#op-1e593785ffff46db05df0313))

<a id="op-25b4a288393e65df0404d747"></a>
## try_into_arrow

`function` · `buoyant_kernel::engine::arrow_conversion::TryIntoArrow::try_into_arrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_into_arrow(self) -> Result<ArrowType, ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L129).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:129`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
