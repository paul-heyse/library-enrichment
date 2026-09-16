# `pyrefly::report::pysa::class`

Crate `pyrefly` · 3 public items · structured records in [`model/pyrefly.report.pysa.class.json`](../model/pyrefly.report.pysa.class.json)

## ClassFieldId

`struct` · `pyrefly::report::pysa::class::ClassFieldId`

```rust
struct ClassFieldId
```

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn from_class_and_name(class: &Class, name: &Name, context: &ModuleAnswersContext) -> ClassFieldId
fn to_int(self) -> u32
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a unique identifier for a field **within a class**.

---

## ClassId

`struct` · `pyrefly::report::pysa::class::ClassId`

```rust
struct ClassId
```

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn from_class(class: &Class) -> ClassId
fn to_int(self) -> u32
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Represents a unique identifier for a class **within a module**.

---

## ClassRef

`struct` · `pyrefly::report::pysa::class::ClassRef`

```rust
struct ClassRef
```

**Fields**: `module_id`, `class_id`, `class`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn from_class(class: &Class, context: &ModuleContext<'_>) -> ClassRef
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

---
