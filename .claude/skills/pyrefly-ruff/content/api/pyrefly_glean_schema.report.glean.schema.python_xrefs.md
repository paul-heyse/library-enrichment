# `pyrefly_glean_schema::report::glean::schema::python_xrefs`

Crate `pyrefly_glean_schema` · 6 public items · structured records in [`model/pyrefly_glean_schema.report.glean.schema.python_xrefs.json`](../model/pyrefly_glean_schema.report.glean.schema.python_xrefs.json)

## XRef

`struct` · `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRef`

```rust
struct XRef
```

**Fields**: `target`, `source`

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

## XRefDeclarationsByFile

`struct` · `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRefDeclarationsByFile`

```rust
struct XRefDeclarationsByFile
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, target: python::DeclarationLocation, source: src::ByteSpan) -> Self
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

## XRefDeclarationsByFile_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRefDeclarationsByFile_key`

```rust
struct XRefDeclarationsByFile_key
```

**Fields**: `file`, `target`, `source`

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

## XRefDefinitionLocation

`struct` · `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRefDefinitionLocation`

```rust
struct XRefDefinitionLocation
```

**Fields**: `name`, `file`

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

## XRefsByFile

`struct` · `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRefsByFile`

```rust
struct XRefsByFile
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, xrefs: Vec<XRef>) -> Self
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

## XRefsByFile_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python_xrefs::XRefsByFile_key`

```rust
struct XRefsByFile_key
```

**Fields**: `file`, `xrefs`

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
