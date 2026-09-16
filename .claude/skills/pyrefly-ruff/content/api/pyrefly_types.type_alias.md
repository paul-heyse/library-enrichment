# `pyrefly_types::type_alias`

Crate `pyrefly_types` · 5 public items · structured records in [`model/pyrefly_types.type_alias.json`](../model/pyrefly_types.type_alias.json)

## TypeAliasData

`enum` · `pyrefly_types::type_alias::TypeAliasData`

```rust
enum TypeAliasData
```

**Variants**: `Ref`, `Value`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn name(&self) -> &Name
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

## TypeAliasStyle

`enum` · `pyrefly_types::type_alias::TypeAliasStyle`

```rust
enum TypeAliasStyle
```

**Variants**: `Scoped`, `LegacyExplicit`, `LegacyImplicit`

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

The style of a type alias declaration.

---

## TypeAlias

`struct` · `pyrefly_types::type_alias::TypeAlias`

```rust
struct TypeAlias
```

**Fields**: `name`, `style`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn as_type(&self) -> Type
fn as_type_mut(&mut self) -> &mut Type
fn as_value(&self, stdlib: &Stdlib) -> Type
fn error(name: Name, style: TypeAliasStyle) -> Self
fn fmt_with_type<O: TypeOutput>(&self, output: &mut O, write_type: &impl Fn(&Type, &mut O) -> fmt::Result, tparams: Option<&TParams>) -> fmt::Result
fn new(name: Name, ty: Type, style: TypeAliasStyle) -> Self
fn unknown(name: Name) -> Self
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

A type alias, which may be scoped (PEP 695) or legacy.

---

## TypeAliasIndex

`struct` · `pyrefly_types::type_alias::TypeAliasIndex`

```rust
struct TypeAliasIndex
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
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

The index of a type alias within a file, used to resolve references to recursive type aliases.

---

## TypeAliasRef

`struct` · `pyrefly_types::type_alias::TypeAliasRef`

```rust
struct TypeAliasRef
```

**Fields**: `name`, `args`, `module_name`, `module_path`, `index`

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
