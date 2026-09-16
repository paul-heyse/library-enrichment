# `pyrefly_types::type_info`

Crate `pyrefly_types` · 2 public items · structured records in [`model/pyrefly_types.type_info.json`](../model/pyrefly_types.type_info.json)

## JoinStyle

`enum` · `pyrefly_types::type_info::JoinStyle`

```rust
enum JoinStyle<T>
```

**Variants**: `SimpleMerge`, `ReassignmentOf`, `NarrowOf`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn map<S>(&self, f: impl FnOnce(&T) -> S) -> JoinStyle<S>
```

The style of a phi.

When present, the base may be used to simplify the result and to
eliminate narrows that we don't want included (e.g. any narrow of `Any`
should be dropped in flow merging).

---

## TypeInfo

`struct` · `pyrefly_types::type_info::TypeInfo`

```rust
struct TypeInfo
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (18)

```rust
fn arc_clone(Arc<self>) -> Self
fn arc_clone_ty(Arc<self>) -> Type
fn at_facet(&self, facet: &FacetKind, fallback: impl Fn() -> Type) -> Self
fn has_facets(&self) -> bool
fn has_value_less_presence(&self, facet: &FacetKind) -> bool
fn into_ty(self) -> Type
fn invalidate_all_indexes_for_assignment(&mut self, facets: &[FacetKind])
fn join(branches: Vec<Self>, union_types: &impl Fn(Vec<Type>) -> Type, is_subset_eq: &impl Fn(&Type, &Type) -> bool, join_style: JoinStyle<&TypeInfo>) -> Self
fn key_facets_at(&self, prefix: &[FacetKind]) -> Vec<(String, Option<Type>)>
fn map_ty(self, f: impl FnOnce(Type) -> Type) -> Self
fn of_ty(ty: Type) -> Self
fn record_key_completion(&mut self, facets: &Vec1<FacetKind>, ty: Option<Type>)
fn record_present(&mut self, facets: &Vec1<FacetKind>)
fn ty(&self) -> &Type
fn type_at_facet(&self, facet: &FacetKind) -> Option<&Type>
fn update_for_assignment(&mut self, facets: &Vec1<FacetKind>, ty: Option<Type>)
fn with_narrow(&self, facets: &Vec1<FacetKind>, ty: Type) -> Self
fn with_ty(self, ty: Type) -> Self
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

The `TypeInfo` datatype represents type information associated with a
name or expression in a control flow context.

This is distinct from `Type` because expressions and bound names can
track, in addition to the type of the top-level value, zero or more
facet narrows where we have access to additional control-flow-dependent
knowledge about how a chain of facet accesses will resolve.

For example:

```python
x: Foo
if x.foo is not None x.foo.bar is None and x.baz is None:
    # here, `x` is still `Foo` but we also can narrow
    # `x.foo`, `x.foo.bar`, and `x.baz`.
```

---
