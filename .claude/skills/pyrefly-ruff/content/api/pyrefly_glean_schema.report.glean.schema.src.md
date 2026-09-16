# `pyrefly_glean_schema::report::glean::schema::src`

Crate `pyrefly_glean_schema` · 25 public items · structured records in [`model/pyrefly_glean_schema.report.glean.schema.src.json`](../model/pyrefly_glean_schema.report.glean.schema.src.json)

## IndexFailureReason

`enum` · `pyrefly_glean_schema::report::glean::schema::src::IndexFailureReason`

```rust
enum IndexFailureReason
```

**Variants**: `CompileError`, `BuildSystemError`, `Unclassified`, `DiscoveryError`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## Language

`enum` · `pyrefly_glean_schema::report::glean::schema::src::Language`

```rust
enum Language
```

**Variants**: `Buck`, `C`, `Cpp`, `Hack`, `Haskell`, `ObjC`, `ObjCpp`, `Python`, `Thrift`, `Java`, `GraphQL`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

---

## ByteRange

`struct` · `pyrefly_glean_schema::report::glean::schema::src::ByteRange`

```rust
struct ByteRange
```

**Fields**: `begin`, `end`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ByteSpan

`struct` · `pyrefly_glean_schema::report::glean::schema::src::ByteSpan`

```rust
struct ByteSpan
```

**Fields**: `start`, `length`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ByteSpanContains

`struct` · `pyrefly_glean_schema::report::glean::schema::src::ByteSpanContains`

```rust
struct ByteSpanContains
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(byteSpan: ByteSpan, contains: ByteSpan) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## ByteSpanContains_key

`struct` · `pyrefly_glean_schema::report::glean::schema::src::ByteSpanContains_key`

```rust
struct ByteSpanContains_key
```

**Fields**: `byteSpan`, `contains`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## File

`struct` · `pyrefly_glean_schema::report::glean::schema::src::File`

```rust
struct File
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: String) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## FileContent

`struct` · `pyrefly_glean_schema::report::glean::schema::src::FileContent`

```rust
struct FileContent
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: File, value: FileContent_value) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## FileDigest

`struct` · `pyrefly_glean_schema::report::glean::schema::src::FileDigest`

```rust
struct FileDigest
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: File, value: FileDigest_value) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## FileLanguage

`struct` · `pyrefly_glean_schema::report::glean::schema::src::FileLanguage`

```rust
struct FileLanguage
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: File, language: Language) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## FileLanguage_key

`struct` · `pyrefly_glean_schema::report::glean::schema::src::FileLanguage_key`

```rust
struct FileLanguage_key
```

**Fields**: `file`, `language`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## FileLines

`struct` · `pyrefly_glean_schema::report::glean::schema::src::FileLines`

```rust
struct FileLines
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: File, lengths: Vec<u64>, endsInNewline: bool, hasUnicodeOrTabs: bool) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## FileLines_key

`struct` · `pyrefly_glean_schema::report::glean::schema::src::FileLines_key`

```rust
struct FileLines_key
```

**Fields**: `file`, `lengths`, `endsInNewline`, `hasUnicodeOrTabs`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## FileLocation

`struct` · `pyrefly_glean_schema::report::glean::schema::src::FileLocation`

```rust
struct FileLocation
```

**Fields**: `file`, `span`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## IndexFailure

`struct` · `pyrefly_glean_schema::report::glean::schema::src::IndexFailure`

```rust
struct IndexFailure
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: File, reason: IndexFailureReason, details: String) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## IndexFailure_key

`struct` · `pyrefly_glean_schema::report::glean::schema::src::IndexFailure_key`

```rust
struct IndexFailure_key
```

**Fields**: `file`, `reason`, `details`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Loc

`struct` · `pyrefly_glean_schema::report::glean::schema::src::Loc`

```rust
struct Loc
```

**Fields**: `file`, `line`, `column`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## PackedByteSpansGroup

`struct` · `pyrefly_glean_schema::report::glean::schema::src::PackedByteSpansGroup`

```rust
struct PackedByteSpansGroup
```

**Fields**: `length`, `offsets`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## Range

`struct` · `pyrefly_glean_schema::report::glean::schema::src::Range`

```rust
struct Range
```

**Fields**: `file`, `lineBegin`, `columnBegin`, `lineEnd`, `columnEnd`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## RangeContains

`struct` · `pyrefly_glean_schema::report::glean::schema::src::RangeContains`

```rust
struct RangeContains
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(fileLines: Range, contains: Range) -> Self
```

**via `pyrefly_glean_schema::report::glean::facts::GleanPredicate`**

```rust
fn GLEAN_name() -> String
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

## RangeContains_key

`struct` · `pyrefly_glean_schema::report::glean::schema::src::RangeContains_key`

```rust
struct RangeContains_key
```

**Fields**: `fileLines`, `contains`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## RelByteSpan

`struct` · `pyrefly_glean_schema::report::glean::schema::src::RelByteSpan`

```rust
struct RelByteSpan
```

**Fields**: `offset`, `length`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## FileContent_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::src::FileContent_value`

```rust
type FileContent_value = String
```

---

## FileDigest_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::src::FileDigest_value`

```rust
type FileDigest_value = String
```

---

## PackedByteSpans

`type_alias` · `pyrefly_glean_schema::report::glean::schema::src::PackedByteSpans`

```rust
type PackedByteSpans = Vec<PackedByteSpansGroup>
```

---
