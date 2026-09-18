# `buoyant_kernel::struct_patch`

Crate `buoyant_kernel` · 4 public items · structured records in [`model/buoyant_kernel.struct_patch.json`](../model/buoyant_kernel.struct_patch.json)

## ExpressionFieldPatch

`struct` · `buoyant_kernel::struct_patch::ExpressionFieldPatch`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.struct_patch.ExpressionFieldPatch.md)

Also reachable as `buoyant_kernel::expressions::ExpressionFieldPatch`, `delta_kernel::struct_patch::ExpressionFieldPatch`

```rust
struct ExpressionFieldPatch
```

**Fields**: `keep_input`, `insertions`, `optional`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A patch affecting a single input field.

A field patch can keep or omit its input field, then insert zero or more expressions after the
input field's output position.

---

## ExpressionStructPatch

`struct` · `buoyant_kernel::struct_patch::ExpressionStructPatch`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.struct_patch.ExpressionStructPatch.md)

Also reachable as `buoyant_kernel::expressions::ExpressionStructPatch`, `delta_kernel::struct_patch::ExpressionStructPatch`

```rust
struct ExpressionStructPatch
```

**Fields**: `input_path`, `field_patches`, `prepended_fields`, `appended_fields`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn input_path(&self) -> Option<&ColumnName>
fn is_empty(&self) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(builder: StructPatchBuilder<ExpressionRef>) -> DeltaResult<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A sparse expression patch over the fields of one input struct.

`ExpressionStructPatch` achieves `O(changes)` space complexity instead of `O(schema_width)` by
only specifying fields that actually change (inserted, replaced, or deleted). Any input field
not specifically mentioned by the patch is passed through, unmodified and with the same relative
field ordering. This is particularly useful for wide schemas where only a few columns need to be
modified and/or dropped, or where a small number of columns need to be injected.

---

## ProjectionStructPatchBuilder

`struct` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.struct_patch.ProjectionStructPatchBuilder.md)

Also reachable as `delta_kernel::struct_patch::ProjectionStructPatchBuilder`

```rust
struct ProjectionStructPatchBuilder<'a>
```

**Derives**: Debug

**Methods** (17)

```rust
fn append(self, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn append_at(self, struct_path: impl CollectInto<ColumnName>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn build(self) -> DeltaResult<(SchemaRef, ExpressionRef)>
fn drop(self, field_name: impl Into<String>) -> Self
fn drop_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
fn drop_if_exists(self, field_name: impl Into<String>) -> Self
fn drop_if_exists_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
fn insert_after(self, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn insert_after_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn new(input_schema: &'a StructType) -> Self
fn new_nested(input_schema: &'a StructType, path: impl CollectInto<ColumnName>) -> Self
fn prepend(self, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn prepend_at(self, struct_path: impl CollectInto<ColumnName>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn replace(self, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn replace_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
fn replace_expr(self, field_name: impl Into<String>, expr: impl Into<ExpressionRef>) -> Self
fn replace_expr_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, expr: impl Into<ExpressionRef>) -> Self
```

Builds schema and expression patches together over an input schema.

Emitted fields are paired with the expression that produces them, keeping the output schema and
sparse expression patch structurally aligned. Because it is bound to the input schema,
[`replace_expr`](Self::replace_expr) can preserve an existing [`StructField`] while replacing
only its expression.

---

## StructPatchBuilder

`struct` · `buoyant_kernel::struct_patch::StructPatchBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.struct_patch.StructPatchBuilder.md)

Also reachable as `delta_kernel::struct_patch::StructPatchBuilder`

```rust
struct StructPatchBuilder<Item>
```

**Derives**: Debug

**Methods** (16)

```rust
fn append(self, item: impl Into<Item>) -> Self
fn append_at(self, struct_path: impl CollectInto<ColumnName>, item: impl Into<Item>) -> Self
fn build(self, input_schema: &StructType) -> DeltaResult<StructType>
fn build(self) -> DeltaResult<ExpressionStructPatch>
fn drop(self, field_name: impl Into<String>) -> Self
fn drop_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
fn drop_if_exists(self, field_name: impl Into<String>) -> Self
fn drop_if_exists_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
fn insert_after(self, field_name: impl Into<String>, item: impl Into<Item>) -> Self
fn insert_after_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, item: impl Into<Item>) -> Self
fn new() -> Self
fn new_nested(path: impl CollectInto<ColumnName>) -> Self
fn prepend(self, item: impl Into<Item>) -> Self
fn prepend_at(self, struct_path: impl CollectInto<ColumnName>, item: impl Into<Item>) -> Self
fn replace(self, field_name: impl Into<String>, item: impl Into<Item>) -> Self
fn replace_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, item: impl Into<Item>) -> Self
```

Builds a sparse struct patch from a sequence of requested patch operations.

The builder records user intent, checks for conflicting destructive operations, and lowers
nested field paths into recursive struct patches. The same builder surface drives both
expression patching
([`ExpressionStructPatchBuilder`](crate::expressions::ExpressionStructPatchBuilder)) and schema
patching ([`SchemaStructPatchBuilder`](crate::schema::SchemaStructPatchBuilder)); only the
terminal `build` step differs.

---
