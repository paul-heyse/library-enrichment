# `buoyant_kernel::engine_data::RowIndexIterator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine_data.RowIndexIterator.json).

<a id="op-7304f0f62a800091dc7533b9"></a>
## RowIndexIterator

`struct` · `buoyant_kernel::engine_data::RowIndexIterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct RowIndexIterator<'sv>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L353).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:353`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An iterator over the indices of selected rows in an engine-data batch.

Each call to [`Iterator::next`] returns the index of the next selected row.

Constructed internally and passed (alongside the column getters) to
[`FilteredRowVisitor::visit_filtered`](../operations/buoyant_kernel.engine_data.FilteredRowVisitor.md#op-1bde2215c626be8fb0850d46).

Unresolved upstream links (retained, not inferred): ``Iterator::next``.

<a id="op-04e67c700f5f15418f8a9502"></a>
## Item

`assoc_type` · `buoyant_kernel::engine_data::RowIndexIterator::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L375).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'sv"}], "constraints": []}}, "id": "buoyant_kernel::engine_data::RowIndexIterator", "path": "RowIndexIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'sv"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [387, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:375`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bd3ea3a30c9a8ce8e05f6ba"></a>
## next

`function` · `buoyant_kernel::engine_data::RowIndexIterator::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L377).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'sv"}], "constraints": []}}, "id": "buoyant_kernel::engine_data::RowIndexIterator", "path": "RowIndexIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'sv"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [387, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:377`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4d3620b4bfee244621e0c54"></a>
## num_rows

`function` · `buoyant_kernel::engine_data::RowIndexIterator::num_rows` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_rows(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L369).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'sv"}], "constraints": []}}, "id": "buoyant_kernel::engine_data::RowIndexIterator", "path": "RowIndexIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'sv"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [359, 1], "end": [372, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:369`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the total number of rows in the batch (selected and deselected).

<a id="op-e1d7c97df3417f4a84fcc680"></a>
## row_count

`struct_field` · `buoyant_kernel::engine_data::RowIndexIterator::row_count` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
row_count: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L356).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:356`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1236309ae93cf070ef4057ba"></a>
## selection_vector

`struct_field` · `buoyant_kernel::engine_data::RowIndexIterator::selection_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
selection_vector: &'sv [bool]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L355).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:355`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c96ef2c993a37d12c6a4674f"></a>
## sv_pos

`struct_field` · `buoyant_kernel::engine_data::RowIndexIterator::sv_pos` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
sv_pos: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L354).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:354`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
