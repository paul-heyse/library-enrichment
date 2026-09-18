# `deltalake_core::protocol`

Crate `deltalake-core` · 8 public items · structured records in [`model/deltalake_core.protocol.json`](../model/deltalake_core.protocol.json)

## ColumnCountStat

`enum` · `deltalake_core::protocol::ColumnCountStat`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.ColumnCountStat.md)

Also reachable as `deltalake::protocol::ColumnCountStat`

```rust
enum ColumnCountStat
```

**Variants**: `Column`, `Value`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn as_column(&self) -> Option<&HashMap<String, ColumnCountStat>>
fn as_value(&self) -> Option<i64>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Struct used to represent nullCount in add action statistics.

---

## ColumnValueStat

`enum` · `deltalake_core::protocol::ColumnValueStat`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.ColumnValueStat.md)

Also reachable as `deltalake::protocol::ColumnValueStat`

```rust
enum ColumnValueStat
```

**Variants**: `Column`, `Value`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn as_column(&self) -> Option<&HashMap<String, ColumnValueStat>>
fn as_value(&self) -> Option<&Value>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Struct used to represent minValues and maxValues in add action statistics.

---

## DeltaOperation

`enum` · `deltalake_core::protocol::DeltaOperation`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.DeltaOperation.md)

Also reachable as `deltalake::protocol::DeltaOperation`

```rust
enum DeltaOperation
```

**Variants**: `AddColumn`, `Create`, `Write`, `Delete`, `Update`, `AddConstraint`, `AddFeature`, `DropConstraint`, `Merge`, `StreamingUpdate`, `SetTableProperties`, `Optimize`, `FileSystemCheck`, `Restore`, `VacuumStart`, `VacuumEnd`, `UpdateFieldMetadata`, `UpdateTableMetadata`, `DropColumnNotNull`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn changes_data(&self) -> bool
fn get_commit_info(&self) -> CommitInfo
fn name(&self) -> &str
fn operation_parameters(&self) -> DeltaResult<HashMap<String, Value>>
fn read_predicate(&self) -> Option<String>
fn read_whole_table(&self) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(opt_input: OptimizeInput) -> Result<Self, Self::Error>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Operation performed when creating a new log entry with one or more actions.
This is a key element of the `CommitInfo` action.

---

## OutputMode

`enum` · `deltalake_core::protocol::OutputMode`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.OutputMode.md)

Also reachable as `deltalake::protocol::OutputMode`

```rust
enum OutputMode
```

**Variants**: `Append`, `Complete`, `Update`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The OutputMode used in streaming operations.

---

## SaveMode

`enum` · `deltalake_core::protocol::SaveMode`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.SaveMode.md)

Also reachable as `deltalake::protocol::SaveMode`

```rust
enum SaveMode
```

**Variants**: `Append`, `Overwrite`, `ErrorIfExists`, `Ignore`

**Implements**: `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The SaveMode used when performing a DeltaOperation

---

## MergePredicate

`struct` · `deltalake_core::protocol::MergePredicate`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.MergePredicate.md)

Also reachable as `deltalake::protocol::MergePredicate`

```rust
struct MergePredicate
```

**Fields**: `action_type`, `predicate`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Used to record the operations performed to the Delta Log

---

## Stats

`struct` · `deltalake_core::protocol::Stats`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.Stats.md)

Also reachable as `deltalake::protocol::Stats`

```rust
struct Stats
```

**Fields**: `num_records`, `min_values`, `max_values`, `null_count`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Statistics associated with Add actions contained in the Delta log.

---

## StatsParsed

`struct` · `deltalake_core::protocol::StatsParsed`
[Full member contracts, output types and access classification](../operations/deltalake_core.protocol.StatsParsed.md)

Also reachable as `deltalake::protocol::StatsParsed`

```rust
struct StatsParsed
```

**Fields**: `num_records`, `min_values`, `max_values`, `null_count`

**Derives**: Debug, Default

File stats parsed from raw parquet format.

---
