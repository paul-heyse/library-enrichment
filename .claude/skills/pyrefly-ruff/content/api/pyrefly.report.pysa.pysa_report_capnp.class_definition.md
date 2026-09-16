# `pyrefly::report::pysa::pysa_report_capnp::class_definition`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.class_definition.json`](../model/pyrefly.report.pysa.pysa_report_capnp.class_definition.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_definition::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (42)

```rust
fn get_bases(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>>
fn get_class_id(self) -> u32
fn get_decorator_callees(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::decorator_callee::Owned>>
fn get_fields(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_class_field::Owned>>
fn get_is_dataclass(self) -> bool
fn get_is_named_tuple(self) -> bool
fn get_is_synthesized(self) -> bool
fn get_is_typed_dict(self) -> bool
fn get_mro(self) -> ::capnp::Result<pysa_report_capnp::pysa_class_mro::Builder<'a>>
fn get_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_name_location(self) -> ::capnp::Result<pysa_report_capnp::pysa_location::Builder<'a>>
fn get_parent(self) -> ::capnp::Result<pysa_report_capnp::scope_parent::Builder<'a>>
fn has_bases(&self) -> bool
fn has_decorator_callees(&self) -> bool
fn has_fields(&self) -> bool
fn has_mro(&self) -> bool
fn has_name(&self) -> bool
fn has_name_location(&self) -> bool
fn has_parent(&self) -> bool
fn init_bases(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::class_ref::Owned>
fn init_decorator_callees(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::decorator_callee::Owned>
fn init_fields(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_class_field::Owned>
fn init_mro(self) -> pysa_report_capnp::pysa_class_mro::Builder<'a>
fn init_name(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_name_location(self) -> pysa_report_capnp::pysa_location::Builder<'a>
fn init_parent(self) -> pysa_report_capnp::scope_parent::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_bases(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::class_ref::Owned>) -> ::capnp::Result<()>
fn set_class_id(&mut self, value: u32)
fn set_decorator_callees(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::decorator_callee::Owned>) -> ::capnp::Result<()>
fn set_fields(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::pysa_class_field::Owned>) -> ::capnp::Result<()>
fn set_is_dataclass(&mut self, value: bool)
fn set_is_named_tuple(&mut self, value: bool)
fn set_is_synthesized(&mut self, value: bool)
fn set_is_typed_dict(&mut self, value: bool)
fn set_mro(&mut self, value: pysa_report_capnp::pysa_class_mro::Reader<'_>) -> ::capnp::Result<()>
fn set_name(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_name_location(&mut self, value: pysa_report_capnp::pysa_location::Reader<'_>) -> ::capnp::Result<()>
fn set_parent(&mut self, value: pysa_report_capnp::scope_parent::Reader<'_>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_definition::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_definition::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (3)

```rust
fn get_mro(&self) -> pysa_report_capnp::pysa_class_mro::Pipeline
fn get_name_location(&self) -> pysa_report_capnp::pysa_location::Pipeline
fn get_parent(&self) -> pysa_report_capnp::scope_parent::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_definition::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (21)

```rust
fn get_bases(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::class_ref::Owned>>
fn get_class_id(self) -> u32
fn get_decorator_callees(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::decorator_callee::Owned>>
fn get_fields(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::pysa_class_field::Owned>>
fn get_is_dataclass(self) -> bool
fn get_is_named_tuple(self) -> bool
fn get_is_synthesized(self) -> bool
fn get_is_typed_dict(self) -> bool
fn get_mro(self) -> ::capnp::Result<pysa_report_capnp::pysa_class_mro::Reader<'a>>
fn get_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_name_location(self) -> ::capnp::Result<pysa_report_capnp::pysa_location::Reader<'a>>
fn get_parent(self) -> ::capnp::Result<pysa_report_capnp::scope_parent::Reader<'a>>
fn has_bases(&self) -> bool
fn has_decorator_callees(&self) -> bool
fn has_fields(&self) -> bool
fn has_mro(&self) -> bool
fn has_name(&self) -> bool
fn has_name_location(&self) -> bool
fn has_parent(&self) -> bool
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
