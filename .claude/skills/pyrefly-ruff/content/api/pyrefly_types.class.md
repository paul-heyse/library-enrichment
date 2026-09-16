# `pyrefly_types::class`

Crate `pyrefly_types` · 9 public items · structured records in [`model/pyrefly_types.class.json`](../model/pyrefly_types.class.json)

## AttrsFieldSpecifierKind

`enum` · `pyrefly_types::class::AttrsFieldSpecifierKind`

```rust
enum AttrsFieldSpecifierKind
```

**Variants**: `Attrib`, `Field`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

`attr.ib`/`attrib` honor a `type=` argument and accept a positional default; `field` neither.

---

## ClassKind

`enum` · `pyrefly_types::class::ClassKind`

```rust
enum ClassKind
```

**Variants**: `StaticMethod`, `ClassMethod`, `Property`, `CachedProperty`, `Class`, `EnumMember`, `EnumNonmember`, `DataclassField`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

## PrecomputedTParams

`enum` · `pyrefly_types::class::PrecomputedTParams`

```rust
enum PrecomputedTParams
```

**Variants**: `NotGeneric`, `FromBinding`, `Precomputed`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

What is known about a class's type parameters after the binding phase.

---

## AttrsFieldSpecifier

`struct` · `pyrefly_types::class::AttrsFieldSpecifier`

```rust
struct AttrsFieldSpecifier
```

**Fields**: `kind`, `default_is_nothing`, `default_decorator_method_range`, `converter_decorator_method_range`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Bundling these with the specifier keeps them unrepresentable without one.

---

## Class

`struct` · `pyrefly_types::class::Class`

```rust
struct Class
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (14)

```rust
fn has_qname(&self, module: &str, parent: &NestingContext, name: &str) -> bool
fn has_toplevel_qname(&self, module: &str, name: &str) -> bool
fn index(&self) -> ClassDefIndex
fn is_builtin(&self, name: &str) -> bool
fn is_protocol(&self) -> bool
fn kind(&self) -> ClassKind
fn module(&self) -> &Module
fn module_name(&self) -> ModuleName
fn module_path(&self) -> &ModulePath
fn name(&self) -> &Name
fn new(def_index: ClassDefIndex, name: Identifier, parent: NestingContext, module: Module, precomputed_tparams: PrecomputedTParams, is_protocol: bool) -> Self
fn precomputed_tparams(&self) -> &PrecomputedTParams
fn qname(&self) -> &QName
fn range(&self) -> TextRange
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

The name of a nominal type, e.g. `str`

---

## ClassDefIndex

`struct` · `pyrefly_types::class::ClassDefIndex`

```rust
struct ClassDefIndex
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

The index of a class within the file, used as a reference to data associated with the class.

---

## ClassFieldProperties

`struct` · `pyrefly_types::class::ClassFieldProperties`

```rust
struct ClassFieldProperties
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn docstring_range(&self) -> Option<TextRange>
fn is_defined_in_class_body(&self) -> bool
fn is_initialized_on_class(&self) -> bool
fn new(is_annotated: bool, has_default_value: bool, is_defined_in_class_body: bool, attrs_field_specifier: Option<AttrsFieldSpecifier>, range: TextRange, docstring_range: Option<TextRange>) -> Self
```

Simple properties of class fields that can be attached to the class definition. Note that this
does not include the type of a field, which needs to be computed lazily to avoid a recursive loop.

---

## ClassFields

`struct` · `pyrefly_types::class::ClassFields`

```rust
struct ClassFields
```

**Derives**: Clone, Debug, Default

**Methods** (20)

```rust
fn attrs_converter_decorator_method_range(&self, name: &Name) -> Option<TextRange>
fn attrs_default_decorator_method_range(&self, name: &Name) -> Option<TextRange>
fn attrs_specifier_honors_type(&self, name: &Name) -> bool
fn class_body_fields(&self) -> impl Iterator<Item = &Name>
fn contains(&self, name: &Name) -> bool
fn default_is_attrs_decorator(&self, name: &Name) -> bool
fn default_is_attrs_nothing(&self, name: &Name) -> bool
fn empty() -> Self
fn field_decl_range(&self, name: &Name) -> Option<TextRange>
fn field_docstring_range(&self, name: &Name) -> Option<TextRange>
fn fields(&self) -> impl ExactSizeIterator<Item = &Name>
fn get_index_of(&self, name: &Name) -> Option<usize>
fn is_attrs_field_specifier(&self, name: &Name) -> bool
fn is_empty(&self) -> bool
fn is_field_annotated(&self, name: &Name) -> bool
fn is_field_initialized_on_class(&self, name: &Name) -> bool
fn iter(&self) -> impl ExactSizeIterator<Item = (&Name, &ClassFieldProperties)>
fn len(&self) -> usize
fn names(&self) -> impl ExactSizeIterator<Item = &Name>
fn new(fields: SmallMap<Name, ClassFieldProperties>) -> Self
```

The set of fields declared on a class, with their properties.

---

## ClassType

`struct` · `pyrefly_types::class::ClassType`

```rust
struct ClassType
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (13)

```rust
fn class_object(&self) -> &Class
fn has_qname(&self, module: &str, name: &str) -> bool
fn into_class_object(self) -> Class
fn is_builtin(&self, name: &str) -> bool
fn name(&self) -> &Name
fn new(class: Class, targs: TArgs) -> Self
fn qname(&self) -> &QName
fn substitute_with(&self, substitution: &Substitution<'_>) -> Self
fn substitution(&self) -> Substitution<'_>
fn targs(&self) -> &TArgs
fn targs_mut(&mut self) -> &mut TArgs
fn to_type(self) -> Type
fn tparams(&self) -> &TParams
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

---
