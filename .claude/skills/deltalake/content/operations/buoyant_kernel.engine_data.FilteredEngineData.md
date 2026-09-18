# `buoyant_kernel::engine_data::FilteredEngineData`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine_data.FilteredEngineData.json).

<a id="op-62b3837332f86d6b0fa41aab"></a>
## FilteredEngineData

`struct` · `buoyant_kernel::engine_data::FilteredEngineData` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FilteredEngineData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Engine data paired with a selection vector indicating which rows are logically selected.

A value of `true` in the selection vector means the corresponding row is selected (i.e., not
deleted), while `false` means the row is logically deleted and should be ignored. If the
selection vector is shorter than the number of rows in `data` then all rows not covered by the
selection vector are assumed to be selected.

Interpreting unselected (`false`) rows will result in incorrect/undefined behavior.

<a id="op-c5e31214bf7fcdf905d25151"></a>
## apply_selection_vector

`function` · `buoyant_kernel::engine_data::FilteredEngineData::apply_selection_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn apply_selection_vector(self) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L75).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:75`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Apply the contained selection vector and return an engine data with only the selected rows
included. This consumes the `FilteredEngineData`.

<a id="op-676bb9be52033db6070d99f7"></a>
## data

`function` · `buoyant_kernel::engine_data::FilteredEngineData::data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn data(&self) -> &dyn EngineData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L48).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:48`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns a reference to the underlying engine data.

<a id="op-1aa010a8ce42f301a17ac4eb"></a>
## from

`function` · `buoyant_kernel::engine_data::FilteredEngineData::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(data: Box<dyn EngineData>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L104).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [107, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "buoyant_kernel::engine_data::EngineData", "path": "EngineData"}}]}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Converts `EngineData` into `FilteredEngineData` with all rows selected.

This is a convenience conversion that wraps the provided engine data
in a `FilteredEngineData` with an empty selection vector, meaning all
rows are logically selected.

# Example
```rust,ignore
let engine_data: Box<dyn EngineData> = ...;
let filtered: FilteredEngineData = engine_data.into();
```

<a id="op-cbdbdbc9272bbdaa58cfd6a4"></a>
## has_selected_rows

`function` · `buoyant_kernel::engine_data::FilteredEngineData::has_selected_rows` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_selected_rows(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L82).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [90, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": {"args": null, "id": "buoyant_kernel::log_replay::HasSelectionVector", "path": "HasSelectionVector"}, "trait_path": "buoyant_kernel::log_replay::HasSelectionVector"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:82`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if any row in the selection vector is marked as selected

<a id="op-3393125ae1e0da1e40ea4c55"></a>
## into_parts

`function` · `buoyant_kernel::engine_data::FilteredEngineData::into_parts` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_parts(self) -> (Box<dyn EngineData>, Vec<bool>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L58).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:58`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Consumes the FilteredEngineData and returns the underlying data and selection vector.

<a id="op-4cfbbaf5d4f1d24ef003e22a"></a>
## selection_vector

`function` · `buoyant_kernel::engine_data::FilteredEngineData::selection_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selection_vector(&self) -> &[bool]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L53).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns a reference to the selection vector.

<a id="op-eebde44bb41975a342467a7f"></a>
## try_new

`function` · `buoyant_kernel::engine_data::FilteredEngineData::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(data: Box<dyn EngineData>, selection_vector: Vec<bool>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L33).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:33`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43a337318704e894197d3a77"></a>
## with_all_rows_selected

`function` · `buoyant_kernel::engine_data::FilteredEngineData::with_all_rows_selected` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_all_rows_selected(data: Box<dyn EngineData>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L66).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine_data::FilteredEngineData", "path": "FilteredEngineData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [78, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new `FilteredEngineData` with all rows selected.

This is a convenience method for the common case where you want to wrap
`EngineData` in `FilteredEngineData` without any filtering.

<a id="op-0319d9ddd1f232b9c2810376"></a>
## data

`struct_field` · `buoyant_kernel::engine_data::FilteredEngineData::data` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data: Box<dyn EngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L25).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:25`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f830760d113c174b67d46a51"></a>
## selection_vector

`struct_field` · `buoyant_kernel::engine_data::FilteredEngineData::selection_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
selection_vector: Vec<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L29).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:29`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
