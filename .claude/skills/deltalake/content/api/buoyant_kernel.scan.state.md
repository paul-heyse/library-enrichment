# `buoyant_kernel::scan::state`

Crate `buoyant_kernel` · 5 public items · structured records in [`model/buoyant_kernel.scan.state.json`](../model/buoyant_kernel.scan.state.json)

## transform_to_logical

`function` · `buoyant_kernel::scan::state::transform_to_logical`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.state.transform_to_logical.md)

Also reachable as `delta_kernel::scan::state::transform_to_logical`

```rust
fn transform_to_logical(engine: &dyn Engine, physical_data: Box<dyn EngineData>, physical_schema: &schema::SchemaRef, logical_schema: &schema::Schema, transform: Option<ExpressionRef>) -> DeltaResult<Box<dyn EngineData>>
```

utility function for applying a transform expression to convert data from physical to logical
format

---

## DvInfo

`struct` · `buoyant_kernel::scan::state::DvInfo`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.state.DvInfo.md)

Also reachable as `delta_kernel::scan::state::DvInfo`

```rust
struct DvInfo
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn get_row_indexes(&self, engine: &dyn Engine, table_root: &url::Url) -> DeltaResult<Option<Vec<u64>>>
fn get_selection_vector(&self, engine: &dyn Engine, table_root: &url::Url) -> DeltaResult<Option<Vec<bool>>>
fn has_vector(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(deletion_vector: DeletionVectorDescriptor) -> Self
```

this struct can be used by an engine to materialize a selection vector

---

## ScanFile

`struct` · `buoyant_kernel::scan::state::ScanFile`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.state.ScanFile.md)

Also reachable as `delta_kernel::scan::state::ScanFile`

```rust
struct ScanFile
```

**Fields**: `path`, `size`, `modification_time`, `stats`, `dv_info`, `transform`, `partition_values`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

A `ScanFile` represents information about one file that needs to be scanned to read a table.

---

## Stats

`struct` · `buoyant_kernel::scan::state::Stats`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.state.Stats.md)

Also reachable as `delta_kernel::scan::state::Stats`

```rust
struct Stats
```

**Fields**: `num_records`

**Implements**: `serde_core::de::Deserialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Give engines an easy way to consume stats

---

## ScanCallback

`type_alias` · `buoyant_kernel::scan::state::ScanCallback`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.scan.state.ScanCallback.md)

Also reachable as `delta_kernel::scan::state::ScanCallback`

```rust
type ScanCallback<T> = fn(&mut T, ScanFile)
```

---
