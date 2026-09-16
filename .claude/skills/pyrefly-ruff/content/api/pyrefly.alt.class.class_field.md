# `pyrefly::alt::class::class_field`

Crate `pyrefly` · 8 public items · structured records in [`model/pyrefly.alt.class.class_field.json`](../model/pyrefly.alt.class.class_field.json)

## ClassAttribute

`enum` · `pyrefly::alt::class::class_field::ClassAttribute`

```rust
enum ClassAttribute
```

**Variants**: `ReadWrite`, `ReadOnly`, `NoAccess`, `Property`, `Descriptor`, `DescriptorRead`

**Derives**: Clone, Debug

**Methods** (8)

```rust
fn as_instance_method(self) -> Option<Type>
fn descriptor(descriptor: Descriptor, base: DescriptorBase) -> Self
fn is_read_only(&self) -> bool
fn no_access(reason: NoAccessReason) -> Self
fn property(getter: Type, setter: Option<Type>, cls: Class) -> Self
fn read_only(ty: Type, reason: ReadOnlyReason) -> Self
fn read_only_equivalent(self, reason: ReadOnlyReason) -> Self
fn read_write(ty: Type) -> Self
```

The result of looking up an attribute access on a class (either as an instance or a
class access, and possibly through a special case lookup such as a type var with a bound).

---

## ClassFieldInitialization

`enum` · `pyrefly::alt::class::class_field::ClassFieldInitialization`

```rust
enum ClassFieldInitialization
```

**Variants**: `ClassBody`, `Method`, `ClassMethod`, `Uninitialized`, `Magic`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Correctly analyzing which attributes are visible on class objects, as well
as handling method binding correctly, requires distinguishing which fields
are assigned values in the class body.

---

## ClassFieldVariance

`enum` · `pyrefly::alt::class::class_field::ClassFieldVariance`

```rust
enum ClassFieldVariance<'a>
```

**Variants**: `Method`, `Property`, `Field`

---

## DataclassMember

`enum` · `pyrefly::alt::class::class_field::DataclassMember`

```rust
enum DataclassMember<'a>
```

**Variants**: `Field`, `InitVar`, `KwOnlyMarker`, `NotAField`

The result of processing a raw dataclass member (any annotated assignment in its body).

---

## DescriptorBase

`enum` · `pyrefly::alt::class::class_field::DescriptorBase`

```rust
enum DescriptorBase
```

**Variants**: `Instance`, `SelfInstance`, `ClassDef`

**Derives**: Clone, Debug

---

## ClassField

`struct` · `pyrefly::alt::class::class_field::ClassField`

```rust
struct ClassField
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (26)

```rust
fn as_named_tuple_requiredness(&self) -> Required
fn as_named_tuple_type(&self) -> Type
fn as_typed_dict_field_info(&self, required_by_default: bool) -> Option<TypedDictField>
fn can_have_override_decorator(&self) -> bool
fn data_descriptor_info(&self) -> Option<(TextRange, ClassType)>
fn for_variance_inference(&self) -> (&Type, Option<&Annotation>, bool)
fn has_choices(&self) -> bool
fn has_explicit_annotation(&self) -> bool
fn invalid_typed_dict_field(heap: &TypeHeap) -> Self
fn is_abstract(&self) -> bool
fn is_class_var(&self) -> bool
fn is_final(&self) -> bool
fn is_foreign_key(&self) -> bool
fn is_init_var(&self) -> bool
fn is_non_classvar_data_attribute(&self) -> bool
fn is_property(&self) -> bool
fn is_simple_instance_attribute(&self) -> bool
fn is_uninit_class_var(&self) -> bool
fn new_synthesized(ty: Type) -> Self
fn new_synthesized_classvar(ty: Type) -> Self
fn new_synthesized_instance_attribute(ty: Type) -> Self
fn non_data_descriptor_info(&self) -> Option<(TextRange, ClassType)>
fn recursive(heap: &TypeHeap) -> Self
fn ty(&self) -> Type
fn typed_dict_field(ty: Type, annotation: Annotation, read_only_reason: Option<ReadOnlyReason>) -> Self
fn variance_inference(&self) -> ClassFieldVariance<'_>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Raw information about an attribute declared somewhere in a class. We need to
know whether it is initialized in the class body in order to determine
both visibility rules and whether method binding should be performed.

---

## Descriptor

`struct` · `pyrefly::alt::class::class_field::Descriptor`

```rust
struct Descriptor
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

---

## WithDefiningClass

`struct` · `pyrefly::alt::class::class_field::WithDefiningClass`

```rust
struct WithDefiningClass<T>
```

**Fields**: `value`, `defining_class`

**Derives**: Debug

Result of looking up a member of a class in the MRO, including a handle to the defining
class which may be some ancestor.

For example, given `class A: x: int; class B(A): pass`, the defining class
for attribute `x` is `A` even when `x` is looked up on `B`.

---
