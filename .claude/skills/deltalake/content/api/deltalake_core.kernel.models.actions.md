# `deltalake_core::kernel::models::actions`

Crate `deltalake-core` · 16 public items · structured records in [`model/deltalake_core.kernel.models.actions.json`](../model/deltalake_core.kernel.models.actions.json)

## IsolationLevel

`enum` · `deltalake_core::kernel::models::actions::IsolationLevel`

Also reachable as `deltalake::kernel::IsolationLevel`, `deltalake::kernel::models::IsolationLevel`, `deltalake_core::kernel::IsolationLevel`, `deltalake_core::kernel::models::IsolationLevel`

```rust
enum IsolationLevel
```

**Variants**: `Serializable`, `WriteSerializable`, `SnapshotIsolation`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The isolation level applied during transaction

---

## StorageType

`enum` · `deltalake_core::kernel::models::actions::StorageType`

Also reachable as `deltalake::kernel::StorageType`, `deltalake::kernel::models::StorageType`, `deltalake_core::kernel::StorageType`, `deltalake_core::kernel::models::StorageType`

```rust
enum StorageType
```

**Variants**: `UuidRelativePath`, `Inline`, `AbsolutePath`

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Storage type of deletion vector

---

## TableFeatures

`enum` · `deltalake_core::kernel::models::actions::TableFeatures`

Also reachable as `deltalake::kernel::TableFeatures`, `deltalake::kernel::models::TableFeatures`, `deltalake_core::kernel::TableFeatures`, `deltalake_core::kernel::models::TableFeatures`

```rust
enum TableFeatures
```

**Variants**: `ColumnMapping`, `DeletionVectors`, `TimestampWithoutTimezone`, `TimestampNanos`, `V2Checkpoint`, `AppendOnly`, `Invariants`, `CheckConstraints`, `ChangeDataFeed`, `GeneratedColumns`, `IdentityColumns`, `RowTracking`, `DomainMetadata`, `IcebergCompatV1`, `VariantType`, `VariantTypePreview`, `VariantShreddingPreview`, `MaterializePartitionColumns`

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn to_reader_writer_features(&self) -> (Option<TableFeature>, Option<TableFeature>)
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(value: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

High level table features

---

## contains_timestamp_nanos

`function` · `deltalake_core::kernel::models::actions::contains_timestamp_nanos`

Also reachable as `deltalake::kernel::contains_timestamp_nanos`, `deltalake::kernel::models::contains_timestamp_nanos`, `deltalake_core::kernel::contains_timestamp_nanos`, `deltalake_core::kernel::models::contains_timestamp_nanos`

```rust
fn contains_timestamp_nanos<'a>(fields: impl Iterator<Item = &'a delta_kernel::schema::StructField>) -> bool
```

checks if table contains timestamp_nanos or timestamp_nanos_ntz in any
field including nested fields. Both primitive types require the same
`timestampNanos` table feature.

---

## contains_timestampntz

`function` · `deltalake_core::kernel::models::actions::contains_timestampntz`

Also reachable as `deltalake::kernel::contains_timestampntz`, `deltalake::kernel::models::contains_timestampntz`, `deltalake_core::kernel::contains_timestampntz`, `deltalake_core::kernel::models::contains_timestampntz`

```rust
fn contains_timestampntz<'a>(fields: impl Iterator<Item = &'a delta_kernel::schema::StructField>) -> bool
```

checks if table contains timestamp_ntz in any field including nested fields.

---

## new_metadata

`function` · `deltalake_core::kernel::models::actions::new_metadata`

Also reachable as `deltalake::kernel::models::new_metadata`, `deltalake::kernel::new_metadata`, `deltalake_core::kernel::models::new_metadata`, `deltalake_core::kernel::new_metadata`

```rust
fn new_metadata(schema: &kernel::StructType, partition_columns: impl IntoIterator<Item = impl ToString>, configuration: impl IntoIterator<Item = (impl ToString, impl ToString)>) -> kernel::DeltaResult<Metadata>
```

Please don't use, this API will be leaving shortly!

Since the adoption of delta-kernel-rs we lost the direct ability to create [Metadata] actions
which is required for some use-cases.

Upstream tracked here: <https://github.com/delta-io/delta-kernel-rs/issues/1055>

---

## Add

`struct` · `deltalake_core::kernel::models::actions::Add`

Also reachable as `deltalake::kernel::Add`, `deltalake::kernel::models::Add`, `deltalake_core::kernel::Add`, `deltalake_core::kernel::models::Add`

```rust
struct Add
```

**Fields**: `path`, `partition_values`, `size`, `modification_time`, `data_change`, `stats`, `tags`, `deletion_vector`, `base_row_id`, `default_row_commit_version`, `clustering_provider`

**Implements**: `deltalake_core::delta_datafusion::cdf::FileAction`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn get_stats(&self) -> Result<Option<Stats>, serde_json::error::Error>
```

**via `deltalake_core::delta_datafusion::cdf::FileAction`**

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
fn has_deletion_vector(&self) -> bool
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
fn path(&self) -> String
fn size(&self) -> DeltaResult<usize>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Defines an add action

---

## AddCDCFile

`struct` · `deltalake_core::kernel::models::actions::AddCDCFile`

Also reachable as `deltalake::kernel::AddCDCFile`, `deltalake::kernel::models::AddCDCFile`, `deltalake_core::kernel::AddCDCFile`, `deltalake_core::kernel::models::AddCDCFile`

```rust
struct AddCDCFile
```

**Fields**: `path`, `size`, `partition_values`, `data_change`, `tags`

**Implements**: `deltalake_core::delta_datafusion::cdf::FileAction`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `deltalake_core::delta_datafusion::cdf::FileAction`**

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
fn path(&self) -> String
fn size(&self) -> DeltaResult<usize>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Delta AddCDCFile action that describes a parquet CDC data file.

---

## CheckpointMetadata

`struct` · `deltalake_core::kernel::models::actions::CheckpointMetadata`

Also reachable as `deltalake::kernel::CheckpointMetadata`, `deltalake::kernel::models::CheckpointMetadata`, `deltalake_core::kernel::CheckpointMetadata`, `deltalake_core::kernel::models::CheckpointMetadata`

```rust
struct CheckpointMetadata
```

**Fields**: `flavor`, `tags`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This action is only allowed in checkpoints following V2 spec. It describes the details about the checkpoint.

---

## CommitInfo

`struct` · `deltalake_core::kernel::models::actions::CommitInfo`

Also reachable as `deltalake::kernel::CommitInfo`, `deltalake::kernel::models::CommitInfo`, `deltalake_core::kernel::CommitInfo`, `deltalake_core::kernel::models::CommitInfo`

```rust
struct CommitInfo
```

**Fields**: `timestamp`, `in_commit_timestamp`, `user_id`, `user_name`, `operation`, `operation_parameters`, `read_version`, `isolation_level`, `is_blind_append`, `engine_info`, `info`, `user_metadata`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The commitInfo is a fairly flexible action within the delta specification, where arbitrary data can be stored.
However, the reference implementation as well as delta-rs store useful information that may for instance
allow us to be more permissive in commit conflict resolution.

---

## DeletionVectorDescriptor

`struct` · `deltalake_core::kernel::models::actions::DeletionVectorDescriptor`

Also reachable as `deltalake::kernel::DeletionVectorDescriptor`, `deltalake::kernel::models::DeletionVectorDescriptor`, `deltalake_core::kernel::DeletionVectorDescriptor`, `deltalake_core::kernel::models::DeletionVectorDescriptor`

```rust
struct DeletionVectorDescriptor
```

**Fields**: `storage_type`, `path_or_inline_dv`, `offset`, `size_in_bytes`, `cardinality`

**Implements**: `core::convert::TryInto`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryInto`**

```rust
fn try_into(self) -> DeltaResult<DeletionVectorDescriptor>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Defines a deletion vector

---

## DomainMetadata

`struct` · `deltalake_core::kernel::models::actions::DomainMetadata`

Also reachable as `deltalake::kernel::DomainMetadata`, `deltalake::kernel::models::DomainMetadata`, `deltalake_core::kernel::DomainMetadata`, `deltalake_core::kernel::models::DomainMetadata`

```rust
struct DomainMetadata
```

**Fields**: `domain`, `configuration`, `removed`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The domain metadata action contains a configuration (string) for a named metadata domain

---

## Remove

`struct` · `deltalake_core::kernel::models::actions::Remove`

Also reachable as `deltalake::kernel::Remove`, `deltalake::kernel::models::Remove`, `deltalake_core::kernel::Remove`, `deltalake_core::kernel::models::Remove`

```rust
struct Remove
```

**Fields**: `path`, `data_change`, `deletion_timestamp`, `extended_file_metadata`, `partition_values`, `size`, `tags`, `deletion_vector`, `base_row_id`, `default_row_commit_version`

**Implements**: `core::borrow::Borrow`, `deltalake_core::delta_datafusion::cdf::FileAction`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &str
```

**via `deltalake_core::delta_datafusion::cdf::FileAction`**

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
fn has_deletion_vector(&self) -> bool
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
fn path(&self) -> String
fn size(&self) -> DeltaResult<usize>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a tombstone (deleted file) in the Delta log.

---

## Sidecar

`struct` · `deltalake_core::kernel::models::actions::Sidecar`

Also reachable as `deltalake::kernel::Sidecar`, `deltalake::kernel::models::Sidecar`, `deltalake_core::kernel::Sidecar`, `deltalake_core::kernel::models::Sidecar`

```rust
struct Sidecar
```

**Fields**: `file_name`, `size_in_bytes`, `modification_time`, `sidecar_type`, `tags`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The sidecar action references a sidecar file which provides some of the checkpoint's file actions.
This action is only allowed in checkpoints following V2 spec.

---

## Transaction

`struct` · `deltalake_core::kernel::models::actions::Transaction`

Also reachable as `deltalake::kernel::Transaction`, `deltalake::kernel::models::Transaction`, `deltalake_core::kernel::Transaction`, `deltalake_core::kernel::models::Transaction`

```rust
struct Transaction
```

**Fields**: `app_id`, `version`, `last_updated`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(app_id: impl ToString, version: i64) -> Self
fn new_with_last_update(app_id: impl ToString, version: i64, last_updated: Option<i64>) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Action used by streaming systems to track progress using application-specific versions to
enable idempotency.

---

## MetadataExt

`trait` · `deltalake_core::kernel::models::actions::MetadataExt`

Also reachable as `deltalake::kernel::MetadataExt`, `deltalake::kernel::models::MetadataExt`, `deltalake_core::kernel::MetadataExt`, `deltalake_core::kernel::models::MetadataExt`

```rust
trait MetadataExt
```

**Implementors** (1)

- `buoyant_kernel::actions::Metadata`

**Methods** (6)

```rust
fn add_config_key(self, key: String, value: String) -> DeltaResult<Metadata>
fn remove_config_key(self, key: &str) -> DeltaResult<Metadata>
fn with_description(self, description: String) -> DeltaResult<Metadata>
fn with_name(self, name: String) -> DeltaResult<Metadata>
fn with_schema(self, schema: &StructType) -> DeltaResult<Metadata>
fn with_table_id(self, table_id: String) -> DeltaResult<Metadata>
```

Extension trait for Metadata action

This trait is a stop-gap to adopt the Metadata action from delta-kernel-rs
while the update / mutation APIs are being implemented. It allows us to implement
additional APIs on the Metadata action and hide specifics of how we do the updates.

---
