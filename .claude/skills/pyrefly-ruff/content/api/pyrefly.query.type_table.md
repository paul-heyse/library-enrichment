# `pyrefly::query::type_table`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.query.type_table.json`](../model/pyrefly.query.type_table.json)

## IndexedTypeShapeKind

`enum` · `pyrefly::query::type_table::IndexedTypeShapeKind`

Also reachable as `pyrefly::query::IndexedTypeShapeKind`

```rust
enum IndexedTypeShapeKind
```

**Variants**: `Named`, `Callable`, `TypeVariable`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## LocatedTypeTableRef

`struct` · `pyrefly::query::type_table::LocatedTypeTableRef`

Also reachable as `pyrefly::query::LocatedTypeTableRef`

```rust
struct LocatedTypeTableRef
```

**Fields**: `location`, `type_index`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## SerializedTypeTableEntry

`struct` · `pyrefly::query::type_table::SerializedTypeTableEntry`

Also reachable as `pyrefly::query::SerializedTypeTableEntry`

```rust
struct SerializedTypeTableEntry
```

**Fields**: `kind`, `hash`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A `type_table` entry as sent on the wire: the shape plus its structural
hash. The hash lets clients keep a cross-file (global) hash -> parsed shape
cache: it is stable across files/requests for structurally-identical shapes
because it incorporates every field the shape is deduped on (name, args,
unspecified arg count, and traits).

---

## TypeTableResponseData

`struct` · `pyrefly::query::type_table::TypeTableResponseData`

Also reachable as `pyrefly::query::TypeTableResponseData`

```rust
struct TypeTableResponseData
```

**Fields**: `type_table`, `types`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
