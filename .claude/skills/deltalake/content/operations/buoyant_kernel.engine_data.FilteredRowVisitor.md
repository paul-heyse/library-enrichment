# `buoyant_kernel::engine_data::FilteredRowVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine_data.FilteredRowVisitor.json).

<a id="op-f783829cb3af5ac14ac1be1d"></a>
## FilteredRowVisitor

`trait` · `buoyant_kernel::engine_data::FilteredRowVisitor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait FilteredRowVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L398).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:398`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A visitor that processes [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab) with automatic row filtering.

Implementors provide [`visit_filtered`] which receives the column getters and a
[`RowIndexIterator`](../operations/buoyant_kernel.engine_data.RowIndexIterator.md#op-7304f0f62a800091dc7533b9) that yields the index of each selected row.
The default [`visit_rows_of`] method handles all the plumbing: extracting the selection
vector, building the bridge, and calling [`EngineData::visit_rows`](../operations/buoyant_kernel.engine_data.EngineData.md#op-570353c5be5947995201ad57).

[`visit_filtered`]: FilteredRowVisitor::visit_filtered
[`visit_rows_of`]: FilteredRowVisitor::visit_rows_of

<a id="op-b4ff3e8058514f5501f9e829"></a>
## selected_column_names_and_types

`function` · `buoyant_kernel::engine_data::FilteredRowVisitor::selected_column_names_and_types` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L399).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:399`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bde2215c626be8fb0850d46"></a>
## visit_filtered

`function` · `buoyant_kernel::engine_data::FilteredRowVisitor::visit_filtered` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_filtered<'a>(&mut self, getters: &[&'a dyn GetData<'a>], rows: RowIndexIterator<'_>) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L405).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:405`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Process this batch. `getters` contains one [`GetData`](../operations/buoyant_kernel.engine_data.GetData.md#op-d74ee987c25b271c4d53e148) item per requested column.
Iterate `rows` to receive the index of each selected row. Use
[`RowIndexIterator::num_rows`](../operations/buoyant_kernel.engine_data.RowIndexIterator.md#op-b4d3620b4bfee244621e0c54) to get the total row count (for padding output
vectors with null values for deselected rows).

<a id="op-803ad728f50242700845099a"></a>
## visit_rows_of

`function` · `buoyant_kernel::engine_data::FilteredRowVisitor::visit_rows_of` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_rows_of(&mut self, data: &FilteredEngineData) -> DeltaResult<()> where Self: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L415).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:415`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Visit the rows of a [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab), automatically respecting the selection vector.

Extracts the selection vector and passes a [`RowIndexIterator`](../operations/buoyant_kernel.engine_data.RowIndexIterator.md#op-7304f0f62a800091dc7533b9) of selected row indices
to [`FilteredRowVisitor::visit_filtered`](../operations/buoyant_kernel.engine_data.FilteredRowVisitor.md#op-1bde2215c626be8fb0850d46).
