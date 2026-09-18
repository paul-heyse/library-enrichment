# `datafusion_common::datatype`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.datatype.json`](../model/datafusion_common.datatype.json)

## DataTypeExt

`trait` · `datafusion_common::datatype::DataTypeExt`

```rust
trait DataTypeExt
```

**Implementors** (1)

- `arrow_schema::datatype::DataType`

**Methods** (2)

```rust
fn into_nullable_field(self) -> Field
fn into_nullable_field_ref(self) -> FieldRef
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.datatype.DataTypeExt.md).


DataFusion extension methods for Arrow [`DataType`]

---

## FieldExt

`trait` · `datafusion_common::datatype::FieldExt`

```rust
trait FieldExt
```

**Implementors** (2)

- `alloc::sync::Arc`
- `arrow_schema::field::Field`

**Methods** (7)

```rust
fn into_fixed_size_list(self, list_size: i32) -> Self
fn into_list(self) -> Self
fn into_list_item(self) -> Self
fn renamed(self, new_name: &str) -> Self
fn retyped(self, new_data_type: DataType) -> Self
fn with_field_metadata(self, metadata: &FieldMetadata) -> Self
fn with_field_metadata_opt(self, metadata: Option<&FieldMetadata>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.datatype.FieldExt.md).


DataFusion extension methods for Arrow [`Field`] and [`FieldRef`]

This trait is implemented for both [`Field`] and [`FieldRef`] and
provides convenience methods for efficiently working with both types.

For [`FieldRef`], the methods will attempt to unwrap the `Arc`
to avoid unnecessary cloning when possible.

---
