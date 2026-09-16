# `pyrefly::report::pysa::pysa_report_capnp::expression_callees`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.expression_callees.json`](../model/pyrefly.report.pysa.pysa_report_capnp.expression_callees.json)

## Which

`enum` · `pyrefly::report::pysa::pysa_report_capnp::expression_callees::Which`

```rust
enum Which<A0, A1, A2, A3, A4, A5, A6>
```

**Variants**: `Call`, `Identifier`, `AttributeAccess`, `Define`, `FormatStringArtificial`, `FormatStringStringify`, `Return`

---

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::expression_callees::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (26)

```rust
fn has_attribute_access(&self) -> bool
fn has_call(&self) -> bool
fn has_define(&self) -> bool
fn has_format_string_artificial(&self) -> bool
fn has_format_string_stringify(&self) -> bool
fn has_identifier(&self) -> bool
fn has_return(&self) -> bool
fn init_attribute_access(self) -> pysa_report_capnp::attribute_access_callees::Builder<'a>
fn init_call(self) -> pysa_report_capnp::call_callees::Builder<'a>
fn init_define(self) -> pysa_report_capnp::define_callees::Builder<'a>
fn init_format_string_artificial(self) -> pysa_report_capnp::format_string_artificial_callees::Builder<'a>
fn init_format_string_stringify(self) -> pysa_report_capnp::format_string_stringify_callees::Builder<'a>
fn init_identifier(self) -> pysa_report_capnp::identifier_callees::Builder<'a>
fn init_return(self) -> pysa_report_capnp::return_shim_callees::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_attribute_access(&mut self, value: pysa_report_capnp::attribute_access_callees::Reader<'_>) -> ::capnp::Result<()>
fn set_call(&mut self, value: pysa_report_capnp::call_callees::Reader<'_>) -> ::capnp::Result<()>
fn set_define(&mut self, value: pysa_report_capnp::define_callees::Reader<'_>) -> ::capnp::Result<()>
fn set_format_string_artificial(&mut self, value: pysa_report_capnp::format_string_artificial_callees::Reader<'_>) -> ::capnp::Result<()>
fn set_format_string_stringify(&mut self, value: pysa_report_capnp::format_string_stringify_callees::Reader<'_>) -> ::capnp::Result<()>
fn set_identifier(&mut self, value: pysa_report_capnp::identifier_callees::Reader<'_>) -> ::capnp::Result<()>
fn set_return(&mut self, value: pysa_report_capnp::return_shim_callees::Reader<'_>) -> ::capnp::Result<()>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<pysa_report_capnp::call_callees::Builder<'a>>, ::capnp::Result<pysa_report_capnp::identifier_callees::Builder<'a>>, ::capnp::Result<pysa_report_capnp::attribute_access_callees::Builder<'a>>, ::capnp::Result<pysa_report_capnp::define_callees::Builder<'a>>, ::capnp::Result<pysa_report_capnp::format_string_artificial_callees::Builder<'a>>, ::capnp::Result<pysa_report_capnp::format_string_stringify_callees::Builder<'a>>, ::capnp::Result<pysa_report_capnp::return_shim_callees::Builder<'a>>>, ::capnp::NotInSchema>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::expression_callees::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::expression_callees::Pipeline`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::expression_callees::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (10)

```rust
fn has_attribute_access(&self) -> bool
fn has_call(&self) -> bool
fn has_define(&self) -> bool
fn has_format_string_artificial(&self) -> bool
fn has_format_string_stringify(&self) -> bool
fn has_identifier(&self) -> bool
fn has_return(&self) -> bool
fn reborrow(&self) -> Reader<'_>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<pysa_report_capnp::call_callees::Reader<'a>>, ::capnp::Result<pysa_report_capnp::identifier_callees::Reader<'a>>, ::capnp::Result<pysa_report_capnp::attribute_access_callees::Reader<'a>>, ::capnp::Result<pysa_report_capnp::define_callees::Reader<'a>>, ::capnp::Result<pysa_report_capnp::format_string_artificial_callees::Reader<'a>>, ::capnp::Result<pysa_report_capnp::format_string_stringify_callees::Reader<'a>>, ::capnp::Result<pysa_report_capnp::return_shim_callees::Reader<'a>>>, ::capnp::NotInSchema>
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
