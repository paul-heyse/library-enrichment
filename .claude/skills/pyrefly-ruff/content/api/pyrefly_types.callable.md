# `pyrefly_types::callable`

Crate `pyrefly_types` · 12 public items · structured records in [`model/pyrefly_types.callable.json`](../model/pyrefly_types.callable.json)

## Param

`enum` · `pyrefly_types::callable::Param`

```rust
enum Param
```

**Variants**: `PosOnly`, `Pos`, `Varargs`, `KwOnly`, `Kwargs`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn as_type(&self) -> &Type
fn as_type_mut(&mut self) -> &mut Type
fn fmt_with_type<O: TypeOutput>(&self, output: &mut O, write_type: &impl Fn(&Type, &mut O) -> fmt::Result) -> fmt::Result
fn format_for_signature(&self, type_ctx: &TypeDisplayContext<'_>) -> String
fn is_required(&self) -> bool
fn name(&self) -> Option<&Name>
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

## ParamOverlay

`enum` · `pyrefly_types::callable::ParamOverlay`

```rust
enum ParamOverlay
```

**Variants**: `All`, `Subset`

**Derives**: Clone, Debug

Controls which parameters are displayed by `ParamList::fmt_with_type`

---

## Params

`enum` · `pyrefly_types::callable::Params`

```rust
enum Params
```

**Variants**: `List`, `Partial`, `Ellipsis`, `Materialization`, `ParamSpec`

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

## PrefixParam

`enum` · `pyrefly_types::callable::PrefixParam`

```rust
enum PrefixParam
```

**Variants**: `PosOnly`, `Pos`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn into_posonly_param(self) -> Param
fn new(ty: Type, required: Required) -> Self
fn to_param_preserve_name(&self) -> Param
fn ty(&self) -> &Type
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

Represents a prefix parameter in `Concatenate`.
Prefix params can be either positional-only or positional (named).

---

## Required

`enum` · `pyrefly_types::callable::Required`

```rust
enum Required
```

**Variants**: `Required`, `Optional`

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

Requiredness for a function parameter.

---

## params_are_gradual_variadic

`function` · `pyrefly_types::callable::params_are_gradual_variadic`

```rust
fn params_are_gradual_variadic(params: &[Param]) -> bool
```

True if `params` has both a `*args` and a `**kwargs` parameter and both are typed `Any`
(explicitly, or implicitly because they are unannotated). Per the typing spec such a
signature is equivalent to `...`.

---

## unexpected_keyword

`function` · `pyrefly_types::callable::unexpected_keyword`

```rust
fn unexpected_keyword(error: &dyn Fn(String), func: &str, keyword: &ruff_python_ast::Keyword)
```

---

## ArgCount

`struct` · `pyrefly_types::callable::ArgCount`

```rust
struct ArgCount
```

**Fields**: `min`, `max`

**Derives**: Clone, Debug

---

## ArgCounts

`struct` · `pyrefly_types::callable::ArgCounts`

```rust
struct ArgCounts
```

**Fields**: `positional`, `keyword`, `overall`

**Derives**: Clone, Debug

---

## Callable

`struct` · `pyrefly_types::callable::Callable`

```rust
struct Callable
```

**Fields**: `params`, `ret`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (19)

```rust
fn accepts_single_positional_arg(&self) -> bool
fn arg_counts(&self) -> ArgCounts
fn concatenate(args: Box<[PrefixParam]>, param_spec: Type, ret: Type) -> Self
fn contains_callable_residual(&self) -> bool
fn ellipsis(ret: Type) -> Self
fn fmt_with_type<O: TypeOutput>(&self, output: &mut O, write_type: &impl Fn(&Type, &mut O) -> fmt::Result) -> fmt::Result
fn fmt_with_type_with_newlines<O: TypeOutput>(&self, output: &mut O, write_type: &impl Fn(&Type, &mut O, usize) -> fmt::Result, indent: usize) -> fmt::Result
fn get_first_param(&self) -> Option<&Type>
fn get_positional_param(&self, index: usize) -> Option<&Type>
fn is_args_kwargs_wrapper(&self) -> bool
fn is_fully_unknown(&self) -> bool
fn is_typeguard(&self) -> bool
fn is_typeis(&self) -> bool
fn list(params: ParamList, ret: Type) -> Self
fn param_spec(p: Type, ret: Type) -> Self
fn partial(params: ParamList, ret: Type) -> Self
fn split_first_param<'a>(&'a self, owner: &'a mut Owner<Type>) -> Option<(&'a Type, Self)>
fn strip_first_param(&self) -> Option<Self>
fn subst_self_type_mut(&mut self, replacement: &Type)
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

## DefaultValue

`struct` · `pyrefly_types::callable::DefaultValue`

```rust
struct DefaultValue
```

**Fields**: `ty`, `display`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(ty: Type) -> Self
fn with_display(ty: Type, display: String) -> Self
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

The default value of an optional parameter, containing its type and an optional
display string for values whose types don't preserve the literal value (e.g. floats).

---

## ParamList

`struct` · `pyrefly_types::callable::ParamList`

```rust
struct ParamList
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
fn everything() -> ParamList
fn fmt_with_type<O: TypeOutput>(&self, output: &mut O, write_type: &impl Fn(&Type, &mut O) -> fmt::Result, overlay: &ParamOverlay) -> fmt::Result
fn fmt_with_type_with_newlines<O: TypeOutput>(&self, output: &mut O, write_type: &impl Fn(&Type, &mut O, usize) -> fmt::Result, indent: usize) -> fmt::Result
fn into_items(self) -> Vec<Param>
fn is_empty(&self) -> bool
fn items(&self) -> &[Param]
fn items_mut(&mut self) -> &mut [Param]
fn len(&self) -> usize
fn new(xs: Vec<Param>) -> Self
fn new_types(xs: Vec<PrefixParam>) -> Self
fn prepend_types(&self, pre: &[PrefixParam]) -> Cow<'_, ParamList>
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
