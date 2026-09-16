# `pyrefly_types::tuple`

Crate `pyrefly_types` · 2 public items · structured records in [`model/pyrefly_types.tuple.json`](../model/pyrefly_types.tuple.json)

## Tuple

`enum` · `pyrefly_types::tuple::Tuple`

```rust
enum Tuple
```

**Variants**: `Concrete`, `Unbounded`, `Unpacked`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn fmt_with_type<O: TypeOutput>(&self, output: &mut O, tuple_qname: Option<&QName>, write_type: &impl Fn(&Type, &mut O) -> fmt::Result) -> fmt::Result
fn is_any_tuple(&self) -> bool
fn unpacked(prefix: Vec<Type>, middle: Type, suffix: Vec<Type>) -> Tuple
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

## UnpackedTupleParts

`struct` · `pyrefly_types::tuple::UnpackedTupleParts`

```rust
struct UnpackedTupleParts
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
fn into_parts(self) -> (Vec<Type>, Type, Vec<Type>)
fn middle(&self) -> &Type
fn parts(&self) -> (&[Type], &Type, &[Type])
fn prefix(&self) -> &[Type]
fn suffix(&self) -> &[Type]
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
