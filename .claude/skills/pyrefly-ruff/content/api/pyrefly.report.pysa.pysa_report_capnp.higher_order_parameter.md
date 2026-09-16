# `pyrefly::report::pysa::pysa_report_capnp::higher_order_parameter`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.higher_order_parameter.json`](../model/pyrefly.report.pysa.pysa_report_capnp.higher_order_parameter.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::higher_order_parameter::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (14)

```rust
fn get_call_targets(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_call_target::Owned>>
fn get_index(self) -> u32
fn get_unresolved(self) -> ::capnp::Result<pysa_report_capnp::unresolved::Builder<'a>>
fn has_call_targets(&self) -> bool
fn has_unresolved(&self) -> bool
fn init_call_targets(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::pysa_call_target::Owned>
fn init_unresolved(self) -> pysa_report_capnp::unresolved::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_call_targets(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::pysa_call_target::Owned>) -> ::capnp::Result<()>
fn set_index(&mut self, value: u32)
fn set_unresolved(&mut self, value: pysa_report_capnp::unresolved::Reader<'_>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::higher_order_parameter::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::higher_order_parameter::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (1)

```rust
fn get_unresolved(&self) -> pysa_report_capnp::unresolved::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::higher_order_parameter::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (7)

```rust
fn get_call_targets(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::pysa_call_target::Owned>>
fn get_index(self) -> u32
fn get_unresolved(self) -> ::capnp::Result<pysa_report_capnp::unresolved::Reader<'a>>
fn has_call_targets(&self) -> bool
fn has_unresolved(&self) -> bool
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
