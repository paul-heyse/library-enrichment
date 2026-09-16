# `pyrefly::binding::narrow`

Crate `pyrefly` · 10 public items · structured records in [`model/pyrefly.binding.narrow.json`](../model/pyrefly.binding.narrow.json)

## AtomicNarrowOp

`enum` · `pyrefly::binding::narrow::AtomicNarrowOp`

```rust
enum AtomicNarrowOp
```

**Variants**: `Is`, `IsNot`, `Eq`, `NotEq`, `IsInstance`, `IsNotInstance`, `IsSubclass`, `IsNotSubclass`, `HasAttr`, `NotHasAttr`, `GetAttr`, `NotGetAttr`, `TypeGuard`, `NotTypeGuard`, `TypeIs`, `NotTypeIs`, `TypeEq`, `TypeNotEq`, `In`, `NotIn`, `HasKey`, `NotHasKey`, `LenEq`, `LenNotEq`, `LenGt`, `LenGte`, `LenLt`, `LenLte`, `IsSequence`, `IsNotSequence`, `IsMapping`, `IsNotMapping`, `Call`, `NotCall`, `IsTruthy`, `IsFalsy`, `PolarsColumnMutation`, `Placeholder`, `ClassCoverageGate`, `ClassCoverageGateNeg`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn as_python_snippet(&self, subject: &str, snippet: &impl Fn(TextRange) -> Option<String>) -> Option<String>
fn negate(&self) -> Self
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

---

## FacetOrigin

`enum` · `pyrefly::binding::narrow::FacetOrigin`

```rust
enum FacetOrigin
```

**Variants**: `Direct`, `GetMethod`, `MatchSubject`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## NarrowOp

`enum` · `pyrefly::binding::narrow::NarrowOp`

```rust
enum NarrowOp
```

**Variants**: `Atomic`, `And`, `Or`

**Implements**: `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn as_python_snippet(&self, base_name: &Name, snippet: &impl Fn(TextRange) -> Option<String>) -> Option<String>
fn for_subject(&self, subject: &NarrowingSubject) -> Self
fn negate(&self) -> Self
fn rebase_onto_subject(&self, subject: &NarrowingSubject) -> Option<Self>
fn set_allow_never_collapse(&mut self)
fn strip_placeholders(&mut self)
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &pyrefly_python::module::Module) -> fmt::Result
```

---

## NarrowSource

`enum` · `pyrefly::binding::narrow::NarrowSource`

```rust
enum NarrowSource
```

**Variants**: `Call`, `Pattern`

**Derives**: Clone, Copy, Debug

Indicates where an isinstance-style narrow operation originated from.
This determines whether validation needs to happen during narrowing.

---

## NarrowingSubject

`enum` · `pyrefly::binding::narrow::NarrowingSubject`

```rust
enum NarrowingSubject
```

**Variants**: `Name`, `Facets`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn name(&self) -> &Name
fn with_facet(&self, prop: UnresolvedFacetKind) -> Self
```

---

## expr_to_subjects

`function` · `pyrefly::binding::narrow::expr_to_subjects`

```rust
fn expr_to_subjects(expr: &ruff_python_ast::Expr) -> Vec<NarrowingSubject>
```

---

## identifier_and_chain_for_expr

`function` · `pyrefly::binding::narrow::identifier_and_chain_for_expr`

```rust
fn identifier_and_chain_for_expr(expr: &ruff_python_ast::Expr) -> Option<(ruff_python_ast::Identifier, types::facet::UnresolvedFacetChain)>
```

Given an expression, determine whether it is a chain of properties (attribute/concrete index) rooted at a name,
and if so, return the name and the chain of properties.
For example: x.y.[0].z

---

## identifier_and_chain_prefix_for_expr

`function` · `pyrefly::binding::narrow::identifier_and_chain_prefix_for_expr`

```rust
fn identifier_and_chain_prefix_for_expr(expr: &ruff_python_ast::Expr) -> Option<(ruff_python_ast::Identifier, Vec<types::facet::UnresolvedFacetKind>)>
```

Similar to identifier_and_chain_for_expr, except if we encounter a non-concrete subscript in the chain
we only return the prefix before that location.
For example: w.x[y].z -> w.x

---

## FacetSubject

`struct` · `pyrefly::binding::narrow::FacetSubject`

```rust
struct FacetSubject
```

**Fields**: `chain`, `origin`, `allow_never_collapse`

**Derives**: Clone, Debug

---

## NarrowOps

`struct` · `pyrefly::binding::narrow::NarrowOps`

```rust
struct NarrowOps
```

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn and_all(&mut self, other: Self)
fn and_for_subject(&mut self, subject: &NarrowingSubject, op: NarrowOp, range: TextRange)
fn from_expr(builder: &BindingsBuilder<'_>, test: Option<&Expr>) -> Self
fn from_single_narrow_op(left: &Expr, op: AtomicNarrowOp, range: TextRange) -> Self
fn from_single_narrow_op_for_subject(subject: NarrowingSubject, op: AtomicNarrowOp, range: TextRange) -> Self
fn negate(&self) -> Self
fn new() -> Self
fn or_all(&mut self, other: Self)
fn set_allow_never_collapse(&mut self)
```

---
