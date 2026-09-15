# `parquet::basic`

Crate `parquet` · 24 public items · structured records in [`model/parquet.basic.json`](../model/parquet.basic.json)

## BloomFilterAlgorithm

`enum` · `parquet::basic::BloomFilterAlgorithm`

```rust
enum BloomFilterAlgorithm
```

**Variants**: `BLOCK`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The algorithm used in Bloom filter.

---

## BloomFilterCompression

`enum` · `parquet::basic::BloomFilterCompression`

```rust
enum BloomFilterCompression
```

**Variants**: `UNCOMPRESSED`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The compression used in the Bloom filter.

---

## BloomFilterHash

`enum` · `parquet::basic::BloomFilterHash`

```rust
enum BloomFilterHash
```

**Variants**: `XXHASH`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The hash function used in Bloom filter. This function takes the hash of a column value
using plain encoding.

---

## BoundaryOrder

`enum` · `parquet::basic::BoundaryOrder`

```rust
enum BoundaryOrder
```

**Variants**: `UNORDERED`, `ASCENDING`, `DESCENDING`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Enum to annotate whether lists of min/max elements inside ColumnIndex
are ordered and if so, in which direction.

---

## ColumnOrder

`enum` · `parquet::basic::ColumnOrder`

```rust
enum ColumnOrder
```

**Variants**: `TYPE_DEFINED_ORDER`, `UNDEFINED`, `UNKNOWN`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn get_sort_order(logical_type: Option<LogicalType>, converted_type: ConvertedType, physical_type: Type) -> SortOrder
fn sort_order(&self) -> SortOrder
fn sort_order_for_type(logical_type: Option<&LogicalType>, converted_type: ConvertedType, physical_type: Type) -> SortOrder
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Column order that specifies what method was used to aggregate min/max values for
statistics.

If column order is undefined, then it is the legacy behaviour and all values should
be compared as signed values/bytes.

---

## Compression

`enum` · `parquet::basic::Compression`

```rust
enum Compression
```

**Variants**: `UNCOMPRESSED`, `SNAPPY`, `GZIP`, `LZO`, `BROTLI`, `LZ4`, `ZSTD`, `LZ4_RAW`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: CompressionCodec) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
```

Supported block compression algorithms.

Block compression can yield non-trivial improvements to storage efficiency at the expense
of potentially significantly worse encode and decode performance. Many applications,
especially those making use of high-throughput and low-cost commodity object storage,
may find storage efficiency less important than decode throughput, and therefore may
wish to not make use of block compression.

The writers in this crate default to no block compression for this reason.

Applications that do still wish to use block compression, will find [`Compression::ZSTD`]
to provide a good balance of compression, performance, and ecosystem support. Alternatively,
[`Compression::LZ4_RAW`] provides much faster decompression speeds, at the cost of typically
worse compression ratios. However, it is not as widely supported by the ecosystem, with the
Hadoop ecosystem historically favoring the non-standard and now deprecated [`Compression::LZ4`].

---

## CompressionCodec

`enum` · `parquet::basic::CompressionCodec`

```rust
enum CompressionCodec
```

**Variants**: `UNCOMPRESSED`, `SNAPPY`, `GZIP`, `LZO`, `BROTLI`, `LZ4`, `ZSTD`, `LZ4_RAW`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: Compression) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Supported compression algorithms.

Codecs added in format version X.Y can be read by readers based on X.Y and later.
Codec support may vary between readers based on the format version and
libraries available at runtime.

See [Compression.md] for a detailed specification of these algorithms.

[Compression.md]: https://github.com/apache/parquet-format/blob/master/Compression.md

---

## ConvertedType

`enum` · `parquet::basic::ConvertedType`

```rust
enum ConvertedType
```

**Variants**: `NONE`, `UTF8`, `MAP`, `MAP_KEY_VALUE`, `LIST`, `ENUM`, `DECIMAL`, `DATE`, `TIME_MILLIS`, `TIME_MICROS`, `TIMESTAMP_MILLIS`, `TIMESTAMP_MICROS`, `UINT_8`, `UINT_16`, `UINT_32`, `UINT_64`, `INT_8`, `INT_16`, `INT_32`, `INT_64`, `JSON`, `BSON`, `INTERVAL`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: Option<LogicalType>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self>
```

Common types (converted types) used by frameworks when using Parquet.

This helps map between types in those frameworks to the base types in Parquet.
This is only metadata and not needed to read or write the data.

This struct was renamed from `LogicalType` in version 4.0.0.
If targeting Parquet format 2.4.0 or above, please use [LogicalType] instead.

---

## EdgeInterpolationAlgorithm

`enum` · `parquet::basic::EdgeInterpolationAlgorithm`

```rust
enum EdgeInterpolationAlgorithm
```

**Variants**: `SPHERICAL`, `VINCENTY`, `THOMAS`, `ANDOYER`, `KARNEY`, `_Unknown`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn try_as_edges(&self) -> Result<parquet_geospatial::WkbEdges>
```

**via `core::convert::From`**

```rust
fn from(value: parquet_geospatial::WkbEdges) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self>
```

Edge interpolation algorithm for [`LogicalType::Geography`]

---

## Encoding

`enum` · `parquet::basic::Encoding`

```rust
enum Encoding
```

**Variants**: `PLAIN`, `PLAIN_DICTIONARY`, `RLE`, `BIT_PACKED`, `DELTA_BINARY_PACKED`, `DELTA_LENGTH_BYTE_ARRAY`, `DELTA_BYTE_ARRAY`, `RLE_DICTIONARY`, `BYTE_STREAM_SPLIT`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Encodings supported by Parquet.

Not all encodings are valid for all types. These enums are also used to specify the
encoding of definition and repetition levels.

By default this crate uses [Encoding::PLAIN], [Encoding::RLE], and [Encoding::RLE_DICTIONARY].
These provide very good encode and decode performance, whilst yielding reasonable storage
efficiency and being supported by all major parquet readers.

The delta encodings are also supported and will be used if a newer [WriterVersion] is
configured, however, it should be noted that these sacrifice encode and decode performance for
improved storage efficiency. This performance regression is particularly pronounced in the case
of record skipping as occurs during predicate push-down. It is recommended users assess the
performance impact when evaluating these encodings.

[WriterVersion]: crate::file::properties::WriterVersion

---

## FieldRepetitionType

`enum` · `parquet::basic::FieldRepetitionType`

```rust
enum FieldRepetitionType
```

**Variants**: `REQUIRED`, `OPTIONAL`, `REPEATED`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Representation of field types in schema.

---

## LogicalType

`enum` · `parquet::basic::LogicalType`

```rust
enum LogicalType
```

**Variants**: `String`, `Map`, `List`, `Enum`, `Decimal`, `Date`, `Time`, `Timestamp`, `Integer`, `Unknown`, `Json`, `Bson`, `Uuid`, `Float16`, `Variant`, `Geometry`, `Geography`, `_Unknown`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn decimal(scale: i32, precision: i32) -> Self
fn geography(crs: Option<String>, algorithm: Option<EdgeInterpolationAlgorithm>) -> Self
fn geometry(crs: Option<String>) -> Self
fn integer(bit_width: i8, is_signed: bool) -> Self
fn time(is_adjusted_to_u_t_c: bool, unit: TimeUnit) -> Self
fn timestamp(is_adjusted_to_u_t_c: bool, unit: TimeUnit) -> Self
fn variant(specification_version: Option<i8>) -> Self
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self>
```

Logical types used by version 2.4.0+ of the Parquet format.

This is an *entirely new* struct as of version
4.0.0. The struct previously named `LogicalType` was renamed to
[`ConvertedType`]. Please see the README.md for more details.

---

## PageType

`enum` · `parquet::basic::PageType`

```rust
enum PageType
```

**Variants**: `DATA_PAGE`, `INDEX_PAGE`, `DICTIONARY_PAGE`, `DATA_PAGE_V2`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Available data pages for Parquet file format.
Note that some of the page types may not be supported.

---

## SortOrder

`enum` · `parquet::basic::SortOrder`

```rust
enum SortOrder
```

**Variants**: `SIGNED`, `UNSIGNED`, `UNDEFINED`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_signed(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Sort order for page and column statistics.

Types are associated with sort orders and column stats are aggregated using a sort
order, and a sort order should be considered when comparing values with statistics
min/max.

See reference in
<https://github.com/apache/arrow/blob/main/cpp/src/parquet/types.h>

---

## TimeUnit

`enum` · `parquet::basic::TimeUnit`

```rust
enum TimeUnit
```

**Variants**: `MILLIS`, `MICROS`, `NANOS`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Time unit for `Time` and `Timestamp` logical types.

---

## Type

`enum` · `parquet::basic::Type`

```rust
enum Type
```

**Variants**: `BOOLEAN`, `INT32`, `INT64`, `INT96`, `FLOAT`, `DOUBLE`, `BYTE_ARRAY`, `FIXED_LEN_BYTE_ARRAY`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self>
```

Types supported by Parquet.

These physical types are intended to be used in combination with the encodings to
control the on disk storage format.
For example INT16 is not included as a type since a good encoding of INT32
would handle this.

---

## DecimalType

`struct` · `parquet::basic::DecimalType`

```rust
struct DecimalType
```

**Fields**: `scale`, `precision`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## EncodingMask

`struct` · `parquet::basic::EncodingMask`

```rust
struct EncodingMask
```

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn all_set<'a>(&self, encodings: impl Iterator<Item = &'a Encoding>) -> bool
fn as_i32(&self) -> i32
fn encodings(&self) -> impl Iterator<Item = Encoding>
fn insert(&mut self, val: Encoding)
fn is_only(&self, val: Encoding) -> bool
fn is_set(&self, val: Encoding) -> bool
fn new_from_encodings<'a>(encodings: impl Iterator<Item = &'a Encoding>) -> Self
fn try_new(val: i32) -> Result<Self>
```

A bitmask representing the [`Encoding`]s employed while encoding a Parquet column chunk.

The Parquet [`ColumnMetaData`] struct contains an array that indicates what encodings were
used when writing that column chunk. For memory and performance reasons, this crate reduces
that array to bitmask, where each bit position represents a different [`Encoding`]. This
struct contains that bitmask, and provides methods to interact with the data.

# Example
```no_run
# use parquet::file::metadata::ParquetMetaDataReader;
# use parquet::basic::Encoding;
# fn open_parquet_file(path: &str) -> std::fs::File { unimplemented!(); }
// read parquet metadata from a file
let file = open_parquet_file("some_path.parquet");
let mut reader = ParquetMetaDataReader::new();
reader.try_parse(&file).unwrap();
let metadata = reader.finish().unwrap();

// find the encodings used by the first column chunk in the first row group
let col_meta = metadata.row_group(0).column(0);
let encodings = col_meta.encodings_mask();

// check to see if a particular encoding was used
let used_rle = encodings.is_set(Encoding::RLE);

// check to see if all of a set of encodings were used
let used_all = encodings.all_set([Encoding::RLE, Encoding::PLAIN].iter());

// convert mask to a Vec<Encoding>
let encodings_vec = encodings.encodings().collect::<Vec<_>>();
```

[`ColumnMetaData`]: https://github.com/apache/parquet-format/blob/9fd57b59e0ce1a82a69237dcf8977d3e72a2965d/src/main/thrift/parquet.thrift#L875

---

## GeographyType

`struct` · `parquet::basic::GeographyType`

```rust
struct GeographyType
```

**Fields**: `crs`, `algorithm`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn algorithm(&self) -> Option<EdgeInterpolationAlgorithm>
```

---

## GeometryType

`struct` · `parquet::basic::GeometryType`

```rust
struct GeometryType
```

**Fields**: `crs`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## IntType

`struct` · `parquet::basic::IntType`

```rust
struct IntType
```

**Fields**: `bit_width`, `is_signed`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## TimestampType

`struct` · `parquet::basic::TimestampType`

Also reachable as `parquet::basic::TimeType`

```rust
struct TimestampType
```

**Fields**: `is_adjusted_to_u_t_c`, `unit`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## VariantType

`struct` · `parquet::basic::VariantType`

```rust
struct VariantType
```

**Fields**: `specification_version`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## Repetition

`type_alias` · `parquet::basic::Repetition`

```rust
type Repetition = FieldRepetitionType
```

Type alias for thrift `FieldRepetitionType`

---
