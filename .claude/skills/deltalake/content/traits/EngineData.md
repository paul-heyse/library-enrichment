# EngineData

`buoyant_kernel::engine_data::EngineData`

```rust
trait EngineData: AsAny
```

Also reachable as `buoyant_kernel::EngineData`, `delta_kernel::engine_data::EngineData`

Prose: [`api/buoyant_kernel.engine_data.md`](../api/buoyant_kernel.engine_data.md#enginedata) · records: [`model/buoyant_kernel.engine_data.json`](../model/buoyant_kernel.engine_data.json)

## Required

Every implementation must supply these.

```rust
fn append_columns(&self, schema: SchemaRef, columns: Vec<ArrayData>) -> DeltaResult<Box<dyn EngineData>>
fn apply_selection_vector(Box<self>, selection_vector: Vec<bool>) -> DeltaResult<Box<dyn EngineData>>
fn has_field(&self, name: &ColumnName) -> bool
fn len(&self) -> usize
fn visit_rows(&self, column_names: &[ColumnName], visitor: &mut dyn RowVisitor) -> DeltaResult<()>
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn is_empty(&self) -> bool
```

## Implementors (1)

Read one before writing your own.

- `buoyant_kernel::engine::arrow_data::ArrowEngineData`

## Documentation

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
