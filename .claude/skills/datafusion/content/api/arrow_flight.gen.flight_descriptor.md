# `arrow_flight::gen::flight_descriptor`

Crate `arrow-flight` · 1 public items · structured records in [`model/arrow_flight.gen.flight_descriptor.json`](../model/arrow_flight.gen.flight_descriptor.json)

## DescriptorType

`enum` · `arrow_flight::gen::flight_descriptor::DescriptorType`

Also reachable as `arrow_flight::flight_descriptor::DescriptorType`

```rust
enum DescriptorType
```

**Variants**: `Unknown`, `Path`, `Cmd`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<DescriptorType>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<DescriptorType, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.gen.flight_descriptor.DescriptorType.md).



Describes what type of descriptor is defined.

---
