# `pyrefly::alt::types::yields`

Crate `pyrefly` · 2 public items · structured records in [`model/pyrefly.alt.types.yields.json`](../model/pyrefly.alt.types.yields.json)

## YieldFromResult

`struct` · `pyrefly::alt::types::yields::YieldFromResult`

```rust
struct YieldFromResult
```

**Fields**: `yield_ty`, `send_ty`, `return_ty`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn any_error(heap: &TypeHeap) -> Self
fn from_generator(generator: (Type, Type, Type)) -> Self
fn from_iterable(heap: &TypeHeap, yield_ty: Type) -> Self
fn recursive(heap: &TypeHeap) -> Self
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

## YieldResult

`struct` · `pyrefly::alt::types::yields::YieldResult`

```rust
struct YieldResult
```

**Fields**: `yield_ty`, `send_ty`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn any_error(heap: &TypeHeap) -> Self
fn recursive(heap: &TypeHeap) -> Self
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
