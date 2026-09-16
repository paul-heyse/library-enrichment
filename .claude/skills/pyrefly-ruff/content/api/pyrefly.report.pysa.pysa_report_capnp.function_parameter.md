# `pyrefly::report::pysa::pysa_report_capnp::function_parameter`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.function_parameter.json`](../model/pyrefly.report.pysa.pysa_report_capnp.function_parameter.json)

## Which

`enum` · `pyrefly::report::pysa::pysa_report_capnp::function_parameter::Which`

```rust
enum Which<A0, A1, A2, A3, A4>
```

**Variants**: `PosOnly`, `Pos`, `VarArg`, `KwOnly`, `Kwargs`

---

## Builder

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_parameter::Builder`

```rust
struct Builder<'a>
```

**Implements**: `capnp::traits::FromPointerBuilder`, `capnp::traits::HasStructSize`, `capnp::traits::HasTypeId`, `capnp::traits::ImbueMut`, `core::convert::From`

**Methods** (20)

```rust
fn has_kw_only(&self) -> bool
fn has_kwargs(&self) -> bool
fn has_pos(&self) -> bool
fn has_pos_only(&self) -> bool
fn has_var_arg(&self) -> bool
fn init_kw_only(self) -> pysa_report_capnp::function_parameter::kw_only_param::Builder<'a>
fn init_kwargs(self) -> pysa_report_capnp::function_parameter::kwargs_param::Builder<'a>
fn init_pos(self) -> pysa_report_capnp::function_parameter::pos_param::Builder<'a>
fn init_pos_only(self) -> pysa_report_capnp::function_parameter::pos_only_param::Builder<'a>
fn init_var_arg(self) -> pysa_report_capnp::function_parameter::var_arg_param::Builder<'a>
fn into_reader(self) -> Reader<'a>
fn reborrow(&mut self) -> Builder<'_>
fn reborrow_as_reader(&self) -> Reader<'_>
fn set_kw_only(&mut self, value: pysa_report_capnp::function_parameter::kw_only_param::Reader<'_>) -> ::capnp::Result<()>
fn set_kwargs(&mut self, value: pysa_report_capnp::function_parameter::kwargs_param::Reader<'_>) -> ::capnp::Result<()>
fn set_pos(&mut self, value: pysa_report_capnp::function_parameter::pos_param::Reader<'_>) -> ::capnp::Result<()>
fn set_pos_only(&mut self, value: pysa_report_capnp::function_parameter::pos_only_param::Reader<'_>) -> ::capnp::Result<()>
fn set_var_arg(&mut self, value: pysa_report_capnp::function_parameter::var_arg_param::Reader<'_>) -> ::capnp::Result<()>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<pysa_report_capnp::function_parameter::pos_only_param::Builder<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::pos_param::Builder<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::var_arg_param::Builder<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::kw_only_param::Builder<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::kwargs_param::Builder<'a>>>, ::capnp::NotInSchema>
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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_parameter::Owned`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_parameter::Pipeline`

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

`struct` · `pyrefly::report::pysa::pysa_report_capnp::function_parameter::Reader`

```rust
struct Reader<'a>
```

**Implements**: `capnp::traits::FromPointerReader`, `capnp::traits::HasTypeId`, `capnp::traits::Imbue`, `capnp::traits::IntoInternalStructReader`, `capnp::traits::SetterInput`, `core::convert::From`

**Derives**: Clone, Copy, Debug

**Methods** (8)

```rust
fn has_kw_only(&self) -> bool
fn has_kwargs(&self) -> bool
fn has_pos(&self) -> bool
fn has_pos_only(&self) -> bool
fn has_var_arg(&self) -> bool
fn reborrow(&self) -> Reader<'_>
fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize>
fn which(self) -> ::core::result::Result<Which<::capnp::Result<pysa_report_capnp::function_parameter::pos_only_param::Reader<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::pos_param::Reader<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::var_arg_param::Reader<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::kw_only_param::Reader<'a>>, ::capnp::Result<pysa_report_capnp::function_parameter::kwargs_param::Reader<'a>>>, ::capnp::NotInSchema>
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
