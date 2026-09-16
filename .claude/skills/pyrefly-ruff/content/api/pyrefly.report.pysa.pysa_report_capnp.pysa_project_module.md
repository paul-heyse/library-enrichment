# `pyrefly::report::pysa::pysa_report_capnp::pysa_project_module`

Crate `pyrefly` · 4 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.pysa_project_module.json`](../model/pyrefly.report.pysa.pysa_report_capnp.pysa_project_module.json)

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_project_module::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (40)

```rust
fn get_failed_to_load(self) -> bool
fn get_info_filename(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_is_init(self) -> bool
fn get_is_interface(self) -> bool
fn get_is_internal(self) -> bool
fn get_is_test(self) -> bool
fn get_module_id(self) -> u32
fn get_module_name(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_platform(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_python_version(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_relative_source_path(self) -> ::capnp::Result<::capnp::text::Builder<'a>>
fn get_source_path(self) -> ::capnp::Result<pysa_report_capnp::source_path::Builder<'a>>
fn has_info_filename(&self) -> bool
fn has_module_name(&self) -> bool
fn has_platform(&self) -> bool
fn has_python_version(&self) -> bool
fn has_relative_source_path(&self) -> bool
fn has_source_path(&self) -> bool
fn init_info_filename(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_module_name(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_platform(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_python_version(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_relative_source_path(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_source_path(self) -> pysa_report_capnp::source_path::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_failed_to_load(&mut self, value: bool)
fn set_info_filename(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_is_init(&mut self, value: bool)
fn set_is_interface(&mut self, value: bool)
fn set_is_internal(&mut self, value: bool)
fn set_is_test(&mut self, value: bool)
fn set_module_id(&mut self, value: u32)
fn set_module_name(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_platform(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_python_version(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_relative_source_path(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_project_module::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_project_module::Pipeline`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::pysa_project_module::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (20)

```rust
fn get_failed_to_load(self) -> bool
fn get_info_filename(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_is_init(self) -> bool
fn get_is_interface(self) -> bool
fn get_is_internal(self) -> bool
fn get_is_test(self) -> bool
fn get_module_id(self) -> u32
fn get_module_name(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_platform(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_python_version(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_relative_source_path(self) -> ::capnp::Result<::capnp::text::Reader<'a>>
fn get_source_path(self) -> ::capnp::Result<pysa_report_capnp::source_path::Reader<'a>>
fn has_info_filename(&self) -> bool
fn has_module_name(&self) -> bool
fn has_platform(&self) -> bool
fn has_python_version(&self) -> bool
fn has_relative_source_path(&self) -> bool
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
