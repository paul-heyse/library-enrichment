# `buoyant_kernel::engine::arrow_utils::ordering_needs_row_indexes`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.ordering_needs_row_indexes.json).

<a id="op-4292d7ba7abceaf3a9e940fd"></a>
## ordering_needs_row_indexes

`function` · `buoyant_kernel::engine::arrow_utils::ordering_needs_row_indexes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ordering_needs_row_indexes(requested_ordering: &[ReorderIndex]) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L852).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:852`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Check if an ordering requires row index computation.

The function only checks if a RowIndex transform is present at the top-level, since metadata
columns are not allowed to be nested.
