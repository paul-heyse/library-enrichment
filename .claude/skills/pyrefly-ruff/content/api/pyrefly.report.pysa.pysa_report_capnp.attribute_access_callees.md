# `pyrefly::report::pysa::pysa_report_capnp::attribute_access_callees`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.attribute_access_callees.json`](../model/pyrefly.report.pysa.pysa_report_capnp.attribute_access_callees.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::attribute_access_callees::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (22)

```rust
fn get_global_targets(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::global_variable_ref::Owned>>
fn get_if_called(self) -> ::capnp::Result<pysa_report_capnp::call_callees::Builder<'a>>
fn get_is_attribute(self) -> bool
fn get_property_getters(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_call_target::Owned>>
fn get_property_setters(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_call_target::Owned>>
fn has_global_targets(&self) -> bool
fn has_if_called(&self) -> bool
fn has_property_getters(&self) -> bool
fn has_property_setters(&self) -> bool
fn init_global_targets(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::global_variable_ref::Owned>
fn init_if_called(self) -> pysa_report_capnp::call_callees::Builder<'a>
fn init_property_getters(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_call_target::Owned>
fn init_property_setters(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_call_target::Owned>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_global_targets(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::global_variable_ref::Owned>) -> ::capnp::Result<()>
fn set_if_called(&mut self, value: pysa_report_capnp::call_callees::Reader<'_>) -> ::capnp::Result<()>
fn set_is_attribute(&mut self, value: bool)
fn set_property_getters(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::pysa_call_target::Owned>) -> ::capnp::Result<()>
fn set_property_setters(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::pysa_call_target::Owned>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::attribute_access_callees::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::attribute_access_callees::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (1)

```rust
fn get_if_called(&self) -> pysa_report_capnp::call_callees::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::attribute_access_callees::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (11)

```rust
fn get_global_targets(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::global_variable_ref::Owned>>
fn get_if_called(self) -> ::capnp::Result<pysa_report_capnp::call_callees::Reader<'a>>
fn get_is_attribute(self) -> bool
fn get_property_getters(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::pysa_call_target::Owned>>
fn get_property_setters(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::pysa_call_target::Owned>>
fn has_global_targets(&self) -> bool
fn has_if_called(&self) -> bool
fn has_property_getters(&self) -> bool
fn has_property_setters(&self) -> bool
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
