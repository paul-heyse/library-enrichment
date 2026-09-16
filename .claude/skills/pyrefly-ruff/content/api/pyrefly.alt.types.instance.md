# `pyrefly::alt::types::instance`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.types.instance.json`](../model/pyrefly.alt.types.instance.json)

## InstanceKind

`enum` · `pyrefly::alt::types::instance::InstanceKind`

```rust
enum InstanceKind
```

**Variants**: `ClassType`, `TypedDict`, `TypeVar`, `SelfType`, `Protocol`, `Metaclass`, `LiteralString`, `ShapedArray`

**Derives**: Debug

---

## Instance

`struct` · `pyrefly::alt::types::instance::Instance`

```rust
struct Instance<'a>
```

**Fields**: `kind`, `class`, `targs`

**Derives**: Debug

**Methods** (12)

```rust
fn instantiate_member(&self, raw_member: &mut Type)
fn literal_string(stdlib: &'a Stdlib) -> Self
fn of_class(cls: &'a ClassType) -> Self
fn of_metaclass(cls: ClassBase, metaclass: &'a ClassType) -> Self
fn of_protocol(cls: &'a ClassType, self_type: Type) -> Self
fn of_self_type(cls: &'a ClassType) -> Self
fn of_shaped_array(shaped_array: &'a ShapedArrayType) -> Self
fn of_type_var(q: Quantified, bound: &'a ClassType) -> Self
fn of_typed_dict(td: &'a TypedDictInner) -> Self
fn to_class_base(&self) -> ClassBase
fn to_descriptor_base(&self) -> Option<DescriptorBase>
fn to_type(&self, heap: &TypeHeap) -> Type
```

Wrapper to hold a specialized instance of a class , unifying ClassType and TypedDict.

---
