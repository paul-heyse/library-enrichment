# `pyrefly::alt::types::decorated_function`

Crate `pyrefly` · 3 public items · structured records in [`model/pyrefly.alt.types.decorated_function.json`](../model/pyrefly.alt.types.decorated_function.json)

## SpecialDecorator

`enum` · `pyrefly::alt::types::decorated_function::SpecialDecorator`

```rust
enum SpecialDecorator<'a>
```

**Variants**: `Overload`, `StaticMethod`, `ClassMethod`, `Property`, `CachedProperty`, `EnumMember`, `Override`, `Final`, `Deprecated`, `PropertySetter`, `PropertyDeleter`, `DataclassTransformCall`, `EnumNonmember`, `AbstractMethod`, `NoTypeCheck`, `UsesShapeDsl`, `DefinesAssertShape`, `DisjointBase`

Decorators that need special handling

---

## Decorator

`struct` · `pyrefly::alt::types::decorated_function::Decorator`

```rust
struct Decorator
```

**Fields**: `ty`, `deprecation`

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

Answer for BindingDecorator

---

## UndecoratedFunction

`struct` · `pyrefly::alt::types::decorated_function::UndecoratedFunction`

```rust
struct UndecoratedFunction
```

**Fields**: `def_index`, `identifier`, `metadata`, `decorators`, `tparams`, `params`, `paramspec`, `defining_cls`, `type_shape_dsl_def`, `resolved_param_types`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn id_range(&self) -> TextRange
fn recursive() -> Self
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

Information about the function def before decorators are applied. The metadata stored here
includes information from decorators, like @classmethod.

---
