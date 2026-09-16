# `pyrefly::report::pysa::pysa_report_capnp::project_file`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.project_file.json`](../model/pyrefly.report.pysa.pysa_report_capnp.project_file.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::project_file::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (28)

```rust
fn get_builtin_module_ids(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a, u32>>
fn get_dict_class_refs(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>>
fn get_modules(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_project_module::Owned>>
fn get_object_class_refs(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>>
fn get_typing_mapping_class_refs(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>>
fn get_typing_module_ids(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a, u32>>
fn has_builtin_module_ids(&self) -> bool
fn has_dict_class_refs(&self) -> bool
fn has_modules(&self) -> bool
fn has_object_class_refs(&self) -> bool
fn has_typing_mapping_class_refs(&self) -> bool
fn has_typing_module_ids(&self) -> bool
fn init_builtin_module_ids(self, size: u32) -> ::capnp::primitive_list::Builder<'a, u32>
fn init_dict_class_refs(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>
fn init_modules(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_project_module::Owned>
fn init_object_class_refs(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>
fn init_typing_mapping_class_refs(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>
fn init_typing_module_ids(self, size: u32) -> ::capnp::primitive_list::Builder<'a, u32>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_builtin_module_ids(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u32>>) -> ::capnp::Result<()>
fn set_dict_class_refs(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::class_ref::Owned>) -> ::capnp::Result<()>
fn set_modules(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::pysa_project_module::Owned>) -> ::capnp::Result<()>
fn set_object_class_refs(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::class_ref::Owned>) -> ::capnp::Result<()>
fn set_typing_mapping_class_refs(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::class_ref::Owned>) -> ::capnp::Result<()>
fn set_typing_module_ids(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u32>>) -> ::capnp::Result<()>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
```

**via `capnp::traits::FromPointerBuilder`**

```rust
fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Self>
fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Self
```

**via `capnp::traits::ImbueMut`**

```rust
fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable)
```

**via `core::convert::From`**

```rust
fn from(builder: ::capnp::private::layout::StructBuilder<'a>) -> Self
```

---

## Owned

`struct` · `pyrefly::report::pysa::pysa_report_capnp::project_file::Owned`

```rust
struct Owned
```

**Implements**: `capnp::introspect::Introspect`, `capnp::traits::Owned`, `capnp::traits::OwnedStruct`, `capnp::traits::Pipelined`

**Derives**: Clone, Copy

**via `capnp::introspect::Introspect`**

```rust
fn introspect() -> ::capnp::introspect::Type
```

---

## Pipeline

`struct` · `pyrefly::report::pysa::pysa_report_capnp::project_file::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::project_file::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (14)

```rust
fn get_builtin_module_ids(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a, u32>>
fn get_dict_class_refs(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::class_ref::Owned>>
fn get_modules(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::pysa_project_module::Owned>>
fn get_object_class_refs(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::class_ref::Owned>>
fn get_typing_mapping_class_refs(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::class_ref::Owned>>
fn get_typing_module_ids(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a, u32>>
fn has_builtin_module_ids(&self) -> bool
fn has_dict_class_refs(&self) -> bool
fn has_modules(&self) -> bool
fn has_object_class_refs(&self) -> bool
fn has_typing_mapping_class_refs(&self) -> bool
fn has_typing_module_ids(&self) -> bool
fn reborrow(&self) -> Reader<'_>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
```

**via `capnp::traits::FromPointerReader`**

```rust
fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Self>
```

**via `capnp::traits::Imbue`**

```rust
fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable)
```

**via `capnp::traits::IntoInternalStructReader`**

```rust
fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a>
```

**via `capnp::traits::SetterInput`**

```rust
fn set_pointer_builder(pointer: ::capnp::private::layout::PointerBuilder<'_>, value: Self, canonicalize: bool) -> ::capnp::Result<()>
```

**via `core::convert::From`**

```rust
fn from(reader: ::capnp::private::layout::StructReader<'a>) -> Self
```

---
