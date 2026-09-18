# `deltalake_core::delta_datafusion`

Crate `deltalake-core` · 5 public items · structured records in [`model/deltalake_core.delta_datafusion.json`](../model/deltalake_core.delta_datafusion.json)

## DeltaColumn

`struct` · `deltalake_core::delta_datafusion::DeltaColumn`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.DeltaColumn.md)

Also reachable as `deltalake::delta_datafusion::DeltaColumn`

```rust
struct DeltaColumn
```

**Implements**: `core::convert::From`

**via `core::convert::From`**

```rust
fn from(c: &String) -> Self
fn from(c: Column) -> Self
fn from(c: &str) -> Self
fn from(c: String) -> Self
```

A wrapper for Deltafusion's Column to preserve case-sensitivity during string conversion

---

## DeltaLogicalCodec

`struct` · `deltalake_core::delta_datafusion::DeltaLogicalCodec`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.DeltaLogicalCodec.md)

Also reachable as `deltalake::delta_datafusion::DeltaLogicalCodec`

```rust
struct DeltaLogicalCodec
```

**Implements**: `datafusion_proto::logical_plan::LogicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::logical_plan::LogicalExtensionCodec`**

```rust
fn try_decode(&self, _buf: &[u8], _inputs: &[LogicalPlan], _ctx: &TaskContext) -> Result<Extension, DataFusionError>
fn try_decode_table_provider(&self, buf: &[u8], _table_ref: &TableReference, _schema: SchemaRef, _ctx: &TaskContext) -> Result<Arc<dyn TableProvider>, DataFusionError>
fn try_encode(&self, _node: &Extension, _buf: &mut Vec<u8>) -> Result<(), DataFusionError>
fn try_encode_table_provider(&self, _table_ref: &TableReference, node: Arc<dyn TableProvider>, buf: &mut Vec<u8>) -> Result<(), DataFusionError>
```

Does serde on DeltaTables

---

## DeltaPhysicalCodec

`struct` · `deltalake_core::delta_datafusion::DeltaPhysicalCodec`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.DeltaPhysicalCodec.md)

> **Deprecated** — DeltaPhysicalCodec only supports the retired physical DeltaScan wrapper. Use DeltaLogicalCodec for table-provider serialization until a DeltaScanExec physical codec is available.

Also reachable as `deltalake::delta_datafusion::DeltaPhysicalCodec`

```rust
struct DeltaPhysicalCodec
```

**Implements**: `datafusion_proto::physical_plan::PhysicalExtensionCodec`

**Derives**: Debug

**via `datafusion_proto::physical_plan::PhysicalExtensionCodec`**

```rust
fn try_decode(&self, buf: &[u8], inputs: &[Arc<dyn ExecutionPlan>], _registry: &TaskContext, _converter: &dyn PhysicalProtoConverterExtension) -> Result<Arc<dyn ExecutionPlan>, DataFusionError>
fn try_encode(&self, node: Arc<dyn ExecutionPlan>, buf: &mut Vec<u8>, _converter: &dyn PhysicalProtoConverterExtension) -> Result<(), DataFusionError>
```

Legacy codec for serialized plans that still contain the retired physical
[`DeltaScan`] wrapper.

---

## DeltaTableFactory

`struct` · `deltalake_core::delta_datafusion::DeltaTableFactory`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.DeltaTableFactory.md)

Also reachable as `deltalake::delta_datafusion::DeltaTableFactory`

```rust
struct DeltaTableFactory
```

**Implements**: `datafusion_session::table::TableProviderFactory`

**Derives**: Debug

**via `datafusion_session::table::TableProviderFactory`**

```rust
async fn create(&self, ctx: &dyn Session, cmd: &CreateExternalTable) -> datafusion::error::Result<Arc<dyn TableProvider>>
```

Responsible for creating deltatables

---

## DataFusionMixins

`trait` · `deltalake_core::delta_datafusion::DataFusionMixins`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.DataFusionMixins.md)

Also reachable as `deltalake::delta_datafusion::DataFusionMixins`

```rust
trait DataFusionMixins
```

**Implementors** (3)

- `deltalake_core::kernel::snapshot::EagerSnapshot`
- `deltalake_core::kernel::snapshot::Snapshot`
- `deltalake_core::kernel::snapshot::log_data::LogDataHandler`

**Methods** (3)

```rust
fn input_schema(&self) -> ArrowSchemaRef
fn parse_predicate_expression(&self, expr: impl AsRef<str>, session: &dyn Session) -> DeltaResult<Expr>
fn read_schema(&self) -> ArrowSchemaRef
```

Convenience trait for calling common methods on snapshot hierarchies

---
