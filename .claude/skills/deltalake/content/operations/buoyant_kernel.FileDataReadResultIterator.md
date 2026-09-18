# `buoyant_kernel::FileDataReadResultIterator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.FileDataReadResultIterator.json).

<a id="op-2c38562abfe66da38e1b8f0e"></a>
## FileDataReadResultIterator

`type_alias` · `buoyant_kernel::FileDataReadResultIterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type FileDataReadResultIterator = DeltaResultIteratorStatic<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L215).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:215`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An iterator of data read from specified files
