# `pyrefly_types::keywords`

Crate `pyrefly_types` · 6 public items · structured records in [`model/pyrefly_types.keywords.json`](../model/pyrefly_types.keywords.json)

## ConverterMap

`struct` · `pyrefly_types::keywords::ConverterMap`

```rust
struct ConverterMap
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn from_map(map: OrderedMap<Type, Type>) -> Self
fn get(&self, key: &Type) -> Option<&Type>
fn is_empty(&self) -> bool
fn new() -> Self
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
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

---

## DataclassFieldKeywords

`struct` · `pyrefly_types::keywords::DataclassFieldKeywords`

```rust
struct DataclassFieldKeywords
```

**Fields**: `init`, `default`, `kw_only`, `init_by_name`, `init_by_alias`, `lt`, `gt`, `ge`, `le`, `strict`, `frozen`, `converter_param`, `attrs_setattr_frozen`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn is_kw_only(&self) -> bool
fn new() -> Self
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

Parameters to dataclass field specifiers.
See https://typing.python.org/en/latest/spec/dataclasses.html#field-specifier-parameters.

---

## DataclassKeywords

`struct` · `pyrefly_types::keywords::DataclassKeywords`

```rust
struct DataclassKeywords
```

**Fields**: `init`, `order`, `frozen`, `match_args`, `kw_only`, `eq`, `unsafe_hash`, `slots`, `extra`, `strict`, `auto_attribs`, `attrs_setattr_frozen`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn attrs_hash_from_map(map: &TypeMap) -> Option<bool>
fn from_type_map(map: &TypeMap, defaults: &DataclassTransformMetadata, strict_default: bool) -> Self
fn new() -> Self
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

Dataclass parameters.
See https://typing.python.org/en/latest/spec/dataclasses.html#decorator-function-and-class-metaclass-parameters.

---

## DataclassTransformMetadata

`struct` · `pyrefly_types::keywords::DataclassTransformMetadata`

```rust
struct DataclassTransformMetadata
```

**Fields**: `eq_default`, `order_default`, `kw_only_default`, `frozen_default`, `field_specifiers`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn from_type_map(map: &TypeMap) -> Self
fn new() -> Self
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

Parameters to `typing.dataclass_transform`.
See https://typing.python.org/en/latest/spec/dataclasses.html#dataclass-transform-parameters.

---

## KwCall

`struct` · `pyrefly_types::keywords::KwCall`

```rust
struct KwCall
```

**Fields**: `func_metadata`, `keywords`, `return_ty`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn has_function_kind(&self, kind: FunctionKind) -> bool
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

Wraps the result of a function call whose keyword arguments have typing effects.

---

## TypeMap

`struct` · `pyrefly_types::keywords::TypeMap`

```rust
struct TypeMap
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn get_bool(&self, name: &Name) -> Option<bool>
fn get_string(&self, name: &Name) -> Option<&str>
fn is_set(&self, name: &Name) -> bool
fn new() -> Self
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
