# `datafusion_proto_models::generated::datafusion`

Crate `datafusion-proto-models` · 288 public items · structured records in [`model/datafusion_proto_models.generated.datafusion.json`](../model/datafusion_proto_models.generated.datafusion.json)

## AggregateMode

`enum` · `datafusion_proto_models::generated::datafusion::AggregateMode`

Also reachable as `datafusion_proto::generated::datafusion::AggregateMode`, `datafusion_proto::protobuf::AggregateMode`, `datafusion_proto_models::protobuf::AggregateMode`

```rust
enum AggregateMode
```

**Variants**: `Partial`, `Final`, `FinalPartitioned`, `Single`, `SinglePartitioned`, `PartialReduce`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<AggregateMode>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<AggregateMode, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AggregateMode.md).


---

## CompressionTypeVariant

`enum` · `datafusion_proto_models::generated::datafusion::CompressionTypeVariant`

Also reachable as `datafusion_proto::generated::datafusion::CompressionTypeVariant`, `datafusion_proto::protobuf::CompressionTypeVariant`, `datafusion_proto_models::protobuf::CompressionTypeVariant`

```rust
enum CompressionTypeVariant
```

**Variants**: `Gzip`, `Bzip2`, `Xz`, `Zstd`, `Uncompressed`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<CompressionTypeVariant>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<CompressionTypeVariant, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CompressionTypeVariant.md).


---

## CsvQuoteStyle

`enum` · `datafusion_proto_models::generated::datafusion::CsvQuoteStyle`

Also reachable as `datafusion_proto::generated::datafusion::CsvQuoteStyle`, `datafusion_proto::protobuf::CsvQuoteStyle`, `datafusion_proto_models::protobuf::CsvQuoteStyle`

```rust
enum CsvQuoteStyle
```

**Variants**: `Necessary`, `Always`, `NonNumeric`, `Never`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<CsvQuoteStyle>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<CsvQuoteStyle, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CsvQuoteStyle.md).


---

## DateUnit

`enum` · `datafusion_proto_models::generated::datafusion::DateUnit`

Also reachable as `datafusion_proto::generated::datafusion::DateUnit`, `datafusion_proto::protobuf::DateUnit`, `datafusion_proto_models::protobuf::DateUnit`

```rust
enum DateUnit
```

**Variants**: `Day`, `DateMillisecond`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<DateUnit>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<DateUnit, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.DateUnit.md).


---

## ExplainFormat

`enum` · `datafusion_proto_models::generated::datafusion::ExplainFormat`

Also reachable as `datafusion_proto::generated::datafusion::ExplainFormat`, `datafusion_proto::protobuf::ExplainFormat`, `datafusion_proto_models::protobuf::ExplainFormat`

```rust
enum ExplainFormat
```

**Variants**: `Indent`, `Tree`, `Pgjson`, `Graphviz`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<ExplainFormat>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<ExplainFormat, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ExplainFormat.md).


---

## FileFormatKind

`enum` · `datafusion_proto_models::generated::datafusion::FileFormatKind`

Also reachable as `datafusion_proto::generated::datafusion::FileFormatKind`, `datafusion_proto::protobuf::FileFormatKind`, `datafusion_proto_models::protobuf::FileFormatKind`

```rust
enum FileFormatKind
```

**Variants**: `Unspecified`, `Csv`, `Json`, `Parquet`, `Arrow`, `Avro`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<FileFormatKind>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<FileFormatKind, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FileFormatKind.md).


Identifies a built-in file format supported by DataFusion.
Used by DefaultLogicalExtensionCodec to serialize/deserialize
FileFormatFactory instances (e.g. in CopyTo plans).

---

## FileOutputMode

`enum` · `datafusion_proto_models::generated::datafusion::FileOutputMode`

Also reachable as `datafusion_proto::generated::datafusion::FileOutputMode`, `datafusion_proto::protobuf::FileOutputMode`, `datafusion_proto_models::protobuf::FileOutputMode`

```rust
enum FileOutputMode
```

**Variants**: `Automatic`, `SingleFile`, `Directory`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<FileOutputMode>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<FileOutputMode, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FileOutputMode.md).


Determines how file sink output paths are interpreted.

---

## GenerateSeriesName

`enum` · `datafusion_proto_models::generated::datafusion::GenerateSeriesName`

Also reachable as `datafusion_proto::generated::datafusion::GenerateSeriesName`, `datafusion_proto::protobuf::GenerateSeriesName`, `datafusion_proto_models::protobuf::GenerateSeriesName`

```rust
enum GenerateSeriesName
```

**Variants**: `GsGenerateSeries`, `GsRange`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<GenerateSeriesName>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<GenerateSeriesName, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GenerateSeriesName.md).


---

## InsertOp

`enum` · `datafusion_proto_models::generated::datafusion::InsertOp`

Also reachable as `datafusion_proto::generated::datafusion::InsertOp`, `datafusion_proto::protobuf::InsertOp`, `datafusion_proto_models::protobuf::InsertOp`

```rust
enum InsertOp
```

**Variants**: `Append`, `Overwrite`, `Replace`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<InsertOp>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<InsertOp, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.InsertOp.md).


---

## IntervalUnit

`enum` · `datafusion_proto_models::generated::datafusion::IntervalUnit`

Also reachable as `datafusion_proto::generated::datafusion::IntervalUnit`, `datafusion_proto::protobuf::IntervalUnit`, `datafusion_proto_models::protobuf::IntervalUnit`

```rust
enum IntervalUnit
```

**Variants**: `YearMonth`, `DayTime`, `MonthDayNano`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<IntervalUnit>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<IntervalUnit, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IntervalUnit.md).


---

## JoinConstraint

`enum` · `datafusion_proto_models::generated::datafusion::JoinConstraint`

Also reachable as `datafusion_proto::generated::datafusion::JoinConstraint`, `datafusion_proto::protobuf::JoinConstraint`, `datafusion_proto_models::protobuf::JoinConstraint`

```rust
enum JoinConstraint
```

**Variants**: `On`, `Using`

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<JoinConstraint>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(t: JoinConstraint) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<JoinConstraint, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JoinConstraint.md).


---

## JoinSide

`enum` · `datafusion_proto_models::generated::datafusion::JoinSide`

Also reachable as `datafusion_proto::generated::datafusion::JoinSide`, `datafusion_proto::protobuf::JoinSide`, `datafusion_proto_models::protobuf::JoinSide`

```rust
enum JoinSide
```

**Variants**: `LeftSide`, `RightSide`, `None`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<JoinSide>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<JoinSide, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JoinSide.md).


---

## JoinType

`enum` · `datafusion_proto_models::generated::datafusion::JoinType`

Also reachable as `datafusion_proto::generated::datafusion::JoinType`, `datafusion_proto::protobuf::JoinType`, `datafusion_proto_models::protobuf::JoinType`

```rust
enum JoinType
```

**Variants**: `Inner`, `Left`, `Right`, `Full`, `Leftsemi`, `Leftanti`, `Rightsemi`, `Rightanti`, `Leftmark`, `Rightmark`

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<JoinType>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(t: JoinType) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<JoinType, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JoinType.md).


---

## MetricCategory

`enum` · `datafusion_proto_models::generated::datafusion::MetricCategory`

Also reachable as `datafusion_proto::generated::datafusion::MetricCategory`, `datafusion_proto::protobuf::MetricCategory`, `datafusion_proto_models::protobuf::MetricCategory`

```rust
enum MetricCategory
```

**Variants**: `Rows`, `Bytes`, `Timing`, `Uncategorized`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<MetricCategory>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<MetricCategory, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MetricCategory.md).


Category of an `EXPLAIN ANALYZE` metric. Mirrors
`datafusion_common::format::MetricCategory`.

---

## MetricType

`enum` · `datafusion_proto_models::generated::datafusion::MetricType`

Also reachable as `datafusion_proto::generated::datafusion::MetricType`, `datafusion_proto::protobuf::MetricType`, `datafusion_proto_models::protobuf::MetricType`

```rust
enum MetricType
```

**Variants**: `Summary`, `Dev`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<MetricType>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<MetricType, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MetricType.md).


Verbosity level for `EXPLAIN ANALYZE`. Mirrors
`datafusion_common::format::MetricType`.

---

## NullEquality

`enum` · `datafusion_proto_models::generated::datafusion::NullEquality`

Also reachable as `datafusion_proto::generated::datafusion::NullEquality`, `datafusion_proto::protobuf::NullEquality`, `datafusion_proto_models::protobuf::NullEquality`

```rust
enum NullEquality
```

**Variants**: `NullEqualsNothing`, `NullEqualsNull`

**Implements**: `core::convert::From`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<NullEquality>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(t: NullEquality) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<NullEquality, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.NullEquality.md).


---

## NullTreatment

`enum` · `datafusion_proto_models::generated::datafusion::NullTreatment`

Also reachable as `datafusion_proto::generated::datafusion::NullTreatment`, `datafusion_proto::protobuf::NullTreatment`, `datafusion_proto_models::protobuf::NullTreatment`

```rust
enum NullTreatment
```

**Variants**: `RespectNulls`, `IgnoreNulls`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<NullTreatment>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<NullTreatment, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.NullTreatment.md).


---

## PartitionMode

`enum` · `datafusion_proto_models::generated::datafusion::PartitionMode`

Also reachable as `datafusion_proto::generated::datafusion::PartitionMode`, `datafusion_proto::protobuf::PartitionMode`, `datafusion_proto_models::protobuf::PartitionMode`

```rust
enum PartitionMode
```

**Variants**: `CollectLeft`, `Partitioned`, `Auto`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<PartitionMode>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<PartitionMode, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PartitionMode.md).


---

## PrecisionInfo

`enum` · `datafusion_proto_models::generated::datafusion::PrecisionInfo`

Also reachable as `datafusion_proto::generated::datafusion::PrecisionInfo`, `datafusion_proto::protobuf::PrecisionInfo`, `datafusion_proto_models::protobuf::PrecisionInfo`

```rust
enum PrecisionInfo
```

**Variants**: `Exact`, `Inexact`, `Absent`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<PrecisionInfo>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<PrecisionInfo, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PrecisionInfo.md).


---

## StreamPartitionMode

`enum` · `datafusion_proto_models::generated::datafusion::StreamPartitionMode`

Also reachable as `datafusion_proto::generated::datafusion::StreamPartitionMode`, `datafusion_proto::protobuf::StreamPartitionMode`, `datafusion_proto_models::protobuf::StreamPartitionMode`

```rust
enum StreamPartitionMode
```

**Variants**: `SinglePartition`, `PartitionedExec`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<StreamPartitionMode>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<StreamPartitionMode, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.StreamPartitionMode.md).


---

## TimeUnit

`enum` · `datafusion_proto_models::generated::datafusion::TimeUnit`

Also reachable as `datafusion_proto::generated::datafusion::TimeUnit`, `datafusion_proto::protobuf::TimeUnit`, `datafusion_proto_models::protobuf::TimeUnit`

```rust
enum TimeUnit
```

**Variants**: `Second`, `Millisecond`, `Microsecond`, `Nanosecond`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<TimeUnit>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<TimeUnit, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.TimeUnit.md).


---

## UnionMode

`enum` · `datafusion_proto_models::generated::datafusion::UnionMode`

Also reachable as `datafusion_proto::generated::datafusion::UnionMode`, `datafusion_proto::protobuf::UnionMode`, `datafusion_proto_models::protobuf::UnionMode`

```rust
enum UnionMode
```

**Variants**: `Sparse`, `Dense`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<UnionMode>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<UnionMode, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnionMode.md).


---

## WindowFrameBoundType

`enum` · `datafusion_proto_models::generated::datafusion::WindowFrameBoundType`

Also reachable as `datafusion_proto::generated::datafusion::WindowFrameBoundType`, `datafusion_proto::protobuf::WindowFrameBoundType`, `datafusion_proto_models::protobuf::WindowFrameBoundType`

```rust
enum WindowFrameBoundType
```

**Variants**: `CurrentRow`, `Preceding`, `Following`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<WindowFrameBoundType>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<WindowFrameBoundType, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WindowFrameBoundType.md).


---

## WindowFrameUnits

`enum` · `datafusion_proto_models::generated::datafusion::WindowFrameUnits`

Also reachable as `datafusion_proto::generated::datafusion::WindowFrameUnits`, `datafusion_proto::protobuf::WindowFrameUnits`, `datafusion_proto_models::protobuf::WindowFrameUnits`

```rust
enum WindowFrameUnits
```

**Variants**: `Rows`, `Range`, `Groups`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<WindowFrameUnits>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<WindowFrameUnits, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WindowFrameUnits.md).


---

## AggLimit

`struct` · `datafusion_proto_models::generated::datafusion::AggLimit`

Also reachable as `datafusion_proto::generated::datafusion::AggLimit`, `datafusion_proto::protobuf::AggLimit`, `datafusion_proto_models::protobuf::AggLimit`

```rust
struct AggLimit
```

**Fields**: `limit`, `descending`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn descending(&self) -> bool
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AggLimit.md).


---

## AggregateExecNode

`struct` · `datafusion_proto_models::generated::datafusion::AggregateExecNode`

Also reachable as `datafusion_proto::generated::datafusion::AggregateExecNode`, `datafusion_proto::protobuf::AggregateExecNode`, `datafusion_proto_models::protobuf::AggregateExecNode`

```rust
struct AggregateExecNode
```

**Fields**: `group_expr`, `aggr_expr`, `mode`, `input`, `group_expr_name`, `aggr_expr_name`, `input_schema`, `null_expr`, `groups`, `filter_expr`, `limit`, `has_grouping_set`, `dynamic_filter`, `schema`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn mode(&self) -> AggregateMode
fn set_mode(&mut self, value: AggregateMode)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AggregateExecNode.md).


---

## AggregateNode

`struct` · `datafusion_proto_models::generated::datafusion::AggregateNode`

Also reachable as `datafusion_proto::generated::datafusion::AggregateNode`, `datafusion_proto::protobuf::AggregateNode`, `datafusion_proto_models::protobuf::AggregateNode`

```rust
struct AggregateNode
```

**Fields**: `input`, `group_expr`, `aggr_expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AggregateNode.md).


---

## AggregateUdfExprNode

`struct` · `datafusion_proto_models::generated::datafusion::AggregateUdfExprNode`

Also reachable as `datafusion_proto::generated::datafusion::AggregateUdfExprNode`, `datafusion_proto::protobuf::AggregateUdfExprNode`, `datafusion_proto_models::protobuf::AggregateUdfExprNode`

```rust
struct AggregateUdfExprNode
```

**Fields**: `fun_name`, `args`, `distinct`, `filter`, `order_by`, `fun_definition`, `null_treatment`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn fun_definition(&self) -> &[u8]
fn null_treatment(&self) -> NullTreatment
fn set_null_treatment(&mut self, value: NullTreatment)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AggregateUdfExprNode.md).


---

## AliasNode

`struct` · `datafusion_proto_models::generated::datafusion::AliasNode`

Also reachable as `datafusion_proto::generated::datafusion::AliasNode`, `datafusion_proto::protobuf::AliasNode`, `datafusion_proto_models::protobuf::AliasNode`

```rust
struct AliasNode
```

**Fields**: `expr`, `alias`, `relation`, `metadata`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AliasNode.md).


---

## AnalyzeExecNode

`struct` · `datafusion_proto_models::generated::datafusion::AnalyzeExecNode`

Also reachable as `datafusion_proto::generated::datafusion::AnalyzeExecNode`, `datafusion_proto::protobuf::AnalyzeExecNode`, `datafusion_proto_models::protobuf::AnalyzeExecNode`

```rust
struct AnalyzeExecNode
```

**Fields**: `verbose`, `show_statistics`, `input`, `schema`, `has_metric_categories`, `metric_categories`, `format`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn format(&self) -> super::datafusion_common::ExplainFormat
fn set_format(&mut self, value: super::datafusion_common::ExplainFormat)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AnalyzeExecNode.md).


---

## AnalyzeNode

`struct` · `datafusion_proto_models::generated::datafusion::AnalyzeNode`

Also reachable as `datafusion_proto::generated::datafusion::AnalyzeNode`, `datafusion_proto::protobuf::AnalyzeNode`, `datafusion_proto_models::protobuf::AnalyzeNode`

```rust
struct AnalyzeNode
```

**Fields**: `input`, `verbose`, `analyze_level`, `analyze_categories`, `format`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn analyze_level(&self) -> super::datafusion_common::MetricType
fn format(&self) -> super::datafusion_common::ExplainFormat
fn set_analyze_level(&mut self, value: super::datafusion_common::MetricType)
fn set_format(&mut self, value: super::datafusion_common::ExplainFormat)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AnalyzeNode.md).


---

## AnalyzedLogicalPlanType

`struct` · `datafusion_proto_models::generated::datafusion::AnalyzedLogicalPlanType`

Also reachable as `datafusion_proto::generated::datafusion::AnalyzedLogicalPlanType`, `datafusion_proto::protobuf::AnalyzedLogicalPlanType`, `datafusion_proto_models::protobuf::AnalyzedLogicalPlanType`

```rust
struct AnalyzedLogicalPlanType
```

**Fields**: `analyzer_name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AnalyzedLogicalPlanType.md).


---

## ArrowFormat

`struct` · `datafusion_proto_models::generated::datafusion::ArrowFormat`

Also reachable as `datafusion_proto::generated::datafusion::ArrowFormat`, `datafusion_proto::protobuf::ArrowFormat`, `datafusion_proto_models::protobuf::ArrowFormat`

```rust
struct ArrowFormat
```

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ArrowFormat.md).


---

## ArrowOptions

`struct` · `datafusion_proto_models::generated::datafusion::ArrowOptions`

Also reachable as `datafusion_proto::generated::datafusion::ArrowOptions`, `datafusion_proto::protobuf::ArrowOptions`, `datafusion_proto_models::protobuf::ArrowOptions`

```rust
struct ArrowOptions
```

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ArrowOptions.md).


---

## ArrowScanExecNode

`struct` · `datafusion_proto_models::generated::datafusion::ArrowScanExecNode`

Also reachable as `datafusion_proto::generated::datafusion::ArrowScanExecNode`, `datafusion_proto::protobuf::ArrowScanExecNode`, `datafusion_proto_models::protobuf::ArrowScanExecNode`

```rust
struct ArrowScanExecNode
```

**Fields**: `base_conf`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ArrowScanExecNode.md).


---

## ArrowType

`struct` · `datafusion_proto_models::generated::datafusion::ArrowType`

Also reachable as `datafusion_proto::generated::datafusion::ArrowType`, `datafusion_proto::protobuf::ArrowType`, `datafusion_proto_models::protobuf::ArrowType`

```rust
struct ArrowType
```

**Fields**: `arrow_type_enum`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ArrowType.md).


Serialized data type

---

## AsyncFuncExecNode

`struct` · `datafusion_proto_models::generated::datafusion::AsyncFuncExecNode`

Also reachable as `datafusion_proto::generated::datafusion::AsyncFuncExecNode`, `datafusion_proto::protobuf::AsyncFuncExecNode`, `datafusion_proto_models::protobuf::AsyncFuncExecNode`

```rust
struct AsyncFuncExecNode
```

**Fields**: `input`, `async_exprs`, `async_expr_names`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AsyncFuncExecNode.md).


---

## AvroFormat

`struct` · `datafusion_proto_models::generated::datafusion::AvroFormat`

Also reachable as `datafusion_proto::generated::datafusion::AvroFormat`, `datafusion_proto::protobuf::AvroFormat`, `datafusion_proto_models::protobuf::AvroFormat`

```rust
struct AvroFormat
```

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AvroFormat.md).


---

## AvroOptions

`struct` · `datafusion_proto_models::generated::datafusion::AvroOptions`

Also reachable as `datafusion_proto::generated::datafusion::AvroOptions`, `datafusion_proto::protobuf::AvroOptions`, `datafusion_proto_models::protobuf::AvroOptions`

```rust
struct AvroOptions
```

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AvroOptions.md).


---

## AvroScanExecNode

`struct` · `datafusion_proto_models::generated::datafusion::AvroScanExecNode`

Also reachable as `datafusion_proto::generated::datafusion::AvroScanExecNode`, `datafusion_proto::protobuf::AvroScanExecNode`, `datafusion_proto_models::protobuf::AvroScanExecNode`

```rust
struct AvroScanExecNode
```

**Fields**: `base_conf`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.AvroScanExecNode.md).


---

## BareTableReference

`struct` · `datafusion_proto_models::generated::datafusion::BareTableReference`

Also reachable as `datafusion_proto::generated::datafusion::BareTableReference`, `datafusion_proto::protobuf::BareTableReference`, `datafusion_proto_models::protobuf::BareTableReference`

```rust
struct BareTableReference
```

**Fields**: `table`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.BareTableReference.md).


---

## BetweenNode

`struct` · `datafusion_proto_models::generated::datafusion::BetweenNode`

Also reachable as `datafusion_proto::generated::datafusion::BetweenNode`, `datafusion_proto::protobuf::BetweenNode`, `datafusion_proto_models::protobuf::BetweenNode`

```rust
struct BetweenNode
```

**Fields**: `expr`, `negated`, `low`, `high`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.BetweenNode.md).


---

## BinaryExprNode

`struct` · `datafusion_proto_models::generated::datafusion::BinaryExprNode`

Also reachable as `datafusion_proto::generated::datafusion::BinaryExprNode`, `datafusion_proto::protobuf::BinaryExprNode`, `datafusion_proto_models::protobuf::BinaryExprNode`

```rust
struct BinaryExprNode
```

**Fields**: `operands`, `op`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.BinaryExprNode.md).


---

## BufferExecNode

`struct` · `datafusion_proto_models::generated::datafusion::BufferExecNode`

Also reachable as `datafusion_proto::generated::datafusion::BufferExecNode`, `datafusion_proto::protobuf::BufferExecNode`, `datafusion_proto_models::protobuf::BufferExecNode`

```rust
struct BufferExecNode
```

**Fields**: `input`, `capacity`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.BufferExecNode.md).


---

## CaseNode

`struct` · `datafusion_proto_models::generated::datafusion::CaseNode`

Also reachable as `datafusion_proto::generated::datafusion::CaseNode`, `datafusion_proto::protobuf::CaseNode`, `datafusion_proto_models::protobuf::CaseNode`

```rust
struct CaseNode
```

**Fields**: `expr`, `when_then_expr`, `else_expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CaseNode.md).


---

## CastNode

`struct` · `datafusion_proto_models::generated::datafusion::CastNode`

Also reachable as `datafusion_proto::generated::datafusion::CastNode`, `datafusion_proto::protobuf::CastNode`, `datafusion_proto_models::protobuf::CastNode`

```rust
struct CastNode
```

**Fields**: `expr`, `arrow_type`, `metadata`, `nullable`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn nullable(&self) -> bool
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CastNode.md).


---

## CoalesceBatchesExecNode

`struct` · `datafusion_proto_models::generated::datafusion::CoalesceBatchesExecNode`

Also reachable as `datafusion_proto::generated::datafusion::CoalesceBatchesExecNode`, `datafusion_proto::protobuf::CoalesceBatchesExecNode`, `datafusion_proto_models::protobuf::CoalesceBatchesExecNode`

```rust
struct CoalesceBatchesExecNode
```

**Fields**: `input`, `target_batch_size`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fetch(&self) -> u32
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CoalesceBatchesExecNode.md).


---

## CoalescePartitionsExecNode

`struct` · `datafusion_proto_models::generated::datafusion::CoalescePartitionsExecNode`

Also reachable as `datafusion_proto::generated::datafusion::CoalescePartitionsExecNode`, `datafusion_proto::protobuf::CoalescePartitionsExecNode`, `datafusion_proto_models::protobuf::CoalescePartitionsExecNode`

```rust
struct CoalescePartitionsExecNode
```

**Fields**: `input`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fetch(&self) -> u32
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CoalescePartitionsExecNode.md).


---

## Column

`struct` · `datafusion_proto_models::generated::datafusion::Column`

Also reachable as `datafusion_proto::generated::datafusion::Column`, `datafusion_proto::protobuf::Column`, `datafusion_proto_models::protobuf::Column`

```rust
struct Column
```

**Fields**: `name`, `relation`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Column.md).


---

## ColumnIndex

`struct` · `datafusion_proto_models::generated::datafusion::ColumnIndex`

Also reachable as `datafusion_proto::generated::datafusion::ColumnIndex`, `datafusion_proto::protobuf::ColumnIndex`, `datafusion_proto_models::protobuf::ColumnIndex`

```rust
struct ColumnIndex
```

**Fields**: `index`, `side`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn set_side(&mut self, value: super::datafusion_common::JoinSide)
fn side(&self) -> super::datafusion_common::JoinSide
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ColumnIndex.md).


---

## ColumnRelation

`struct` · `datafusion_proto_models::generated::datafusion::ColumnRelation`

Also reachable as `datafusion_proto::generated::datafusion::ColumnRelation`, `datafusion_proto::protobuf::ColumnRelation`, `datafusion_proto_models::protobuf::ColumnRelation`

```rust
struct ColumnRelation
```

**Fields**: `relation`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ColumnRelation.md).


---

## ColumnStats

`struct` · `datafusion_proto_models::generated::datafusion::ColumnStats`

Also reachable as `datafusion_proto::generated::datafusion::ColumnStats`, `datafusion_proto::protobuf::ColumnStats`, `datafusion_proto_models::protobuf::ColumnStats`

```rust
struct ColumnStats
```

**Fields**: `min_value`, `max_value`, `sum_value`, `null_count`, `distinct_count`, `byte_size`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ColumnStats.md).


---

## ColumnUnnestListItem

`struct` · `datafusion_proto_models::generated::datafusion::ColumnUnnestListItem`

Also reachable as `datafusion_proto::generated::datafusion::ColumnUnnestListItem`, `datafusion_proto::protobuf::ColumnUnnestListItem`, `datafusion_proto_models::protobuf::ColumnUnnestListItem`

```rust
struct ColumnUnnestListItem
```

**Fields**: `input_index`, `recursion`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ColumnUnnestListItem.md).


---

## ColumnUnnestListRecursion

`struct` · `datafusion_proto_models::generated::datafusion::ColumnUnnestListRecursion`

Also reachable as `datafusion_proto::generated::datafusion::ColumnUnnestListRecursion`, `datafusion_proto::protobuf::ColumnUnnestListRecursion`, `datafusion_proto_models::protobuf::ColumnUnnestListRecursion`

```rust
struct ColumnUnnestListRecursion
```

**Fields**: `output_column`, `depth`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ColumnUnnestListRecursion.md).


---

## ColumnUnnestListRecursions

`struct` · `datafusion_proto_models::generated::datafusion::ColumnUnnestListRecursions`

Also reachable as `datafusion_proto::generated::datafusion::ColumnUnnestListRecursions`, `datafusion_proto::protobuf::ColumnUnnestListRecursions`, `datafusion_proto_models::protobuf::ColumnUnnestListRecursions`

```rust
struct ColumnUnnestListRecursions
```

**Fields**: `recursions`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ColumnUnnestListRecursions.md).


---

## Constraint

`struct` · `datafusion_proto_models::generated::datafusion::Constraint`

Also reachable as `datafusion_proto::generated::datafusion::Constraint`, `datafusion_proto::protobuf::Constraint`, `datafusion_proto_models::protobuf::Constraint`

```rust
struct Constraint
```

**Fields**: `constraint_mode`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Constraint.md).


---

## Constraints

`struct` · `datafusion_proto_models::generated::datafusion::Constraints`

Also reachable as `datafusion_proto::generated::datafusion::Constraints`, `datafusion_proto::protobuf::Constraints`, `datafusion_proto_models::protobuf::Constraints`

```rust
struct Constraints
```

**Fields**: `constraints`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Constraints.md).


---

## CooperativeExecNode

`struct` · `datafusion_proto_models::generated::datafusion::CooperativeExecNode`

Also reachable as `datafusion_proto::generated::datafusion::CooperativeExecNode`, `datafusion_proto::protobuf::CooperativeExecNode`, `datafusion_proto_models::protobuf::CooperativeExecNode`

```rust
struct CooperativeExecNode
```

**Fields**: `input`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CooperativeExecNode.md).


---

## CopyToNode

`struct` · `datafusion_proto_models::generated::datafusion::CopyToNode`

Also reachable as `datafusion_proto::generated::datafusion::CopyToNode`, `datafusion_proto::protobuf::CopyToNode`, `datafusion_proto_models::protobuf::CopyToNode`

```rust
struct CopyToNode
```

**Fields**: `input`, `output_url`, `file_type`, `partition_by`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CopyToNode.md).


---

## CreateCatalogNode

`struct` · `datafusion_proto_models::generated::datafusion::CreateCatalogNode`

Also reachable as `datafusion_proto::generated::datafusion::CreateCatalogNode`, `datafusion_proto::protobuf::CreateCatalogNode`, `datafusion_proto_models::protobuf::CreateCatalogNode`

```rust
struct CreateCatalogNode
```

**Fields**: `catalog_name`, `if_not_exists`, `schema`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CreateCatalogNode.md).


---

## CreateCatalogSchemaNode

`struct` · `datafusion_proto_models::generated::datafusion::CreateCatalogSchemaNode`

Also reachable as `datafusion_proto::generated::datafusion::CreateCatalogSchemaNode`, `datafusion_proto::protobuf::CreateCatalogSchemaNode`, `datafusion_proto_models::protobuf::CreateCatalogSchemaNode`

```rust
struct CreateCatalogSchemaNode
```

**Fields**: `schema_name`, `if_not_exists`, `schema`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CreateCatalogSchemaNode.md).


---

## CreateExternalTableNode

`struct` · `datafusion_proto_models::generated::datafusion::CreateExternalTableNode`

Also reachable as `datafusion_proto::generated::datafusion::CreateExternalTableNode`, `datafusion_proto::protobuf::CreateExternalTableNode`, `datafusion_proto_models::protobuf::CreateExternalTableNode`

```rust
struct CreateExternalTableNode
```

**Fields**: `name`, `location`, `locations`, `file_type`, `schema`, `table_partition_cols`, `if_not_exists`, `or_replace`, `temporary`, `definition`, `order_exprs`, `unbounded`, `options`, `constraints`, `column_defaults`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CreateExternalTableNode.md).


---

## CreateViewNode

`struct` · `datafusion_proto_models::generated::datafusion::CreateViewNode`

Also reachable as `datafusion_proto::generated::datafusion::CreateViewNode`, `datafusion_proto::protobuf::CreateViewNode`, `datafusion_proto_models::protobuf::CreateViewNode`

```rust
struct CreateViewNode
```

**Fields**: `name`, `input`, `or_replace`, `temporary`, `definition`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CreateViewNode.md).


---

## CrossJoinExecNode

`struct` · `datafusion_proto_models::generated::datafusion::CrossJoinExecNode`

Also reachable as `datafusion_proto::generated::datafusion::CrossJoinExecNode`, `datafusion_proto::protobuf::CrossJoinExecNode`, `datafusion_proto_models::protobuf::CrossJoinExecNode`

```rust
struct CrossJoinExecNode
```

**Fields**: `left`, `right`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CrossJoinExecNode.md).


---

## CrossJoinNode

`struct` · `datafusion_proto_models::generated::datafusion::CrossJoinNode`

Also reachable as `datafusion_proto::generated::datafusion::CrossJoinNode`, `datafusion_proto::protobuf::CrossJoinNode`, `datafusion_proto_models::protobuf::CrossJoinNode`

```rust
struct CrossJoinNode
```

**Fields**: `left`, `right`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CrossJoinNode.md).


---

## CsvFormat

`struct` · `datafusion_proto_models::generated::datafusion::CsvFormat`

Also reachable as `datafusion_proto::generated::datafusion::CsvFormat`, `datafusion_proto::protobuf::CsvFormat`, `datafusion_proto_models::protobuf::CsvFormat`

```rust
struct CsvFormat
```

**Fields**: `options`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CsvFormat.md).


---

## CsvOptions

`struct` · `datafusion_proto_models::generated::datafusion::CsvOptions`

Also reachable as `datafusion_proto::generated::datafusion::CsvOptions`, `datafusion_proto::protobuf::CsvOptions`, `datafusion_proto_models::protobuf::CsvOptions`

```rust
struct CsvOptions
```

**Fields**: `has_header`, `delimiter`, `quote`, `escape`, `compression`, `schema_infer_max_rec`, `date_format`, `datetime_format`, `timestamp_format`, `timestamp_tz_format`, `time_format`, `null_value`, `null_regex`, `comment`, `double_quote`, `newlines_in_values`, `terminator`, `truncated_rows`, `compression_level`, `quote_style`, `ignore_leading_whitespace`, `ignore_trailing_whitespace`

**Implements**: `core::convert::From`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn compression(&self) -> CompressionTypeVariant
fn compression_level(&self) -> u32
fn quote_style(&self) -> CsvQuoteStyle
fn schema_infer_max_rec(&self) -> u64
fn set_compression(&mut self, value: CompressionTypeVariant)
fn set_quote_style(&mut self, value: CsvQuoteStyle)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CsvOptions.md).


Options controlling CSV format

---

## CsvScanExecNode

`struct` · `datafusion_proto_models::generated::datafusion::CsvScanExecNode`

Also reachable as `datafusion_proto::generated::datafusion::CsvScanExecNode`, `datafusion_proto::protobuf::CsvScanExecNode`, `datafusion_proto_models::protobuf::CsvScanExecNode`

```rust
struct CsvScanExecNode
```

**Fields**: `base_conf`, `has_header`, `delimiter`, `quote`, `newlines_in_values`, `truncate_rows`, `optional_escape`, `optional_comment`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CsvScanExecNode.md).


---

## CsvSink

`struct` · `datafusion_proto_models::generated::datafusion::CsvSink`

Also reachable as `datafusion_proto::generated::datafusion::CsvSink`, `datafusion_proto::protobuf::CsvSink`, `datafusion_proto_models::protobuf::CsvSink`

```rust
struct CsvSink
```

**Fields**: `config`, `writer_options`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CsvSink.md).


---

## CsvSinkExecNode

`struct` · `datafusion_proto_models::generated::datafusion::CsvSinkExecNode`

Also reachable as `datafusion_proto::generated::datafusion::CsvSinkExecNode`, `datafusion_proto::protobuf::CsvSinkExecNode`, `datafusion_proto_models::protobuf::CsvSinkExecNode`

```rust
struct CsvSinkExecNode
```

**Fields**: `input`, `sink`, `sink_schema`, `sort_order`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CsvSinkExecNode.md).


---

## CsvWriterOptions

`struct` · `datafusion_proto_models::generated::datafusion::CsvWriterOptions`

Also reachable as `datafusion_proto::generated::datafusion::CsvWriterOptions`, `datafusion_proto::protobuf::CsvWriterOptions`, `datafusion_proto_models::protobuf::CsvWriterOptions`

```rust
struct CsvWriterOptions
```

**Fields**: `compression`, `delimiter`, `has_header`, `date_format`, `datetime_format`, `timestamp_format`, `time_format`, `null_value`, `quote`, `escape`, `double_quote`, `quote_style`, `ignore_leading_whitespace`, `ignore_trailing_whitespace`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn compression(&self) -> CompressionTypeVariant
fn quote_style(&self) -> CsvQuoteStyle
fn set_compression(&mut self, value: CompressionTypeVariant)
fn set_quote_style(&mut self, value: CsvQuoteStyle)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CsvWriterOptions.md).


---

## CteWorkTableScanNode

`struct` · `datafusion_proto_models::generated::datafusion::CteWorkTableScanNode`

Also reachable as `datafusion_proto::generated::datafusion::CteWorkTableScanNode`, `datafusion_proto::protobuf::CteWorkTableScanNode`, `datafusion_proto_models::protobuf::CteWorkTableScanNode`

```rust
struct CteWorkTableScanNode
```

**Fields**: `name`, `schema`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CteWorkTableScanNode.md).


---

## CubeNode

`struct` · `datafusion_proto_models::generated::datafusion::CubeNode`

Also reachable as `datafusion_proto::generated::datafusion::CubeNode`, `datafusion_proto::protobuf::CubeNode`, `datafusion_proto_models::protobuf::CubeNode`

```rust
struct CubeNode
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CubeNode.md).


---

## CustomTableScanNode

`struct` · `datafusion_proto_models::generated::datafusion::CustomTableScanNode`

Also reachable as `datafusion_proto::generated::datafusion::CustomTableScanNode`, `datafusion_proto::protobuf::CustomTableScanNode`, `datafusion_proto_models::protobuf::CustomTableScanNode`

```rust
struct CustomTableScanNode
```

**Fields**: `table_name`, `projection`, `schema`, `filters`, `custom_table_data`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.CustomTableScanNode.md).


Logical Plan to Scan a CustomTableProvider registered at runtime

---

## Decimal128

`struct` · `datafusion_proto_models::generated::datafusion::Decimal128`

Also reachable as `datafusion_proto::generated::datafusion::Decimal128`, `datafusion_proto::protobuf::Decimal128`, `datafusion_proto_models::protobuf::Decimal128`

```rust
struct Decimal128
```

**Fields**: `value`, `p`, `s`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal128.md).


---

## Decimal128Type

`struct` · `datafusion_proto_models::generated::datafusion::Decimal128Type`

Also reachable as `datafusion_proto::generated::datafusion::Decimal128Type`, `datafusion_proto::protobuf::Decimal128Type`, `datafusion_proto_models::protobuf::Decimal128Type`

```rust
struct Decimal128Type
```

**Fields**: `precision`, `scale`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal128Type.md).


---

## Decimal256

`struct` · `datafusion_proto_models::generated::datafusion::Decimal256`

Also reachable as `datafusion_proto::generated::datafusion::Decimal256`, `datafusion_proto::protobuf::Decimal256`, `datafusion_proto_models::protobuf::Decimal256`

```rust
struct Decimal256
```

**Fields**: `value`, `p`, `s`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal256.md).


---

## Decimal256Type

`struct` · `datafusion_proto_models::generated::datafusion::Decimal256Type`

Also reachable as `datafusion_proto::generated::datafusion::Decimal256Type`, `datafusion_proto::protobuf::Decimal256Type`, `datafusion_proto_models::protobuf::Decimal256Type`

```rust
struct Decimal256Type
```

**Fields**: `precision`, `scale`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal256Type.md).


---

## Decimal32

`struct` · `datafusion_proto_models::generated::datafusion::Decimal32`

Also reachable as `datafusion_proto::generated::datafusion::Decimal32`, `datafusion_proto::protobuf::Decimal32`, `datafusion_proto_models::protobuf::Decimal32`

```rust
struct Decimal32
```

**Fields**: `value`, `p`, `s`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal32.md).


---

## Decimal32Type

`struct` · `datafusion_proto_models::generated::datafusion::Decimal32Type`

Also reachable as `datafusion_proto::generated::datafusion::Decimal32Type`, `datafusion_proto::protobuf::Decimal32Type`, `datafusion_proto_models::protobuf::Decimal32Type`

```rust
struct Decimal32Type
```

**Fields**: `precision`, `scale`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal32Type.md).


---

## Decimal64

`struct` · `datafusion_proto_models::generated::datafusion::Decimal64`

Also reachable as `datafusion_proto::generated::datafusion::Decimal64`, `datafusion_proto::protobuf::Decimal64`, `datafusion_proto_models::protobuf::Decimal64`

```rust
struct Decimal64
```

**Fields**: `value`, `p`, `s`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal64.md).


---

## Decimal64Type

`struct` · `datafusion_proto_models::generated::datafusion::Decimal64Type`

Also reachable as `datafusion_proto::generated::datafusion::Decimal64Type`, `datafusion_proto::protobuf::Decimal64Type`, `datafusion_proto_models::protobuf::Decimal64Type`

```rust
struct Decimal64Type
```

**Fields**: `precision`, `scale`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Decimal64Type.md).


---

## DfField

`struct` · `datafusion_proto_models::generated::datafusion::DfField`

Also reachable as `datafusion_proto::generated::datafusion::DfField`, `datafusion_proto::protobuf::DfField`, `datafusion_proto_models::protobuf::DfField`

```rust
struct DfField
```

**Fields**: `field`, `qualifier`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.DfField.md).


---

## DfSchema

`struct` · `datafusion_proto_models::generated::datafusion::DfSchema`

Also reachable as `datafusion_proto::generated::datafusion::DfSchema`, `datafusion_proto::protobuf::DfSchema`, `datafusion_proto_models::protobuf::DfSchema`

```rust
struct DfSchema
```

**Fields**: `columns`, `metadata`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.DfSchema.md).


---

## Dictionary

`struct` · `datafusion_proto_models::generated::datafusion::Dictionary`

Also reachable as `datafusion_proto::generated::datafusion::Dictionary`, `datafusion_proto::protobuf::Dictionary`, `datafusion_proto_models::protobuf::Dictionary`

```rust
struct Dictionary
```

**Fields**: `key`, `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Dictionary.md).


---

## DistinctNode

`struct` · `datafusion_proto_models::generated::datafusion::DistinctNode`

Also reachable as `datafusion_proto::generated::datafusion::DistinctNode`, `datafusion_proto::protobuf::DistinctNode`, `datafusion_proto_models::protobuf::DistinctNode`

```rust
struct DistinctNode
```

**Fields**: `input`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.DistinctNode.md).


---

## DistinctOnNode

`struct` · `datafusion_proto_models::generated::datafusion::DistinctOnNode`

Also reachable as `datafusion_proto::generated::datafusion::DistinctOnNode`, `datafusion_proto::protobuf::DistinctOnNode`, `datafusion_proto_models::protobuf::DistinctOnNode`

```rust
struct DistinctOnNode
```

**Fields**: `on_expr`, `select_expr`, `sort_expr`, `input`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.DistinctOnNode.md).


---

## DmlNode

`struct` · `datafusion_proto_models::generated::datafusion::DmlNode`

Also reachable as `datafusion_proto::generated::datafusion::DmlNode`, `datafusion_proto::protobuf::DmlNode`, `datafusion_proto_models::protobuf::DmlNode`

```rust
struct DmlNode
```

**Fields**: `dml_type`, `input`, `table_name`, `target`, `merge_into`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn dml_type(&self) -> dml_node::Type
fn set_dml_type(&mut self, value: dml_node::Type)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.DmlNode.md).


---

## DropViewNode

`struct` · `datafusion_proto_models::generated::datafusion::DropViewNode`

Also reachable as `datafusion_proto::generated::datafusion::DropViewNode`, `datafusion_proto::protobuf::DropViewNode`, `datafusion_proto_models::protobuf::DropViewNode`

```rust
struct DropViewNode
```

**Fields**: `name`, `if_exists`, `schema`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.DropViewNode.md).


---

## EmptyExecNode

`struct` · `datafusion_proto_models::generated::datafusion::EmptyExecNode`

Also reachable as `datafusion_proto::generated::datafusion::EmptyExecNode`, `datafusion_proto::protobuf::EmptyExecNode`, `datafusion_proto_models::protobuf::EmptyExecNode`

```rust
struct EmptyExecNode
```

**Fields**: `schema`, `partitions`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.EmptyExecNode.md).


---

## EmptyMessage

`struct` · `datafusion_proto_models::generated::datafusion::EmptyMessage`

Also reachable as `datafusion_proto::generated::datafusion::EmptyMessage`, `datafusion_proto::protobuf::EmptyMessage`, `datafusion_proto_models::protobuf::EmptyMessage`

```rust
struct EmptyMessage
```

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.EmptyMessage.md).


Useful for representing an empty enum variant in rust
E.G. enum example{One, Two(i32)}
maps to
message example{
    oneof{
        EmptyMessage One = 1;
        i32 Two = 2;
   }
}

---

## EmptyRelationNode

`struct` · `datafusion_proto_models::generated::datafusion::EmptyRelationNode`

Also reachable as `datafusion_proto::generated::datafusion::EmptyRelationNode`, `datafusion_proto::protobuf::EmptyRelationNode`, `datafusion_proto_models::protobuf::EmptyRelationNode`

```rust
struct EmptyRelationNode
```

**Fields**: `produce_one_row`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.EmptyRelationNode.md).


---

## EmptyTableScanNode

`struct` · `datafusion_proto_models::generated::datafusion::EmptyTableScanNode`

Also reachable as `datafusion_proto::generated::datafusion::EmptyTableScanNode`, `datafusion_proto::protobuf::EmptyTableScanNode`, `datafusion_proto_models::protobuf::EmptyTableScanNode`

```rust
struct EmptyTableScanNode
```

**Fields**: `table_name`, `schema`, `projection`, `filters`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.EmptyTableScanNode.md).


---

## ExplainAnalyzeCategoriesNode

`struct` · `datafusion_proto_models::generated::datafusion::ExplainAnalyzeCategoriesNode`

Also reachable as `datafusion_proto::generated::datafusion::ExplainAnalyzeCategoriesNode`, `datafusion_proto::protobuf::ExplainAnalyzeCategoriesNode`, `datafusion_proto_models::protobuf::ExplainAnalyzeCategoriesNode`

```rust
struct ExplainAnalyzeCategoriesNode
```

**Fields**: `all`, `only`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn only(&self) -> ::core::iter::FilterMap<::core::iter::Cloned<::core::slice::Iter<'_, i32>>, fn(i32) -> ::core::option::Option<MetricCategory>>
fn push_only(&mut self, value: MetricCategory)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ExplainAnalyzeCategoriesNode.md).


Wire encoding for `datafusion_common::format::ExplainAnalyzeCategories`.

If `all` is true, every category is shown (the `only` list is ignored).
If `all` is false, only the categories listed in `only` are shown — an
empty `only` means "plan only", i.e. suppress all metrics.

---

## ExplainExecNode

`struct` · `datafusion_proto_models::generated::datafusion::ExplainExecNode`

Also reachable as `datafusion_proto::generated::datafusion::ExplainExecNode`, `datafusion_proto::protobuf::ExplainExecNode`, `datafusion_proto_models::protobuf::ExplainExecNode`

```rust
struct ExplainExecNode
```

**Fields**: `schema`, `stringified_plans`, `verbose`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ExplainExecNode.md).


---

## ExplainNode

`struct` · `datafusion_proto_models::generated::datafusion::ExplainNode`

Also reachable as `datafusion_proto::generated::datafusion::ExplainNode`, `datafusion_proto::protobuf::ExplainNode`, `datafusion_proto_models::protobuf::ExplainNode`

```rust
struct ExplainNode
```

**Fields**: `input`, `verbose`, `format`, `show_statistics`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn format(&self) -> super::datafusion_common::ExplainFormat
fn set_format(&mut self, value: super::datafusion_common::ExplainFormat)
fn show_statistics(&self) -> bool
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ExplainNode.md).


---

## Field

`struct` · `datafusion_proto_models::generated::datafusion::Field`

Also reachable as `datafusion_proto::generated::datafusion::Field`, `datafusion_proto::protobuf::Field`, `datafusion_proto_models::protobuf::Field`

```rust
struct Field
```

**Fields**: `name`, `arrow_type`, `nullable`, `children`, `metadata`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Field.md).


---

## FileFormatProto

`struct` · `datafusion_proto_models::generated::datafusion::FileFormatProto`

Also reachable as `datafusion_proto::generated::datafusion::FileFormatProto`, `datafusion_proto::protobuf::FileFormatProto`, `datafusion_proto_models::protobuf::FileFormatProto`

```rust
struct FileFormatProto
```

**Fields**: `kind`, `encoded_file_format`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn kind(&self) -> FileFormatKind
fn set_kind(&mut self, value: FileFormatKind)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FileFormatProto.md).


Wraps a serialized FileFormatFactory with its format kind tag,
so the decoder can dispatch to the correct format-specific codec.

---

## FileGroup

`struct` · `datafusion_proto_models::generated::datafusion::FileGroup`

Also reachable as `datafusion_proto::generated::datafusion::FileGroup`, `datafusion_proto::protobuf::FileGroup`, `datafusion_proto_models::protobuf::FileGroup`

```rust
struct FileGroup
```

**Fields**: `files`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(files: &[T]) -> Result<Self, Self::Error>
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FileGroup.md).


---

## FileRange

`struct` · `datafusion_proto_models::generated::datafusion::FileRange`

Also reachable as `datafusion_proto::generated::datafusion::FileRange`, `datafusion_proto::protobuf::FileRange`, `datafusion_proto_models::protobuf::FileRange`

```rust
struct FileRange
```

**Fields**: `start`, `end`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FileRange.md).


---

## FileScanExecConf

`struct` · `datafusion_proto_models::generated::datafusion::FileScanExecConf`

Also reachable as `datafusion_proto::generated::datafusion::FileScanExecConf`, `datafusion_proto::protobuf::FileScanExecConf`, `datafusion_proto_models::protobuf::FileScanExecConf`

```rust
struct FileScanExecConf
```

**Fields**: `file_groups`, `schema`, `projection`, `limit`, `statistics`, `table_partition_cols`, `object_store_url`, `output_ordering`, `constraints`, `batch_size`, `projection_exprs`, `output_partitioning`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn batch_size(&self) -> u64
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FileScanExecConf.md).


---

## FileSinkConfig

`struct` · `datafusion_proto_models::generated::datafusion::FileSinkConfig`

Also reachable as `datafusion_proto::generated::datafusion::FileSinkConfig`, `datafusion_proto::protobuf::FileSinkConfig`, `datafusion_proto_models::protobuf::FileSinkConfig`

```rust
struct FileSinkConfig
```

**Fields**: `object_store_url`, `file_groups`, `table_paths`, `output_schema`, `table_partition_cols`, `keep_partition_by_columns`, `insert_op`, `file_extension`, `file_output_mode`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn file_output_mode(&self) -> FileOutputMode
fn insert_op(&self) -> InsertOp
fn set_file_output_mode(&mut self, value: FileOutputMode)
fn set_insert_op(&mut self, value: InsertOp)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FileSinkConfig.md).


---

## FilterExecNode

`struct` · `datafusion_proto_models::generated::datafusion::FilterExecNode`

Also reachable as `datafusion_proto::generated::datafusion::FilterExecNode`, `datafusion_proto::protobuf::FilterExecNode`, `datafusion_proto_models::protobuf::FilterExecNode`

```rust
struct FilterExecNode
```

**Fields**: `input`, `expr`, `default_filter_selectivity`, `projection`, `batch_size`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fetch(&self) -> u32
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FilterExecNode.md).


---

## FixedSizeBinary

`struct` · `datafusion_proto_models::generated::datafusion::FixedSizeBinary`

Also reachable as `datafusion_proto::generated::datafusion::FixedSizeBinary`, `datafusion_proto::protobuf::FixedSizeBinary`, `datafusion_proto_models::protobuf::FixedSizeBinary`

```rust
struct FixedSizeBinary
```

**Fields**: `length`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FixedSizeBinary.md).


---

## FixedSizeList

`struct` · `datafusion_proto_models::generated::datafusion::FixedSizeList`

Also reachable as `datafusion_proto::generated::datafusion::FixedSizeList`, `datafusion_proto::protobuf::FixedSizeList`, `datafusion_proto_models::protobuf::FixedSizeList`

```rust
struct FixedSizeList
```

**Fields**: `field_type`, `list_size`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FixedSizeList.md).


---

## FullTableReference

`struct` · `datafusion_proto_models::generated::datafusion::FullTableReference`

Also reachable as `datafusion_proto::generated::datafusion::FullTableReference`, `datafusion_proto::protobuf::FullTableReference`, `datafusion_proto_models::protobuf::FullTableReference`

```rust
struct FullTableReference
```

**Fields**: `catalog`, `schema`, `table`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.FullTableReference.md).


---

## GenerateSeriesArgsContainsNull

`struct` · `datafusion_proto_models::generated::datafusion::GenerateSeriesArgsContainsNull`

Also reachable as `datafusion_proto::generated::datafusion::GenerateSeriesArgsContainsNull`, `datafusion_proto::protobuf::GenerateSeriesArgsContainsNull`, `datafusion_proto_models::protobuf::GenerateSeriesArgsContainsNull`

```rust
struct GenerateSeriesArgsContainsNull
```

**Fields**: `name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self) -> GenerateSeriesName
fn set_name(&mut self, value: GenerateSeriesName)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GenerateSeriesArgsContainsNull.md).


---

## GenerateSeriesArgsDate

`struct` · `datafusion_proto_models::generated::datafusion::GenerateSeriesArgsDate`

Also reachable as `datafusion_proto::generated::datafusion::GenerateSeriesArgsDate`, `datafusion_proto::protobuf::GenerateSeriesArgsDate`, `datafusion_proto_models::protobuf::GenerateSeriesArgsDate`

```rust
struct GenerateSeriesArgsDate
```

**Fields**: `start`, `end`, `step`, `include_end`, `name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self) -> GenerateSeriesName
fn set_name(&mut self, value: GenerateSeriesName)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GenerateSeriesArgsDate.md).


---

## GenerateSeriesArgsInt64

`struct` · `datafusion_proto_models::generated::datafusion::GenerateSeriesArgsInt64`

Also reachable as `datafusion_proto::generated::datafusion::GenerateSeriesArgsInt64`, `datafusion_proto::protobuf::GenerateSeriesArgsInt64`, `datafusion_proto_models::protobuf::GenerateSeriesArgsInt64`

```rust
struct GenerateSeriesArgsInt64
```

**Fields**: `start`, `end`, `step`, `include_end`, `name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn name(&self) -> GenerateSeriesName
fn set_name(&mut self, value: GenerateSeriesName)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GenerateSeriesArgsInt64.md).


---

## GenerateSeriesArgsTimestamp

`struct` · `datafusion_proto_models::generated::datafusion::GenerateSeriesArgsTimestamp`

Also reachable as `datafusion_proto::generated::datafusion::GenerateSeriesArgsTimestamp`, `datafusion_proto::protobuf::GenerateSeriesArgsTimestamp`, `datafusion_proto_models::protobuf::GenerateSeriesArgsTimestamp`

```rust
struct GenerateSeriesArgsTimestamp
```

**Fields**: `start`, `end`, `step`, `tz`, `include_end`, `name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn name(&self) -> GenerateSeriesName
fn set_name(&mut self, value: GenerateSeriesName)
fn tz(&self) -> &str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GenerateSeriesArgsTimestamp.md).


---

## GenerateSeriesNode

`struct` · `datafusion_proto_models::generated::datafusion::GenerateSeriesNode`

Also reachable as `datafusion_proto::generated::datafusion::GenerateSeriesNode`, `datafusion_proto::protobuf::GenerateSeriesNode`, `datafusion_proto_models::protobuf::GenerateSeriesNode`

```rust
struct GenerateSeriesNode
```

**Fields**: `schema`, `target_batch_size`, `args`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GenerateSeriesNode.md).


---

## GlobalLimitExecNode

`struct` · `datafusion_proto_models::generated::datafusion::GlobalLimitExecNode`

Also reachable as `datafusion_proto::generated::datafusion::GlobalLimitExecNode`, `datafusion_proto::protobuf::GlobalLimitExecNode`, `datafusion_proto_models::protobuf::GlobalLimitExecNode`

```rust
struct GlobalLimitExecNode
```

**Fields**: `input`, `skip`, `fetch`, `required_ordering`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GlobalLimitExecNode.md).


---

## GroupingSetNode

`struct` · `datafusion_proto_models::generated::datafusion::GroupingSetNode`

Also reachable as `datafusion_proto::generated::datafusion::GroupingSetNode`, `datafusion_proto::protobuf::GroupingSetNode`, `datafusion_proto_models::protobuf::GroupingSetNode`

```rust
struct GroupingSetNode
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.GroupingSetNode.md).


---

## HashJoinExecNode

`struct` · `datafusion_proto_models::generated::datafusion::HashJoinExecNode`

Also reachable as `datafusion_proto::generated::datafusion::HashJoinExecNode`, `datafusion_proto::protobuf::HashJoinExecNode`, `datafusion_proto_models::protobuf::HashJoinExecNode`

```rust
struct HashJoinExecNode
```

**Fields**: `left`, `right`, `on`, `join_type`, `partition_mode`, `null_equality`, `filter`, `projection`, `null_aware`, `dynamic_filter`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn fetch(&self) -> u64
fn join_type(&self) -> super::datafusion_common::JoinType
fn null_equality(&self) -> super::datafusion_common::NullEquality
fn partition_mode(&self) -> PartitionMode
fn set_join_type(&mut self, value: super::datafusion_common::JoinType)
fn set_null_equality(&mut self, value: super::datafusion_common::NullEquality)
fn set_partition_mode(&mut self, value: PartitionMode)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.HashJoinExecNode.md).


---

## HashRepartition

`struct` · `datafusion_proto_models::generated::datafusion::HashRepartition`

Also reachable as `datafusion_proto::generated::datafusion::HashRepartition`, `datafusion_proto::protobuf::HashRepartition`, `datafusion_proto_models::protobuf::HashRepartition`

```rust
struct HashRepartition
```

**Fields**: `hash_expr`, `partition_count`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.HashRepartition.md).


---

## HigherOrderUdfExprNode

`struct` · `datafusion_proto_models::generated::datafusion::HigherOrderUdfExprNode`

Also reachable as `datafusion_proto::generated::datafusion::HigherOrderUdfExprNode`, `datafusion_proto::protobuf::HigherOrderUdfExprNode`, `datafusion_proto_models::protobuf::HigherOrderUdfExprNode`

```rust
struct HigherOrderUdfExprNode
```

**Fields**: `fun_name`, `args`, `fun_definition`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fun_definition(&self) -> &[u8]
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.HigherOrderUdfExprNode.md).


---

## ILikeNode

`struct` · `datafusion_proto_models::generated::datafusion::ILikeNode`

Also reachable as `datafusion_proto::generated::datafusion::ILikeNode`, `datafusion_proto::protobuf::ILikeNode`, `datafusion_proto_models::protobuf::ILikeNode`

```rust
struct ILikeNode
```

**Fields**: `negated`, `expr`, `pattern`, `escape_char`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ILikeNode.md).


---

## InListNode

`struct` · `datafusion_proto_models::generated::datafusion::InListNode`

Also reachable as `datafusion_proto::generated::datafusion::InListNode`, `datafusion_proto::protobuf::InListNode`, `datafusion_proto_models::protobuf::InListNode`

```rust
struct InListNode
```

**Fields**: `expr`, `list`, `negated`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.InListNode.md).


---

## InterleaveExecNode

`struct` · `datafusion_proto_models::generated::datafusion::InterleaveExecNode`

Also reachable as `datafusion_proto::generated::datafusion::InterleaveExecNode`, `datafusion_proto::protobuf::InterleaveExecNode`, `datafusion_proto_models::protobuf::InterleaveExecNode`

```rust
struct InterleaveExecNode
```

**Fields**: `inputs`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.InterleaveExecNode.md).


---

## IntervalDayTimeValue

`struct` · `datafusion_proto_models::generated::datafusion::IntervalDayTimeValue`

Also reachable as `datafusion_proto::generated::datafusion::IntervalDayTimeValue`, `datafusion_proto::protobuf::IntervalDayTimeValue`, `datafusion_proto_models::protobuf::IntervalDayTimeValue`

```rust
struct IntervalDayTimeValue
```

**Fields**: `days`, `milliseconds`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IntervalDayTimeValue.md).


---

## IntervalMonthDayNanoValue

`struct` · `datafusion_proto_models::generated::datafusion::IntervalMonthDayNanoValue`

Also reachable as `datafusion_proto::generated::datafusion::IntervalMonthDayNanoValue`, `datafusion_proto::protobuf::IntervalMonthDayNanoValue`, `datafusion_proto_models::protobuf::IntervalMonthDayNanoValue`

```rust
struct IntervalMonthDayNanoValue
```

**Fields**: `months`, `days`, `nanos`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IntervalMonthDayNanoValue.md).


---

## IsFalse

`struct` · `datafusion_proto_models::generated::datafusion::IsFalse`

Also reachable as `datafusion_proto::generated::datafusion::IsFalse`, `datafusion_proto::protobuf::IsFalse`, `datafusion_proto_models::protobuf::IsFalse`

```rust
struct IsFalse
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsFalse.md).


---

## IsNotFalse

`struct` · `datafusion_proto_models::generated::datafusion::IsNotFalse`

Also reachable as `datafusion_proto::generated::datafusion::IsNotFalse`, `datafusion_proto::protobuf::IsNotFalse`, `datafusion_proto_models::protobuf::IsNotFalse`

```rust
struct IsNotFalse
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsNotFalse.md).


---

## IsNotNull

`struct` · `datafusion_proto_models::generated::datafusion::IsNotNull`

Also reachable as `datafusion_proto::generated::datafusion::IsNotNull`, `datafusion_proto::protobuf::IsNotNull`, `datafusion_proto_models::protobuf::IsNotNull`

```rust
struct IsNotNull
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsNotNull.md).


---

## IsNotTrue

`struct` · `datafusion_proto_models::generated::datafusion::IsNotTrue`

Also reachable as `datafusion_proto::generated::datafusion::IsNotTrue`, `datafusion_proto::protobuf::IsNotTrue`, `datafusion_proto_models::protobuf::IsNotTrue`

```rust
struct IsNotTrue
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsNotTrue.md).


---

## IsNotUnknown

`struct` · `datafusion_proto_models::generated::datafusion::IsNotUnknown`

Also reachable as `datafusion_proto::generated::datafusion::IsNotUnknown`, `datafusion_proto::protobuf::IsNotUnknown`, `datafusion_proto_models::protobuf::IsNotUnknown`

```rust
struct IsNotUnknown
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsNotUnknown.md).


---

## IsNull

`struct` · `datafusion_proto_models::generated::datafusion::IsNull`

Also reachable as `datafusion_proto::generated::datafusion::IsNull`, `datafusion_proto::protobuf::IsNull`, `datafusion_proto_models::protobuf::IsNull`

```rust
struct IsNull
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsNull.md).


---

## IsTrue

`struct` · `datafusion_proto_models::generated::datafusion::IsTrue`

Also reachable as `datafusion_proto::generated::datafusion::IsTrue`, `datafusion_proto::protobuf::IsTrue`, `datafusion_proto_models::protobuf::IsTrue`

```rust
struct IsTrue
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsTrue.md).


---

## IsUnknown

`struct` · `datafusion_proto_models::generated::datafusion::IsUnknown`

Also reachable as `datafusion_proto::generated::datafusion::IsUnknown`, `datafusion_proto::protobuf::IsUnknown`, `datafusion_proto_models::protobuf::IsUnknown`

```rust
struct IsUnknown
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.IsUnknown.md).


---

## JoinFilter

`struct` · `datafusion_proto_models::generated::datafusion::JoinFilter`

Also reachable as `datafusion_proto::generated::datafusion::JoinFilter`, `datafusion_proto::protobuf::JoinFilter`, `datafusion_proto_models::protobuf::JoinFilter`

```rust
struct JoinFilter
```

**Fields**: `expression`, `column_indices`, `schema`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JoinFilter.md).


---

## JoinNode

`struct` · `datafusion_proto_models::generated::datafusion::JoinNode`

Also reachable as `datafusion_proto::generated::datafusion::JoinNode`, `datafusion_proto::protobuf::JoinNode`, `datafusion_proto_models::protobuf::JoinNode`

```rust
struct JoinNode
```

**Fields**: `left`, `right`, `join_type`, `join_constraint`, `left_join_key`, `right_join_key`, `null_equality`, `filter`, `null_aware`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn join_constraint(&self) -> super::datafusion_common::JoinConstraint
fn join_type(&self) -> super::datafusion_common::JoinType
fn null_equality(&self) -> super::datafusion_common::NullEquality
fn set_join_constraint(&mut self, value: super::datafusion_common::JoinConstraint)
fn set_join_type(&mut self, value: super::datafusion_common::JoinType)
fn set_null_equality(&mut self, value: super::datafusion_common::NullEquality)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JoinNode.md).


---

## JoinOn

`struct` · `datafusion_proto_models::generated::datafusion::JoinOn`

Also reachable as `datafusion_proto::generated::datafusion::JoinOn`, `datafusion_proto::protobuf::JoinOn`, `datafusion_proto_models::protobuf::JoinOn`

```rust
struct JoinOn
```

**Fields**: `left`, `right`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JoinOn.md).


---

## JsonOptions

`struct` · `datafusion_proto_models::generated::datafusion::JsonOptions`

Also reachable as `datafusion_proto::generated::datafusion::JsonOptions`, `datafusion_proto::protobuf::JsonOptions`, `datafusion_proto_models::protobuf::JsonOptions`

```rust
struct JsonOptions
```

**Fields**: `compression`, `schema_infer_max_rec`, `compression_level`, `newline_delimited`

**Implements**: `core::convert::From`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn compression(&self) -> CompressionTypeVariant
fn compression_level(&self) -> u32
fn newline_delimited(&self) -> bool
fn schema_infer_max_rec(&self) -> u64
fn set_compression(&mut self, value: CompressionTypeVariant)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JsonOptions.md).


Options controlling CSV format

---

## JsonScanExecNode

`struct` · `datafusion_proto_models::generated::datafusion::JsonScanExecNode`

Also reachable as `datafusion_proto::generated::datafusion::JsonScanExecNode`, `datafusion_proto::protobuf::JsonScanExecNode`, `datafusion_proto_models::protobuf::JsonScanExecNode`

```rust
struct JsonScanExecNode
```

**Fields**: `base_conf`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JsonScanExecNode.md).


---

## JsonSink

`struct` · `datafusion_proto_models::generated::datafusion::JsonSink`

Also reachable as `datafusion_proto::generated::datafusion::JsonSink`, `datafusion_proto::protobuf::JsonSink`, `datafusion_proto_models::protobuf::JsonSink`

```rust
struct JsonSink
```

**Fields**: `config`, `writer_options`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JsonSink.md).


---

## JsonSinkExecNode

`struct` · `datafusion_proto_models::generated::datafusion::JsonSinkExecNode`

Also reachable as `datafusion_proto::generated::datafusion::JsonSinkExecNode`, `datafusion_proto::protobuf::JsonSinkExecNode`, `datafusion_proto_models::protobuf::JsonSinkExecNode`

```rust
struct JsonSinkExecNode
```

**Fields**: `input`, `sink`, `sink_schema`, `sort_order`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JsonSinkExecNode.md).


---

## JsonWriterOptions

`struct` · `datafusion_proto_models::generated::datafusion::JsonWriterOptions`

Also reachable as `datafusion_proto::generated::datafusion::JsonWriterOptions`, `datafusion_proto::protobuf::JsonWriterOptions`, `datafusion_proto_models::protobuf::JsonWriterOptions`

```rust
struct JsonWriterOptions
```

**Fields**: `compression`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn compression(&self) -> CompressionTypeVariant
fn set_compression(&mut self, value: CompressionTypeVariant)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.JsonWriterOptions.md).


---

## Lambda

`struct` · `datafusion_proto_models::generated::datafusion::Lambda`

Also reachable as `datafusion_proto::generated::datafusion::Lambda`, `datafusion_proto::protobuf::Lambda`, `datafusion_proto_models::protobuf::Lambda`

```rust
struct Lambda
```

**Fields**: `params`, `body`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Lambda.md).


---

## LambdaVariable

`struct` · `datafusion_proto_models::generated::datafusion::LambdaVariable`

Also reachable as `datafusion_proto::generated::datafusion::LambdaVariable`, `datafusion_proto::protobuf::LambdaVariable`, `datafusion_proto_models::protobuf::LambdaVariable`

```rust
struct LambdaVariable
```

**Fields**: `name`, `field`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LambdaVariable.md).


---

## LikeNode

`struct` · `datafusion_proto_models::generated::datafusion::LikeNode`

Also reachable as `datafusion_proto::generated::datafusion::LikeNode`, `datafusion_proto::protobuf::LikeNode`, `datafusion_proto_models::protobuf::LikeNode`

```rust
struct LikeNode
```

**Fields**: `negated`, `expr`, `pattern`, `escape_char`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LikeNode.md).


---

## LimitNode

`struct` · `datafusion_proto_models::generated::datafusion::LimitNode`

Also reachable as `datafusion_proto::generated::datafusion::LimitNode`, `datafusion_proto::protobuf::LimitNode`, `datafusion_proto_models::protobuf::LimitNode`

```rust
struct LimitNode
```

**Fields**: `input`, `skip`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LimitNode.md).


---

## List

`struct` · `datafusion_proto_models::generated::datafusion::List`

Also reachable as `datafusion_proto::generated::datafusion::List`, `datafusion_proto::protobuf::List`, `datafusion_proto_models::protobuf::List`

```rust
struct List
```

**Fields**: `field_type`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.List.md).


---

## ListIndex

`struct` · `datafusion_proto_models::generated::datafusion::ListIndex`

Also reachable as `datafusion_proto::generated::datafusion::ListIndex`, `datafusion_proto::protobuf::ListIndex`, `datafusion_proto_models::protobuf::ListIndex`

```rust
struct ListIndex
```

**Fields**: `key`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ListIndex.md).


---

## ListRange

`struct` · `datafusion_proto_models::generated::datafusion::ListRange`

Also reachable as `datafusion_proto::generated::datafusion::ListRange`, `datafusion_proto::protobuf::ListRange`, `datafusion_proto_models::protobuf::ListRange`

```rust
struct ListRange
```

**Fields**: `start`, `stop`, `stride`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ListRange.md).


---

## ListUnnest

`struct` · `datafusion_proto_models::generated::datafusion::ListUnnest`

Also reachable as `datafusion_proto::generated::datafusion::ListUnnest`, `datafusion_proto::protobuf::ListUnnest`, `datafusion_proto_models::protobuf::ListUnnest`

```rust
struct ListUnnest
```

**Fields**: `index_in_input_schema`, `depth`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ListUnnest.md).


---

## ListingTableScanNode

`struct` · `datafusion_proto_models::generated::datafusion::ListingTableScanNode`

Also reachable as `datafusion_proto::generated::datafusion::ListingTableScanNode`, `datafusion_proto::protobuf::ListingTableScanNode`, `datafusion_proto_models::protobuf::ListingTableScanNode`

```rust
struct ListingTableScanNode
```

**Fields**: `table_name`, `paths`, `file_extension`, `projection`, `schema`, `filters`, `table_partition_cols`, `file_sort_order`, `file_format_type`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ListingTableScanNode.md).


---

## LocalLimitExecNode

`struct` · `datafusion_proto_models::generated::datafusion::LocalLimitExecNode`

Also reachable as `datafusion_proto::generated::datafusion::LocalLimitExecNode`, `datafusion_proto::protobuf::LocalLimitExecNode`, `datafusion_proto_models::protobuf::LocalLimitExecNode`

```rust
struct LocalLimitExecNode
```

**Fields**: `input`, `fetch`, `required_ordering`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LocalLimitExecNode.md).


---

## LogicalExprList

`struct` · `datafusion_proto_models::generated::datafusion::LogicalExprList`

Also reachable as `datafusion_proto::generated::datafusion::LogicalExprList`, `datafusion_proto::protobuf::LogicalExprList`, `datafusion_proto_models::protobuf::LogicalExprList`

```rust
struct LogicalExprList
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LogicalExprList.md).


---

## LogicalExprNode

`struct` · `datafusion_proto_models::generated::datafusion::LogicalExprNode`

Also reachable as `datafusion_proto::generated::datafusion::LogicalExprNode`, `datafusion_proto::protobuf::LogicalExprNode`, `datafusion_proto_models::protobuf::LogicalExprNode`

```rust
struct LogicalExprNode
```

**Fields**: `expr_type`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LogicalExprNode.md).


logical expressions

---

## LogicalExprNodeCollection

`struct` · `datafusion_proto_models::generated::datafusion::LogicalExprNodeCollection`

Also reachable as `datafusion_proto::generated::datafusion::LogicalExprNodeCollection`, `datafusion_proto::protobuf::LogicalExprNodeCollection`, `datafusion_proto_models::protobuf::LogicalExprNodeCollection`

```rust
struct LogicalExprNodeCollection
```

**Fields**: `logical_expr_nodes`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LogicalExprNodeCollection.md).


---

## LogicalExtensionNode

`struct` · `datafusion_proto_models::generated::datafusion::LogicalExtensionNode`

Also reachable as `datafusion_proto::generated::datafusion::LogicalExtensionNode`, `datafusion_proto::protobuf::LogicalExtensionNode`, `datafusion_proto_models::protobuf::LogicalExtensionNode`

```rust
struct LogicalExtensionNode
```

**Fields**: `node`, `inputs`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LogicalExtensionNode.md).


---

## LogicalPlanNode

`struct` · `datafusion_proto_models::generated::datafusion::LogicalPlanNode`

Also reachable as `datafusion_proto::generated::datafusion::LogicalPlanNode`, `datafusion_proto::protobuf::LogicalPlanNode`, `datafusion_proto_models::protobuf::LogicalPlanNode`

```rust
struct LogicalPlanNode
```

**Fields**: `logical_plan_type`

**Implements**: `datafusion_proto::logical_plan::AsLogicalPlan`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.LogicalPlanNode.md).


logical plan
LogicalPlan is a nested type

---

## Map

`struct` · `datafusion_proto_models::generated::datafusion::Map`

Also reachable as `datafusion_proto::generated::datafusion::Map`, `datafusion_proto::protobuf::Map`, `datafusion_proto_models::protobuf::Map`

```rust
struct Map
```

**Fields**: `field_type`, `keys_sorted`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Map.md).


---

## MaybeFilter

`struct` · `datafusion_proto_models::generated::datafusion::MaybeFilter`

Also reachable as `datafusion_proto::generated::datafusion::MaybeFilter`, `datafusion_proto::protobuf::MaybeFilter`, `datafusion_proto_models::protobuf::MaybeFilter`

```rust
struct MaybeFilter
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MaybeFilter.md).


---

## MaybePhysicalSortExprs

`struct` · `datafusion_proto_models::generated::datafusion::MaybePhysicalSortExprs`

Also reachable as `datafusion_proto::generated::datafusion::MaybePhysicalSortExprs`, `datafusion_proto::protobuf::MaybePhysicalSortExprs`, `datafusion_proto_models::protobuf::MaybePhysicalSortExprs`

```rust
struct MaybePhysicalSortExprs
```

**Fields**: `sort_expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MaybePhysicalSortExprs.md).


---

## MemoryScanExecNode

`struct` · `datafusion_proto_models::generated::datafusion::MemoryScanExecNode`

Also reachable as `datafusion_proto::generated::datafusion::MemoryScanExecNode`, `datafusion_proto::protobuf::MemoryScanExecNode`, `datafusion_proto_models::protobuf::MemoryScanExecNode`

```rust
struct MemoryScanExecNode
```

**Fields**: `partitions`, `schema`, `projection`, `sort_information`, `show_sizes`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fetch(&self) -> u32
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MemoryScanExecNode.md).


---

## MergeAssignment

`struct` · `datafusion_proto_models::generated::datafusion::MergeAssignment`

Also reachable as `datafusion_proto::generated::datafusion::MergeAssignment`, `datafusion_proto::protobuf::MergeAssignment`, `datafusion_proto_models::protobuf::MergeAssignment`

```rust
struct MergeAssignment
```

**Fields**: `column`, `value`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MergeAssignment.md).


---

## MergeDeleteAction

`struct` · `datafusion_proto_models::generated::datafusion::MergeDeleteAction`

Also reachable as `datafusion_proto::generated::datafusion::MergeDeleteAction`, `datafusion_proto::protobuf::MergeDeleteAction`, `datafusion_proto_models::protobuf::MergeDeleteAction`

```rust
struct MergeDeleteAction
```

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MergeDeleteAction.md).


---

## MergeInsertAction

`struct` · `datafusion_proto_models::generated::datafusion::MergeInsertAction`

Also reachable as `datafusion_proto::generated::datafusion::MergeInsertAction`, `datafusion_proto::protobuf::MergeInsertAction`, `datafusion_proto_models::protobuf::MergeInsertAction`

```rust
struct MergeInsertAction
```

**Fields**: `columns`, `values`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MergeInsertAction.md).


---

## MergeIntoActionNode

`struct` · `datafusion_proto_models::generated::datafusion::MergeIntoActionNode`

Also reachable as `datafusion_proto::generated::datafusion::MergeIntoActionNode`, `datafusion_proto::protobuf::MergeIntoActionNode`, `datafusion_proto_models::protobuf::MergeIntoActionNode`

```rust
struct MergeIntoActionNode
```

**Fields**: `action`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MergeIntoActionNode.md).


The action for a single WHEN clause.

---

## MergeIntoClauseNode

`struct` · `datafusion_proto_models::generated::datafusion::MergeIntoClauseNode`

Also reachable as `datafusion_proto::generated::datafusion::MergeIntoClauseNode`, `datafusion_proto::protobuf::MergeIntoClauseNode`, `datafusion_proto_models::protobuf::MergeIntoClauseNode`

```rust
struct MergeIntoClauseNode
```

**Fields**: `kind`, `predicate`, `action`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn kind(&self) -> merge_into_clause_node::Kind
fn set_kind(&mut self, value: merge_into_clause_node::Kind)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MergeIntoClauseNode.md).


A single WHEN clause within a MERGE INTO statement.

---

## MergeIntoOpNode

`struct` · `datafusion_proto_models::generated::datafusion::MergeIntoOpNode`

Also reachable as `datafusion_proto::generated::datafusion::MergeIntoOpNode`, `datafusion_proto::protobuf::MergeIntoOpNode`, `datafusion_proto_models::protobuf::MergeIntoOpNode`

```rust
struct MergeIntoOpNode
```

**Fields**: `on`, `clauses`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MergeIntoOpNode.md).


Carries the ON condition and WHEN clauses of a MERGE INTO operation.

---

## MergeUpdateAction

`struct` · `datafusion_proto_models::generated::datafusion::MergeUpdateAction`

Also reachable as `datafusion_proto::generated::datafusion::MergeUpdateAction`, `datafusion_proto::protobuf::MergeUpdateAction`, `datafusion_proto_models::protobuf::MergeUpdateAction`

```rust
struct MergeUpdateAction
```

**Fields**: `assignments`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.MergeUpdateAction.md).


---

## NamedStructField

`struct` · `datafusion_proto_models::generated::datafusion::NamedStructField`

Also reachable as `datafusion_proto::generated::datafusion::NamedStructField`, `datafusion_proto::protobuf::NamedStructField`, `datafusion_proto_models::protobuf::NamedStructField`

```rust
struct NamedStructField
```

**Fields**: `name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.NamedStructField.md).


---

## NdJsonFormat

`struct` · `datafusion_proto_models::generated::datafusion::NdJsonFormat`

Also reachable as `datafusion_proto::generated::datafusion::NdJsonFormat`, `datafusion_proto::protobuf::NdJsonFormat`, `datafusion_proto_models::protobuf::NdJsonFormat`

```rust
struct NdJsonFormat
```

**Fields**: `options`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.NdJsonFormat.md).


---

## NegativeNode

`struct` · `datafusion_proto_models::generated::datafusion::NegativeNode`

Also reachable as `datafusion_proto::generated::datafusion::NegativeNode`, `datafusion_proto::protobuf::NegativeNode`, `datafusion_proto_models::protobuf::NegativeNode`

```rust
struct NegativeNode
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.NegativeNode.md).


---

## NestedLoopJoinExecNode

`struct` · `datafusion_proto_models::generated::datafusion::NestedLoopJoinExecNode`

Also reachable as `datafusion_proto::generated::datafusion::NestedLoopJoinExecNode`, `datafusion_proto::protobuf::NestedLoopJoinExecNode`, `datafusion_proto_models::protobuf::NestedLoopJoinExecNode`

```rust
struct NestedLoopJoinExecNode
```

**Fields**: `left`, `right`, `join_type`, `filter`, `projection`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn join_type(&self) -> super::datafusion_common::JoinType
fn set_join_type(&mut self, value: super::datafusion_common::JoinType)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.NestedLoopJoinExecNode.md).


---

## Not

`struct` · `datafusion_proto_models::generated::datafusion::Not`

Also reachable as `datafusion_proto::generated::datafusion::Not`, `datafusion_proto::protobuf::Not`, `datafusion_proto_models::protobuf::Not`

```rust
struct Not
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Not.md).


---

## OptimizedLogicalPlanType

`struct` · `datafusion_proto_models::generated::datafusion::OptimizedLogicalPlanType`

Also reachable as `datafusion_proto::generated::datafusion::OptimizedLogicalPlanType`, `datafusion_proto::protobuf::OptimizedLogicalPlanType`, `datafusion_proto_models::protobuf::OptimizedLogicalPlanType`

```rust
struct OptimizedLogicalPlanType
```

**Fields**: `optimizer_name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.OptimizedLogicalPlanType.md).


---

## OptimizedPhysicalPlanType

`struct` · `datafusion_proto_models::generated::datafusion::OptimizedPhysicalPlanType`

Also reachable as `datafusion_proto::generated::datafusion::OptimizedPhysicalPlanType`, `datafusion_proto::protobuf::OptimizedPhysicalPlanType`, `datafusion_proto_models::protobuf::OptimizedPhysicalPlanType`

```rust
struct OptimizedPhysicalPlanType
```

**Fields**: `optimizer_name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.OptimizedPhysicalPlanType.md).


---

## ParquetCdcOptions

`struct` · `datafusion_proto_models::generated::datafusion::ParquetCdcOptions`

Also reachable as `datafusion_proto::generated::datafusion::ParquetCdcOptions`, `datafusion_proto::protobuf::ParquetCdcOptions`, `datafusion_proto_models::protobuf::ParquetCdcOptions`

```rust
struct ParquetCdcOptions
```

**Fields**: `enabled`, `min_chunk_size`, `max_chunk_size`, `norm_level`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetCdcOptions.md).


Content-defined chunking (CDC) options for writing parquet files.

---

## ParquetColumnOptions

`struct` · `datafusion_proto_models::generated::datafusion::ParquetColumnOptions`

Also reachable as `datafusion_proto::generated::datafusion::ParquetColumnOptions`, `datafusion_proto::protobuf::ParquetColumnOptions`, `datafusion_proto_models::protobuf::ParquetColumnOptions`

```rust
struct ParquetColumnOptions
```

**Fields**: `bloom_filter_enabled_opt`, `encoding_opt`, `dictionary_enabled_opt`, `compression_opt`, `statistics_enabled_opt`, `bloom_filter_fpp_opt`, `bloom_filter_ndv_opt`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetColumnOptions.md).


---

## ParquetColumnSpecificOptions

`struct` · `datafusion_proto_models::generated::datafusion::ParquetColumnSpecificOptions`

Also reachable as `datafusion_proto::generated::datafusion::ParquetColumnSpecificOptions`, `datafusion_proto::protobuf::ParquetColumnSpecificOptions`, `datafusion_proto_models::protobuf::ParquetColumnSpecificOptions`

```rust
struct ParquetColumnSpecificOptions
```

**Fields**: `column_name`, `options`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetColumnSpecificOptions.md).


---

## ParquetFormat

`struct` · `datafusion_proto_models::generated::datafusion::ParquetFormat`

Also reachable as `datafusion_proto::generated::datafusion::ParquetFormat`, `datafusion_proto::protobuf::ParquetFormat`, `datafusion_proto_models::protobuf::ParquetFormat`

```rust
struct ParquetFormat
```

**Fields**: `options`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetFormat.md).


---

## ParquetOptions

`struct` · `datafusion_proto_models::generated::datafusion::ParquetOptions`

Also reachable as `datafusion_proto::generated::datafusion::ParquetOptions`, `datafusion_proto::protobuf::ParquetOptions`, `datafusion_proto_models::protobuf::ParquetOptions`

```rust
struct ParquetOptions
```

**Fields**: `enable_page_index`, `pruning`, `skip_metadata`, `pushdown_filters`, `reorder_filters`, `force_filter_selections`, `data_pagesize_limit`, `write_batch_size`, `writer_version`, `allow_single_file_parallelism`, `maximum_parallel_row_group_writers`, `maximum_buffered_record_batches_per_stream`, `bloom_filter_on_read`, `bloom_filter_on_write`, `schema_force_view_types`, `binary_as_string`, `skip_arrow_metadata`, `dictionary_page_size_limit`, `data_page_row_count_limit`, `max_row_group_size`, `max_in_list_size`, `created_by`, `content_defined_chunking`, `metadata_size_hint_opt`, `compression_opt`, `dictionary_enabled_opt`, `statistics_enabled_opt`, `column_index_truncate_length_opt`, `statistics_truncate_length_opt`, `encoding_opt`, `bloom_filter_fpp_opt`, `bloom_filter_ndv_opt`, `coerce_int96_opt`, `max_predicate_cache_size_opt`, `max_row_group_bytes_opt`, `coerce_int96_tz_opt`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetOptions.md).


---

## ParquetScanExecNode

`struct` · `datafusion_proto_models::generated::datafusion::ParquetScanExecNode`

Also reachable as `datafusion_proto::generated::datafusion::ParquetScanExecNode`, `datafusion_proto::protobuf::ParquetScanExecNode`, `datafusion_proto_models::protobuf::ParquetScanExecNode`

```rust
struct ParquetScanExecNode
```

**Fields**: `base_conf`, `predicate`, `parquet_options`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetScanExecNode.md).


---

## ParquetSink

`struct` · `datafusion_proto_models::generated::datafusion::ParquetSink`

Also reachable as `datafusion_proto::generated::datafusion::ParquetSink`, `datafusion_proto::protobuf::ParquetSink`, `datafusion_proto_models::protobuf::ParquetSink`

```rust
struct ParquetSink
```

**Fields**: `config`, `parquet_options`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetSink.md).


---

## ParquetSinkExecNode

`struct` · `datafusion_proto_models::generated::datafusion::ParquetSinkExecNode`

Also reachable as `datafusion_proto::generated::datafusion::ParquetSinkExecNode`, `datafusion_proto::protobuf::ParquetSinkExecNode`, `datafusion_proto_models::protobuf::ParquetSinkExecNode`

```rust
struct ParquetSinkExecNode
```

**Fields**: `input`, `sink`, `sink_schema`, `sort_order`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ParquetSinkExecNode.md).


---

## PartialTableReference

`struct` · `datafusion_proto_models::generated::datafusion::PartialTableReference`

Also reachable as `datafusion_proto::generated::datafusion::PartialTableReference`, `datafusion_proto::protobuf::PartialTableReference`, `datafusion_proto_models::protobuf::PartialTableReference`

```rust
struct PartialTableReference
```

**Fields**: `schema`, `table`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PartialTableReference.md).


---

## PartiallySortedInputOrderMode

`struct` · `datafusion_proto_models::generated::datafusion::PartiallySortedInputOrderMode`

Also reachable as `datafusion_proto::generated::datafusion::PartiallySortedInputOrderMode`, `datafusion_proto::protobuf::PartiallySortedInputOrderMode`, `datafusion_proto_models::protobuf::PartiallySortedInputOrderMode`

```rust
struct PartiallySortedInputOrderMode
```

**Fields**: `columns`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PartiallySortedInputOrderMode.md).


---

## PartitionColumn

`struct` · `datafusion_proto_models::generated::datafusion::PartitionColumn`

Also reachable as `datafusion_proto::generated::datafusion::PartitionColumn`, `datafusion_proto::protobuf::PartitionColumn`, `datafusion_proto_models::protobuf::PartitionColumn`

```rust
struct PartitionColumn
```

**Fields**: `name`, `arrow_type`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PartitionColumn.md).


---

## PartitionStats

`struct` · `datafusion_proto_models::generated::datafusion::PartitionStats`

Also reachable as `datafusion_proto::generated::datafusion::PartitionStats`, `datafusion_proto::protobuf::PartitionStats`, `datafusion_proto_models::protobuf::PartitionStats`

```rust
struct PartitionStats
```

**Fields**: `num_rows`, `num_batches`, `num_bytes`, `column_stats`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PartitionStats.md).


---

## PartitionedFile

`struct` · `datafusion_proto_models::generated::datafusion::PartitionedFile`

Also reachable as `datafusion_proto::generated::datafusion::PartitionedFile`, `datafusion_proto::protobuf::PartitionedFile`, `datafusion_proto_models::protobuf::PartitionedFile`

```rust
struct PartitionedFile
```

**Fields**: `path`, `size`, `last_modified_ns`, `partition_values`, `range`, `statistics`, `arrow_schema`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PartitionedFile.md).


---

## Partitioning

`struct` · `datafusion_proto_models::generated::datafusion::Partitioning`

Also reachable as `datafusion_proto::generated::datafusion::Partitioning`, `datafusion_proto::protobuf::Partitioning`, `datafusion_proto_models::protobuf::Partitioning`

```rust
struct Partitioning
```

**Fields**: `partition_method`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Partitioning.md).


---

## PhysicalAggregateExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalAggregateExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalAggregateExprNode`, `datafusion_proto::protobuf::PhysicalAggregateExprNode`, `datafusion_proto_models::protobuf::PhysicalAggregateExprNode`

```rust
struct PhysicalAggregateExprNode
```

**Fields**: `expr`, `ordering_req`, `distinct`, `ignore_nulls`, `fun_definition`, `human_display`, `is_reversed`, `aggregate_function`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fun_definition(&self) -> &[u8]
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalAggregateExprNode.md).


---

## PhysicalAliasNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalAliasNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalAliasNode`, `datafusion_proto::protobuf::PhysicalAliasNode`, `datafusion_proto_models::protobuf::PhysicalAliasNode`

```rust
struct PhysicalAliasNode
```

**Fields**: `expr`, `alias`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalAliasNode.md).


---

## PhysicalBinaryExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalBinaryExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalBinaryExprNode`, `datafusion_proto::protobuf::PhysicalBinaryExprNode`, `datafusion_proto_models::protobuf::PhysicalBinaryExprNode`

```rust
struct PhysicalBinaryExprNode
```

**Fields**: `l`, `r`, `op`, `operands`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalBinaryExprNode.md).


---

## PhysicalCaseNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalCaseNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalCaseNode`, `datafusion_proto::protobuf::PhysicalCaseNode`, `datafusion_proto_models::protobuf::PhysicalCaseNode`

```rust
struct PhysicalCaseNode
```

**Fields**: `expr`, `when_then_expr`, `else_expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalCaseNode.md).


---

## PhysicalCastNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalCastNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalCastNode`, `datafusion_proto::protobuf::PhysicalCastNode`, `datafusion_proto_models::protobuf::PhysicalCastNode`

```rust
struct PhysicalCastNode
```

**Fields**: `expr`, `arrow_type`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalCastNode.md).


---

## PhysicalColumn

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalColumn`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalColumn`, `datafusion_proto::protobuf::PhysicalColumn`, `datafusion_proto_models::protobuf::PhysicalColumn`

```rust
struct PhysicalColumn
```

**Fields**: `name`, `index`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalColumn.md).


---

## PhysicalDateTimeIntervalExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalDateTimeIntervalExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalDateTimeIntervalExprNode`, `datafusion_proto::protobuf::PhysicalDateTimeIntervalExprNode`, `datafusion_proto_models::protobuf::PhysicalDateTimeIntervalExprNode`

```rust
struct PhysicalDateTimeIntervalExprNode
```

**Fields**: `l`, `r`, `op`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalDateTimeIntervalExprNode.md).


---

## PhysicalDynamicFilterNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalDynamicFilterNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalDynamicFilterNode`, `datafusion_proto::protobuf::PhysicalDynamicFilterNode`, `datafusion_proto_models::protobuf::PhysicalDynamicFilterNode`

```rust
struct PhysicalDynamicFilterNode
```

**Fields**: `children`, `remapped_children`, `generation`, `inner_expr`, `is_complete`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalDynamicFilterNode.md).


---

## PhysicalExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalExprNode`, `datafusion_proto::protobuf::PhysicalExprNode`, `datafusion_proto_models::protobuf::PhysicalExprNode`

```rust
struct PhysicalExprNode
```

**Fields**: `expr_id`, `expr_type`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn expr_id(&self) -> u64
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalExprNode.md).


physical expressions

---

## PhysicalExtensionExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalExtensionExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalExtensionExprNode`, `datafusion_proto::protobuf::PhysicalExtensionExprNode`, `datafusion_proto_models::protobuf::PhysicalExtensionExprNode`

```rust
struct PhysicalExtensionExprNode
```

**Fields**: `expr`, `inputs`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalExtensionExprNode.md).


---

## PhysicalExtensionNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalExtensionNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalExtensionNode`, `datafusion_proto::protobuf::PhysicalExtensionNode`, `datafusion_proto_models::protobuf::PhysicalExtensionNode`

```rust
struct PhysicalExtensionNode
```

**Fields**: `node`, `inputs`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalExtensionNode.md).


---

## PhysicalHashExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalHashExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalHashExprNode`, `datafusion_proto::protobuf::PhysicalHashExprNode`, `datafusion_proto_models::protobuf::PhysicalHashExprNode`

```rust
struct PhysicalHashExprNode
```

**Fields**: `on_columns`, `seed0`, `description`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalHashExprNode.md).


---

## PhysicalHashRepartition

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalHashRepartition`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalHashRepartition`, `datafusion_proto::protobuf::PhysicalHashRepartition`, `datafusion_proto_models::protobuf::PhysicalHashRepartition`

```rust
struct PhysicalHashRepartition
```

**Fields**: `hash_expr`, `partition_count`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalHashRepartition.md).


---

## PhysicalHigherOrderUdfNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalHigherOrderUdfNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalHigherOrderUdfNode`, `datafusion_proto::protobuf::PhysicalHigherOrderUdfNode`, `datafusion_proto_models::protobuf::PhysicalHigherOrderUdfNode`

```rust
struct PhysicalHigherOrderUdfNode
```

**Fields**: `name`, `args`, `fun_definition`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fun_definition(&self) -> &[u8]
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalHigherOrderUdfNode.md).


---

## PhysicalInListNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalInListNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalInListNode`, `datafusion_proto::protobuf::PhysicalInListNode`, `datafusion_proto_models::protobuf::PhysicalInListNode`

```rust
struct PhysicalInListNode
```

**Fields**: `expr`, `list`, `negated`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalInListNode.md).


---

## PhysicalIsNotNull

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalIsNotNull`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalIsNotNull`, `datafusion_proto::protobuf::PhysicalIsNotNull`, `datafusion_proto_models::protobuf::PhysicalIsNotNull`

```rust
struct PhysicalIsNotNull
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalIsNotNull.md).


---

## PhysicalIsNull

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalIsNull`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalIsNull`, `datafusion_proto::protobuf::PhysicalIsNull`, `datafusion_proto_models::protobuf::PhysicalIsNull`

```rust
struct PhysicalIsNull
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalIsNull.md).


---

## PhysicalLambdaExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalLambdaExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalLambdaExprNode`, `datafusion_proto::protobuf::PhysicalLambdaExprNode`, `datafusion_proto_models::protobuf::PhysicalLambdaExprNode`

```rust
struct PhysicalLambdaExprNode
```

**Fields**: `params`, `body`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalLambdaExprNode.md).


---

## PhysicalLambdaVariableExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalLambdaVariableExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalLambdaVariableExprNode`, `datafusion_proto::protobuf::PhysicalLambdaVariableExprNode`, `datafusion_proto_models::protobuf::PhysicalLambdaVariableExprNode`

```rust
struct PhysicalLambdaVariableExprNode
```

**Fields**: `index`, `field`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalLambdaVariableExprNode.md).


---

## PhysicalLikeExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalLikeExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalLikeExprNode`, `datafusion_proto::protobuf::PhysicalLikeExprNode`, `datafusion_proto_models::protobuf::PhysicalLikeExprNode`

```rust
struct PhysicalLikeExprNode
```

**Fields**: `negated`, `case_insensitive`, `expr`, `pattern`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalLikeExprNode.md).


---

## PhysicalNegativeNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalNegativeNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalNegativeNode`, `datafusion_proto::protobuf::PhysicalNegativeNode`, `datafusion_proto_models::protobuf::PhysicalNegativeNode`

```rust
struct PhysicalNegativeNode
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalNegativeNode.md).


---

## PhysicalNot

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalNot`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalNot`, `datafusion_proto::protobuf::PhysicalNot`, `datafusion_proto_models::protobuf::PhysicalNot`

```rust
struct PhysicalNot
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalNot.md).


---

## PhysicalPlanNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalPlanNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalPlanNode`, `datafusion_proto::protobuf::PhysicalPlanNode`, `datafusion_proto_models::protobuf::PhysicalPlanNode`

```rust
struct PhysicalPlanNode
```

**Fields**: `physical_plan_type`

**Implements**: `datafusion_proto::physical_plan::AsExecutionPlan`, `datafusion_proto::physical_plan::PhysicalPlanNodeExt`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalPlanNode.md).


PhysicalPlanNode is a nested type

---

## PhysicalRangeExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalRangeExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalRangeExprNode`, `datafusion_proto::protobuf::PhysicalRangeExprNode`, `datafusion_proto_models::protobuf::PhysicalRangeExprNode`

```rust
struct PhysicalRangeExprNode
```

**Fields**: `sort_expr`, `split_point`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalRangeExprNode.md).


---

## PhysicalRangePartitioning

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalRangePartitioning`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalRangePartitioning`, `datafusion_proto::protobuf::PhysicalRangePartitioning`, `datafusion_proto_models::protobuf::PhysicalRangePartitioning`

```rust
struct PhysicalRangePartitioning
```

**Fields**: `sort_expr`, `split_point`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalRangePartitioning.md).


---

## PhysicalRangeSplitPoint

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalRangeSplitPoint`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalRangeSplitPoint`, `datafusion_proto::protobuf::PhysicalRangeSplitPoint`, `datafusion_proto_models::protobuf::PhysicalRangeSplitPoint`

```rust
struct PhysicalRangeSplitPoint
```

**Fields**: `value`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalRangeSplitPoint.md).


---

## PhysicalScalarSubqueryExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalScalarSubqueryExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalScalarSubqueryExprNode`, `datafusion_proto::protobuf::PhysicalScalarSubqueryExprNode`, `datafusion_proto_models::protobuf::PhysicalScalarSubqueryExprNode`

```rust
struct PhysicalScalarSubqueryExprNode
```

**Fields**: `data_type`, `nullable`, `index`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalScalarSubqueryExprNode.md).


---

## PhysicalScalarUdfNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalScalarUdfNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalScalarUdfNode`, `datafusion_proto::protobuf::PhysicalScalarUdfNode`, `datafusion_proto_models::protobuf::PhysicalScalarUdfNode`

```rust
struct PhysicalScalarUdfNode
```

**Fields**: `name`, `args`, `fun_definition`, `return_type`, `nullable`, `return_field_name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fun_definition(&self) -> &[u8]
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalScalarUdfNode.md).


---

## PhysicalSortExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalSortExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalSortExprNode`, `datafusion_proto::protobuf::PhysicalSortExprNode`, `datafusion_proto_models::protobuf::PhysicalSortExprNode`

```rust
struct PhysicalSortExprNode
```

**Fields**: `expr`, `asc`, `nulls_first`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalSortExprNode.md).


---

## PhysicalSortExprNodeCollection

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalSortExprNodeCollection`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalSortExprNodeCollection`, `datafusion_proto::protobuf::PhysicalSortExprNodeCollection`, `datafusion_proto_models::protobuf::PhysicalSortExprNodeCollection`

```rust
struct PhysicalSortExprNodeCollection
```

**Fields**: `physical_sort_expr_nodes`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalSortExprNodeCollection.md).


---

## PhysicalTryCastNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalTryCastNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalTryCastNode`, `datafusion_proto::protobuf::PhysicalTryCastNode`, `datafusion_proto_models::protobuf::PhysicalTryCastNode`

```rust
struct PhysicalTryCastNode
```

**Fields**: `expr`, `arrow_type`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalTryCastNode.md).


---

## PhysicalWhenThen

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalWhenThen`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalWhenThen`, `datafusion_proto::protobuf::PhysicalWhenThen`, `datafusion_proto_models::protobuf::PhysicalWhenThen`

```rust
struct PhysicalWhenThen
```

**Fields**: `when_expr`, `then_expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalWhenThen.md).


---

## PhysicalWindowExprNode

`struct` · `datafusion_proto_models::generated::datafusion::PhysicalWindowExprNode`

Also reachable as `datafusion_proto::generated::datafusion::PhysicalWindowExprNode`, `datafusion_proto::protobuf::PhysicalWindowExprNode`, `datafusion_proto_models::protobuf::PhysicalWindowExprNode`

```rust
struct PhysicalWindowExprNode
```

**Fields**: `args`, `partition_by`, `order_by`, `window_frame`, `name`, `fun_definition`, `ignore_nulls`, `distinct`, `window_function`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fun_definition(&self) -> &[u8]
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PhysicalWindowExprNode.md).


---

## PlaceholderNode

`struct` · `datafusion_proto_models::generated::datafusion::PlaceholderNode`

Also reachable as `datafusion_proto::generated::datafusion::PlaceholderNode`, `datafusion_proto::protobuf::PlaceholderNode`, `datafusion_proto_models::protobuf::PlaceholderNode`

```rust
struct PlaceholderNode
```

**Fields**: `id`, `data_type`, `nullable`, `metadata`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn nullable(&self) -> bool
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PlaceholderNode.md).


---

## PlaceholderRowExecNode

`struct` · `datafusion_proto_models::generated::datafusion::PlaceholderRowExecNode`

Also reachable as `datafusion_proto::generated::datafusion::PlaceholderRowExecNode`, `datafusion_proto::protobuf::PlaceholderRowExecNode`, `datafusion_proto_models::protobuf::PlaceholderRowExecNode`

```rust
struct PlaceholderRowExecNode
```

**Fields**: `schema`, `partitions`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PlaceholderRowExecNode.md).


---

## PlanType

`struct` · `datafusion_proto_models::generated::datafusion::PlanType`

Also reachable as `datafusion_proto::generated::datafusion::PlanType`, `datafusion_proto::protobuf::PlanType`, `datafusion_proto_models::protobuf::PlanType`

```rust
struct PlanType
```

**Fields**: `plan_type_enum`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PlanType.md).


---

## Precision

`struct` · `datafusion_proto_models::generated::datafusion::Precision`

Also reachable as `datafusion_proto::generated::datafusion::Precision`, `datafusion_proto::protobuf::Precision`, `datafusion_proto_models::protobuf::Precision`

```rust
struct Precision
```

**Fields**: `precision_info`, `val`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn precision_info(&self) -> PrecisionInfo
fn set_precision_info(&mut self, value: PrecisionInfo)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Precision.md).


---

## PrepareNode

`struct` · `datafusion_proto_models::generated::datafusion::PrepareNode`

Also reachable as `datafusion_proto::generated::datafusion::PrepareNode`, `datafusion_proto::protobuf::PrepareNode`, `datafusion_proto_models::protobuf::PrepareNode`

```rust
struct PrepareNode
```

**Fields**: `name`, `data_types`, `input`, `fields`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PrepareNode.md).


---

## PrimaryKeyConstraint

`struct` · `datafusion_proto_models::generated::datafusion::PrimaryKeyConstraint`

Also reachable as `datafusion_proto::generated::datafusion::PrimaryKeyConstraint`, `datafusion_proto::protobuf::PrimaryKeyConstraint`, `datafusion_proto_models::protobuf::PrimaryKeyConstraint`

```rust
struct PrimaryKeyConstraint
```

**Fields**: `indices`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.PrimaryKeyConstraint.md).


---

## ProjectionColumns

`struct` · `datafusion_proto_models::generated::datafusion::ProjectionColumns`

Also reachable as `datafusion_proto::generated::datafusion::ProjectionColumns`, `datafusion_proto::protobuf::ProjectionColumns`, `datafusion_proto_models::protobuf::ProjectionColumns`

```rust
struct ProjectionColumns
```

**Fields**: `columns`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ProjectionColumns.md).


---

## ProjectionExecNode

`struct` · `datafusion_proto_models::generated::datafusion::ProjectionExecNode`

Also reachable as `datafusion_proto::generated::datafusion::ProjectionExecNode`, `datafusion_proto::protobuf::ProjectionExecNode`, `datafusion_proto_models::protobuf::ProjectionExecNode`

```rust
struct ProjectionExecNode
```

**Fields**: `input`, `expr`, `expr_name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ProjectionExecNode.md).


---

## ProjectionExpr

`struct` · `datafusion_proto_models::generated::datafusion::ProjectionExpr`

Also reachable as `datafusion_proto::generated::datafusion::ProjectionExpr`, `datafusion_proto::protobuf::ProjectionExpr`, `datafusion_proto_models::protobuf::ProjectionExpr`

```rust
struct ProjectionExpr
```

**Fields**: `alias`, `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ProjectionExpr.md).


---

## ProjectionExprs

`struct` · `datafusion_proto_models::generated::datafusion::ProjectionExprs`

Also reachable as `datafusion_proto::generated::datafusion::ProjectionExprs`, `datafusion_proto::protobuf::ProjectionExprs`, `datafusion_proto_models::protobuf::ProjectionExprs`

```rust
struct ProjectionExprs
```

**Fields**: `projections`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ProjectionExprs.md).


---

## ProjectionNode

`struct` · `datafusion_proto_models::generated::datafusion::ProjectionNode`

Also reachable as `datafusion_proto::generated::datafusion::ProjectionNode`, `datafusion_proto::protobuf::ProjectionNode`, `datafusion_proto_models::protobuf::ProjectionNode`

```rust
struct ProjectionNode
```

**Fields**: `input`, `expr`, `optional_alias`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ProjectionNode.md).


---

## RangeRepartition

`struct` · `datafusion_proto_models::generated::datafusion::RangeRepartition`

Also reachable as `datafusion_proto::generated::datafusion::RangeRepartition`, `datafusion_proto::protobuf::RangeRepartition`, `datafusion_proto_models::protobuf::RangeRepartition`

```rust
struct RangeRepartition
```

**Fields**: `sort_expr`, `split_point`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RangeRepartition.md).


---

## RangeSplitPoint

`struct` · `datafusion_proto_models::generated::datafusion::RangeSplitPoint`

Also reachable as `datafusion_proto::generated::datafusion::RangeSplitPoint`, `datafusion_proto::protobuf::RangeSplitPoint`, `datafusion_proto_models::protobuf::RangeSplitPoint`

```rust
struct RangeSplitPoint
```

**Fields**: `value`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RangeSplitPoint.md).


---

## RecursionUnnestOption

`struct` · `datafusion_proto_models::generated::datafusion::RecursionUnnestOption`

Also reachable as `datafusion_proto::generated::datafusion::RecursionUnnestOption`, `datafusion_proto::protobuf::RecursionUnnestOption`, `datafusion_proto_models::protobuf::RecursionUnnestOption`

```rust
struct RecursionUnnestOption
```

**Fields**: `output_column`, `input_column`, `depth`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RecursionUnnestOption.md).


---

## RecursiveQueryNode

`struct` · `datafusion_proto_models::generated::datafusion::RecursiveQueryNode`

Also reachable as `datafusion_proto::generated::datafusion::RecursiveQueryNode`, `datafusion_proto::protobuf::RecursiveQueryNode`, `datafusion_proto_models::protobuf::RecursiveQueryNode`

```rust
struct RecursiveQueryNode
```

**Fields**: `name`, `static_term`, `recursive_term`, `is_distinct`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RecursiveQueryNode.md).


---

## RepartitionExecNode

`struct` · `datafusion_proto_models::generated::datafusion::RepartitionExecNode`

Also reachable as `datafusion_proto::generated::datafusion::RepartitionExecNode`, `datafusion_proto::protobuf::RepartitionExecNode`, `datafusion_proto_models::protobuf::RepartitionExecNode`

```rust
struct RepartitionExecNode
```

**Fields**: `input`, `partitioning`, `preserve_order`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RepartitionExecNode.md).


---

## RepartitionNode

`struct` · `datafusion_proto_models::generated::datafusion::RepartitionNode`

Also reachable as `datafusion_proto::generated::datafusion::RepartitionNode`, `datafusion_proto::protobuf::RepartitionNode`, `datafusion_proto_models::protobuf::RepartitionNode`

```rust
struct RepartitionNode
```

**Fields**: `input`, `partition_method`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RepartitionNode.md).


---

## RollupNode

`struct` · `datafusion_proto_models::generated::datafusion::RollupNode`

Also reachable as `datafusion_proto::generated::datafusion::RollupNode`, `datafusion_proto::protobuf::RollupNode`, `datafusion_proto_models::protobuf::RollupNode`

```rust
struct RollupNode
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RollupNode.md).


---

## RunEndEncoded

`struct` · `datafusion_proto_models::generated::datafusion::RunEndEncoded`

Also reachable as `datafusion_proto::generated::datafusion::RunEndEncoded`, `datafusion_proto::protobuf::RunEndEncoded`, `datafusion_proto_models::protobuf::RunEndEncoded`

```rust
struct RunEndEncoded
```

**Fields**: `run_ends_field`, `values_field`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.RunEndEncoded.md).


---

## ScalarDictionaryValue

`struct` · `datafusion_proto_models::generated::datafusion::ScalarDictionaryValue`

Also reachable as `datafusion_proto::generated::datafusion::ScalarDictionaryValue`, `datafusion_proto::protobuf::ScalarDictionaryValue`, `datafusion_proto_models::protobuf::ScalarDictionaryValue`

```rust
struct ScalarDictionaryValue
```

**Fields**: `index_type`, `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarDictionaryValue.md).


---

## ScalarFixedSizeBinary

`struct` · `datafusion_proto_models::generated::datafusion::ScalarFixedSizeBinary`

Also reachable as `datafusion_proto::generated::datafusion::ScalarFixedSizeBinary`, `datafusion_proto::protobuf::ScalarFixedSizeBinary`, `datafusion_proto_models::protobuf::ScalarFixedSizeBinary`

```rust
struct ScalarFixedSizeBinary
```

**Fields**: `values`, `length`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarFixedSizeBinary.md).


---

## ScalarNestedValue

`struct` · `datafusion_proto_models::generated::datafusion::ScalarNestedValue`

Also reachable as `datafusion_proto::generated::datafusion::ScalarNestedValue`, `datafusion_proto::protobuf::ScalarNestedValue`, `datafusion_proto_models::protobuf::ScalarNestedValue`

```rust
struct ScalarNestedValue
```

**Fields**: `ipc_message`, `arrow_data`, `schema`, `dictionaries`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarNestedValue.md).


Used for List/FixedSizeList/LargeList/ListView/LargeListView/Struct/Map

---

## ScalarRunEndEncodedValue

`struct` · `datafusion_proto_models::generated::datafusion::ScalarRunEndEncodedValue`

Also reachable as `datafusion_proto::generated::datafusion::ScalarRunEndEncodedValue`, `datafusion_proto::protobuf::ScalarRunEndEncodedValue`, `datafusion_proto_models::protobuf::ScalarRunEndEncodedValue`

```rust
struct ScalarRunEndEncodedValue
```

**Fields**: `run_ends_field`, `values_field`, `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarRunEndEncodedValue.md).


---

## ScalarSubqueryExecNode

`struct` · `datafusion_proto_models::generated::datafusion::ScalarSubqueryExecNode`

Also reachable as `datafusion_proto::generated::datafusion::ScalarSubqueryExecNode`, `datafusion_proto::protobuf::ScalarSubqueryExecNode`, `datafusion_proto_models::protobuf::ScalarSubqueryExecNode`

```rust
struct ScalarSubqueryExecNode
```

**Fields**: `input`, `subqueries`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarSubqueryExecNode.md).


---

## ScalarSubqueryExprNode

`struct` · `datafusion_proto_models::generated::datafusion::ScalarSubqueryExprNode`

Also reachable as `datafusion_proto::generated::datafusion::ScalarSubqueryExprNode`, `datafusion_proto::protobuf::ScalarSubqueryExprNode`, `datafusion_proto_models::protobuf::ScalarSubqueryExprNode`

```rust
struct ScalarSubqueryExprNode
```

**Fields**: `subquery`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarSubqueryExprNode.md).


---

## ScalarTime32Value

`struct` · `datafusion_proto_models::generated::datafusion::ScalarTime32Value`

Also reachable as `datafusion_proto::generated::datafusion::ScalarTime32Value`, `datafusion_proto::protobuf::ScalarTime32Value`, `datafusion_proto_models::protobuf::ScalarTime32Value`

```rust
struct ScalarTime32Value
```

**Fields**: `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarTime32Value.md).


---

## ScalarTime64Value

`struct` · `datafusion_proto_models::generated::datafusion::ScalarTime64Value`

Also reachable as `datafusion_proto::generated::datafusion::ScalarTime64Value`, `datafusion_proto::protobuf::ScalarTime64Value`, `datafusion_proto_models::protobuf::ScalarTime64Value`

```rust
struct ScalarTime64Value
```

**Fields**: `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarTime64Value.md).


---

## ScalarTimestampValue

`struct` · `datafusion_proto_models::generated::datafusion::ScalarTimestampValue`

Also reachable as `datafusion_proto::generated::datafusion::ScalarTimestampValue`, `datafusion_proto::protobuf::ScalarTimestampValue`, `datafusion_proto_models::protobuf::ScalarTimestampValue`

```rust
struct ScalarTimestampValue
```

**Fields**: `timezone`, `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarTimestampValue.md).


---

## ScalarUdfExprNode

`struct` · `datafusion_proto_models::generated::datafusion::ScalarUdfExprNode`

Also reachable as `datafusion_proto::generated::datafusion::ScalarUdfExprNode`, `datafusion_proto::protobuf::ScalarUdfExprNode`, `datafusion_proto_models::protobuf::ScalarUdfExprNode`

```rust
struct ScalarUdfExprNode
```

**Fields**: `fun_name`, `args`, `fun_definition`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn fun_definition(&self) -> &[u8]
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarUdfExprNode.md).


---

## ScalarValue

`struct` · `datafusion_proto_models::generated::datafusion::ScalarValue`

Also reachable as `datafusion_proto::generated::datafusion::ScalarValue`, `datafusion_proto::protobuf::ScalarValue`, `datafusion_proto_models::protobuf::ScalarValue`

```rust
struct ScalarValue
```

**Fields**: `value`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScalarValue.md).


---

## ScanLimit

`struct` · `datafusion_proto_models::generated::datafusion::ScanLimit`

Also reachable as `datafusion_proto::generated::datafusion::ScanLimit`, `datafusion_proto::protobuf::ScanLimit`, `datafusion_proto_models::protobuf::ScanLimit`

```rust
struct ScanLimit
```

**Fields**: `limit`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ScanLimit.md).


---

## Schema

`struct` · `datafusion_proto_models::generated::datafusion::Schema`

Also reachable as `datafusion_proto::generated::datafusion::Schema`, `datafusion_proto::protobuf::Schema`, `datafusion_proto_models::protobuf::Schema`

```rust
struct Schema
```

**Fields**: `columns`, `metadata`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Schema.md).


---

## SelectionExecNode

`struct` · `datafusion_proto_models::generated::datafusion::SelectionExecNode`

Also reachable as `datafusion_proto::generated::datafusion::SelectionExecNode`, `datafusion_proto::protobuf::SelectionExecNode`, `datafusion_proto_models::protobuf::SelectionExecNode`

```rust
struct SelectionExecNode
```

**Fields**: `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SelectionExecNode.md).


---

## SelectionNode

`struct` · `datafusion_proto_models::generated::datafusion::SelectionNode`

Also reachable as `datafusion_proto::generated::datafusion::SelectionNode`, `datafusion_proto::protobuf::SelectionNode`, `datafusion_proto_models::protobuf::SelectionNode`

```rust
struct SelectionNode
```

**Fields**: `input`, `expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SelectionNode.md).


---

## SimilarToNode

`struct` · `datafusion_proto_models::generated::datafusion::SimilarToNode`

Also reachable as `datafusion_proto::generated::datafusion::SimilarToNode`, `datafusion_proto::protobuf::SimilarToNode`, `datafusion_proto_models::protobuf::SimilarToNode`

```rust
struct SimilarToNode
```

**Fields**: `negated`, `expr`, `pattern`, `escape_char`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SimilarToNode.md).


---

## SortExecNode

`struct` · `datafusion_proto_models::generated::datafusion::SortExecNode`

Also reachable as `datafusion_proto::generated::datafusion::SortExecNode`, `datafusion_proto::protobuf::SortExecNode`, `datafusion_proto_models::protobuf::SortExecNode`

```rust
struct SortExecNode
```

**Fields**: `input`, `expr`, `fetch`, `preserve_partitioning`, `dynamic_filter`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SortExecNode.md).


---

## SortExprNode

`struct` · `datafusion_proto_models::generated::datafusion::SortExprNode`

Also reachable as `datafusion_proto::generated::datafusion::SortExprNode`, `datafusion_proto::protobuf::SortExprNode`, `datafusion_proto_models::protobuf::SortExprNode`

```rust
struct SortExprNode
```

**Fields**: `expr`, `asc`, `nulls_first`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SortExprNode.md).


---

## SortExprNodeCollection

`struct` · `datafusion_proto_models::generated::datafusion::SortExprNodeCollection`

Also reachable as `datafusion_proto::generated::datafusion::SortExprNodeCollection`, `datafusion_proto::protobuf::SortExprNodeCollection`, `datafusion_proto_models::protobuf::SortExprNodeCollection`

```rust
struct SortExprNodeCollection
```

**Fields**: `sort_expr_nodes`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SortExprNodeCollection.md).


---

## SortMergeJoinExecNode

`struct` · `datafusion_proto_models::generated::datafusion::SortMergeJoinExecNode`

Also reachable as `datafusion_proto::generated::datafusion::SortMergeJoinExecNode`, `datafusion_proto::protobuf::SortMergeJoinExecNode`, `datafusion_proto_models::protobuf::SortMergeJoinExecNode`

```rust
struct SortMergeJoinExecNode
```

**Fields**: `left`, `right`, `on`, `join_type`, `filter`, `sort_options`, `null_equality`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn join_type(&self) -> super::datafusion_common::JoinType
fn null_equality(&self) -> super::datafusion_common::NullEquality
fn set_join_type(&mut self, value: super::datafusion_common::JoinType)
fn set_null_equality(&mut self, value: super::datafusion_common::NullEquality)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SortMergeJoinExecNode.md).


---

## SortNode

`struct` · `datafusion_proto_models::generated::datafusion::SortNode`

Also reachable as `datafusion_proto::generated::datafusion::SortNode`, `datafusion_proto::protobuf::SortNode`, `datafusion_proto_models::protobuf::SortNode`

```rust
struct SortNode
```

**Fields**: `input`, `expr`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SortNode.md).


---

## SortPreservingMergeExecNode

`struct` · `datafusion_proto_models::generated::datafusion::SortPreservingMergeExecNode`

Also reachable as `datafusion_proto::generated::datafusion::SortPreservingMergeExecNode`, `datafusion_proto::protobuf::SortPreservingMergeExecNode`, `datafusion_proto_models::protobuf::SortPreservingMergeExecNode`

```rust
struct SortPreservingMergeExecNode
```

**Fields**: `input`, `expr`, `fetch`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SortPreservingMergeExecNode.md).


---

## Statistics

`struct` · `datafusion_proto_models::generated::datafusion::Statistics`

Also reachable as `datafusion_proto::generated::datafusion::Statistics`, `datafusion_proto::protobuf::Statistics`, `datafusion_proto_models::protobuf::Statistics`

```rust
struct Statistics
```

**Fields**: `num_rows`, `total_byte_size`, `column_stats`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Statistics.md).


---

## StringifiedPlan

`struct` · `datafusion_proto_models::generated::datafusion::StringifiedPlan`

Also reachable as `datafusion_proto::generated::datafusion::StringifiedPlan`, `datafusion_proto::protobuf::StringifiedPlan`, `datafusion_proto_models::protobuf::StringifiedPlan`

```rust
struct StringifiedPlan
```

**Fields**: `plan_type`, `plan`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(stringified_plan: &StringifiedPlan) -> Self
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.StringifiedPlan.md).


---

## Struct

`struct` · `datafusion_proto_models::generated::datafusion::Struct`

Also reachable as `datafusion_proto::generated::datafusion::Struct`, `datafusion_proto::protobuf::Struct`, `datafusion_proto_models::protobuf::Struct`

```rust
struct Struct
```

**Fields**: `sub_field_types`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Struct.md).


---

## SubqueryAliasNode

`struct` · `datafusion_proto_models::generated::datafusion::SubqueryAliasNode`

Also reachable as `datafusion_proto::generated::datafusion::SubqueryAliasNode`, `datafusion_proto::protobuf::SubqueryAliasNode`, `datafusion_proto_models::protobuf::SubqueryAliasNode`

```rust
struct SubqueryAliasNode
```

**Fields**: `input`, `alias`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SubqueryAliasNode.md).


---

## SubqueryNode

`struct` · `datafusion_proto_models::generated::datafusion::SubqueryNode`

Also reachable as `datafusion_proto::generated::datafusion::SubqueryNode`, `datafusion_proto::protobuf::SubqueryNode`, `datafusion_proto_models::protobuf::SubqueryNode`

```rust
struct SubqueryNode
```

**Fields**: `subquery`, `outer_ref_columns`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SubqueryNode.md).


---

## SymmetricHashJoinExecNode

`struct` · `datafusion_proto_models::generated::datafusion::SymmetricHashJoinExecNode`

Also reachable as `datafusion_proto::generated::datafusion::SymmetricHashJoinExecNode`, `datafusion_proto::protobuf::SymmetricHashJoinExecNode`, `datafusion_proto_models::protobuf::SymmetricHashJoinExecNode`

```rust
struct SymmetricHashJoinExecNode
```

**Fields**: `left`, `right`, `on`, `join_type`, `partition_mode`, `null_equality`, `filter`, `left_sort_exprs`, `right_sort_exprs`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn join_type(&self) -> super::datafusion_common::JoinType
fn null_equality(&self) -> super::datafusion_common::NullEquality
fn partition_mode(&self) -> StreamPartitionMode
fn set_join_type(&mut self, value: super::datafusion_common::JoinType)
fn set_null_equality(&mut self, value: super::datafusion_common::NullEquality)
fn set_partition_mode(&mut self, value: StreamPartitionMode)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.SymmetricHashJoinExecNode.md).


---

## TableParquetOptions

`struct` · `datafusion_proto_models::generated::datafusion::TableParquetOptions`

Also reachable as `datafusion_proto::generated::datafusion::TableParquetOptions`, `datafusion_proto::protobuf::TableParquetOptions`, `datafusion_proto_models::protobuf::TableParquetOptions`

```rust
struct TableParquetOptions
```

**Fields**: `global`, `column_specific_options`, `key_value_metadata`

**Implements**: `core::convert::From`, `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.TableParquetOptions.md).


---

## TableReference

`struct` · `datafusion_proto_models::generated::datafusion::TableReference`

Also reachable as `datafusion_proto::generated::datafusion::TableReference`, `datafusion_proto::protobuf::TableReference`, `datafusion_proto_models::protobuf::TableReference`

```rust
struct TableReference
```

**Fields**: `table_reference_enum`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(t: TableReference) -> Self
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.TableReference.md).


---

## Timestamp

`struct` · `datafusion_proto_models::generated::datafusion::Timestamp`

Also reachable as `datafusion_proto::generated::datafusion::Timestamp`, `datafusion_proto::protobuf::Timestamp`, `datafusion_proto_models::protobuf::Timestamp`

```rust
struct Timestamp
```

**Fields**: `time_unit`, `timezone`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn set_time_unit(&mut self, value: TimeUnit)
fn time_unit(&self) -> TimeUnit
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Timestamp.md).


---

## TryCastNode

`struct` · `datafusion_proto_models::generated::datafusion::TryCastNode`

Also reachable as `datafusion_proto::generated::datafusion::TryCastNode`, `datafusion_proto::protobuf::TryCastNode`, `datafusion_proto_models::protobuf::TryCastNode`

```rust
struct TryCastNode
```

**Fields**: `expr`, `arrow_type`, `metadata`, `nullable`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn nullable(&self) -> bool
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.TryCastNode.md).


---

## Union

`struct` · `datafusion_proto_models::generated::datafusion::Union`

Also reachable as `datafusion_proto::generated::datafusion::Union`, `datafusion_proto::protobuf::Union`, `datafusion_proto_models::protobuf::Union`

```rust
struct Union
```

**Fields**: `union_types`, `union_mode`, `type_ids`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn set_union_mode(&mut self, value: UnionMode)
fn union_mode(&self) -> UnionMode
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Union.md).


---

## UnionExecNode

`struct` · `datafusion_proto_models::generated::datafusion::UnionExecNode`

Also reachable as `datafusion_proto::generated::datafusion::UnionExecNode`, `datafusion_proto::protobuf::UnionExecNode`, `datafusion_proto_models::protobuf::UnionExecNode`

```rust
struct UnionExecNode
```

**Fields**: `inputs`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnionExecNode.md).


---

## UnionField

`struct` · `datafusion_proto_models::generated::datafusion::UnionField`

Also reachable as `datafusion_proto::generated::datafusion::UnionField`, `datafusion_proto::protobuf::UnionField`, `datafusion_proto_models::protobuf::UnionField`

```rust
struct UnionField
```

**Fields**: `field_id`, `field`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnionField.md).


---

## UnionNode

`struct` · `datafusion_proto_models::generated::datafusion::UnionNode`

Also reachable as `datafusion_proto::generated::datafusion::UnionNode`, `datafusion_proto::protobuf::UnionNode`, `datafusion_proto_models::protobuf::UnionNode`

```rust
struct UnionNode
```

**Fields**: `inputs`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnionNode.md).


---

## UnionValue

`struct` · `datafusion_proto_models::generated::datafusion::UnionValue`

Also reachable as `datafusion_proto::generated::datafusion::UnionValue`, `datafusion_proto::protobuf::UnionValue`, `datafusion_proto_models::protobuf::UnionValue`

```rust
struct UnionValue
```

**Fields**: `value_id`, `value`, `fields`, `mode`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn mode(&self) -> UnionMode
fn set_mode(&mut self, value: UnionMode)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnionValue.md).


---

## UniqueConstraint

`struct` · `datafusion_proto_models::generated::datafusion::UniqueConstraint`

Also reachable as `datafusion_proto::generated::datafusion::UniqueConstraint`, `datafusion_proto::protobuf::UniqueConstraint`, `datafusion_proto_models::protobuf::UniqueConstraint`

```rust
struct UniqueConstraint
```

**Fields**: `indices`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UniqueConstraint.md).


---

## UnknownColumn

`struct` · `datafusion_proto_models::generated::datafusion::UnknownColumn`

Also reachable as `datafusion_proto::generated::datafusion::UnknownColumn`, `datafusion_proto::protobuf::UnknownColumn`, `datafusion_proto_models::protobuf::UnknownColumn`

```rust
struct UnknownColumn
```

**Fields**: `name`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnknownColumn.md).


---

## Unnest

`struct` · `datafusion_proto_models::generated::datafusion::Unnest`

Also reachable as `datafusion_proto::generated::datafusion::Unnest`, `datafusion_proto::protobuf::Unnest`, `datafusion_proto_models::protobuf::Unnest`

```rust
struct Unnest
```

**Fields**: `exprs`, `outer`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Unnest.md).


---

## UnnestExecNode

`struct` · `datafusion_proto_models::generated::datafusion::UnnestExecNode`

Also reachable as `datafusion_proto::generated::datafusion::UnnestExecNode`, `datafusion_proto::protobuf::UnnestExecNode`, `datafusion_proto_models::protobuf::UnnestExecNode`

```rust
struct UnnestExecNode
```

**Fields**: `input`, `schema`, `list_type_columns`, `struct_type_columns`, `options`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnnestExecNode.md).


---

## UnnestNode

`struct` · `datafusion_proto_models::generated::datafusion::UnnestNode`

Also reachable as `datafusion_proto::generated::datafusion::UnnestNode`, `datafusion_proto::protobuf::UnnestNode`, `datafusion_proto_models::protobuf::UnnestNode`

```rust
struct UnnestNode
```

**Fields**: `input`, `exec_columns`, `list_type_columns`, `struct_type_columns`, `dependency_indices`, `schema`, `options`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnnestNode.md).


---

## UnnestOptions

`struct` · `datafusion_proto_models::generated::datafusion::UnnestOptions`

Also reachable as `datafusion_proto::generated::datafusion::UnnestOptions`, `datafusion_proto::protobuf::UnnestOptions`, `datafusion_proto_models::protobuf::UnnestOptions`

```rust
struct UnnestOptions
```

**Fields**: `null_handling`, `recursions`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn null_handling(&self) -> unnest_options::NullHandling
fn set_null_handling(&mut self, value: unnest_options::NullHandling)
```

**via `core::convert::From`**

```rust
fn from(opts: &UnnestOptions) -> Self
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.UnnestOptions.md).


---

## ValuesNode

`struct` · `datafusion_proto_models::generated::datafusion::ValuesNode`

Also reachable as `datafusion_proto::generated::datafusion::ValuesNode`, `datafusion_proto::protobuf::ValuesNode`, `datafusion_proto_models::protobuf::ValuesNode`

```rust
struct ValuesNode
```

**Fields**: `n_cols`, `values_list`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ValuesNode.md).


a node containing data for defining values list. unlike in SQL where it's two dimensional, here
the list is flattened, and with the field n_cols it can be parsed and partitioned into rows

---

## ViewTableScanNode

`struct` · `datafusion_proto_models::generated::datafusion::ViewTableScanNode`

Also reachable as `datafusion_proto::generated::datafusion::ViewTableScanNode`, `datafusion_proto::protobuf::ViewTableScanNode`, `datafusion_proto_models::protobuf::ViewTableScanNode`

```rust
struct ViewTableScanNode
```

**Fields**: `table_name`, `input`, `schema`, `projection`, `definition`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.ViewTableScanNode.md).


---

## WhenThen

`struct` · `datafusion_proto_models::generated::datafusion::WhenThen`

Also reachable as `datafusion_proto::generated::datafusion::WhenThen`, `datafusion_proto::protobuf::WhenThen`, `datafusion_proto_models::protobuf::WhenThen`

```rust
struct WhenThen
```

**Fields**: `when_expr`, `then_expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WhenThen.md).


---

## Wildcard

`struct` · `datafusion_proto_models::generated::datafusion::Wildcard`

Also reachable as `datafusion_proto::generated::datafusion::Wildcard`, `datafusion_proto::protobuf::Wildcard`, `datafusion_proto_models::protobuf::Wildcard`

```rust
struct Wildcard
```

**Fields**: `qualifier`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.Wildcard.md).


---

## WindowAggExecNode

`struct` · `datafusion_proto_models::generated::datafusion::WindowAggExecNode`

Also reachable as `datafusion_proto::generated::datafusion::WindowAggExecNode`, `datafusion_proto::protobuf::WindowAggExecNode`, `datafusion_proto_models::protobuf::WindowAggExecNode`

```rust
struct WindowAggExecNode
```

**Fields**: `input`, `window_expr`, `partition_keys`, `input_order_mode`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WindowAggExecNode.md).


---

## WindowExprNode

`struct` · `datafusion_proto_models::generated::datafusion::WindowExprNode`

Also reachable as `datafusion_proto::generated::datafusion::WindowExprNode`, `datafusion_proto::protobuf::WindowExprNode`, `datafusion_proto_models::protobuf::WindowExprNode`

```rust
struct WindowExprNode
```

**Fields**: `exprs`, `partition_by`, `order_by`, `window_frame`, `fun_definition`, `null_treatment`, `distinct`, `filter`, `window_function`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn fun_definition(&self) -> &[u8]
fn null_treatment(&self) -> NullTreatment
fn set_null_treatment(&mut self, value: NullTreatment)
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WindowExprNode.md).


---

## WindowFrame

`struct` · `datafusion_proto_models::generated::datafusion::WindowFrame`

Also reachable as `datafusion_proto::generated::datafusion::WindowFrame`, `datafusion_proto::protobuf::WindowFrame`, `datafusion_proto_models::protobuf::WindowFrame`

```rust
struct WindowFrame
```

**Fields**: `window_frame_units`, `start_bound`, `end_bound`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn set_window_frame_units(&mut self, value: WindowFrameUnits)
fn window_frame_units(&self) -> WindowFrameUnits
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WindowFrame.md).


---

## WindowFrameBound

`struct` · `datafusion_proto_models::generated::datafusion::WindowFrameBound`

Also reachable as `datafusion_proto::generated::datafusion::WindowFrameBound`, `datafusion_proto::protobuf::WindowFrameBound`, `datafusion_proto_models::protobuf::WindowFrameBound`

```rust
struct WindowFrameBound
```

**Fields**: `window_frame_bound_type`, `bound_value`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn set_window_frame_bound_type(&mut self, value: WindowFrameBoundType)
fn window_frame_bound_type(&self) -> WindowFrameBoundType
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WindowFrameBound.md).


---

## WindowNode

`struct` · `datafusion_proto_models::generated::datafusion::WindowNode`

Also reachable as `datafusion_proto::generated::datafusion::WindowNode`, `datafusion_proto::protobuf::WindowNode`, `datafusion_proto_models::protobuf::WindowNode`

```rust
struct WindowNode
```

**Fields**: `input`, `window_expr`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_models.generated.datafusion.WindowNode.md).


---
