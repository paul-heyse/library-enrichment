# `buoyant_kernel::engine_data::EngineData`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine_data.EngineData.json).

<a id="op-f9036ab52623c01f96e1f809"></a>
## EngineData

`trait` · `buoyant_kernel::engine_data::EngineData` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait EngineData: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L513).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:513`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Any type that an engine wants to return as "data" needs to implement this trait. The bulk of the
work is in the [`EngineData::visit_rows`](../operations/buoyant_kernel.engine_data.EngineData.md#op-570353c5be5947995201ad57) method. See the docs for that method for more details.
```rust
# use buoyant_kernel as delta_kernel;
# use std::any::Any;
# use delta_kernel::DeltaResult;
# use delta_kernel::engine_data::{RowVisitor, EngineData, GetData};
# use delta_kernel::expressions::{ArrayData, ColumnName};
# use delta_kernel::schema::SchemaRef;
struct MyDataType; // Whatever the engine wants here
impl MyDataType {
  fn do_extraction<'a>(&self) -> Vec<&'a dyn GetData<'a>> {
     /// Actually do the extraction into getters
     todo!()
  }
}

impl EngineData for MyDataType {
  fn visit_rows(&self, leaf_columns: &[ColumnName], visitor: &mut dyn RowVisitor) -> DeltaResult<()> {
    let getters = self.do_extraction(); // do the extraction
    visitor.visit(self.len(), &getters); // call the visitor back with the getters
    Ok(())
  }
  fn len(&self) -> usize {
    todo!() // actually get the len here
  }
  fn append_columns(&self, schema: SchemaRef, columns: Vec<ArrayData>) -> DeltaResult<Box<dyn EngineData>> {
    todo!() // convert `SchemaRef` and `ArrayData` into local representation and append them
  }
  fn apply_selection_vector(self: Box<Self>, selection_vector: Vec<bool>) -> DeltaResult<Box<dyn EngineData>> {
    todo!() // filter out unselected rows; rows beyond the selection vector's end are selected
  }
  fn has_field(&self, name: &ColumnName) -> bool {
    todo!() // determine whether the field exists in the data
  }
}
```

<a id="op-c891d8ad6d4b19f183f8ec34"></a>
## append_columns

`function` · `buoyant_kernel::engine_data::EngineData::append_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn append_columns(&self, schema: SchemaRef, columns: Vec<ArrayData>) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L552).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:552`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Append new columns provided by Kernel to the existing data.

This method creates a new [`EngineData`](../operations/buoyant_kernel.engine_data.EngineData.md#op-f9036ab52623c01f96e1f809) instance that combines the existing columns
with the provided new columns. The original data remains unchanged.

# Parameters
- `schema`: The schema of the columns being appended (not the entire resulting schema). This
  schema must describe exactly the columns being added in the `columns` parameter.
- `columns`: The column data to append. Each [`ArrayData`](../operations/buoyant_kernel.expressions.scalars.ArrayData.md#op-a5aa855d83dfb4d57e0420d7) corresponds to one field in the
  schema.

# Returns
A new `EngineData` instance containing both the original columns and the appended columns.
The schema of the result will contain all original fields followed by the new schema fields.

# Errors
Returns an error if:
- The number of rows in any appended column doesn't match the existing data.
- The number of new columns doesn't match the number of schema fields.
- Data type conversion to the engine's native data types fails.
- The engine cannot create the combined data structure.

<a id="op-1e67ac80541d2398db9da79f"></a>
## apply_selection_vector

`function` · `buoyant_kernel::engine_data::EngineData::apply_selection_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn apply_selection_vector(Box<self>, selection_vector: Vec<bool>) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L566).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:566`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Apply a selection vector to the data and return a data where only the selected rows are
included. This consumes the EngineData, allowing engines to implement this "in place" if
desired.

The selection vector may be shorter than the data; rows beyond its end are selected and
must be retained (see [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab)). An empty selection vector selects all rows.
A selection vector longer than the data is invalid and must be rejected, e.g. with
[`Error::InvalidSelectionVector`](../operations/buoyant_kernel.error.Error.md#op-2e1cb840440643bea6559dfa).

<a id="op-ac824a3c381a3bf53376b4a6"></a>
## has_field

`function` · `buoyant_kernel::engine_data::EngineData::has_field` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_field(&self, name: &ColumnName) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L575).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:575`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if a field at the given (possibly nested) path exists in this data's schema.

For a top-level field named `"foo"`, use `ColumnName::new(["foo"])`. For nested fields,
each non-leaf element of the path must be a struct field at that level.

<a id="op-6d47e01294b800c48ca93c11"></a>
## is_empty

`function` · `buoyant_kernel::engine_data::EngineData::is_empty` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_empty(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L527).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:527`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if the data is empty (i.e., has no rows).

<a id="op-0aeeda6dd3a828033c3dd5a2"></a>
## len

`function` · `buoyant_kernel::engine_data::EngineData::len` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn len(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L524).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:524`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Return the number of items (rows) in blob

<a id="op-570353c5be5947995201ad57"></a>
## visit_rows

`function` · `buoyant_kernel::engine_data::EngineData::visit_rows` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_rows(&self, column_names: &[ColumnName], visitor: &mut dyn RowVisitor) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L517).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:517`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Visits a subset of leaf columns in each row of this data, passing a `GetData` item for each
requested column to the visitor's `visit` method (along with the number of rows of data to
be visited).
