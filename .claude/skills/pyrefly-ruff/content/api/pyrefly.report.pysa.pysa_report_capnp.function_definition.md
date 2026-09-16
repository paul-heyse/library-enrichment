# `pyrefly::report::pysa::pysa_report_capnp::function_definition`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.function_definition.json`](../model/pyrefly.report.pysa.pysa_report_capnp.function_definition.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_definition::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (54)

```rust
fn get_captured_variables(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::captured_variable_ref::Owned>>
fn get_decorator_callees(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::decorator_callee::Owned>>
fn get_define_name_location(self) -> ::capnp::Result<pysa_report_capnp::pysa_location::Builder<'a>>
fn get_defining_class(self) -> ::capnp::Result<pysa_report_capnp::class_ref::Builder<'a>>
fn get_function_id(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_is_classmethod(self) -> bool
fn get_is_def_statement(self) -> bool
fn get_is_overload(self) -> bool
fn get_is_property_getter(self) -> bool
fn get_is_property_setter(self) -> bool
fn get_is_staticmethod(self) -> bool
fn get_is_stub(self) -> bool
fn get_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_overridden_base_method(self) -> ::capnp::Result<pysa_report_capnp::function_ref::Builder<'a>>
fn get_parent(self) -> ::capnp::Result<pysa_report_capnp::scope_parent::Builder<'a>>
fn get_undecorated_signatures(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::function_signature::Owned>>
fn has_captured_variables(&self) -> bool
fn has_decorator_callees(&self) -> bool
fn has_define_name_location(&self) -> bool
fn has_defining_class(&self) -> bool
fn has_function_id(&self) -> bool
fn has_name(&self) -> bool
fn has_overridden_base_method(&self) -> bool
fn has_parent(&self) -> bool
fn has_undecorated_signatures(&self) -> bool
fn init_captured_variables(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::captured_variable_ref::Owned>
fn init_decorator_callees(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::decorator_callee::Owned>
fn init_define_name_location(self) -> pysa_report_capnp::pysa_location::Builder<'a>
fn init_defining_class(self) -> pysa_report_capnp::class_ref::Builder<'a>
fn init_function_id(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_name(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_overridden_base_method(self) -> pysa_report_capnp::function_ref::Builder<'a>
fn init_parent(self) -> pysa_report_capnp::scope_parent::Builder<'a>
fn init_undecorated_signatures(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::function_signature::Owned>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_captured_variables(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::captured_variable_ref::Owned>) -> ::capnp::Result<()>
fn set_decorator_callees(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::decorator_callee::Owned>) -> ::capnp::Result<()>
fn set_define_name_location(&mut self, value: pysa_report_capnp::pysa_location::Reader<'_>) -> ::capnp::Result<()>
fn set_defining_class(&mut self, value: pysa_report_capnp::class_ref::Reader<'_>) -> ::capnp::Result<()>
fn set_function_id(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_is_classmethod(&mut self, value: bool)
fn set_is_def_statement(&mut self, value: bool)
fn set_is_overload(&mut self, value: bool)
fn set_is_property_getter(&mut self, value: bool)
fn set_is_property_setter(&mut self, value: bool)
fn set_is_staticmethod(&mut self, value: bool)
fn set_is_stub(&mut self, value: bool)
fn set_name(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_overridden_base_method(&mut self, value: pysa_report_capnp::function_ref::Reader<'_>) -> ::capnp::Result<()>
fn set_parent(&mut self, value: pysa_report_capnp::scope_parent::Reader<'_>) -> ::capnp::Result<()>
fn set_undecorated_signatures(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::function_signature::Owned>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_definition::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_definition::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (4)

```rust
fn get_define_name_location(&self) -> pysa_report_capnp::pysa_location::Pipeline
fn get_defining_class(&self) -> pysa_report_capnp::class_ref::Pipeline
fn get_overridden_base_method(&self) -> pysa_report_capnp::function_ref::Pipeline
fn get_parent(&self) -> pysa_report_capnp::scope_parent::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_definition::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (27)

```rust
fn get_captured_variables(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::captured_variable_ref::Owned>>
fn get_decorator_callees(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::decorator_callee::Owned>>
fn get_define_name_location(self) -> ::capnp::Result<pysa_report_capnp::pysa_location::Reader<'a>>
fn get_defining_class(self) -> ::capnp::Result<pysa_report_capnp::class_ref::Reader<'a>>
fn get_function_id(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_is_classmethod(self) -> bool
fn get_is_def_statement(self) -> bool
fn get_is_overload(self) -> bool
fn get_is_property_getter(self) -> bool
fn get_is_property_setter(self) -> bool
fn get_is_staticmethod(self) -> bool
fn get_is_stub(self) -> bool
fn get_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_overridden_base_method(self) -> ::capnp::Result<pysa_report_capnp::function_ref::Reader<'a>>
fn get_parent(self) -> ::capnp::Result<pysa_report_capnp::scope_parent::Reader<'a>>
fn get_undecorated_signatures(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::function_signature::Owned>>
fn has_captured_variables(&self) -> bool
fn has_decorator_callees(&self) -> bool
fn has_define_name_location(&self) -> bool
fn has_defining_class(&self) -> bool
fn has_function_id(&self) -> bool
fn has_name(&self) -> bool
fn has_overridden_base_method(&self) -> bool
fn has_parent(&self) -> bool
fn has_undecorated_signatures(&self) -> bool
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
