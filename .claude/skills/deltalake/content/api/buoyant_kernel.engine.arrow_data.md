# `buoyant_kernel::engine::arrow_data`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.engine.arrow_data.json`](../model/buoyant_kernel.engine.arrow_data.json)

## ArrowEngineData

`struct` · `buoyant_kernel::engine::arrow_data::ArrowEngineData`

Also reachable as `delta_kernel::engine::arrow_data::ArrowEngineData`

```rust
struct ArrowEngineData
```

**Implements**: `buoyant_kernel::engine_data::EngineData`, `core::convert::From`

**Methods** (3)

```rust
fn new(data: RecordBatch) -> Self
fn record_batch(&self) -> &RecordBatch
fn try_from_engine_data(engine_data: Box<dyn EngineData>) -> DeltaResult<Box<Self>>
```

**via `buoyant_kernel::engine_data::EngineData`**

```rust
fn append_columns(&self, schema: SchemaRef, columns: Vec<ArrayData>) -> DeltaResult<Box<dyn EngineData>>
fn apply_selection_vector(Box<self>, selection_vector: Vec<bool>) -> DeltaResult<Box<dyn EngineData>>
fn has_field(&self, name: &ColumnName) -> bool
fn len(&self) -> usize
fn visit_rows(&self, leaf_columns: &[ColumnName], visitor: &mut dyn RowVisitor) -> DeltaResult<()>
```

**via `core::convert::From`**

```rust
fn from(value: StructArray) -> Self
fn from(value: RecordBatch) -> Self
```

ArrowEngineData holds an Arrow `RecordBatch`, implements `EngineData` so the kernel can extract
from it.

WARNING: Row visitors require that all leaf columns of the record batch have correctly computed
NULL masks. The arrow parquet reader is known to produce incomplete NULL masks, for
example. When in doubt, call [`fix_nested_null_masks`] first.

---

## EngineDataArrowExt

`trait` · `buoyant_kernel::engine::arrow_data::EngineDataArrowExt`

Also reachable as `delta_kernel::engine::arrow_data::EngineDataArrowExt`

```rust
trait EngineDataArrowExt
```

**Implementors** (2)

- `alloc::boxed::Box`
- `buoyant_kernel::error::DeltaResult`

**Methods** (1)

```rust
fn try_into_record_batch(self) -> DeltaResult<RecordBatch>
```

A trait to allow easy conversion from [`EngineData`] to an arrow [``RecordBatch`]. Returns an
error if called on an `EngineData` that is not an `ArrowEngineData`.

---
