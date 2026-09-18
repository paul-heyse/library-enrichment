# `buoyant_kernel::FileDataReadResult`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.FileDataReadResult.json).

<a id="op-400c5b886e67f770d8c3febe"></a>
## FileDataReadResult

`type_alias` · `buoyant_kernel::FileDataReadResult` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type FileDataReadResult = (FileMeta, Box<dyn EngineData>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L212).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:212`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Data read from a Delta table file and the corresponding scan file information.
