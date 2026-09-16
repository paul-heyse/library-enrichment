# `buoyant_kernel::table_features`

Crate `buoyant_kernel` · 10 public items · structured records in [`model/buoyant_kernel.table_features.json`](../model/buoyant_kernel.table_features.json)

## MAX_VALID_READER_VERSION

`constant` · `buoyant_kernel::table_features::MAX_VALID_READER_VERSION`

Also reachable as `delta_kernel::table_features::MAX_VALID_READER_VERSION`

```rust
const MAX_VALID_READER_VERSION: i32 = 3
```

Maximum reader protocol version that the kernel can handle.

---

## MAX_VALID_WRITER_VERSION

`constant` · `buoyant_kernel::table_features::MAX_VALID_WRITER_VERSION`

Also reachable as `delta_kernel::table_features::MAX_VALID_WRITER_VERSION`

```rust
const MAX_VALID_WRITER_VERSION: i32 = 7
```

Maximum writer protocol version that the kernel can handle.

---

## MIN_VALID_RW_VERSION

`constant` · `buoyant_kernel::table_features::MIN_VALID_RW_VERSION`

Also reachable as `delta_kernel::table_features::MIN_VALID_RW_VERSION`

```rust
const MIN_VALID_RW_VERSION: i32 = 1
```

Minimum reader/writer protocol version that the kernel can handle.

---

## SET_TABLE_FEATURE_SUPPORTED_PREFIX

`constant` · `buoyant_kernel::table_features::SET_TABLE_FEATURE_SUPPORTED_PREFIX`

Also reachable as `delta_kernel::table_features::SET_TABLE_FEATURE_SUPPORTED_PREFIX`

```rust
const SET_TABLE_FEATURE_SUPPORTED_PREFIX: &str = "delta.feature."
```

Prefix for table feature override properties.
Properties with this prefix (e.g., `delta.feature.deletionVectors`) are used to
explicitly turn on support for the feature in the protocol.

---

## SET_TABLE_FEATURE_SUPPORTED_VALUE

`constant` · `buoyant_kernel::table_features::SET_TABLE_FEATURE_SUPPORTED_VALUE`

Also reachable as `delta_kernel::table_features::SET_TABLE_FEATURE_SUPPORTED_VALUE`

```rust
const SET_TABLE_FEATURE_SUPPORTED_VALUE: &str = "supported"
```

Value to add support for a table feature when used with [`SET_TABLE_FEATURE_SUPPORTED_PREFIX`].
Example: `"delta.feature.deletionVectors" -> "supported"`

---

## TABLE_FEATURES_MIN_READER_VERSION

`constant` · `buoyant_kernel::table_features::TABLE_FEATURES_MIN_READER_VERSION`

Also reachable as `delta_kernel::table_features::TABLE_FEATURES_MIN_READER_VERSION`

```rust
const TABLE_FEATURES_MIN_READER_VERSION: i32 = 3
```

Minimum reader version for tables that use table features.
When set to 3, the protocol requires an explicit `readerFeatures` array.

---

## TABLE_FEATURES_MIN_WRITER_VERSION

`constant` · `buoyant_kernel::table_features::TABLE_FEATURES_MIN_WRITER_VERSION`

Also reachable as `delta_kernel::table_features::TABLE_FEATURES_MIN_WRITER_VERSION`

```rust
const TABLE_FEATURES_MIN_WRITER_VERSION: i32 = 7
```

Minimum writer version for tables that use table features.
When set to 7, the protocol requires an explicit `writerFeatures` array.

---

## Operation

`enum` · `buoyant_kernel::table_features::Operation`

Also reachable as `delta_kernel::table_features::Operation`

```rust
enum Operation
```

**Variants**: `Scan`, `Cdf`, `Write`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Represents the type of operation being performed on a table

---

## TableFeature

`enum` · `buoyant_kernel::table_features::TableFeature`

Also reachable as `delta_kernel::table_features::TableFeature`

```rust
enum TableFeature
```

**Variants**: `AppendOnly`, `Invariants`, `CheckConstraints`, `ChangeDataFeed`, `GeneratedColumns`, `IdentityColumns`, `InCommitTimestamp`, `RowTracking`, `DomainMetadata`, `IcebergCompatV1`, `IcebergCompatV2`, `IcebergCompatV3`, `ClusteredTable`, `MaterializePartitionColumns`, `AllowColumnDefaults`, `CatalogManaged`, `CatalogOwnedPreview`, `ColumnMapping`, `DeletionVectors`, `TimestampNanos`, `TimestampWithoutTimezone`, `TypeWidening`, `TypeWideningPreview`, `V2Checkpoint`, `VacuumProtocolCheck`, `VariantType`, `VariantTypePreview`, `VariantShredding`, `VariantShreddingPreview`, `Unknown`

**Implements**: `buoyant_kernel::schema::derive_macro_utils::ToDataType`, `core::convert::AsRef`, `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `strum::EnumCount`, `strum::IntoEnumIterator`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `buoyant_kernel::schema::derive_macro_utils::ToDataType`**

```rust
fn to_data_type() -> DataType
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::convert::From`**

```rust
fn from(value: &TableFeature) -> Self
fn from(value: String) -> Self
fn from(s: &str) -> TableFeature
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error>
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<TableFeature, <Self as ::core::str::FromStr>::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `strum::IntoEnumIterator`**

```rust
fn iter() -> TableFeatureIter
```

Table features represent protocol capabilities required to correctly read or write a given
table.
- Readers must implement all features required for correct table reads.
- Writers must implement all features required for correct table writes.

Each variant corresponds to one such feature. A feature is either:
- **ReaderWriter** (must be supported by both readers and writers), or
- **WriterOnly** (applies only to writers).
There are no ReaderOnly features. See `TableFeature::feature_type` for the category of each.

The kernel currently supports all reader features.

---

## TableFeatureIter

`struct` · `buoyant_kernel::table_features::TableFeatureIter`

Also reachable as `delta_kernel::table_features::TableFeatureIter`

```rust
struct TableFeatureIter
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item>
fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item>
fn size_hint(&self) -> (usize, ::core::option::Option<usize>)
```

An iterator over the variants of [TableFeature]

---
