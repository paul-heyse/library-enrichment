# `buoyant_kernel::error::DeltaResultIteratorStatic`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.error.DeltaResultIteratorStatic.json).

<a id="op-99193b86ddcb610ad4e3cd79"></a>
## DeltaResultIteratorStatic

`type_alias` · `buoyant_kernel::error::DeltaResultIteratorStatic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DeltaResultIteratorStatic<T> = DeltaResultIterator<'static, T>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L27).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:27`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`'static` counterpart to [`DeltaResultIterator`](../operations/buoyant_kernel.error.DeltaResultIterator.md#op-10329196adb30aa54824619f) for cases where the iterator does not
reference borrowed data.
