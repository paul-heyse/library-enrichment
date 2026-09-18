# `buoyant_kernel::engine::arrow_utils::make_arrow_error`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.make_arrow_error.json).

<a id="op-cc2dda68ee36fea01aa9d379"></a>
## make_arrow_error

`function` · `buoyant_kernel::engine::arrow_utils::make_arrow_error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn make_arrow_error(s: impl Into<String>) -> Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L104).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create an [`Error::Arrow`](../operations/buoyant_kernel.error.Error.md#op-1dfd346043afb342350a8eba) with a backtrace from the given message.
