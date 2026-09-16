# `pyrefly_glean_schema::report::glean::schema::gencode`

Crate `pyrefly_glean_schema` · 8 public items · structured records in [`model/pyrefly_glean_schema.report.glean.schema.gencode.json`](../model/pyrefly_glean_schema.report.glean.schema.gencode.json)

## GenCodeVariant

`enum` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeVariant`

```rust
enum GenCodeVariant
```

**Variants**: `Full`, `Partial`

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

## GenCode

`struct` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCode`

```rust
struct GenCode
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, variant: GenCodeVariant, source: Option<src::File>, command: Option<GenCodeCommand>, class_: Option<GenCodeClass>, signature: Option<GenCodeSignature>) -> Self
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

## GenCodeBySource

`struct` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeBySource`

```rust
struct GenCodeBySource
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(source: src::File, gencode: src::File) -> Self
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

## GenCodeBySource_key

`struct` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeBySource_key`

```rust
struct GenCodeBySource_key
```

**Fields**: `source`, `gencode`

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

## GenCodeClass

`struct` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeClass`

```rust
struct GenCodeClass
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

## GenCodeCommand

`struct` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeCommand`

```rust
struct GenCodeCommand
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

## GenCodeSignature

`struct` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCodeSignature`

```rust
struct GenCodeSignature
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

## GenCode_key

`struct` · `pyrefly_glean_schema::report::glean::schema::gencode::GenCode_key`

```rust
struct GenCode_key
```

**Fields**: `file`, `variant`, `source`, `command`, `class_`, `signature`

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
