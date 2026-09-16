# `pyrefly_types::typed_dict`

Crate `pyrefly_types` · 7 public items · structured records in [`model/pyrefly_types.typed_dict.json`](../model/pyrefly_types.typed_dict.json)

## ExtraItems

`enum` · `pyrefly_types::typed_dict::ExtraItems`

```rust
enum ExtraItems
```

**Variants**: `Default`, `Closed`, `Extra`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn extra(ty: Type, qualifiers: &[Qualifier]) -> Self
fn extra_item(&self, stdlib: &Stdlib) -> ExtraItem
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

How does the TypedDict handle extra items? See https://peps.python.org/pep-0728.

---

## TypedDict

`enum` · `pyrefly_types::typed_dict::TypedDict`

```rust
enum TypedDict
```

**Variants**: `TypedDict`, `Anonymous`

**Implements**: `pyrefly::alt::class::typed_dict::TypedDictErrorKind`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn is_anonymous(&self) -> bool
fn label(&self) -> String
fn name(&self) -> &Name
fn new(class: Class, args: TArgs) -> Self
fn to_type(self, heap: &TypeHeap) -> Type
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

## ANONYMOUS_TYPED_DICT

`static` · `pyrefly_types::typed_dict::ANONYMOUS_TYPED_DICT`

```rust
static ANONYMOUS_TYPED_DICT: std::sync::LazyLock<ruff_python_ast::name::Name>
```

---

## AnonymousTypedDictInner

`struct` · `pyrefly_types::typed_dict::AnonymousTypedDictInner`

```rust
struct AnonymousTypedDictInner
```

**Fields**: `fields`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn compute_value_type(&self, heap: &TypeHeap) -> Type
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

## ExtraItem

`struct` · `pyrefly_types::typed_dict::ExtraItem`

```rust
struct ExtraItem
```

**Fields**: `ty`, `read_only`

**Implements**: `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

---

## TypedDictField

`struct` · `pyrefly_types::typed_dict::TypedDictField`

```rust
struct TypedDictField
```

**Fields**: `ty`, `required`, `read_only_reason`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn is_read_only(&self) -> bool
fn substitute_with(self, substitution: &Substitution<'_>) -> Self
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

## TypedDictInner

`struct` · `pyrefly_types::typed_dict::TypedDictInner`

```rust
struct TypedDictInner
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn class_object(&self) -> &Class
fn name(&self) -> &Name
fn qname(&self) -> &QName
fn targs(&self) -> &TArgs
fn targs_mut(&mut self) -> &mut TArgs
fn to_type(self, heap: &TypeHeap) -> Type
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
