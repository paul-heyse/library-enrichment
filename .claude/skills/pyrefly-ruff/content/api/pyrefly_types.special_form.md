# `pyrefly_types::special_form`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.special_form.json`](../model/pyrefly_types.special_form.json)

## SpecialForm

`enum` · `pyrefly_types::special_form::SpecialForm`

```rust
enum SpecialForm
```

**Variants**: `Annotated`, `Callable`, `ClassVar`, `Concatenate`, `Final`, `Generic`, `Literal`, `LiteralString`, `Never`, `NoReturn`, `NotRequired`, `Optional`, `Protocol`, `ReadOnly`, `Required`, `SelfType`, `Tuple`, `Type`, `TypeAlias`, `TypeForm`, `TypeGuard`, `TypeIs`, `TypedDict`, `Union`, `Unpack`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn can_be_subscripted(self) -> bool
fn is_valid_bare_type_expression(self) -> bool
fn isinstance_safe(self) -> bool
fn new(name: &Name, annotation: &Expr) -> Option<Self>
fn to_qualifier(self) -> Option<Qualifier>
fn to_type(self, heap: &TypeHeap) -> Type
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err>
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
