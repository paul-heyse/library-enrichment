# `pyrefly::report::pysa::pysa_report_capnp::module_call_graphs`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.module_call_graphs.json`](../model/pyrefly.report.pysa.pysa_report_capnp.module_call_graphs.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::module_call_graphs::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (18)

```rust
fn get_call_graphs(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a, pysa_report_capnp::function_call_graph::Owned>>
fn get_module_id(self) -> u32
fn get_module_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_source_path(self) -> ::capnp::Result<pysa_report_capnp::source_path::Builder<'a>>
fn has_call_graphs(&self) -> bool
fn has_module_name(&self) -> bool
fn has_source_path(&self) -> bool
fn init_call_graphs(self, size: u32) -> ::capnp::struct_list::Builder<'a, pysa_report_capnp::function_call_graph::Owned>
fn init_module_name(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_source_path(self) -> pysa_report_capnp::source_path::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_call_graphs(&mut self, value: ::capnp::struct_list::Reader<'_, pysa_report_capnp::function_call_graph::Owned>) -> ::capnp::Result<()>
fn set_module_id(&mut self, value: u32)
fn set_module_name(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_source_path(&mut self, value: pysa_report_capnp::source_path::Reader<'_>) -> ::capnp::Result<()>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::module_call_graphs::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::module_call_graphs::Pipeline`

```rust
struct Pipeline
```

**Implements**: `capnp::capability::FromTypelessPipeline`

**Methods** (1)

```rust
fn get_source_path(&self) -> pysa_report_capnp::source_path::Pipeline
```

**via `capnp::capability::FromTypelessPipeline`**

```rust
fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self
```

---

## Reader

`struct` · `pyrefly::report::pysa::pysa_report_capnp::module_call_graphs::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (9)

```rust
fn get_call_graphs(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a, pysa_report_capnp::function_call_graph::Owned>>
fn get_module_id(self) -> u32
fn get_module_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_source_path(self) -> ::capnp::Result<pysa_report_capnp::source_path::Reader<'a>>
fn has_call_graphs(&self) -> bool
fn has_module_name(&self) -> bool
fn has_source_path(&self) -> bool
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
