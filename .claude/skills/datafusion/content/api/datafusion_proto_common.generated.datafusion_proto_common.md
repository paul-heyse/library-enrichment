# `datafusion_proto_common::generated::datafusion_proto_common`

Crate `datafusion-proto-common` · 73 public items · structured records in [`model/datafusion_proto_common.generated.datafusion_proto_common.json`](../model/datafusion_proto_common.generated.datafusion_proto_common.json)

## CompressionTypeVariant

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::CompressionTypeVariant`

Also reachable as `datafusion_proto::generated::datafusion_common::CompressionTypeVariant`, `datafusion_proto_common::CompressionTypeVariant`, `datafusion_proto_common::protobuf_common::CompressionTypeVariant`, `datafusion_proto_models::datafusion_common::CompressionTypeVariant`, `datafusion_proto_models::generated::datafusion_common::CompressionTypeVariant`

```rust
enum CompressionTypeVariant
```

**Variants**: `Gzip`, `Bzip2`, `Xz`, `Zstd`, `Uncompressed`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<CompressionTypeVariant>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: CompressionTypeVariant) -> Self
fn from(value: &CompressionTypeVariant) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<CompressionTypeVariant, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## CsvQuoteStyle

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::CsvQuoteStyle`

Also reachable as `datafusion_proto::generated::datafusion_common::CsvQuoteStyle`, `datafusion_proto_common::CsvQuoteStyle`, `datafusion_proto_common::protobuf_common::CsvQuoteStyle`, `datafusion_proto_models::datafusion_common::CsvQuoteStyle`, `datafusion_proto_models::generated::datafusion_common::CsvQuoteStyle`

```rust
enum CsvQuoteStyle
```

**Variants**: `Necessary`, `Always`, `NonNumeric`, `Never`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<CsvQuoteStyle>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: CsvQuoteStyle) -> Self
fn from(value: QuoteStyle) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<CsvQuoteStyle, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## ExplainFormat

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainFormat`

Also reachable as `datafusion_proto::generated::datafusion_common::ExplainFormat`, `datafusion_proto_common::ExplainFormat`, `datafusion_proto_common::protobuf_common::ExplainFormat`, `datafusion_proto_models::datafusion_common::ExplainFormat`, `datafusion_proto_models::generated::datafusion_common::ExplainFormat`

```rust
enum ExplainFormat
```

**Variants**: `Indent`, `Tree`, `Pgjson`, `Graphviz`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## IntervalUnit

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::IntervalUnit`

Also reachable as `datafusion_proto::generated::datafusion_common::IntervalUnit`, `datafusion_proto_common::IntervalUnit`, `datafusion_proto_common::protobuf_common::IntervalUnit`, `datafusion_proto_models::datafusion_common::IntervalUnit`, `datafusion_proto_models::generated::datafusion_common::IntervalUnit`

```rust
enum IntervalUnit
```

**Variants**: `YearMonth`, `DayTime`, `MonthDayNano`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<IntervalUnit>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(interval_unit: &IntervalUnit) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<IntervalUnit, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## JoinConstraint

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::JoinConstraint`

Also reachable as `datafusion_proto::generated::datafusion_common::JoinConstraint`, `datafusion_proto_common::JoinConstraint`, `datafusion_proto_common::protobuf_common::JoinConstraint`, `datafusion_proto_models::datafusion_common::JoinConstraint`, `datafusion_proto_models::generated::datafusion_common::JoinConstraint`

```rust
enum JoinConstraint
```

**Variants**: `On`, `Using`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<JoinConstraint>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<JoinConstraint, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## JoinSide

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::JoinSide`

Also reachable as `datafusion_proto::generated::datafusion_common::JoinSide`, `datafusion_proto::protobuf::JoinSide`, `datafusion_proto_common::JoinSide`, `datafusion_proto_common::protobuf_common::JoinSide`, `datafusion_proto_models::datafusion_common::JoinSide`, `datafusion_proto_models::generated::datafusion_common::JoinSide`

```rust
enum JoinSide
```

**Variants**: `LeftSide`, `RightSide`, `None`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<JoinSide>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(t: JoinSide) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<JoinSide, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## JoinType

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::JoinType`

Also reachable as `datafusion_proto::generated::datafusion_common::JoinType`, `datafusion_proto_common::JoinType`, `datafusion_proto_common::protobuf_common::JoinType`, `datafusion_proto_models::datafusion_common::JoinType`, `datafusion_proto_models::generated::datafusion_common::JoinType`

```rust
enum JoinType
```

**Variants**: `Inner`, `Left`, `Right`, `Full`, `Leftsemi`, `Leftanti`, `Rightsemi`, `Rightanti`, `Leftmark`, `Rightmark`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<JoinType>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<JoinType, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## MetricCategory

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::MetricCategory`

Also reachable as `datafusion_proto::generated::datafusion_common::MetricCategory`, `datafusion_proto_common::MetricCategory`, `datafusion_proto_common::protobuf_common::MetricCategory`, `datafusion_proto_models::datafusion_common::MetricCategory`, `datafusion_proto_models::generated::datafusion_common::MetricCategory`

```rust
enum MetricCategory
```

**Variants**: `Rows`, `Bytes`, `Timing`, `Uncategorized`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Category of an `EXPLAIN ANALYZE` metric. Mirrors
`datafusion_common::format::MetricCategory`.

---

## MetricType

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::MetricType`

Also reachable as `datafusion_proto::generated::datafusion_common::MetricType`, `datafusion_proto_common::MetricType`, `datafusion_proto_common::protobuf_common::MetricType`, `datafusion_proto_models::datafusion_common::MetricType`, `datafusion_proto_models::generated::datafusion_common::MetricType`

```rust
enum MetricType
```

**Variants**: `Summary`, `Dev`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Verbosity level for `EXPLAIN ANALYZE`. Mirrors
`datafusion_common::format::MetricType`.

---

## NullEquality

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::NullEquality`

Also reachable as `datafusion_proto::generated::datafusion_common::NullEquality`, `datafusion_proto_common::NullEquality`, `datafusion_proto_common::protobuf_common::NullEquality`, `datafusion_proto_models::datafusion_common::NullEquality`, `datafusion_proto_models::generated::datafusion_common::NullEquality`

```rust
enum NullEquality
```

**Variants**: `NullEqualsNothing`, `NullEqualsNull`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<NullEquality>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<NullEquality, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## PrecisionInfo

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::PrecisionInfo`

Also reachable as `datafusion_proto::generated::datafusion_common::PrecisionInfo`, `datafusion_proto_common::PrecisionInfo`, `datafusion_proto_common::protobuf_common::PrecisionInfo`, `datafusion_proto_models::datafusion_common::PrecisionInfo`, `datafusion_proto_models::generated::datafusion_common::PrecisionInfo`

```rust
enum PrecisionInfo
```

**Variants**: `Exact`, `Inexact`, `Absent`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## TimeUnit

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::TimeUnit`

Also reachable as `datafusion_proto::generated::datafusion_common::TimeUnit`, `datafusion_proto_common::TimeUnit`, `datafusion_proto_common::protobuf_common::TimeUnit`, `datafusion_proto_models::datafusion_common::TimeUnit`, `datafusion_proto_models::generated::datafusion_common::TimeUnit`

```rust
enum TimeUnit
```

**Variants**: `Second`, `Millisecond`, `Microsecond`, `Nanosecond`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<TimeUnit>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
const fn is_valid(value: i32) -> bool
```

**via `core::convert::From`**

```rust
fn from(val: &TimeUnit) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<TimeUnit, ::prost::UnknownEnumValue>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## UnionMode

`enum` · `datafusion_proto_common::generated::datafusion_proto_common::UnionMode`

Also reachable as `datafusion_proto::generated::datafusion_common::UnionMode`, `datafusion_proto_common::UnionMode`, `datafusion_proto_common::protobuf_common::UnionMode`, `datafusion_proto_models::datafusion_common::UnionMode`, `datafusion_proto_models::generated::datafusion_common::UnionMode`

```rust
enum UnionMode
```

**Variants**: `Sparse`, `Dense`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## ArrowFormat

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowFormat`

Also reachable as `datafusion_proto::generated::datafusion_common::ArrowFormat`, `datafusion_proto::protobuf::ArrowFormat`, `datafusion_proto_common::ArrowFormat`, `datafusion_proto_common::protobuf_common::ArrowFormat`, `datafusion_proto_models::datafusion_common::ArrowFormat`, `datafusion_proto_models::generated::datafusion_common::ArrowFormat`

```rust
struct ArrowFormat
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

---

## ArrowOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::ArrowOptions`, `datafusion_proto::protobuf::ArrowOptions`, `datafusion_proto_common::ArrowOptions`, `datafusion_proto_common::protobuf_common::ArrowOptions`, `datafusion_proto_models::datafusion_common::ArrowOptions`, `datafusion_proto_models::generated::datafusion_common::ArrowOptions`

```rust
struct ArrowOptions
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

---

## ArrowType

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ArrowType`

Also reachable as `datafusion_proto::generated::datafusion_common::ArrowType`, `datafusion_proto::protobuf::ArrowType`, `datafusion_proto_common::ArrowType`, `datafusion_proto_common::protobuf_common::ArrowType`, `datafusion_proto_models::datafusion_common::ArrowType`, `datafusion_proto_models::generated::datafusion_common::ArrowType`

```rust
struct ArrowType
```

**Fields**: `arrow_type_enum`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(val: &DataType) -> Result<Self, Self::Error>
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

Serialized data type

---

## AvroFormat

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::AvroFormat`

Also reachable as `datafusion_proto::generated::datafusion_common::AvroFormat`, `datafusion_proto::protobuf::AvroFormat`, `datafusion_proto_common::AvroFormat`, `datafusion_proto_common::protobuf_common::AvroFormat`, `datafusion_proto_models::datafusion_common::AvroFormat`, `datafusion_proto_models::generated::datafusion_common::AvroFormat`

```rust
struct AvroFormat
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

---

## AvroOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::AvroOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::AvroOptions`, `datafusion_proto::protobuf::AvroOptions`, `datafusion_proto_common::AvroOptions`, `datafusion_proto_common::protobuf_common::AvroOptions`, `datafusion_proto_models::datafusion_common::AvroOptions`, `datafusion_proto_models::generated::datafusion_common::AvroOptions`

```rust
struct AvroOptions
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

---

## Column

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Column`

Also reachable as `datafusion_proto::generated::datafusion_common::Column`, `datafusion_proto_common::Column`, `datafusion_proto_common::protobuf_common::Column`, `datafusion_proto_models::datafusion_common::Column`, `datafusion_proto_models::generated::datafusion_common::Column`

```rust
struct Column
```

**Fields**: `name`, `relation`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(c: Column) -> Self
fn from(c: &Column) -> Self
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

---

## ColumnRelation

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ColumnRelation`

Also reachable as `datafusion_proto::generated::datafusion_common::ColumnRelation`, `datafusion_proto_common::ColumnRelation`, `datafusion_proto_common::protobuf_common::ColumnRelation`, `datafusion_proto_models::datafusion_common::ColumnRelation`, `datafusion_proto_models::generated::datafusion_common::ColumnRelation`

```rust
struct ColumnRelation
```

**Fields**: `relation`

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

---

## ColumnStats

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ColumnStats`

Also reachable as `datafusion_proto::generated::datafusion_common::ColumnStats`, `datafusion_proto_common::ColumnStats`, `datafusion_proto_common::protobuf_common::ColumnStats`, `datafusion_proto_models::datafusion_common::ColumnStats`, `datafusion_proto_models::generated::datafusion_common::ColumnStats`

```rust
struct ColumnStats
```

**Fields**: `min_value`, `max_value`, `sum_value`, `null_count`, `distinct_count`, `byte_size`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(s: &ColumnStatistics) -> protobuf::ColumnStats
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

---

## Constraint

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Constraint`

Also reachable as `datafusion_proto::generated::datafusion_common::Constraint`, `datafusion_proto_common::Constraint`, `datafusion_proto_common::protobuf_common::Constraint`, `datafusion_proto_models::datafusion_common::Constraint`, `datafusion_proto_models::generated::datafusion_common::Constraint`

```rust
struct Constraint
```

**Fields**: `constraint_mode`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: Constraint) -> Self
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

---

## Constraints

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Constraints`

Also reachable as `datafusion_proto::generated::datafusion_common::Constraints`, `datafusion_proto_common::Constraints`, `datafusion_proto_common::protobuf_common::Constraints`, `datafusion_proto_models::datafusion_common::Constraints`, `datafusion_proto_models::generated::datafusion_common::Constraints`

```rust
struct Constraints
```

**Fields**: `constraints`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: Constraints) -> Self
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

---

## CsvFormat

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::CsvFormat`

Also reachable as `datafusion_proto::generated::datafusion_common::CsvFormat`, `datafusion_proto::protobuf::CsvFormat`, `datafusion_proto_common::CsvFormat`, `datafusion_proto_common::protobuf_common::CsvFormat`, `datafusion_proto_models::datafusion_common::CsvFormat`, `datafusion_proto_models::generated::datafusion_common::CsvFormat`

```rust
struct CsvFormat
```

**Fields**: `options`

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

---

## CsvOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::CsvOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::CsvOptions`, `datafusion_proto_common::CsvOptions`, `datafusion_proto_common::protobuf_common::CsvOptions`, `datafusion_proto_models::datafusion_common::CsvOptions`, `datafusion_proto_models::generated::datafusion_common::CsvOptions`

```rust
struct CsvOptions
```

**Fields**: `has_header`, `delimiter`, `quote`, `escape`, `compression`, `schema_infer_max_rec`, `date_format`, `datetime_format`, `timestamp_format`, `timestamp_tz_format`, `time_format`, `null_value`, `null_regex`, `comment`, `double_quote`, `newlines_in_values`, `terminator`, `truncated_rows`, `compression_level`, `quote_style`, `ignore_leading_whitespace`, `ignore_trailing_whitespace`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `core::convert::TryFrom`**

```rust
fn try_from(opts: &CsvOptions) -> datafusion_common::Result<Self, Self::Error>
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

Options controlling CSV format

---

## CsvWriterOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::CsvWriterOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::CsvWriterOptions`, `datafusion_proto_common::CsvWriterOptions`, `datafusion_proto_common::protobuf_common::CsvWriterOptions`, `datafusion_proto_models::datafusion_common::CsvWriterOptions`, `datafusion_proto_models::generated::datafusion_common::CsvWriterOptions`

```rust
struct CsvWriterOptions
```

**Fields**: `compression`, `delimiter`, `has_header`, `date_format`, `datetime_format`, `timestamp_format`, `time_format`, `null_value`, `quote`, `escape`, `double_quote`, `quote_style`, `ignore_leading_whitespace`, `ignore_trailing_whitespace`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn compression(&self) -> CompressionTypeVariant
fn quote_style(&self) -> CsvQuoteStyle
fn set_compression(&mut self, value: CompressionTypeVariant)
fn set_quote_style(&mut self, value: CsvQuoteStyle)
```

**via `core::convert::TryFrom`**

```rust
fn try_from(opts: &CsvWriterOptions) -> datafusion_common::Result<Self, Self::Error>
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

---

## Decimal128

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal128`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal128`, `datafusion_proto_common::Decimal128`, `datafusion_proto_common::protobuf_common::Decimal128`, `datafusion_proto_models::datafusion_common::Decimal128`, `datafusion_proto_models::generated::datafusion_common::Decimal128`

```rust
struct Decimal128
```

**Fields**: `value`, `p`, `s`

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

---

## Decimal128Type

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal128Type`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal128Type`, `datafusion_proto_common::Decimal128Type`, `datafusion_proto_common::protobuf_common::Decimal128Type`, `datafusion_proto_models::datafusion_common::Decimal128Type`, `datafusion_proto_models::generated::datafusion_common::Decimal128Type`

```rust
struct Decimal128Type
```

**Fields**: `precision`, `scale`

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

---

## Decimal256

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal256`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal256`, `datafusion_proto_common::Decimal256`, `datafusion_proto_common::protobuf_common::Decimal256`, `datafusion_proto_models::datafusion_common::Decimal256`, `datafusion_proto_models::generated::datafusion_common::Decimal256`

```rust
struct Decimal256
```

**Fields**: `value`, `p`, `s`

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

---

## Decimal256Type

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal256Type`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal256Type`, `datafusion_proto_common::Decimal256Type`, `datafusion_proto_common::protobuf_common::Decimal256Type`, `datafusion_proto_models::datafusion_common::Decimal256Type`, `datafusion_proto_models::generated::datafusion_common::Decimal256Type`

```rust
struct Decimal256Type
```

**Fields**: `precision`, `scale`

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

---

## Decimal32

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal32`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal32`, `datafusion_proto_common::Decimal32`, `datafusion_proto_common::protobuf_common::Decimal32`, `datafusion_proto_models::datafusion_common::Decimal32`, `datafusion_proto_models::generated::datafusion_common::Decimal32`

```rust
struct Decimal32
```

**Fields**: `value`, `p`, `s`

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

---

## Decimal32Type

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal32Type`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal32Type`, `datafusion_proto_common::Decimal32Type`, `datafusion_proto_common::protobuf_common::Decimal32Type`, `datafusion_proto_models::datafusion_common::Decimal32Type`, `datafusion_proto_models::generated::datafusion_common::Decimal32Type`

```rust
struct Decimal32Type
```

**Fields**: `precision`, `scale`

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

---

## Decimal64

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal64`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal64`, `datafusion_proto_common::Decimal64`, `datafusion_proto_common::protobuf_common::Decimal64`, `datafusion_proto_models::datafusion_common::Decimal64`, `datafusion_proto_models::generated::datafusion_common::Decimal64`

```rust
struct Decimal64
```

**Fields**: `value`, `p`, `s`

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

---

## Decimal64Type

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Decimal64Type`

Also reachable as `datafusion_proto::generated::datafusion_common::Decimal64Type`, `datafusion_proto_common::Decimal64Type`, `datafusion_proto_common::protobuf_common::Decimal64Type`, `datafusion_proto_models::datafusion_common::Decimal64Type`, `datafusion_proto_models::generated::datafusion_common::Decimal64Type`

```rust
struct Decimal64Type
```

**Fields**: `precision`, `scale`

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

---

## DfField

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::DfField`

Also reachable as `datafusion_proto::generated::datafusion_common::DfField`, `datafusion_proto_common::DfField`, `datafusion_proto_common::protobuf_common::DfField`, `datafusion_proto_models::datafusion_common::DfField`, `datafusion_proto_models::generated::datafusion_common::DfField`

```rust
struct DfField
```

**Fields**: `field`, `qualifier`

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

---

## DfSchema

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::DfSchema`

Also reachable as `datafusion_proto::generated::datafusion_common::DfSchema`, `datafusion_proto::protobuf::DfSchema`, `datafusion_proto_common::DfSchema`, `datafusion_proto_common::protobuf_common::DfSchema`, `datafusion_proto_models::datafusion_common::DfSchema`, `datafusion_proto_models::generated::datafusion_common::DfSchema`

```rust
struct DfSchema
```

**Fields**: `columns`, `metadata`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(s: &DFSchema) -> Result<Self, Self::Error>
fn try_from(s: &DFSchemaRef) -> Result<Self, Self::Error>
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

---

## Dictionary

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Dictionary`

Also reachable as `datafusion_proto::generated::datafusion_common::Dictionary`, `datafusion_proto_common::Dictionary`, `datafusion_proto_common::protobuf_common::Dictionary`, `datafusion_proto_models::datafusion_common::Dictionary`, `datafusion_proto_models::generated::datafusion_common::Dictionary`

```rust
struct Dictionary
```

**Fields**: `key`, `value`

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

---

## EmptyMessage

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::EmptyMessage`

Also reachable as `datafusion_proto::generated::datafusion_common::EmptyMessage`, `datafusion_proto::protobuf::EmptyMessage`, `datafusion_proto_common::EmptyMessage`, `datafusion_proto_common::protobuf_common::EmptyMessage`, `datafusion_proto_models::datafusion_common::EmptyMessage`, `datafusion_proto_models::generated::datafusion_common::EmptyMessage`

```rust
struct EmptyMessage
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

## ExplainAnalyzeCategoriesNode

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ExplainAnalyzeCategoriesNode`

Also reachable as `datafusion_proto::generated::datafusion_common::ExplainAnalyzeCategoriesNode`, `datafusion_proto_common::ExplainAnalyzeCategoriesNode`, `datafusion_proto_common::protobuf_common::ExplainAnalyzeCategoriesNode`, `datafusion_proto_models::datafusion_common::ExplainAnalyzeCategoriesNode`, `datafusion_proto_models::generated::datafusion_common::ExplainAnalyzeCategoriesNode`

```rust
struct ExplainAnalyzeCategoriesNode
```

**Fields**: `all`, `only`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Wire encoding for `datafusion_common::format::ExplainAnalyzeCategories`.

If `all` is true, every category is shown (the `only` list is ignored).
If `all` is false, only the categories listed in `only` are shown — an
empty `only` means "plan only", i.e. suppress all metrics.

---

## Field

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Field`

Also reachable as `datafusion_proto::generated::datafusion_common::Field`, `datafusion_proto::protobuf::Field`, `datafusion_proto_common::Field`, `datafusion_proto_common::protobuf_common::Field`, `datafusion_proto_models::datafusion_common::Field`, `datafusion_proto_models::generated::datafusion_common::Field`

```rust
struct Field
```

**Fields**: `name`, `arrow_type`, `nullable`, `children`, `metadata`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(field: &Field) -> Result<Self, Self::Error>
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

---

## FixedSizeList

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::FixedSizeList`

Also reachable as `datafusion_proto::generated::datafusion_common::FixedSizeList`, `datafusion_proto_common::FixedSizeList`, `datafusion_proto_common::protobuf_common::FixedSizeList`, `datafusion_proto_models::datafusion_common::FixedSizeList`, `datafusion_proto_models::generated::datafusion_common::FixedSizeList`

```rust
struct FixedSizeList
```

**Fields**: `field_type`, `list_size`

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

---

## IntervalDayTimeValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::IntervalDayTimeValue`

Also reachable as `datafusion_proto::generated::datafusion_common::IntervalDayTimeValue`, `datafusion_proto_common::IntervalDayTimeValue`, `datafusion_proto_common::protobuf_common::IntervalDayTimeValue`, `datafusion_proto_models::datafusion_common::IntervalDayTimeValue`, `datafusion_proto_models::generated::datafusion_common::IntervalDayTimeValue`

```rust
struct IntervalDayTimeValue
```

**Fields**: `days`, `milliseconds`

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

---

## IntervalMonthDayNanoValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::IntervalMonthDayNanoValue`

Also reachable as `datafusion_proto::generated::datafusion_common::IntervalMonthDayNanoValue`, `datafusion_proto_common::IntervalMonthDayNanoValue`, `datafusion_proto_common::protobuf_common::IntervalMonthDayNanoValue`, `datafusion_proto_models::datafusion_common::IntervalMonthDayNanoValue`, `datafusion_proto_models::generated::datafusion_common::IntervalMonthDayNanoValue`

```rust
struct IntervalMonthDayNanoValue
```

**Fields**: `months`, `days`, `nanos`

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

---

## JsonOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::JsonOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::JsonOptions`, `datafusion_proto_common::JsonOptions`, `datafusion_proto_common::protobuf_common::JsonOptions`, `datafusion_proto_models::datafusion_common::JsonOptions`, `datafusion_proto_models::generated::datafusion_common::JsonOptions`

```rust
struct JsonOptions
```

**Fields**: `compression`, `schema_infer_max_rec`, `compression_level`, `newline_delimited`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn compression(&self) -> CompressionTypeVariant
fn compression_level(&self) -> u32
fn newline_delimited(&self) -> bool
fn schema_infer_max_rec(&self) -> u64
fn set_compression(&mut self, value: CompressionTypeVariant)
```

**via `core::convert::TryFrom`**

```rust
fn try_from(opts: &JsonOptions) -> datafusion_common::Result<Self, Self::Error>
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

Options controlling CSV format

---

## JsonWriterOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::JsonWriterOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::JsonWriterOptions`, `datafusion_proto_common::JsonWriterOptions`, `datafusion_proto_common::protobuf_common::JsonWriterOptions`, `datafusion_proto_models::datafusion_common::JsonWriterOptions`, `datafusion_proto_models::generated::datafusion_common::JsonWriterOptions`

```rust
struct JsonWriterOptions
```

**Fields**: `compression`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn compression(&self) -> CompressionTypeVariant
fn set_compression(&mut self, value: CompressionTypeVariant)
```

**via `core::convert::TryFrom`**

```rust
fn try_from(opts: &JsonWriterOptions) -> datafusion_common::Result<Self, Self::Error>
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

---

## List

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::List`

Also reachable as `datafusion_proto::generated::datafusion_common::List`, `datafusion_proto_common::List`, `datafusion_proto_common::protobuf_common::List`, `datafusion_proto_models::datafusion_common::List`, `datafusion_proto_models::generated::datafusion_common::List`

```rust
struct List
```

**Fields**: `field_type`

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

---

## Map

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Map`

Also reachable as `datafusion_proto::generated::datafusion_common::Map`, `datafusion_proto_common::Map`, `datafusion_proto_common::protobuf_common::Map`, `datafusion_proto_models::datafusion_common::Map`, `datafusion_proto_models::generated::datafusion_common::Map`

```rust
struct Map
```

**Fields**: `field_type`, `keys_sorted`

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

---

## NdJsonFormat

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::NdJsonFormat`

Also reachable as `datafusion_proto::generated::datafusion_common::NdJsonFormat`, `datafusion_proto::protobuf::NdJsonFormat`, `datafusion_proto_common::NdJsonFormat`, `datafusion_proto_common::protobuf_common::NdJsonFormat`, `datafusion_proto_models::datafusion_common::NdJsonFormat`, `datafusion_proto_models::generated::datafusion_common::NdJsonFormat`

```rust
struct NdJsonFormat
```

**Fields**: `options`

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

---

## ParquetCdcOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetCdcOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::ParquetCdcOptions`, `datafusion_proto_common::ParquetCdcOptions`, `datafusion_proto_common::protobuf_common::ParquetCdcOptions`, `datafusion_proto_models::datafusion_common::ParquetCdcOptions`, `datafusion_proto_models::generated::datafusion_common::ParquetCdcOptions`

```rust
struct ParquetCdcOptions
```

**Fields**: `enabled`, `min_chunk_size`, `max_chunk_size`, `norm_level`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &ParquetCdcOptions) -> Self
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

Content-defined chunking (CDC) options for writing parquet files.

---

## ParquetColumnOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetColumnOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::ParquetColumnOptions`, `datafusion_proto_common::ParquetColumnOptions`, `datafusion_proto_common::protobuf_common::ParquetColumnOptions`, `datafusion_proto_models::datafusion_common::ParquetColumnOptions`, `datafusion_proto_models::generated::datafusion_common::ParquetColumnOptions`

```rust
struct ParquetColumnOptions
```

**Fields**: `bloom_filter_enabled_opt`, `encoding_opt`, `dictionary_enabled_opt`, `compression_opt`, `statistics_enabled_opt`, `bloom_filter_fpp_opt`, `bloom_filter_ndv_opt`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &ParquetColumnOptions) -> datafusion_common::Result<Self, Self::Error>
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

---

## ParquetColumnSpecificOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetColumnSpecificOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::ParquetColumnSpecificOptions`, `datafusion_proto_common::ParquetColumnSpecificOptions`, `datafusion_proto_common::protobuf_common::ParquetColumnSpecificOptions`, `datafusion_proto_models::datafusion_common::ParquetColumnSpecificOptions`, `datafusion_proto_models::generated::datafusion_common::ParquetColumnSpecificOptions`

```rust
struct ParquetColumnSpecificOptions
```

**Fields**: `column_name`, `options`

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

---

## ParquetFormat

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetFormat`

Also reachable as `datafusion_proto::generated::datafusion_common::ParquetFormat`, `datafusion_proto::protobuf::ParquetFormat`, `datafusion_proto_common::ParquetFormat`, `datafusion_proto_common::protobuf_common::ParquetFormat`, `datafusion_proto_models::datafusion_common::ParquetFormat`, `datafusion_proto_models::generated::datafusion_common::ParquetFormat`

```rust
struct ParquetFormat
```

**Fields**: `options`

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

---

## ParquetOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ParquetOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::ParquetOptions`, `datafusion_proto_common::ParquetOptions`, `datafusion_proto_common::protobuf_common::ParquetOptions`, `datafusion_proto_models::datafusion_common::ParquetOptions`, `datafusion_proto_models::generated::datafusion_common::ParquetOptions`

```rust
struct ParquetOptions
```

**Fields**: `enable_page_index`, `pruning`, `skip_metadata`, `pushdown_filters`, `reorder_filters`, `force_filter_selections`, `data_pagesize_limit`, `write_batch_size`, `writer_version`, `allow_single_file_parallelism`, `maximum_parallel_row_group_writers`, `maximum_buffered_record_batches_per_stream`, `bloom_filter_on_read`, `bloom_filter_on_write`, `schema_force_view_types`, `binary_as_string`, `skip_arrow_metadata`, `dictionary_page_size_limit`, `data_page_row_count_limit`, `max_row_group_size`, `max_in_list_size`, `created_by`, `content_defined_chunking`, `metadata_size_hint_opt`, `compression_opt`, `dictionary_enabled_opt`, `statistics_enabled_opt`, `column_index_truncate_length_opt`, `statistics_truncate_length_opt`, `encoding_opt`, `bloom_filter_fpp_opt`, `bloom_filter_ndv_opt`, `coerce_int96_opt`, `max_predicate_cache_size_opt`, `max_row_group_bytes_opt`, `coerce_int96_tz_opt`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &ParquetOptions) -> datafusion_common::Result<Self, Self::Error>
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

---

## Precision

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Precision`

Also reachable as `datafusion_proto::generated::datafusion_common::Precision`, `datafusion_proto_common::Precision`, `datafusion_proto_common::protobuf_common::Precision`, `datafusion_proto_models::datafusion_common::Precision`, `datafusion_proto_models::generated::datafusion_common::Precision`

```rust
struct Precision
```

**Fields**: `precision_info`, `val`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn precision_info(&self) -> PrecisionInfo
fn set_precision_info(&mut self, value: PrecisionInfo)
```

**via `core::convert::From`**

```rust
fn from(s: &Precision<usize>) -> protobuf::Precision
fn from(s: &Precision<ScalarValue>) -> protobuf::Precision
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

---

## PrimaryKeyConstraint

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::PrimaryKeyConstraint`

Also reachable as `datafusion_proto::generated::datafusion_common::PrimaryKeyConstraint`, `datafusion_proto_common::PrimaryKeyConstraint`, `datafusion_proto_common::protobuf_common::PrimaryKeyConstraint`, `datafusion_proto_models::datafusion_common::PrimaryKeyConstraint`, `datafusion_proto_models::generated::datafusion_common::PrimaryKeyConstraint`

```rust
struct PrimaryKeyConstraint
```

**Fields**: `indices`

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

---

## RunEndEncoded

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::RunEndEncoded`

Also reachable as `datafusion_proto::generated::datafusion_common::RunEndEncoded`, `datafusion_proto_common::RunEndEncoded`, `datafusion_proto_common::protobuf_common::RunEndEncoded`, `datafusion_proto_models::datafusion_common::RunEndEncoded`, `datafusion_proto_models::generated::datafusion_common::RunEndEncoded`

```rust
struct RunEndEncoded
```

**Fields**: `run_ends_field`, `values_field`

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

---

## ScalarDictionaryValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarDictionaryValue`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarDictionaryValue`, `datafusion_proto_common::ScalarDictionaryValue`, `datafusion_proto_common::protobuf_common::ScalarDictionaryValue`, `datafusion_proto_models::datafusion_common::ScalarDictionaryValue`, `datafusion_proto_models::generated::datafusion_common::ScalarDictionaryValue`

```rust
struct ScalarDictionaryValue
```

**Fields**: `index_type`, `value`

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

---

## ScalarFixedSizeBinary

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarFixedSizeBinary`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarFixedSizeBinary`, `datafusion_proto_common::ScalarFixedSizeBinary`, `datafusion_proto_common::protobuf_common::ScalarFixedSizeBinary`, `datafusion_proto_models::datafusion_common::ScalarFixedSizeBinary`, `datafusion_proto_models::generated::datafusion_common::ScalarFixedSizeBinary`

```rust
struct ScalarFixedSizeBinary
```

**Fields**: `values`, `length`

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

---

## ScalarNestedValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarNestedValue`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarNestedValue`, `datafusion_proto_common::ScalarNestedValue`, `datafusion_proto_common::protobuf_common::ScalarNestedValue`, `datafusion_proto_models::datafusion_common::ScalarNestedValue`, `datafusion_proto_models::generated::datafusion_common::ScalarNestedValue`

```rust
struct ScalarNestedValue
```

**Fields**: `ipc_message`, `arrow_data`, `schema`, `dictionaries`

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

Used for List/FixedSizeList/LargeList/ListView/LargeListView/Struct/Map

---

## ScalarRunEndEncodedValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarRunEndEncodedValue`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarRunEndEncodedValue`, `datafusion_proto_common::ScalarRunEndEncodedValue`, `datafusion_proto_common::protobuf_common::ScalarRunEndEncodedValue`, `datafusion_proto_models::datafusion_common::ScalarRunEndEncodedValue`, `datafusion_proto_models::generated::datafusion_common::ScalarRunEndEncodedValue`

```rust
struct ScalarRunEndEncodedValue
```

**Fields**: `run_ends_field`, `values_field`, `value`

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

---

## ScalarTime32Value

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarTime32Value`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarTime32Value`, `datafusion_proto_common::ScalarTime32Value`, `datafusion_proto_common::protobuf_common::ScalarTime32Value`, `datafusion_proto_models::datafusion_common::ScalarTime32Value`, `datafusion_proto_models::generated::datafusion_common::ScalarTime32Value`

```rust
struct ScalarTime32Value
```

**Fields**: `value`

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

---

## ScalarTime64Value

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarTime64Value`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarTime64Value`, `datafusion_proto_common::ScalarTime64Value`, `datafusion_proto_common::protobuf_common::ScalarTime64Value`, `datafusion_proto_models::datafusion_common::ScalarTime64Value`, `datafusion_proto_models::generated::datafusion_common::ScalarTime64Value`

```rust
struct ScalarTime64Value
```

**Fields**: `value`

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

---

## ScalarTimestampValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarTimestampValue`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarTimestampValue`, `datafusion_proto_common::ScalarTimestampValue`, `datafusion_proto_common::protobuf_common::ScalarTimestampValue`, `datafusion_proto_models::datafusion_common::ScalarTimestampValue`, `datafusion_proto_models::generated::datafusion_common::ScalarTimestampValue`

```rust
struct ScalarTimestampValue
```

**Fields**: `timezone`, `value`

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

---

## ScalarValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::ScalarValue`

Also reachable as `datafusion_proto::generated::datafusion_common::ScalarValue`, `datafusion_proto::protobuf::ScalarValue`, `datafusion_proto_common::ScalarValue`, `datafusion_proto_common::protobuf_common::ScalarValue`, `datafusion_proto_models::datafusion_common::ScalarValue`, `datafusion_proto_models::generated::datafusion_common::ScalarValue`

```rust
struct ScalarValue
```

**Fields**: `value`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(val: &ScalarValue) -> Result<Self, Self::Error>
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

---

## Schema

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Schema`

Also reachable as `datafusion_proto::generated::datafusion_common::Schema`, `datafusion_proto::protobuf::Schema`, `datafusion_proto_common::Schema`, `datafusion_proto_common::protobuf_common::Schema`, `datafusion_proto_models::datafusion_common::Schema`, `datafusion_proto_models::generated::datafusion_common::Schema`

```rust
struct Schema
```

**Fields**: `columns`, `metadata`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(schema: SchemaRef) -> Result<Self, Self::Error>
fn try_from(schema: &Schema) -> Result<Self, Self::Error>
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

---

## Statistics

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Statistics`

Also reachable as `datafusion_proto::generated::datafusion_common::Statistics`, `datafusion_proto_common::Statistics`, `datafusion_proto_common::protobuf_common::Statistics`, `datafusion_proto_models::datafusion_common::Statistics`, `datafusion_proto_models::generated::datafusion_common::Statistics`

```rust
struct Statistics
```

**Fields**: `num_rows`, `total_byte_size`, `column_stats`

**Implements**: `core::convert::From`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(s: &Statistics) -> protobuf::Statistics
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

---

## Struct

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Struct`

Also reachable as `datafusion_proto::generated::datafusion_common::Struct`, `datafusion_proto_common::Struct`, `datafusion_proto_common::protobuf_common::Struct`, `datafusion_proto_models::datafusion_common::Struct`, `datafusion_proto_models::generated::datafusion_common::Struct`

```rust
struct Struct
```

**Fields**: `sub_field_types`

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

---

## TableParquetOptions

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::TableParquetOptions`

Also reachable as `datafusion_proto::generated::datafusion_common::TableParquetOptions`, `datafusion_proto_common::TableParquetOptions`, `datafusion_proto_common::protobuf_common::TableParquetOptions`, `datafusion_proto_models::datafusion_common::TableParquetOptions`, `datafusion_proto_models::generated::datafusion_common::TableParquetOptions`

```rust
struct TableParquetOptions
```

**Fields**: `global`, `column_specific_options`, `key_value_metadata`

**Implements**: `core::convert::TryFrom`, `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &TableParquetOptions) -> datafusion_common::Result<Self, Self::Error>
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

---

## Timestamp

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Timestamp`

Also reachable as `datafusion_proto::generated::datafusion_common::Timestamp`, `datafusion_proto_common::Timestamp`, `datafusion_proto_common::protobuf_common::Timestamp`, `datafusion_proto_models::datafusion_common::Timestamp`, `datafusion_proto_models::generated::datafusion_common::Timestamp`

```rust
struct Timestamp
```

**Fields**: `time_unit`, `timezone`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## Union

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::Union`

Also reachable as `datafusion_proto::generated::datafusion_common::Union`, `datafusion_proto_common::Union`, `datafusion_proto_common::protobuf_common::Union`, `datafusion_proto_models::datafusion_common::Union`, `datafusion_proto_models::generated::datafusion_common::Union`

```rust
struct Union
```

**Fields**: `union_types`, `union_mode`, `type_ids`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## UnionField

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::UnionField`

Also reachable as `datafusion_proto::generated::datafusion_common::UnionField`, `datafusion_proto_common::UnionField`, `datafusion_proto_common::protobuf_common::UnionField`, `datafusion_proto_models::datafusion_common::UnionField`, `datafusion_proto_models::generated::datafusion_common::UnionField`

```rust
struct UnionField
```

**Fields**: `field_id`, `field`

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

---

## UnionValue

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::UnionValue`

Also reachable as `datafusion_proto::generated::datafusion_common::UnionValue`, `datafusion_proto_common::UnionValue`, `datafusion_proto_common::protobuf_common::UnionValue`, `datafusion_proto_models::datafusion_common::UnionValue`, `datafusion_proto_models::generated::datafusion_common::UnionValue`

```rust
struct UnionValue
```

**Fields**: `value_id`, `value`, `fields`, `mode`

**Implements**: `prost::message::Message`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

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

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## UniqueConstraint

`struct` · `datafusion_proto_common::generated::datafusion_proto_common::UniqueConstraint`

Also reachable as `datafusion_proto::generated::datafusion_common::UniqueConstraint`, `datafusion_proto_common::UniqueConstraint`, `datafusion_proto_common::protobuf_common::UniqueConstraint`, `datafusion_proto_models::datafusion_common::UniqueConstraint`, `datafusion_proto_models::generated::datafusion_common::UniqueConstraint`

```rust
struct UniqueConstraint
```

**Fields**: `indices`

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

---
