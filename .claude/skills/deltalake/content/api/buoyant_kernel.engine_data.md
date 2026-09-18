# `buoyant_kernel::engine_data`

Crate `buoyant_kernel` · 10 public items · structured records in [`model/buoyant_kernel.engine_data.json`](../model/buoyant_kernel.engine_data.json)

## FilteredEngineData

`struct` · `buoyant_kernel::engine_data::FilteredEngineData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.FilteredEngineData.md)

Also reachable as `buoyant_kernel::FilteredEngineData`, `delta_kernel::engine_data::FilteredEngineData`

```rust
struct FilteredEngineData
```

**Implements**: `buoyant_kernel::log_replay::HasSelectionVector`, `core::convert::From`

**Methods** (6)

```rust
fn apply_selection_vector(self) -> DeltaResult<Box<dyn EngineData>>
fn data(&self) -> &dyn EngineData
fn into_parts(self) -> (Box<dyn EngineData>, Vec<bool>)
fn selection_vector(&self) -> &[bool]
fn try_new(data: Box<dyn EngineData>, selection_vector: Vec<bool>) -> DeltaResult<Self>
fn with_all_rows_selected(data: Box<dyn EngineData>) -> Self
```

**via `buoyant_kernel::log_replay::HasSelectionVector`**

```rust
fn has_selected_rows(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(data: Box<dyn EngineData>) -> Self
```

Engine data paired with a selection vector indicating which rows are logically selected.

A value of `true` in the selection vector means the corresponding row is selected (i.e., not
deleted), while `false` means the row is logically deleted and should be ignored. If the
selection vector is shorter than the number of rows in `data` then all rows not covered by the
selection vector are assumed to be selected.

Interpreting unselected (`false`) rows will result in incorrect/undefined behavior.

---

## ListItem

`struct` · `buoyant_kernel::engine_data::ListItem`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.ListItem.md)

Also reachable as `delta_kernel::engine_data::ListItem`

```rust
struct ListItem<'a>
```

**Methods** (5)

```rust
fn get(&self, list_index: usize) -> String
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn materialize(&self) -> Vec<String>
fn new(values: &'a dyn StringArrayAccessor, offsets: Range<usize>) -> ListItem<'a>
```

A pre-resolved view into a single row's list of strings. The string array type is resolved
once at construction, so subsequent element accesses use virtual dispatch rather than
repeated downcasting.

---

## MapItem

`struct` · `buoyant_kernel::engine_data::MapItem`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.MapItem.md)

Also reachable as `delta_kernel::engine_data::MapItem`

```rust
struct MapItem<'a>
```

**Methods** (3)

```rust
fn get(&self, key: &str) -> Option<&'a str>
fn materialize(&self) -> HashMap<String, String>
fn new(keys: &'a dyn StringArrayAccessor, values: &'a dyn StringArrayAccessor, offsets: Range<usize>) -> MapItem<'a>
```

A pre-resolved view into a single row's map of string keys to string values. Like
[`ListItem`], the string array types are resolved once at construction.

Note: in conjunction with the `allow_null_container_values` attribute, [`materialize`]
_drops_ any (key, value) pairs where the underlying value was null. If preserving null
values is important, use the `allow_null_container_values` attribute and manually
materialize the map using [`MapItem::get`].

[`materialize`]: MapItem::materialize

---

## RowIndexIterator

`struct` · `buoyant_kernel::engine_data::RowIndexIterator`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.RowIndexIterator.md)

Also reachable as `buoyant_kernel::RowIndexIterator`, `delta_kernel::engine_data::RowIndexIterator`

```rust
struct RowIndexIterator<'sv>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (1)

```rust
fn num_rows(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<usize>
```

An iterator over the indices of selected rows in an engine-data batch.

Each call to [`Iterator::next`] returns the index of the next selected row.

Constructed internally and passed (alongside the column getters) to
[`FilteredRowVisitor::visit_filtered`].

---

## EngineData

`trait` · `buoyant_kernel::engine_data::EngineData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.EngineData.md)

Also reachable as `buoyant_kernel::EngineData`, `delta_kernel::engine_data::EngineData`

```rust
trait EngineData: AsAny
```

**Implementors** (1)

- `buoyant_kernel::engine::arrow_data::ArrowEngineData`

**Methods** (6)

```rust
fn append_columns(&self, schema: SchemaRef, columns: Vec<ArrayData>) -> DeltaResult<Box<dyn EngineData>>
fn apply_selection_vector(Box<self>, selection_vector: Vec<bool>) -> DeltaResult<Box<dyn EngineData>>
fn has_field(&self, name: &ColumnName) -> bool
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn visit_rows(&self, column_names: &[ColumnName], visitor: &mut dyn RowVisitor) -> DeltaResult<()>
```

Any type that an engine wants to return as "data" needs to implement this trait. The bulk of the
work is in the [`EngineData::visit_rows`] method. See the docs for that method for more details.
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

---

## FilteredRowVisitor

`trait` · `buoyant_kernel::engine_data::FilteredRowVisitor`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.FilteredRowVisitor.md)

Also reachable as `buoyant_kernel::FilteredRowVisitor`, `delta_kernel::engine_data::FilteredRowVisitor`

```rust
trait FilteredRowVisitor
```

**Implementors** (2)

- `buoyant_kernel::scan::state::ScanFileVisitor`
- `buoyant_kernel::transaction::update::DvMatchVisitor`

**Methods** (3)

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit_filtered<'a>(&mut self, getters: &[&'a dyn GetData<'a>], rows: RowIndexIterator<'_>) -> DeltaResult<()>
fn visit_rows_of(&mut self, data: &FilteredEngineData) -> DeltaResult<()> where Self: Sized
```

A visitor that processes [`FilteredEngineData`] with automatic row filtering.

Implementors provide [`visit_filtered`] which receives the column getters and a
[`RowIndexIterator`] that yields the index of each selected row.
The default [`visit_rows_of`] method handles all the plumbing: extracting the selection
vector, building the bridge, and calling [`EngineData::visit_rows`].

[`visit_filtered`]: FilteredRowVisitor::visit_filtered
[`visit_rows_of`]: FilteredRowVisitor::visit_rows_of

---

## GetData

`trait` · `buoyant_kernel::engine_data::GetData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.GetData.md)

Also reachable as `buoyant_kernel::GetData`, `delta_kernel::engine_data::GetData`

```rust
trait GetData<'a>
```

**Implementors** (9)

- `arrow_array::array::boolean_array::BooleanArray`
- `arrow_array::array::byte_array::GenericByteArray`
- `arrow_array::array::byte_view_array::BinaryViewArray`
- `arrow_array::array::byte_view_array::StringViewArray`
- `arrow_array::array::list_array::GenericListArray`
- `arrow_array::array::list_view_array::GenericListViewArray`
- `arrow_array::array::map_array::MapArray`
- `arrow_array::array::primitive_array::PrimitiveArray`
- `arrow_array::array::run_array::RunArray`

**Methods** (14)

```rust
fn get_binary(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<&'a [u8]>>
fn get_bool(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<bool>>
fn get_byte(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<i8>>
fn get_date(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<i32>>
fn get_decimal(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<i128>>
fn get_double(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<f64>>
fn get_float(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<f32>>
fn get_int(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<i32>>
fn get_list(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<ListItem<'a>>>
fn get_long(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<i64>>
fn get_map(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<MapItem<'a>>>
fn get_short(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<i16>>
fn get_str(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<&'a str>>
fn get_timestamp(&'a self, _row_index: usize, field_name: &str) -> DeltaResult<Option<i64>>
```

When calling back into a [`RowVisitor`], the engine needs to provide a slice of items that
implement this trait. This allows type_safe extraction from the raw data by the kernel. By
default all these methods will return an `Error` that an incorrect type has been asked
for. Therefore, for each "data container" an Engine has, it is only necessary to implement the
`get_x` method for the type it holds.

---

## RowVisitor

`trait` · `buoyant_kernel::engine_data::RowVisitor`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.RowVisitor.md)

Also reachable as `buoyant_kernel::RowVisitor`, `delta_kernel::engine_data::RowVisitor`

```rust
trait RowVisitor
```

**Implementors** (22)

- `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationVisitor`
- `buoyant_kernel::actions::visitors::AddVisitor`
- `buoyant_kernel::actions::visitors::CdcVisitor`
- `buoyant_kernel::actions::visitors::DomainMetadataVisitor`
- `buoyant_kernel::actions::visitors::InCommitTimestampVisitor`
- `buoyant_kernel::actions::visitors::MetadataVisitor`
- `buoyant_kernel::actions::visitors::ProtocolVisitor`
- `buoyant_kernel::actions::visitors::RemoveVisitor`
- `buoyant_kernel::actions::visitors::SelectionVectorVisitor`
- `buoyant_kernel::actions::visitors::SetTransactionVisitor`
- `buoyant_kernel::actions::visitors::SidecarVisitor`
- `buoyant_kernel::crc::file_stats::FileStatsVisitor`
- `buoyant_kernel::engine_data::FilteredVisitorBridge`
- `buoyant_kernel::incremental_scan::IncrementalDedupVisitor`
- `buoyant_kernel::log_segment::crc_replay::CrcReplayVisitor`
- `buoyant_kernel::row_tracking::RowTrackingVisitor`
- `buoyant_kernel::scan::log_replay::AddRemoveDedupVisitor`
- `buoyant_kernel::table_changes::log_replay::FileActionSelectionVisitor`
- `buoyant_kernel::table_changes::log_replay::PreparePhaseVisitor`
- `buoyant_kernel::table_changes::scan_file::CdfScanFileVisitor`
- `buoyant_kernel::transaction::stats_verifier::ColumnStatsValidator`
- `buoyant_kernel::transaction::stats_verifier::NumRecordsValidator`

**Methods** (3)

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
fn visit_rows_of(&mut self, data: &dyn EngineData) -> DeltaResult<()> where Self: Sized
```

A `RowVisitor` can be called back to visit extracted data. Aside from calling
[`RowVisitor::visit`] on the visitor passed to [`EngineData::visit_rows`], engines do
not need to worry about this trait.

---

## StringArrayAccessor

`trait` · `buoyant_kernel::engine_data::StringArrayAccessor`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.StringArrayAccessor.md)

Also reachable as `delta_kernel::engine_data::StringArrayAccessor`

```rust
trait StringArrayAccessor
```

**Implementors** (2)

- `arrow_array::array::byte_array::GenericByteArray`
- `arrow_array::array::byte_view_array::StringViewArray`

**Methods** (4)

```rust
fn is_empty(&self) -> bool
fn is_valid(&self, index: usize) -> bool
fn len(&self) -> usize
fn value(&self, index: usize) -> &str
```

Uniform read access to a string array, abstracting over the various string representations
that list and map columns may use (e.g. Utf8, LargeUtf8, Utf8View). Engines implement this
for their string array types so that [`ListItem`] and [`MapItem`] can resolve the concrete
type once at construction and access elements via virtual dispatch thereafter.

---

## TypedGetData

`trait` · `buoyant_kernel::engine_data::TypedGetData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine_data.TypedGetData.md)

Also reachable as `delta_kernel::engine_data::TypedGetData`

```rust
trait TypedGetData<'a, T>
```

**Methods** (2)

```rust
fn get(&'a self, row_index: usize, field_name: &str) -> DeltaResult<T>
fn get_opt(&'a self, row_index: usize, field_name: &str) -> DeltaResult<Option<T>>
```

This is a convenience wrapper over `GetData` to allow code like: `let name: Option<String> =
getters[1].get_opt(row_index, "metadata.name")?;`

---
