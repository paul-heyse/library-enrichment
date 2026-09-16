# `pyrefly::report::pysa::pysa_report_capnp::type_error`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.type_error.json`](../model/pyrefly.report.pysa.pysa_report_capnp.type_error.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::type_error::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (24)

```rust
fn get_kind(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_location(self) -> ::capnp::Result<pysa_report_capnp::pysa_location::Builder<'a>>
fn get_message(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_module_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_module_path(self) -> ::capnp::Result<pysa_report_capnp::source_path::Builder<'a>>
fn has_kind(&self) -> bool
fn has_location(&self) -> bool
fn has_message(&self) -> bool
fn has_module_name(&self) -> bool
fn has_module_path(&self) -> bool
fn init_kind(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_location(self) -> pysa_report_capnp::pysa_location::Builder<'a>
fn init_message(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_module_name(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_module_path(self) -> pysa_report_capnp::source_path::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_kind(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_location(&mut self, value: pysa_report_capnp::pysa_location::Reader<'_>) -> ::capnp::Result<()>
fn set_message(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_module_name(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_module_path(&mut self, value: pysa_report_capnp::source_path::Reader<'_>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::type_error::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::type_error::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (2)

```rust
fn get_location(&self) -> pysa_report_capnp::pysa_location::Pipeline
fn get_module_path(&self) -> pysa_report_capnp::source_path::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::type_error::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (12)

```rust
fn get_kind(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_location(self) -> ::capnp::Result<pysa_report_capnp::pysa_location::Reader<'a>>
fn get_message(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_module_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_module_path(self) -> ::capnp::Result<pysa_report_capnp::source_path::Reader<'a>>
fn has_kind(&self) -> bool
fn has_location(&self) -> bool
fn has_message(&self) -> bool
fn has_module_name(&self) -> bool
fn has_module_path(&self) -> bool
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
