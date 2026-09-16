# `pyrefly::alt::types::class_metadata`

Crate `pyrefly` · 20 public items · structured records in [`model/pyrefly.alt.types.class_metadata.json`](../model/pyrefly.alt.types.class_metadata.json)

## ClassMro

`enum` · `pyrefly::alt::types::class_metadata::ClassMro`

```rust
enum ClassMro
```

**Variants**: `Resolved`, `Cyclic`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn ancestors<'a>(&'a self, stdlib: &'a Stdlib) -> impl Iterator<Item = &'a ClassType>
fn ancestors_no_object(&self) -> &[ClassType]
fn linearization_complete(&self) -> bool
fn new(cls: &Class, bases_with_mro: Vec<(&ClassType, &ClassMro)>, errors: &ErrorCollector) -> Self
fn recursive() -> &'static Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
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

A struct representing a class's ancestors, in method resolution order (MRO)
and after dropping cycles and nonlinearizable inheritance.

Each ancestor is represented as a pair of a class and the type arguments
for that class, relative to the body of the current class, so for example
in
```python
class A[T]: pass
class B[S](A[list[S]]): pass
class C(B[int]): pass
```
we would get `[B[int], A[list[int]]]`.

If a class is present in multiple places of the inheritance tree (and is
linearizable using C3 linearization), it is possible it appears with
different type arguments. The type arguments computed here will always be
those coming from the instance that was selected during linearization.

`linearization_complete` is false when `ancestors` is only a recovery prefix
after nonlinearizable inheritance. Callers that need an exact ancestor list
must check it.

---

## DataclassKind

`enum` · `pyrefly::alt::types::class_metadata::DataclassKind`

```rust
enum DataclassKind
```

**Variants**: `Dataclass`, `Attrs`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn field_specifiers(&self) -> &[CalleeKind]
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

The dataclass flavor. `auto_attribs` is attrs-only; both variants carry the
`field_specifiers` recognized as field declarations (e.g. `attr.ib`).

---

## DjangoRestFrameworkSerializerKind

`enum` · `pyrefly::alt::types::class_metadata::DjangoRestFrameworkSerializerKind`

```rust
enum DjangoRestFrameworkSerializerKind
```

**Variants**: `Serializer`, `ModelSerializer`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## ExplicitSlots

`enum` · `pyrefly::alt::types::class_metadata::ExplicitSlots`

```rust
enum ExplicitSlots
```

**Variants**: `Absent`, `Unknown`, `Known`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn has_explicit_slots(&self) -> bool
fn slots_info(&self) -> Option<&SlotsInfo>
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

Whether a class body declares `__slots__`, and whether the slot names are
known. Dynamic `__slots__` values are present but not usable for slot-name
enforcement or explicit-slot disjoint-base promotion.

---

## ClassDisjointBase

`struct` · `pyrefly::alt::types::class_metadata::ClassDisjointBase`

```rust
struct ClassDisjointBase
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn from_representative(representative: Option<Class>) -> Self
fn recursive() -> &'static Self
fn representative(&self) -> Option<&Class>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
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

A class's disjoint-base representative for PEP 800 narrowing and
downstream inheritance checks.

`representative` is the chosen disjoint-base class, if any: the class
itself when locally disjoint (explicit `@disjoint_base`, non-empty
`__slots__`, or `@dataclass(slots=True)` that materializes fields), or
the inherited representative from a direct base. `None` means no
disjoint-base information, in which case narrowing falls back to
`object`.

---

## ClassMetadata

`struct` · `pyrefly::alt::types::class_metadata::ClassMetadata`

```rust
struct ClassMetadata
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (43)

```rust
fn base_class_objects(&self) -> &[Class]
fn capture_init(&self) -> Option<&[Name]>
fn custom_metaclass(&self) -> Option<&ClassType>
fn dataclass_metadata(&self) -> Option<&DataclassMetadata>
fn dataclass_transform_metadata(&self) -> Option<&DataclassTransformMetadata>
fn deprecation(&self) -> Option<&Deprecation>
fn django_model_metadata(&self) -> Option<&DjangoModelMetadata>
fn enum_metadata(&self) -> Option<&EnumMetadata>
fn extends_abc(&self) -> bool
fn has_base_any(&self) -> bool
fn has_explicit_slots(&self) -> bool
fn has_generic_base_class(&self) -> bool
fn has_local_dataclass_slots_request(&self) -> bool
fn is_attrs_class(&self) -> bool
fn is_django_model(&self) -> bool
fn is_django_rest_framework_model_serializer(&self) -> bool
fn is_django_rest_framework_serializer(&self) -> bool
fn is_enum(&self) -> bool
fn is_explicitly_abstract(&self) -> bool
fn is_factory_boy_factory(&self) -> bool
fn is_final(&self) -> bool
fn is_local_disjoint_base(&self) -> bool
fn is_marshmallow_schema(&self) -> bool
fn is_metaclass(&self) -> bool
fn is_new_type(&self) -> bool
fn is_protocol(&self) -> bool
fn is_pydantic_model(&self) -> bool
fn is_runtime_checkable_protocol(&self) -> bool
fn is_shaped_array(&self) -> bool
fn is_total_ordering(&self) -> bool
fn is_typed_dict(&self) -> bool
fn keywords(&self) -> &[(Name, Type)]
fn metaclass<'a>(&'a self, stdlib: &'a Stdlib) -> &'a ClassType
fn named_tuple_metadata(&self) -> Option<&NamedTupleMetadata>
fn new(bases: Vec<Class>, metaclass: Option<Metaclass>, keywords: Vec<(Name, Type)>, typed_dict_metadata: Option<TypedDictMetadata>, named_tuple_metadata: Option<NamedTupleMetadata>, enum_metadata: Option<EnumMetadata>, protocol_metadata: Option<ProtocolMetadata>, dataclass_metadata: Option<DataclassMetadata>, extends_abc: bool, has_generic_base_class: bool, has_base_any: bool, is_new_type: bool, is_final: bool, deprecation: Option<Deprecation>, is_local_disjoint_base: bool, has_local_dataclass_slots_request: bool, total_ordering_metadata: Option<TotalOrderingMetadata>, dataclass_transform_metadata: Option<DataclassTransformMetadata>, pydantic_model_kind: Option<PydanticModelKind>, is_attrs_class: bool, django_model_metadata: Option<DjangoModelMetadata>, django_rest_framework_serializer_kind: Option<DjangoRestFrameworkSerializerKind>, is_marshmallow_schema: bool, is_factory_boy_factory: bool, is_metaclass: bool, explicit_slots: ExplicitSlots, capture_init: Option<Vec<Name>>, shaped_array: Option<ShapedArrayMetadata>) -> ClassMetadata
fn protocol_metadata(&self) -> Option<&ProtocolMetadata>
fn pydantic_model_kind(&self) -> Option<PydanticModelKind>
fn recursive() -> &'static Self
fn shaped_array_shape(&self) -> Option<&Quantified>
fn slots_info(&self) -> Option<&SlotsInfo>
fn total_ordering_metadata(&self) -> Option<&TotalOrderingMetadata>
fn typed_dict_metadata(&self) -> Option<&TypedDictMetadata>
fn uses_builtin_shaped_array_indexing(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

---

## ClassSynthesizedField

`struct` · `pyrefly::alt::types::class_metadata::ClassSynthesizedField`

```rust
struct ClassSynthesizedField
```

**Fields**: `inner`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(ty: Type) -> Self
fn new_classvar(ty: Type) -> Self
fn new_instance_attribute(ty: Type) -> Self
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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

A field that we synthesize and add to a class. Note that if a non-synthesized field already
exists on the class, it will take precedence over the synthesized field in attribute lookup.
If you want to modify the type of a non-synthesized field, see
AnswersSolver::get_special_class_field_type() in class_field.rs.

---

## ClassSynthesizedFields

`struct` · `pyrefly::alt::types::class_metadata::ClassSynthesizedFields`

```rust
struct ClassSynthesizedFields
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn combine(self, other: Self) -> Self
fn fields(&self) -> impl ExactSizeIterator<Item = (&Name, &ClassSynthesizedField)>
fn get(&self, name: &Name) -> Option<&ClassSynthesizedField>
fn get_index_of(&self, name: &Name) -> Option<usize>
fn new(fields: SmallMap<Name, ClassSynthesizedField>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
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

A class's synthesized fields, such as a dataclass's `__init__` method.

---

## DataclassMetadata

`struct` · `pyrefly::alt::types::class_metadata::DataclassMetadata`

```rust
struct DataclassMetadata
```

**Fields**: `fields`, `pseudo_field_names`, `kws`, `alias_keyword`, `init_defaults`, `default_can_be_positional`, `pydantic_before_validator_fields`, `kind`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn instance_fields(&self) -> impl Iterator<Item = &Name> + '_
fn is_instance_field(&self, name: &Name) -> bool
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## DjangoModelMetadata

`struct` · `pyrefly::alt::types::class_metadata::DjangoModelMetadata`

```rust
struct DjangoModelMetadata
```

**Fields**: `custom_primary_key_field`, `foreign_key_like_fields`, `fields_with_choices`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## DjangoReverseRelationIndex

`struct` · `pyrefly::alt::types::class_metadata::DjangoReverseRelationIndex`

```rust
struct DjangoReverseRelationIndex
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn get(&self, cls: &Class) -> Option<&ClassSynthesizedFields>
fn iter(&self) -> impl ExactSizeIterator<Item = (&Class, &ClassSynthesizedFields)>
fn new(map: SmallMap<Class, ClassSynthesizedFields>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

---

## EnumMetadata

`struct` · `pyrefly::alt::types::class_metadata::EnumMetadata`

```rust
struct EnumMetadata
```

**Fields**: `cls`, `has_value`, `is_django`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## InitDefaults

`struct` · `pyrefly::alt::types::class_metadata::InitDefaults`

```rust
struct InitDefaults
```

**Fields**: `init_by_name`, `init_by_alias`, `alias_generator`, `pydantic_validation_flags`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

Defaults for per-field flags that control `__init__` parameter names.
Pydantic validation options retain `None` so subclasses can distinguish defaults from
inherited configuration.

---

## Metaclass

`struct` · `pyrefly::alt::types::class_metadata::Metaclass`

```rust
struct Metaclass
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn get(&self) -> &ClassType
fn get_mut(&mut self) -> &mut ClassType
fn new(metaclass: ClassType, is_explicitly_abstract: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

A struct representing a class's metaclass.

---

## NamedTupleMetadata

`struct` · `pyrefly::alt::types::class_metadata::NamedTupleMetadata`

```rust
struct NamedTupleMetadata
```

**Fields**: `elements`, `has_dynamic_fields`, `directly_extends_named_tuple`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## ProtocolMetadata

`struct` · `pyrefly::alt::types::class_metadata::ProtocolMetadata`

```rust
struct ProtocolMetadata
```

**Fields**: `members`, `is_runtime_checkable`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## ShapedArrayMetadata

`struct` · `pyrefly::alt::types::class_metadata::ShapedArrayMetadata`

```rust
struct ShapedArrayMetadata
```

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(shape: Quantified, builtin_indexing: bool) -> Self
fn shape(&self) -> &Quantified
fn uses_builtin_indexing(&self) -> bool
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

Metadata for a class registered with `@shaped_array`.

---

## SlotsInfo

`struct` · `pyrefly::alt::types::class_metadata::SlotsInfo`

```rust
struct SlotsInfo
```

**Fields**: `names`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn has_dict(&self) -> bool
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

Slot names declared directly on a class via `__slots__`, when the value is
statically extractable.

---

## TotalOrderingMetadata

`struct` · `pyrefly::alt::types::class_metadata::TotalOrderingMetadata`

```rust
struct TotalOrderingMetadata
```

**Fields**: `location`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## TypedDictMetadata

`struct` · `pyrefly::alt::types::class_metadata::TypedDictMetadata`

```rust
struct TypedDictMetadata
```

**Fields**: `fields`, `extra_items`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---
