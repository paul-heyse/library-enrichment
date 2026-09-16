# `pyrefly_glean_schema::report::glean::schema::digest`

Crate `pyrefly_glean_schema` · 3 public items · structured records in [`model/pyrefly_glean_schema.report.glean.schema.digest.json`](../model/pyrefly_glean_schema.report.glean.schema.digest.json)

## Digest

`struct` · `pyrefly_glean_schema::report::glean::schema::digest::Digest`

```rust
struct Digest
```

**Fields**: `hash`, `size`

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

## FileDigest

`struct` · `pyrefly_glean_schema::report::glean::schema::digest::FileDigest`

```rust
struct FileDigest
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, digest: Digest) -> Self
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

## FileDigest_key

`struct` · `pyrefly_glean_schema::report::glean::schema::digest::FileDigest_key`

```rust
struct FileDigest_key
```

**Fields**: `file`, `digest`

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
