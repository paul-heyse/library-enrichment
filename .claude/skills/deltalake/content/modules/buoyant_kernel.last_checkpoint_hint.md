# `buoyant_kernel::last_checkpoint_hint`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.last_checkpoint_hint.json).

<a id="op-f582bdac3b994f6591b43e60"></a>
## last_checkpoint_hint

`module` · `buoyant_kernel::last_checkpoint_hint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod last_checkpoint_hint
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/last_checkpoint_hint.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/last_checkpoint_hint.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Utilities for reading the `_last_checkpoint` file. Maybe this file should instead go under
log_segment module since it should only really be used there? as hint for listing?
