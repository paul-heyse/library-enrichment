# `buoyant_kernel::actions::visitors`

Crate `buoyant_kernel` · 9 public items · structured records in [`model/buoyant_kernel.actions.visitors.json`](../model/buoyant_kernel.actions.visitors.json)

## visit_metadata_at

`function` · `buoyant_kernel::actions::visitors::visit_metadata_at`

Also reachable as `delta_kernel::actions::visitors::visit_metadata_at`

```rust
fn visit_metadata_at<'a>(row_index: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Option<Metadata>>
```

Get a Metadata out of some engine data. Note that Ok(None) is returned if there is no Metadata
found. The caller is responsible for slicing the `getters` slice such that the first element
contains the `id` element of the metadata.

---

## visit_protocol_at

`function` · `buoyant_kernel::actions::visitors::visit_protocol_at`

Also reachable as `delta_kernel::actions::visitors::visit_protocol_at`

```rust
fn visit_protocol_at<'a>(row_index: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Option<Protocol>>
```

Get a Protocol out of some engine data. Note that Ok(None) is returned if there is no Protocol
found. The caller is responsible for slicing the `getters` slice such that the first element
contains the `min_reader_version` element of the protocol.

---

## AddVisitor

`struct` · `buoyant_kernel::actions::visitors::AddVisitor`

Also reachable as `delta_kernel::actions::visitors::AddVisitor`

```rust
struct AddVisitor
```

**Implements**: `buoyant_kernel::engine_data::RowVisitor`

**Derives**: Default

**Methods** (1)

```rust
fn visit_add<'a>(row_index: usize, path: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Add>
```

**via `buoyant_kernel::engine_data::RowVisitor`**

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

---

## CdcVisitor

`struct` · `buoyant_kernel::actions::visitors::CdcVisitor`

Also reachable as `delta_kernel::actions::visitors::CdcVisitor`

```rust
struct CdcVisitor
```

**Implements**: `buoyant_kernel::engine_data::RowVisitor`

**Derives**: Default

**Methods** (1)

```rust
fn visit_cdc<'a>(row_index: usize, path: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Cdc>
```

**via `buoyant_kernel::engine_data::RowVisitor`**

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

---

## MetadataVisitor

`struct` · `buoyant_kernel::actions::visitors::MetadataVisitor`

Also reachable as `delta_kernel::actions::visitors::MetadataVisitor`

```rust
struct MetadataVisitor
```

**Implements**: `buoyant_kernel::engine_data::RowVisitor`

**Derives**: Default

**via `buoyant_kernel::engine_data::RowVisitor`**

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

---

## ProtocolVisitor

`struct` · `buoyant_kernel::actions::visitors::ProtocolVisitor`

Also reachable as `delta_kernel::actions::visitors::ProtocolVisitor`

```rust
struct ProtocolVisitor
```

**Implements**: `buoyant_kernel::engine_data::RowVisitor`

**Derives**: Default

**via `buoyant_kernel::engine_data::RowVisitor`**

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

---

## RemoveVisitor

`struct` · `buoyant_kernel::actions::visitors::RemoveVisitor`

Also reachable as `delta_kernel::actions::visitors::RemoveVisitor`

```rust
struct RemoveVisitor
```

**Implements**: `buoyant_kernel::engine_data::RowVisitor`

**Derives**: Default

**Methods** (1)

```rust
fn visit_remove<'a>(row_index: usize, path: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Remove>
```

**via `buoyant_kernel::engine_data::RowVisitor`**

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

---

## SetTransactionVisitor

`struct` · `buoyant_kernel::actions::visitors::SetTransactionVisitor`

Also reachable as `delta_kernel::actions::visitors::SetTransactionVisitor`

```rust
struct SetTransactionVisitor
```

**Implements**: `buoyant_kernel::engine_data::RowVisitor`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn visit_txn<'a>(row_index: usize, app_id: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<SetTransaction>
```

**via `buoyant_kernel::engine_data::RowVisitor`**

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

Extract application transaction actions from the log into a map

This visitor maintains the first entry for each application id it
encounters.  When a specific application id is required then
`application_id` can be set. This bounds the memory required for the
visitor to at most one entry and reduces the amount of processing
required.

---

## SidecarVisitor

`struct` · `buoyant_kernel::actions::visitors::SidecarVisitor`

Also reachable as `delta_kernel::actions::visitors::SidecarVisitor`

```rust
struct SidecarVisitor
```

**Implements**: `buoyant_kernel::engine_data::RowVisitor`

**Derives**: Default

**Methods** (1)

```rust
fn visit_sidecar<'a>(row_index: usize, path: String, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<Sidecar>
```

**via `buoyant_kernel::engine_data::RowVisitor`**

```rust
fn selected_column_names_and_types(&self) -> (&'static [ColumnName], &'static [DataType])
fn visit<'a>(&mut self, row_count: usize, getters: &[&'a dyn GetData<'a>]) -> DeltaResult<()>
```

---
