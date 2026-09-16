# `pyrefly::report::pysa::pysa_report_capnp::target`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.target.json`](../model/pyrefly.report.pysa.pysa_report_capnp.target.json)

## Which

`enum` · `pyrefly::report::pysa::pysa_report_capnp::target::Which`

```rust
enum Which<A0, A1>
```

**Variants**: `Function`, `Overrides`, `FormatString`

---

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::target::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (12)

```rust
fn has_function(&self) -> bool
fn has_overrides(&self) -> bool
fn init_function(self) -> pysa_report_capnp::function_ref::Builder<'a>
fn init_overrides(self) -> pysa_report_capnp::function_ref::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_format_string(&mut self, _value: ())
fn set_function(&mut self, value: pysa_report_capnp::function_ref::Reader<'_>) -> ::capnp::Result<()>
fn set_overrides(&mut self, value: pysa_report_capnp::function_ref::Reader<'_>) -> ::capnp::Result<()>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<pysa_report_capnp::function_ref::Builder<'a>>, ::capnp::Result<pysa_report_capnp::function_ref::Builder<'a>>>, ::capnp::NotInSchema>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::target::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::target::Pipeline`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::target::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (5)

```rust
fn has_function(&self) -> bool
fn has_overrides(&self) -> bool
fn reborrow(&self) -> Reader<'_>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<pysa_report_capnp::function_ref::Reader<'a>>, ::capnp::Result<pysa_report_capnp::function_ref::Reader<'a>>>, ::capnp::NotInSchema>
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
