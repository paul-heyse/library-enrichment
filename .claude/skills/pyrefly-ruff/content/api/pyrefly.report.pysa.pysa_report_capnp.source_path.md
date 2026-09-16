# `pyrefly::report::pysa::pysa_report_capnp::source_path`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.source_path.json`](../model/pyrefly.report.pysa.pysa_report_capnp.source_path.json)

## Which

`enum` · `pyrefly::report::pysa::pysa_report_capnp::source_path::Which`

```rust
enum Which<A0, A1, A2, A3, A4, A5>
```

**Variants**: `FileSystem`, `Namespace`, `Memory`, `BundledTypeshed`, `BundledTypeshedThirdParty`, `BundledThirdParty`

---

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::source_path::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (23)

```rust
fn has_bundled_third_party(&self) -> bool
fn has_bundled_typeshed(&self) -> bool
fn has_bundled_typeshed_third_party(&self) -> bool
fn has_file_system(&self) -> bool
fn has_memory(&self) -> bool
fn has_namespace(&self) -> bool
fn init_bundled_third_party(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_bundled_typeshed(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_bundled_typeshed_third_party(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_file_system(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_memory(self, size: u32) -> ::capnp::text::Builder<'a>
fn init_namespace(self, size: u32) -> ::capnp::text::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_bundled_third_party(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_bundled_typeshed(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_bundled_typeshed_third_party(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_file_system(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_memory(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn set_namespace(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::text::Owned>)
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<::capnp::text::Builder<'a>>, ::capnp::Result<::capnp::text::Builder<'a>>, ::capnp::Result<::capnp::text::Builder<'a>>, ::capnp::Result<::capnp::text::Builder<'a>>, ::capnp::Result<::capnp::text::Builder<'a>>, ::capnp::Result<::capnp::text::Builder<'a>>>, ::capnp::NotInSchema>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::source_path::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::source_path::Pipeline`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::source_path::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (9)

```rust
fn has_bundled_third_party(&self) -> bool
fn has_bundled_typeshed(&self) -> bool
fn has_bundled_typeshed_third_party(&self) -> bool
fn has_file_system(&self) -> bool
fn has_memory(&self) -> bool
fn has_namespace(&self) -> bool
fn reborrow(&self) -> Reader<'_>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<::capnp::text::Reader<'a>>, ::capnp::Result<::capnp::text::Reader<'a>>, ::capnp::Result<::capnp::text::Reader<'a>>, ::capnp::Result<::capnp::text::Reader<'a>>, ::capnp::Result<::capnp::text::Reader<'a>>, ::capnp::Result<::capnp::text::Reader<'a>>>, ::capnp::NotInSchema>
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
