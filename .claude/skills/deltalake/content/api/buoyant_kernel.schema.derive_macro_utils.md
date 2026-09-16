# `buoyant_kernel::schema::derive_macro_utils`

Crate `buoyant_kernel` · 3 public items · structured records in [`model/buoyant_kernel.schema.derive_macro_utils.json`](../model/buoyant_kernel.schema.derive_macro_utils.json)

## GetNullableContainerStructField

`trait` · `buoyant_kernel::schema::derive_macro_utils::GetNullableContainerStructField`

Also reachable as `delta_kernel::schema::derive_macro_utils::GetNullableContainerStructField`

```rust
trait GetNullableContainerStructField
```

**Implementors** (1)

- `core::option::Option`

**Methods** (1)

```rust
fn get_nullable_container_struct_field(name: impl Into<String>) -> StructField
```

---

## GetStructField

`trait` · `buoyant_kernel::schema::derive_macro_utils::GetStructField`

Also reachable as `delta_kernel::schema::derive_macro_utils::GetStructField`

```rust
trait GetStructField
```

**Implementors** (1)

- `core::option::Option`

**Methods** (1)

```rust
fn get_struct_field(name: impl Into<String>) -> StructField
```

The [`delta_kernel_derive::ToSchema`] macro uses this to convert a struct field's name + type
into a `StructField` definition. A blanket impl for `Option<T: ToDataType>` supports nullable
struct fields, which otherwise default to non-nullable.

---

## ToDataType

`trait` · `buoyant_kernel::schema::derive_macro_utils::ToDataType`

Also reachable as `delta_kernel::schema::derive_macro_utils::ToDataType`

```rust
trait ToDataType
```

**Implementors** (6)

- `alloc::string::String`
- `alloc::vec::Vec`
- `buoyant_kernel::actions::deletion_vector::DeletionVectorStorageType`
- `buoyant_kernel::table_features::TableFeature`
- `std::collections::hash::map::HashMap`
- `std::collections::hash::set::HashSet`

**Methods** (1)

```rust
fn to_data_type() -> DataType
```

Converts a type to a [`DataType`]. Implemented for the primitive types and automatically derived
for all types that implement [`ToSchema`].

---
