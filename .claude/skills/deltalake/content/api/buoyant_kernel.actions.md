# `buoyant_kernel::actions`

Crate `buoyant_kernel` · 37 public items · structured records in [`model/buoyant_kernel.actions.json`](../model/buoyant_kernel.actions.json)

## ADD_NAME

`constant` · `buoyant_kernel::actions::ADD_NAME`

Also reachable as `delta_kernel::actions::ADD_NAME`

```rust
const ADD_NAME: &str = "add"
```

---

## CDC_NAME

`constant` · `buoyant_kernel::actions::CDC_NAME`

Also reachable as `delta_kernel::actions::CDC_NAME`

```rust
const CDC_NAME: &str = "cdc"
```

---

## CHECKPOINT_METADATA_NAME

`constant` · `buoyant_kernel::actions::CHECKPOINT_METADATA_NAME`

Also reachable as `delta_kernel::actions::CHECKPOINT_METADATA_NAME`

```rust
const CHECKPOINT_METADATA_NAME: &str = "checkpointMetadata"
```

---

## COMMIT_INFO_NAME

`constant` · `buoyant_kernel::actions::COMMIT_INFO_NAME`

Also reachable as `delta_kernel::actions::COMMIT_INFO_NAME`

```rust
const COMMIT_INFO_NAME: &str = "commitInfo"
```

---

## DOMAIN_METADATA_NAME

`constant` · `buoyant_kernel::actions::DOMAIN_METADATA_NAME`

Also reachable as `delta_kernel::actions::DOMAIN_METADATA_NAME`

```rust
const DOMAIN_METADATA_NAME: &str = "domainMetadata"
```

---

## MAX_VALUES

`constant` · `buoyant_kernel::actions::MAX_VALUES`

Also reachable as `delta_kernel::actions::MAX_VALUES`

```rust
const MAX_VALUES: &str = "maxValues"
```

Per-column upper bounds, as a nested struct mirroring the table schema.

---

## METADATA_NAME

`constant` · `buoyant_kernel::actions::METADATA_NAME`

Also reachable as `delta_kernel::actions::METADATA_NAME`

```rust
const METADATA_NAME: &str = "metaData"
```

---

## MIN_VALUES

`constant` · `buoyant_kernel::actions::MIN_VALUES`

Also reachable as `delta_kernel::actions::MIN_VALUES`

```rust
const MIN_VALUES: &str = "minValues"
```

Per-column lower bounds, as a nested struct mirroring the table schema.

---

## NULL_COUNT

`constant` · `buoyant_kernel::actions::NULL_COUNT`

Also reachable as `delta_kernel::actions::NULL_COUNT`

```rust
const NULL_COUNT: &str = "nullCount"
```

Per-column null counts, as a nested struct mirroring the table schema.

---

## NUM_RECORDS

`constant` · `buoyant_kernel::actions::NUM_RECORDS`

Also reachable as `delta_kernel::actions::NUM_RECORDS`

```rust
const NUM_RECORDS: &str = "numRecords"
```

Logical (post-DV) row count, stored as a `long`.

---

## PROTOCOL_NAME

`constant` · `buoyant_kernel::actions::PROTOCOL_NAME`

Also reachable as `delta_kernel::actions::PROTOCOL_NAME`

```rust
const PROTOCOL_NAME: &str = "protocol"
```

---

## REMOVE_NAME

`constant` · `buoyant_kernel::actions::REMOVE_NAME`

Also reachable as `delta_kernel::actions::REMOVE_NAME`

```rust
const REMOVE_NAME: &str = "remove"
```

---

## SET_TRANSACTION_NAME

`constant` · `buoyant_kernel::actions::SET_TRANSACTION_NAME`

Also reachable as `delta_kernel::actions::SET_TRANSACTION_NAME`

```rust
const SET_TRANSACTION_NAME: &str = "txn"
```

---

## SIDECAR_NAME

`constant` · `buoyant_kernel::actions::SIDECAR_NAME`

Also reachable as `delta_kernel::actions::SIDECAR_NAME`

```rust
const SIDECAR_NAME: &str = "sidecar"
```

---

## STATS_PARSED

`constant` · `buoyant_kernel::actions::STATS_PARSED`

Also reachable as `delta_kernel::actions::STATS_PARSED`

```rust
const STATS_PARSED: &str = "stats_parsed"
```

Struct-encoded per-file statistics column (checkpoints with `writeStatsAsStruct=true`).

---

## TIGHT_BOUNDS

`constant` · `buoyant_kernel::actions::TIGHT_BOUNDS`

Also reachable as `delta_kernel::actions::TIGHT_BOUNDS`

```rust
const TIGHT_BOUNDS: &str = "tightBounds"
```

Whether the min/max/nullCount stats are tight or wide. Defaults to `true` when absent.

---

## get_all_actions_schema

`function` · `buoyant_kernel::actions::get_all_actions_schema`

Also reachable as `delta_kernel::actions::get_all_actions_schema`

```rust
fn get_all_actions_schema() -> &'static schema::SchemaRef
```

Gets a schema for all actions defined by the delta spec.

---

## get_commit_schema

`function` · `buoyant_kernel::actions::get_commit_schema`

Also reachable as `delta_kernel::actions::get_commit_schema`

```rust
fn get_commit_schema() -> &'static schema::SchemaRef
```

Gets the schema for all actions that can appear in commits
logs.  This excludes actions that can only appear in checkpoints.

---

## schema_contains_file_actions

`function` · `buoyant_kernel::actions::schema_contains_file_actions`

Also reachable as `delta_kernel::actions::schema_contains_file_actions`

```rust
fn schema_contains_file_actions(schema: &schema::SchemaRef) -> bool
```

Returns true if the schema contains file actions (add or remove)
columns.

---

## LOG_ADD_SCHEMA

`static` · `buoyant_kernel::actions::LOG_ADD_SCHEMA`

Also reachable as `delta_kernel::actions::LOG_ADD_SCHEMA`

```rust
static LOG_ADD_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

Schema for Add actions in the Delta log.
Wraps the Add action schema in a top-level struct with "add" field name.

---

## LOG_COMMIT_INFO_SCHEMA

`static` · `buoyant_kernel::actions::LOG_COMMIT_INFO_SCHEMA`

Also reachable as `delta_kernel::actions::LOG_COMMIT_INFO_SCHEMA`

```rust
static LOG_COMMIT_INFO_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

Schema for CommitInfo actions in the Delta log.
Wraps the CommitInfo schema in a top-level struct with "commitInfo" field name.

---

## LOG_DOMAIN_METADATA_SCHEMA

`static` · `buoyant_kernel::actions::LOG_DOMAIN_METADATA_SCHEMA`

Also reachable as `delta_kernel::actions::LOG_DOMAIN_METADATA_SCHEMA`

```rust
static LOG_DOMAIN_METADATA_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

---

## LOG_METADATA_SCHEMA

`static` · `buoyant_kernel::actions::LOG_METADATA_SCHEMA`

Also reachable as `delta_kernel::actions::LOG_METADATA_SCHEMA`

```rust
static LOG_METADATA_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

---

## LOG_PROTOCOL_SCHEMA

`static` · `buoyant_kernel::actions::LOG_PROTOCOL_SCHEMA`

Also reachable as `delta_kernel::actions::LOG_PROTOCOL_SCHEMA`

```rust
static LOG_PROTOCOL_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

---

## LOG_REMOVE_SCHEMA

`static` · `buoyant_kernel::actions::LOG_REMOVE_SCHEMA`

Also reachable as `delta_kernel::actions::LOG_REMOVE_SCHEMA`

```rust
static LOG_REMOVE_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

Schema for Remove actions in the Delta log.
Wraps the Remove action schema in a top-level struct with "remove" field name.

---

## LOG_TXN_SCHEMA

`static` · `buoyant_kernel::actions::LOG_TXN_SCHEMA`

Also reachable as `delta_kernel::actions::LOG_TXN_SCHEMA`

```rust
static LOG_TXN_SCHEMA: std::sync::LazyLock<schema::SchemaRef>
```

Schema for transaction (txn) actions in the Delta log.
Wraps the SetTransaction schema in a top-level struct with "txn" field name.

---

## Add

`struct` · `buoyant_kernel::actions::Add`

Also reachable as `delta_kernel::actions::Add`

```rust
struct Add
```

**Fields**: `stats`, `tags`, `deletion_vector`, `base_row_id`, `default_row_commit_version`, `clustering_provider`

**Implements**: `buoyant_kernel::schema::ToSchema`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn dv_unique_id(&self) -> Option<String>
```

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

---

## Cdc

`struct` · `buoyant_kernel::actions::Cdc`

Also reachable as `delta_kernel::actions::Cdc`

```rust
struct Cdc
```

**Fields**: `path`, `partition_values`, `size`, `data_change`, `tags`

**Implements**: `buoyant_kernel::schema::ToSchema`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

---

## CheckpointMetadata

`struct` · `buoyant_kernel::actions::CheckpointMetadata`

Also reachable as `delta_kernel::actions::CheckpointMetadata`

```rust
struct CheckpointMetadata
```

**Implements**: `buoyant_kernel::schema::ToSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The CheckpointMetadata action describes details about a checkpoint following the V2
specification.

[More info]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#checkpoint-metadata

---

## CommitInfo

`struct` · `buoyant_kernel::actions::CommitInfo`

Also reachable as `delta_kernel::actions::CommitInfo`

```rust
struct CommitInfo
```

**Implements**: `buoyant_kernel::IntoEngineData`, `buoyant_kernel::schema::ToSchema`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::IntoEngineData`**

```rust
fn into_engine_data(self, schema: delta_kernel::schema::SchemaRef, engine: &dyn delta_kernel::Engine) -> delta_kernel::DeltaResult<Box<dyn delta_kernel::EngineData>>
```

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

---

## DomainMetadata

`struct` · `buoyant_kernel::actions::DomainMetadata`

Also reachable as `delta_kernel::actions::DomainMetadata`

```rust
struct DomainMetadata
```

**Implements**: `buoyant_kernel::IntoEngineData`, `buoyant_kernel::schema::ToSchema`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn configuration(&self) -> &str
fn domain(&self) -> &str
fn is_internal(&self) -> bool
```

**via `buoyant_kernel::IntoEngineData`**

```rust
fn into_engine_data(self, schema: delta_kernel::schema::SchemaRef, engine: &dyn delta_kernel::Engine) -> delta_kernel::DeltaResult<Box<dyn delta_kernel::EngineData>>
```

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

**via `core::convert::TryFrom`**

```rust
fn try_from(metadata: RowTrackingDomainMetadata) -> DeltaResult<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The [DomainMetadata] action contains a configuration (string) for a named metadata domain. Two
overlapping transactions conflict if they both contain a domain metadata action for the same
metadata domain.

Note that the `delta.*` domain is reserved for internal use.

[DomainMetadata]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#domain-metadata

---

## Format

`struct` · `buoyant_kernel::actions::Format`

Also reachable as `delta_kernel::actions::Format`

```rust
struct Format
```

**Implements**: `buoyant_kernel::schema::ToSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Metadata

`struct` · `buoyant_kernel::actions::Metadata`

Also reachable as `delta_kernel::actions::Metadata`, `deltalake::kernel::Metadata`, `deltalake::kernel::models::Metadata`, `deltalake_core::kernel::Metadata`, `deltalake_core::kernel::models::Metadata`

```rust
struct Metadata
```

**Implements**: `buoyant_kernel::IntoEngineData`, `buoyant_kernel::schema::ToSchema`, `deltalake_core::kernel::models::actions::MetadataExt`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn configuration(&self) -> &HashMap<String, String>
fn created_time(&self) -> Option<i64>
fn description(&self) -> Option<&str>
fn format_provider(&self) -> &str
fn id(&self) -> &str
fn name(&self) -> Option<&str>
fn parse_schema(&self) -> DeltaResult<StructType>
fn parse_table_properties(&self) -> TableProperties
fn partition_columns(&self) -> &[String]
fn schema_string(&self) -> &String
fn try_new(name: Option<String>, description: Option<String>, schema: SchemaRef, partition_columns: Vec<String>, created_time: i64, configuration: HashMap<String, String>) -> DeltaResult<Self>
fn try_new_from_data(data: &dyn EngineData) -> DeltaResult<Option<Metadata>>
```

**via `buoyant_kernel::IntoEngineData`**

```rust
fn into_engine_data(self, schema: SchemaRef, engine: &dyn Engine) -> DeltaResult<Box<dyn EngineData>>
```

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Protocol

`struct` · `buoyant_kernel::actions::Protocol`

Also reachable as `delta_kernel::actions::Protocol`, `deltalake::kernel::Protocol`, `deltalake::kernel::models::Protocol`, `deltalake_core::kernel::Protocol`, `deltalake_core::kernel::models::Protocol`

```rust
struct Protocol
```

**Implements**: `buoyant_kernel::IntoEngineData`, `buoyant_kernel::schema::ToSchema`, `deltalake_core::kernel::models::actions::ProtocolExt`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn min_reader_version(&self) -> i32
fn min_writer_version(&self) -> i32
fn reader_features(&self) -> Option<&[TableFeature]>
fn writer_features(&self) -> Option<&[TableFeature]>
```

**via `buoyant_kernel::IntoEngineData`**

```rust
fn into_engine_data(self, schema: delta_kernel::schema::SchemaRef, engine: &dyn delta_kernel::Engine) -> delta_kernel::DeltaResult<Box<dyn delta_kernel::EngineData>>
```

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Remove

`struct` · `buoyant_kernel::actions::Remove`

Also reachable as `delta_kernel::actions::Remove`

```rust
struct Remove
```

**Fields**: `stats`

**Implements**: `buoyant_kernel::schema::ToSchema`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

---

## SetTransaction

`struct` · `buoyant_kernel::actions::SetTransaction`

Also reachable as `delta_kernel::actions::SetTransaction`

```rust
struct SetTransaction
```

**Implements**: `buoyant_kernel::IntoEngineData`, `buoyant_kernel::schema::ToSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::IntoEngineData`**

```rust
fn into_engine_data(self, schema: delta_kernel::schema::SchemaRef, engine: &dyn delta_kernel::Engine) -> delta_kernel::DeltaResult<Box<dyn delta_kernel::EngineData>>
```

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Sidecar

`struct` · `buoyant_kernel::actions::Sidecar`

Also reachable as `delta_kernel::actions::Sidecar`

```rust
struct Sidecar
```

**Fields**: `path`, `size_in_bytes`, `modification_time`, `tags`

**Implements**: `buoyant_kernel::schema::ToSchema`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `buoyant_kernel::schema::ToSchema`**

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The sidecar action references a sidecar file which provides some of the checkpoint's
file actions. This action is only allowed in checkpoints following the V2 spec.

[More info]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#sidecar-file-information

---
