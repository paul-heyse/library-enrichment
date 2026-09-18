# `buoyant_kernel::log_replay::HasSelectionVector`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_replay.HasSelectionVector.json).

<a id="op-36cc3f5a3879263a644b7e2a"></a>
## HasSelectionVector

`trait` · `buoyant_kernel::log_replay::HasSelectionVector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait HasSelectionVector
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L395).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:395`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This trait is used to determine if a processor's output contains any selected rows.
This is used to filter out batches with no selected rows from the log replay results.

<a id="op-d31a8cd0d7587a12d7010659"></a>
## has_selected_rows

`function` · `buoyant_kernel::log_replay::HasSelectionVector::has_selected_rows` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_selected_rows(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_replay/mod.rs#L397).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_replay/mod.rs:397`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Check if the selection vector contains at least one selected row
