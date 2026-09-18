# `buoyant_kernel::engine::arrow_utils::RowIndexBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.RowIndexBuilder.json).

<a id="op-84adf2e6bdead3539ea4265f"></a>
## RowIndexBuilder

`struct` · `buoyant_kernel::engine::arrow_utils::RowIndexBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RowIndexBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L113).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Prepares to enumerate row indexes of rows in a parquet file, accounting for row group skipping.

<a id="op-e26bf3c1f15915ea7bee7226"></a>
## build

`function` · `buoyant_kernel::engine::arrow_utils::RowIndexBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DeltaResult<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i64>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L149).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_utils::RowIndexBuilder", "path": "RowIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [175, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:149`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Build an iterator of row indexes, filtering out row groups that were skipped.

# Errors

Returns an error if there are duplicate or out of bounds row group ordinals.

<a id="op-ae70459b8b7fed26d84ec083"></a>
## new

`function` · `buoyant_kernel::engine::arrow_utils::RowIndexBuilder::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(row_groups: &[RowGroupMetaData]) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L120).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_utils::RowIndexBuilder", "path": "RowIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [175, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:120`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-638248597fd956def76b0c52"></a>
## select_row_groups

`function` · `buoyant_kernel::engine::arrow_utils::RowIndexBuilder::select_row_groups` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn select_row_groups(&mut self, ordinals: &[usize])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L137).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_utils::RowIndexBuilder", "path": "RowIndexBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [175, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:137`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Only produce row indexes for the row groups specified by the ordinals that survived row
group skipping. The ordinals must be in 0..num_row_groups.

<a id="op-c1acc9d762c738217183db87"></a>
## row_group_ordinals

`struct_field` · `buoyant_kernel::engine::arrow_utils::RowIndexBuilder::row_group_ordinals` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
row_group_ordinals: Option<Vec<usize>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L115).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f16e12b5965e253958b7bcd"></a>
## row_group_row_index_ranges

`struct_field` · `buoyant_kernel::engine::arrow_utils::RowIndexBuilder::row_group_row_index_ranges` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
row_group_row_index_ranges: Vec<std::ops::Range<i64>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L114).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:114`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
