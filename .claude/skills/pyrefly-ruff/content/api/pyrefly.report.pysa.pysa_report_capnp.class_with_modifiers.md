# `pyrefly::report::pysa::pysa_report_capnp::class_with_modifiers`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.class_with_modifiers.json`](../model/pyrefly.report.pysa.pysa_report_capnp.class_with_modifiers.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_with_modifiers::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (12)

```rust
fn get_class(self) -> ::capnp::Result<pysa_report_capnp::class_ref::Builder<'a>>
fn get_modifiers(self) -> ::capnp::Result<::capnp::enum_list::Builder<'a, pysa_report_capnp::TypeModifier>>
fn has_class(&self) -> bool
fn has_modifiers(&self) -> bool
fn init_class(self) -> pysa_report_capnp::class_ref::Builder<'a>
fn init_modifiers(self, size: u32) -> ::capnp::enum_list::Builder<'a, pysa_report_capnp::TypeModifier>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_class(&mut self, value: pysa_report_capnp::class_ref::Reader<'_>) -> ::capnp::Result<()>
fn set_modifiers(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::enum_list::Owned<pysa_report_capnp::TypeModifier>>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_with_modifiers::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_with_modifiers::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (1)

```rust
fn get_class(&self) -> pysa_report_capnp::class_ref::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::class_with_modifiers::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (6)

```rust
fn get_class(self) -> ::capnp::Result<pysa_report_capnp::class_ref::Reader<'a>>
fn get_modifiers(self) -> ::capnp::Result<::capnp::enum_list::Reader<'a, pysa_report_capnp::TypeModifier>>
fn has_class(&self) -> bool
fn has_modifiers(&self) -> bool
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
