# `buoyant_kernel::engine_data::RowVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine_data.RowVisitor.json).

<a id="op-4cb6a6c8c9e1c025811a750f"></a>
## RowVisitor

`trait` · `buoyant_kernel::engine_data::RowVisitor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait RowVisitor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L449).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:449`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A `RowVisitor` can be called back to visit extracted data. Aside from calling
[`RowVisitor::visit`](../operations/buoyant_kernel.engine_data.RowVisitor.md#op-b95deab29a27b5d47e69a03a) on the visitor passed to [`EngineData::visit_rows`](../operations/buoyant_kernel.engine_data.EngineData.md#op-570353c5be5947995201ad57), engines do
not need to worry about this trait.

<a id="op-1c45553b85ac6f485f1d2f61"></a>
## selected_column_names_and_types

`function` · `buoyant_kernel::engine_data::RowVisitor::selected_column_names_and_types` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L455).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:455`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The names and types of leaf fields this visitor accesses. The `EngineData` being visited
validates these types when extracting column getters, and [`RowVisitor::visit`](../operations/buoyant_kernel.engine_data.RowVisitor.md#op-b95deab29a27b5d47e69a03a) will receive
one getter for each selected field, in the requested order. The column names are used by
[`RowVisitor::visit_rows_of`](../operations/buoyant_kernel.engine_data.RowVisitor.md#op-9e5d80d6ca43cadfdfeffd81) to select fields from a "typical" `EngineData`; callers whose
engine data has different column names can manually invoke [`EngineData::visit_rows`](../operations/buoyant_kernel.engine_data.EngineData.md#op-570353c5be5947995201ad57).

<a id="op-b95deab29a27b5d47e69a03a"></a>
## visit

`function` · `buoyant_kernel::engine_data::RowVisitor::visit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L463).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:463`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Have the visitor visit the data. This will be called on a visitor passed to
[`EngineData::visit_rows`](../operations/buoyant_kernel.engine_data.EngineData.md#op-570353c5be5947995201ad57). For each leaf in the schema that was passed to `extract` a
"getter" of type [`GetData`](../operations/buoyant_kernel.engine_data.GetData.md#op-d74ee987c25b271c4d53e148) will be present. This can be used to actually get at the data
for each row. You can `use` the `TypedGetData` trait if you want to have a way to extract
typed data that will fail if the "getter" is for an unexpected type.  The data in `getters`
does not outlive the call to this function (i.e. it should be copied if needed).

<a id="op-9e5d80d6ca43cadfdfeffd81"></a>
## visit_rows_of

`function` · `buoyant_kernel::engine_data::RowVisitor::visit_rows_of` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_rows_of(&mut self, data: &dyn EngineData) -> DeltaResult<()> where Self: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L468).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:468`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Visit the rows of an [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809), selecting the leaf column names given by
[`RowVisitor::selected_column_names_and_types`](../operations/buoyant_kernel.engine_data.RowVisitor.md#op-1c45553b85ac6f485f1d2f61). This is a thin wrapper around
[`EngineData::visit_rows`](../operations/buoyant_kernel.engine_data.EngineData.md#op-570353c5be5947995201ad57) which in turn will eventually invoke [`RowVisitor::visit`](../operations/buoyant_kernel.engine_data.RowVisitor.md#op-b95deab29a27b5d47e69a03a).
