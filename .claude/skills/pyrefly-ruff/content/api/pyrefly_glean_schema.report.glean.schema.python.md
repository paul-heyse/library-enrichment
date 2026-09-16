# `pyrefly_glean_schema::report::glean::schema::python`

Crate `pyrefly_glean_schema` · 141 public items · structured records in [`model/pyrefly_glean_schema.report.glean.schema.python.json`](../model/pyrefly_glean_schema.report.glean.schema.python.json)

## Argument

`enum` · `pyrefly_glean_schema::report::glean::schema::python::Argument`

```rust
enum Argument
```

**Variants**: `lit`

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

## Declaration

`enum` · `pyrefly_glean_schema::report::glean::schema::python::Declaration`

```rust
enum Declaration
```

**Variants**: `cls`, `func`, `variable`, `imp`, `module`

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

## DeclarationContainer

`enum` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationContainer`

```rust
enum DeclarationContainer
```

**Variants**: `module`, `cls`, `func`

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

## Definition

`enum` · `pyrefly_glean_schema::report::glean::schema::python::Definition`

```rust
enum Definition
```

**Variants**: `cls`, `func`, `variable`, `module`

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

## BaseClassToDerived

`struct` · `pyrefly_glean_schema::report::glean::schema::python::BaseClassToDerived`

```rust
struct BaseClassToDerived
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(base: ClassDeclaration, derived: ClassDeclaration) -> Self
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

## BaseClassToDerived_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::BaseClassToDerived_key`

```rust
struct BaseClassToDerived_key
```

**Fields**: `base`, `derived`

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

## CallArgument

`struct` · `pyrefly_glean_schema::report::glean::schema::python::CallArgument`

```rust
struct CallArgument
```

**Fields**: `label`, `span`, `argument`

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

## CalleeToCaller

`struct` · `pyrefly_glean_schema::report::glean::schema::python::CalleeToCaller`

```rust
struct CalleeToCaller
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(callee: Name, caller: Name) -> Self
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

## CalleeToCaller_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::CalleeToCaller_key`

```rust
struct CalleeToCaller_key
```

**Fields**: `callee`, `caller`

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

## ClassBySName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ClassBySName`

```rust
struct ClassBySName
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: SName, value: ClassBySName_value) -> Self
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

## ClassDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ClassDeclaration`

```rust
struct ClassDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: Name, bases: Option<Vec<Name>>) -> Self
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

## ClassDeclaration_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ClassDeclaration_key`

```rust
struct ClassDeclaration_key
```

**Fields**: `name`, `bases`

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

## ClassDefinition

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ClassDefinition`

```rust
struct ClassDefinition
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: ClassDeclaration, bases: Option<Vec<ClassDeclaration>>, keywords: Option<Vec<Parameter>>, decorators: Option<Vec<Decorator>>, container: Option<DeclarationContainer>) -> Self
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

## ClassDefinition_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ClassDefinition_key`

```rust
struct ClassDefinition_key
```

**Fields**: `declaration`, `bases`, `keywords`, `decorators`, `container`

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

## ContainedBy

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ContainedBy`

```rust
struct ContainedBy
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(containee: Declaration, container: Declaration) -> Self
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

## ContainedByTopLevelDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ContainedByTopLevelDeclaration`

```rust
struct ContainedByTopLevelDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(container: Declaration, declaration: Declaration) -> Self
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

## ContainedByTopLevelDeclaration_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ContainedByTopLevelDeclaration_key`

```rust
struct ContainedByTopLevelDeclaration_key
```

**Fields**: `container`, `declaration`

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

## ContainedBy_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ContainedBy_key`

```rust
struct ContainedBy_key
```

**Fields**: `containee`, `container`

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

## ContainingTopLevelDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ContainingTopLevelDeclaration`

```rust
struct ContainingTopLevelDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: Declaration, container: Declaration) -> Self
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

## ContainingTopLevelDeclaration_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ContainingTopLevelDeclaration_key`

```rust
struct ContainingTopLevelDeclaration_key
```

**Fields**: `declaration`, `container`

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

## Contains

`struct` · `pyrefly_glean_schema::report::glean::schema::python::Contains`

```rust
struct Contains
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(container: Declaration, containee: Declaration) -> Self
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

## Contains_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::Contains_key`

```rust
struct Contains_key
```

**Fields**: `container`, `containee`

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

## DeclarationDefinition

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationDefinition`

```rust
struct DeclarationDefinition
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: Declaration, definition: Definition) -> Self
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

## DeclarationDefinition_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationDefinition_key`

```rust
struct DeclarationDefinition_key
```

**Fields**: `declaration`, `definition`

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

## DeclarationDocstring

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationDocstring`

```rust
struct DeclarationDocstring
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: Declaration, location: src::ByteSpan, pretty_text: String) -> Self
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

## DeclarationDocstring_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationDocstring_key`

```rust
struct DeclarationDocstring_key
```

**Fields**: `declaration`, `location`, `pretty_text`

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

## DeclarationLocation

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationLocation`

```rust
struct DeclarationLocation
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: Declaration, file: src::File, span: src::ByteSpan) -> Self
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

## DeclarationLocation_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationLocation_key`

```rust
struct DeclarationLocation_key
```

**Fields**: `declaration`, `file`, `span`

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

## DeclarationReference

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationReference`

```rust
struct DeclarationReference
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(target: Declaration, source: Declaration) -> Self
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

## DeclarationReference_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationReference_key`

```rust
struct DeclarationReference_key
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

## DeclarationToName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationToName`

```rust
struct DeclarationToName
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: Declaration, value: DeclarationToName_value) -> Self
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

## DeclarationUses

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationUses`

```rust
struct DeclarationUses
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: Declaration, file: src::File, span: src::ByteSpan) -> Self
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

## DeclarationUses_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationUses_key`

```rust
struct DeclarationUses_key
```

**Fields**: `declaration`, `file`, `span`

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

## DeclarationWithLocalName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithLocalName`

```rust
struct DeclarationWithLocalName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(local_name: Name, declaration: Declaration) -> Self
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

## DeclarationWithLocalName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithLocalName_key`

```rust
struct DeclarationWithLocalName_key
```

**Fields**: `local_name`, `declaration`

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

## DeclarationWithName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithName`

```rust
struct DeclarationWithName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: Name, declaration: Declaration) -> Self
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

## DeclarationWithName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithName_key`

```rust
struct DeclarationWithName_key
```

**Fields**: `name`, `declaration`

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

## DeclarationWithSName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithSName`

```rust
struct DeclarationWithSName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(sname: SName, declaration: Declaration) -> Self
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

## DeclarationWithSName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationWithSName_key`

```rust
struct DeclarationWithSName_key
```

**Fields**: `sname`, `declaration`

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

## DeclarationsByFile

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationsByFile`

```rust
struct DeclarationsByFile
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, span: src::ByteSpan, declaration: Declaration) -> Self
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

## DeclarationsByFile_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationsByFile_key`

```rust
struct DeclarationsByFile_key
```

**Fields**: `file`, `span`, `declaration`

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

## DefinitionDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DefinitionDeclaration`

```rust
struct DefinitionDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(definition: Definition, declaration: Declaration) -> Self
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

## DefinitionDeclaration_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DefinitionDeclaration_key`

```rust
struct DefinitionDeclaration_key
```

**Fields**: `definition`, `declaration`

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

## DefinitionLocation

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DefinitionLocation`

```rust
struct DefinitionLocation
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(definition: Definition, file: src::File, span: src::ByteSpan) -> Self
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

## DefinitionLocation_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DefinitionLocation_key`

```rust
struct DefinitionLocation_key
```

**Fields**: `definition`, `file`, `span`

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

## DefinitionsByFile

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DefinitionsByFile`

```rust
struct DefinitionsByFile
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, span: src::ByteSpan, definition: Definition) -> Self
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

## DefinitionsByFile_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DefinitionsByFile_key`

```rust
struct DefinitionsByFile_key
```

**Fields**: `file`, `span`, `definition`

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

## DerivedClassToBase

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DerivedClassToBase`

```rust
struct DerivedClassToBase
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(derived: ClassDeclaration, base: ClassDeclaration) -> Self
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

## DerivedClassToBase_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DerivedClassToBase_key`

```rust
struct DerivedClassToBase_key
```

**Fields**: `derived`, `base`

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

## DirectXRef

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DirectXRef`

```rust
struct DirectXRef
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

## DirectXRefsByFile

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DirectXRefsByFile`

```rust
struct DirectXRefsByFile
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, xref: DirectXRef) -> Self
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

## DirectXRefsByFile_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::DirectXRefsByFile_key`

```rust
struct DirectXRefsByFile_key
```

**Fields**: `file`, `xref`

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

## FileCall

`struct` · `pyrefly_glean_schema::report::glean::schema::python::FileCall`

```rust
struct FileCall
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, callee_span: src::ByteSpan, call_args: Vec<CallArgument>) -> Self
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

## FileCall_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::FileCall_key`

```rust
struct FileCall_key
```

**Fields**: `file`, `callee_span`, `call_args`

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

## FunctionBySName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::FunctionBySName`

```rust
struct FunctionBySName
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: SName, value: FunctionBySName_value) -> Self
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

## FunctionDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::FunctionDeclaration`

```rust
struct FunctionDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: Name) -> Self
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

## FunctionDeclaration_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::FunctionDeclaration_key`

```rust
struct FunctionDeclaration_key
```

**Fields**: `name`

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

## FunctionDefinition

`struct` · `pyrefly_glean_schema::report::glean::schema::python::FunctionDefinition`

```rust
struct FunctionDefinition
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: FunctionDeclaration, is_async: bool, returnsInfo: Option<TypeInfo>, params: Vec<Parameter>, posonly_params: Option<Vec<Parameter>>, kwonly_params: Option<Vec<Parameter>>, star_arg: Option<Parameter>, star_kwarg: Option<Parameter>, decorators: Option<Vec<Decorator>>, container: Option<DeclarationContainer>) -> Self
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

## FunctionDefinition_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::FunctionDefinition_key`

```rust
struct FunctionDefinition_key
```

**Fields**: `declaration`, `is_async`, `returnsInfo`, `params`, `posonly_params`, `kwonly_params`, `star_arg`, `star_kwarg`, `decorators`, `container`

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

## ImportStarLocation

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStarLocation`

```rust
struct ImportStarLocation
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(import_star: ImportStarStatement, file: src::File, span: src::ByteSpan) -> Self
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

## ImportStarLocation_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStarLocation_key`

```rust
struct ImportStarLocation_key
```

**Fields**: `import_star`, `file`, `span`

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

## ImportStarStatement

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStarStatement`

```rust
struct ImportStarStatement
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(from_name: Name, into_module: Module) -> Self
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

## ImportStarStatement_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStarStatement_key`

```rust
struct ImportStarStatement_key
```

**Fields**: `from_name`, `into_module`

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

## ImportStarsByFile

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStarsByFile`

```rust
struct ImportStarsByFile
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, span: src::ByteSpan, declaration: ImportStarStatement) -> Self
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

## ImportStarsByFile_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStarsByFile_key`

```rust
struct ImportStarsByFile_key
```

**Fields**: `file`, `span`, `declaration`

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

## ImportStatement

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStatement`

```rust
struct ImportStatement
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(from_name: Name, as_name: Name) -> Self
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

## ImportStatementByAsName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStatementByAsName`

```rust
struct ImportStatementByAsName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: Name, import_: ImportStatement) -> Self
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

## ImportStatementByAsName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStatementByAsName_key`

```rust
struct ImportStatementByAsName_key
```

**Fields**: `name`, `import_`

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

## ImportStatementByAsSName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStatementByAsSName`

```rust
struct ImportStatementByAsSName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(sname: SName, import_: ImportStatement) -> Self
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

## ImportStatementByAsSName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStatementByAsSName_key`

```rust
struct ImportStatementByAsSName_key
```

**Fields**: `sname`, `import_`

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

## ImportStatement_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ImportStatement_key`

```rust
struct ImportStatement_key
```

**Fields**: `from_name`, `as_name`

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

## IsAbstract

`struct` · `pyrefly_glean_schema::report::glean::schema::python::IsAbstract`

```rust
struct IsAbstract
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: Declaration) -> Self
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

## IsTopLevelDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::IsTopLevelDeclaration`

```rust
struct IsTopLevelDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: Declaration) -> Self
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

## IsTopLevelDefinition

`struct` · `pyrefly_glean_schema::report::glean::schema::python::IsTopLevelDefinition`

```rust
struct IsTopLevelDefinition
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: Definition) -> Self
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

## MethodByLocalNameStr

`struct` · `pyrefly_glean_schema::report::glean::schema::python::MethodByLocalNameStr`

```rust
struct MethodByLocalNameStr
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(cls: ClassDeclaration, method_local_name: String, method: FunctionDeclaration) -> Self
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

## MethodByLocalNameStr_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::MethodByLocalNameStr_key`

```rust
struct MethodByLocalNameStr_key
```

**Fields**: `cls`, `method_local_name`, `method`

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

## MethodOverriden

`struct` · `pyrefly_glean_schema::report::glean::schema::python::MethodOverriden`

```rust
struct MethodOverriden
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(base: FunctionDeclaration, derived: FunctionDeclaration) -> Self
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

## MethodOverriden_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::MethodOverriden_key`

```rust
struct MethodOverriden_key
```

**Fields**: `base`, `derived`

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

## MethodOverrides

`struct` · `pyrefly_glean_schema::report::glean::schema::python::MethodOverrides`

```rust
struct MethodOverrides
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(derived: FunctionDeclaration, base: FunctionDeclaration) -> Self
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

## MethodOverrides_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::MethodOverrides_key`

```rust
struct MethodOverrides_key
```

**Fields**: `derived`, `base`

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

## Module

`struct` · `pyrefly_glean_schema::report::glean::schema::python::Module`

```rust
struct Module
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: Name) -> Self
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

## ModuleBySName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ModuleBySName`

```rust
struct ModuleBySName
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: SName, value: ModuleBySName_value) -> Self
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

## ModuleDefinition

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ModuleDefinition`

```rust
struct ModuleDefinition
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(module: Module) -> Self
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

## ModuleDefinition_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ModuleDefinition_key`

```rust
struct ModuleDefinition_key
```

**Fields**: `module`

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

## Module_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::Module_key`

```rust
struct Module_key
```

**Fields**: `name`

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

## Name

`struct` · `pyrefly_glean_schema::report::glean::schema::python::Name`

```rust
struct Name
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

## NameToSName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::NameToSName`

```rust
struct NameToSName
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: Name, value: NameToSName_value) -> Self
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

## NonImportDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::NonImportDeclaration`

```rust
struct NonImportDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: Declaration) -> Self
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

## Parameter

`struct` · `pyrefly_glean_schema::report::glean::schema::python::Parameter`

```rust
struct Parameter
```

**Fields**: `name`, `typeInfo`, `value`

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

## ResolveOriginalName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ResolveOriginalName`

```rust
struct ResolveOriginalName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: Name, original_name: Name) -> Self
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

## ResolveOriginalName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::ResolveOriginalName_key`

```rust
struct ResolveOriginalName_key
```

**Fields**: `name`, `original_name`

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

## SName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SName`

```rust
struct SName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(local_name: Name, parent: Option<SName>) -> Self
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

## SNameToName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SNameToName`

```rust
struct SNameToName
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: SName, value: SNameToName_value) -> Self
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

## SNameWithDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SNameWithDeclaration`

```rust
struct SNameWithDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: Declaration, sname: SName) -> Self
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

## SNameWithDeclaration_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SNameWithDeclaration_key`

```rust
struct SNameWithDeclaration_key
```

**Fields**: `declaration`, `sname`

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

## SName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SName_key`

```rust
struct SName_key
```

**Fields**: `local_name`, `parent`

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

## SearchClassByLowerCaseName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchClassByLowerCaseName`

```rust
struct SearchClassByLowerCaseName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name_lowercase: String, name: String) -> Self
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

## SearchClassByLowerCaseName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchClassByLowerCaseName_key`

```rust
struct SearchClassByLowerCaseName_key
```

**Fields**: `name_lowercase`, `name`

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

## SearchClassByName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchClassByName`

```rust
struct SearchClassByName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: String, parent: Option<SName>, decl: ClassDeclaration) -> Self
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

## SearchClassByName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchClassByName_key`

```rust
struct SearchClassByName_key
```

**Fields**: `name`, `parent`, `decl`

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

## SearchFieldByLowerCaseName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFieldByLowerCaseName`

```rust
struct SearchFieldByLowerCaseName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name_lowercase: String, name: String) -> Self
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

## SearchFieldByLowerCaseName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFieldByLowerCaseName_key`

```rust
struct SearchFieldByLowerCaseName_key
```

**Fields**: `name_lowercase`, `name`

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

## SearchFieldByName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFieldByName`

```rust
struct SearchFieldByName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: String, parent: Option<SName>, decl: VariableDeclaration) -> Self
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

## SearchFieldByName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFieldByName_key`

```rust
struct SearchFieldByName_key
```

**Fields**: `name`, `parent`, `decl`

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

## SearchFunctionByLowerCaseName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFunctionByLowerCaseName`

```rust
struct SearchFunctionByLowerCaseName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name_lowercase: String, name: String) -> Self
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

## SearchFunctionByLowerCaseName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFunctionByLowerCaseName_key`

```rust
struct SearchFunctionByLowerCaseName_key
```

**Fields**: `name_lowercase`, `name`

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

## SearchFunctionByName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFunctionByName`

```rust
struct SearchFunctionByName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: String, parent: Option<SName>, decl: FunctionDeclaration) -> Self
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

## SearchFunctionByName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchFunctionByName_key`

```rust
struct SearchFunctionByName_key
```

**Fields**: `name`, `parent`, `decl`

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

## SearchMethodByLowerCaseName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchMethodByLowerCaseName`

```rust
struct SearchMethodByLowerCaseName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name_lowercase: String, name: String) -> Self
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

## SearchMethodByLowerCaseName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchMethodByLowerCaseName_key`

```rust
struct SearchMethodByLowerCaseName_key
```

**Fields**: `name_lowercase`, `name`

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

## SearchMethodByName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchMethodByName`

```rust
struct SearchMethodByName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: String, parent: Option<SName>, decl: FunctionDeclaration) -> Self
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

## SearchMethodByName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchMethodByName_key`

```rust
struct SearchMethodByName_key
```

**Fields**: `name`, `parent`, `decl`

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

## SearchModuleByLowerCaseName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchModuleByLowerCaseName`

```rust
struct SearchModuleByLowerCaseName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name_lowercase: String, name: String) -> Self
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

## SearchModuleByLowerCaseName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchModuleByLowerCaseName_key`

```rust
struct SearchModuleByLowerCaseName_key
```

**Fields**: `name_lowercase`, `name`

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

## SearchModuleByName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchModuleByName`

```rust
struct SearchModuleByName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: String, parent: Option<SName>, decl: Module) -> Self
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

## SearchModuleByName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchModuleByName_key`

```rust
struct SearchModuleByName_key
```

**Fields**: `name`, `parent`, `decl`

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

## SearchVariableByLowerCaseName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchVariableByLowerCaseName`

```rust
struct SearchVariableByLowerCaseName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name_lowercase: String, name: String) -> Self
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

## SearchVariableByLowerCaseName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchVariableByLowerCaseName_key`

```rust
struct SearchVariableByLowerCaseName_key
```

**Fields**: `name_lowercase`, `name`

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

## SearchVariableByName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchVariableByName`

```rust
struct SearchVariableByName
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: String, parent: Option<SName>, decl: VariableDeclaration) -> Self
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

## SearchVariableByName_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::SearchVariableByName_key`

```rust
struct SearchVariableByName_key
```

**Fields**: `name`, `parent`, `decl`

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

## StringLiteral

`struct` · `pyrefly_glean_schema::report::glean::schema::python::StringLiteral`

```rust
struct StringLiteral
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

## Type

`struct` · `pyrefly_glean_schema::report::glean::schema::python::Type`

```rust
struct Type
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

## TypeInfo

`struct` · `pyrefly_glean_schema::report::glean::schema::python::TypeInfo`

```rust
struct TypeInfo
```

**Fields**: `displayType`, `xrefs`

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

## VariableBySName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::VariableBySName`

```rust
struct VariableBySName
```

**Fields**: `id`, `key`, `value`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(key: SName, value: VariableBySName_value) -> Self
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

## VariableDeclaration

`struct` · `pyrefly_glean_schema::report::glean::schema::python::VariableDeclaration`

```rust
struct VariableDeclaration
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(name: Name) -> Self
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

## VariableDeclaration_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::VariableDeclaration_key`

```rust
struct VariableDeclaration_key
```

**Fields**: `name`

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

## VariableDefinition

`struct` · `pyrefly_glean_schema::report::glean::schema::python::VariableDefinition`

```rust
struct VariableDefinition
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(declaration: VariableDeclaration, typeInfo: Option<TypeInfo>, container: Option<DeclarationContainer>) -> Self
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

## VariableDefinition_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::VariableDefinition_key`

```rust
struct VariableDefinition_key
```

**Fields**: `declaration`, `typeInfo`, `container`

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

## XRefViaName

`struct` · `pyrefly_glean_schema::report::glean::schema::python::XRefViaName`

```rust
struct XRefViaName
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

## XRefsViaNameByFile

`struct` · `pyrefly_glean_schema::report::glean::schema::python::XRefsViaNameByFile`

```rust
struct XRefsViaNameByFile
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(file: src::File, xrefs: Vec<XRefViaName>) -> Self
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

## XRefsViaNameByFile_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::XRefsViaNameByFile_key`

```rust
struct XRefsViaNameByFile_key
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

## XRefsViaNameByTarget

`struct` · `pyrefly_glean_schema::report::glean::schema::python::XRefsViaNameByTarget`

```rust
struct XRefsViaNameByTarget
```

**Fields**: `id`, `key`

**Implements**: `pyrefly_glean_schema::report::glean::facts::GleanPredicate`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(target: Name, file: src::File, spans: Vec<src::ByteSpan>) -> Self
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

## XRefsViaNameByTarget_key

`struct` · `pyrefly_glean_schema::report::glean::schema::python::XRefsViaNameByTarget_key`

```rust
struct XRefsViaNameByTarget_key
```

**Fields**: `target`, `file`, `spans`

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

## ClassBySName_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::ClassBySName_value`

```rust
type ClassBySName_value = ClassDeclaration
```

---

## DeclarationToName_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::DeclarationToName_value`

```rust
type DeclarationToName_value = Name
```

---

## Decorator

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::Decorator`

```rust
type Decorator = String
```

---

## FunctionBySName_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::FunctionBySName_value`

```rust
type FunctionBySName_value = FunctionDeclaration
```

---

## ModuleBySName_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::ModuleBySName_value`

```rust
type ModuleBySName_value = Module
```

---

## NameToSName_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::NameToSName_value`

```rust
type NameToSName_value = SName
```

---

## SNameToName_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::SNameToName_value`

```rust
type SNameToName_value = Name
```

---

## VariableBySName_value

`type_alias` · `pyrefly_glean_schema::report::glean::schema::python::VariableBySName_value`

```rust
type VariableBySName_value = VariableDeclaration
```

---
