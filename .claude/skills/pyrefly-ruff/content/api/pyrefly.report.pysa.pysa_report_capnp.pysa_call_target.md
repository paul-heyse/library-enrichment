# `pyrefly::report::pysa::pysa_report_capnp::pysa_call_target`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.pysa_call_target.json`](../model/pyrefly.report.pysa.pysa_report_capnp.pysa_call_target.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_call_target::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (24)

```rust
fn get_implicit_dunder_call(self) -> bool
fn get_implicit_receiver(self) -> ::core::result::Result<pysa_report_capnp::ImplicitReceiver, ::capnp::NotInSchema>
fn get_is_class_method(self) -> bool
fn get_is_static_method(self) -> bool
fn get_receiver_class(self) -> ::capnp::Result<pysa_report_capnp::class_ref::Builder<'a>>
fn get_return_type(self) -> ::capnp::Result<pysa_report_capnp::scalar_type_properties::Builder<'a>>
fn get_target(self) -> ::capnp::Result<pysa_report_capnp::target::Builder<'a>>
fn has_receiver_class(&self) -> bool
fn has_return_type(&self) -> bool
fn has_target(&self) -> bool
fn init_receiver_class(self) -> pysa_report_capnp::class_ref::Builder<'a>
fn init_return_type(self) -> pysa_report_capnp::scalar_type_properties::Builder<'a>
fn init_target(self) -> pysa_report_capnp::target::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_implicit_dunder_call(&mut self, value: bool)
fn set_implicit_receiver(&mut self, value: pysa_report_capnp::ImplicitReceiver)
fn set_is_class_method(&mut self, value: bool)
fn set_is_static_method(&mut self, value: bool)
fn set_receiver_class(&mut self, value: pysa_report_capnp::class_ref::Reader<'_>) -> ::capnp::Result<()>
fn set_return_type(&mut self, value: pysa_report_capnp::scalar_type_properties::Reader<'_>) -> ::capnp::Result<()>
fn set_target(&mut self, value: pysa_report_capnp::target::Reader<'_>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_call_target::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_call_target::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (3)

```rust
fn get_receiver_class(&self) -> pysa_report_capnp::class_ref::Pipeline
fn get_return_type(&self) -> pysa_report_capnp::scalar_type_properties::Pipeline
fn get_target(&self) -> pysa_report_capnp::target::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_call_target::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (12)

```rust
fn get_implicit_dunder_call(self) -> bool
fn get_implicit_receiver(self) -> ::core::result::Result<pysa_report_capnp::ImplicitReceiver, ::capnp::NotInSchema>
fn get_is_class_method(self) -> bool
fn get_is_static_method(self) -> bool
fn get_receiver_class(self) -> ::capnp::Result<pysa_report_capnp::class_ref::Reader<'a>>
fn get_return_type(self) -> ::capnp::Result<pysa_report_capnp::scalar_type_properties::Reader<'a>>
fn get_target(self) -> ::capnp::Result<pysa_report_capnp::target::Reader<'a>>
fn has_receiver_class(&self) -> bool
fn has_return_type(&self) -> bool
fn has_target(&self) -> bool
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
