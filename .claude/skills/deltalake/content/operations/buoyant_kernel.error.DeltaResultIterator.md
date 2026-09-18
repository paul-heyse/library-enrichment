# `buoyant_kernel::error::DeltaResultIterator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.error.DeltaResultIterator.json).

<a id="op-10329196adb30aa54824619f"></a>
## DeltaResultIterator

`type_alias` · `buoyant_kernel::error::DeltaResultIterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DeltaResultIterator<'a, T> = Box<dyn Iterator<Item = DeltaResult<T>> + Send + 'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/error.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/error.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A boxed, `Send` iterator of [`DeltaResult<T>`](../operations/buoyant_kernel.error.DeltaResult.md#op-3db788f17aa90cfeefaa890f) items.

Convenience alias for the common pattern of returning a streaming, fallible iterator from
kernel APIs.
