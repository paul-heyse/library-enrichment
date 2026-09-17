# `ast_grep_outline::model`

Crate `ast-grep-outline` · 7 public items · structured records in [`model/ast_grep_outline.model.json`](../model/ast_grep_outline.model.json)

## EntryRole

`enum` · `ast_grep_outline::model::EntryRole`

```rust
enum EntryRole
```

**Variants**: `Item`, `Member`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Entry placement in the outline tree.

---

## SymbolType

`enum` · `ast_grep_outline::model::SymbolType`

```rust
enum SymbolType
```

**Variants**: `File`, `Module`, `Namespace`, `Package`, `Class`, `Method`, `Property`, `Field`, `Constructor`, `Enum`, `Interface`, `Function`, `Variable`, `Constant`, `String`, `Number`, `Boolean`, `Array`, `Object`, `Key`, `Null`, `EnumMember`, `Struct`, `Event`, `Operator`, `TypeParameter`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Outline symbol category.

The names follow LSP `DocumentSymbol.kind`, but ast-grep stores the symbolic
category directly instead of exposing LSP numeric values.
See https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#textDocument_documentSymbol

---

## OutlineEntry

`struct` · `ast_grep_outline::model::OutlineEntry`

```rust
struct OutlineEntry<'a>
```

**Fields**: `role`, `symbol_type`, `name`, `range`, `signature`, `ast_kind`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Shared structural data for either a top-level item or a direct member.

---

## OutlineItem

`struct` · `ast_grep_outline::model::OutlineItem`

```rust
struct OutlineItem<'a>
```

**Fields**: `entry`, `is_import`, `is_exported`, `members`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One top-level outline item.

---

## OutlineMember

`struct` · `ast_grep_outline::model::OutlineMember`

```rust
struct OutlineMember<'a>
```

**Fields**: `entry`, `is_public`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

One direct member under an outline item.

---

## SourcePosition

`struct` · `ast_grep_outline::model::SourcePosition`

```rust
struct SourcePosition
```

**Fields**: `line`, `column`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Zero-based character position in a file.

This mirrors scan JSON's private `Position` shape. Core `Position` is not
serializable, and config's serializable range type is an internal matcher
input shape with optional columns, so outline keeps its output contract local.

---

## SourceRange

`struct` · `ast_grep_outline::model::SourceRange`

```rust
struct SourceRange
```

**Fields**: `byte_offset`, `start`, `end`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Source range for an outline entry.

This mirrors scan JSON's private `Range` shape. Outline owns the type here
so the model does not depend on scan rendering internals.

---
