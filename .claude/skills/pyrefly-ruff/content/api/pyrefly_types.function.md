# `pyrefly_types::function`

Crate `pyrefly_types` · 12 public items · structured records in [`model/pyrefly_types.function.json`](../model/pyrefly_types.function.json)

## BodyKind

`enum` · `pyrefly_types::function::BodyKind`

```rust
enum BodyKind
```

**Variants**: `RaiseNotImplementedError`, `ReturnNotImplemented`, `Ellipsis`, `Trivial`, `Other`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn is_placeholder_or_trivial(&self) -> bool
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

Shape of a function body.

---

## FunctionKind

`enum` · `pyrefly_types::function::FunctionKind`

```rust
enum FunctionKind
```

**Variants**: `IsInstance`, `IsSubclass`, `Callable`, `Len`, `Dataclass`, `DataclassField`, `DataclassReplace`, `CopyReplace`, `DataclassAsdict`, `AttrsFields`, `AttrsFieldsDict`, `AttrsEvolve`, `AttrsAssoc`, `DataclassTransform`, `ClassMethod`, `Overload`, `Override`, `Cast`, `AssertType`, `AssertShape`, `RevealType`, `Final`, `RuntimeCheckable`, `Synthesized`, `Def`, `AbstractMethod`, `NoTypeCheck`, `CallbackProtocol`, `SingleDispatchRegister`, `TotalOrdering`, `DisjointBase`, `NumbaJit`, `NumbaNjit`, `ShapeDsl`, `TypeShapeDsl`, `UsesShapeDsl`, `DefinesAssertShape`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (9)

```rust
fn as_func_def_id(&self) -> Option<&FuncDefId>
fn class(&self) -> Option<Class>
fn format(&self, current_module: ModuleName) -> String
fn from_definition(id: Arc<FuncDefId>) -> Self
fn function_name(&self) -> Cow<'_, Name>
fn is_signature_preserving_decorator(&self) -> bool
fn module_name(&self) -> ModuleName
fn outer_funcs(&self) -> Option<Name>
fn to_func_symbol(&self) -> Option<FuncSymbol>
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

## PropertyRole

`enum` · `pyrefly_types::function::PropertyRole`

```rust
enum PropertyRole
```

**Variants**: `Getter`, `Setter`, `SetterDecorator`, `DeleterDecorator`

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

## Deprecation

`struct` · `pyrefly_types::function::Deprecation`

```rust
struct Deprecation
```

**Fields**: `message`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn as_error_detail(&self) -> Option<String>
fn new(message: Option<String>) -> Self
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

Metadata extracted from a `@deprecated` decorator.

---

## FuncDefId

`struct` · `pyrefly_types::function::FuncDefId`

```rust
struct FuncDefId
```

**Fields**: `qname`, `cls`, `def_index`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (1)

```rust
fn has_toplevel_qname(&self, module: &str, name: &str) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

Identity of a source-backed function definition.

---

## FuncDefIndex

`struct` · `pyrefly_types::function::FuncDefIndex`

```rust
struct FuncDefIndex
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

The index of a function definition (`def ..():` statement) within the module,
used as a reference to data associated with the function.

---

## FuncFacts

`struct` · `pyrefly_types::function::FuncFacts`

```rust
struct FuncFacts
```

**Fields**: `body_kind`, `is_in_protocol_class`, `is_in_type_checking_block`, `is_abstract_method`, `is_overload`

**Derives**: Debug

**Methods** (3)

```rust
fn allows_missing_implementation(&self) -> bool
fn is_in_interface_like_context(&self) -> bool
fn is_stub(&self) -> bool
```

Ephemeral struct for computing facts about a function definition.

---

## FuncFlags

`struct` · `pyrefly_types::function::FuncFlags`

```rust
struct FuncFlags
```

**Fields**: `is_overload`, `is_staticmethod`, `is_classmethod`, `shape_flag_constructor_sources`, `deprecation`, `property_metadata`, `is_cached_property`, `has_enum_member_decoration`, `is_override`, `has_final_decoration`, `is_abstract_method`, `has_no_type_check`, `module_style`, `is_async`, `body_kind`, `is_return_inferred`, `calls_super_method`, `dataclass_transform_metadata`, `shape_transform`, `is_assert_shape`, `is_in_protocol_class`, `is_in_type_checking_block`, `has_gradual_variadic_params`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn facts(&self) -> FuncFacts
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

## FuncMetadata

`struct` · `pyrefly_types::function::FuncMetadata`

```rust
struct FuncMetadata
```

**Fields**: `kind`, `flags`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn method(cls: &Class, name: Name) -> Self
fn synthesized(module: &Module, cls: Option<&Class>, name: Name) -> Self
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

## FuncSymbol

`struct` · `pyrefly_types::function::FuncSymbol`

```rust
struct FuncSymbol
```

**Fields**: `module`, `cls`, `name`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

A function identified nominally by its module, class, and name.

---

## Function

`struct` · `pyrefly_types::function::Function`

```rust
struct Function
```

**Fields**: `signature`, `metadata`

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

## PropertyMetadata

`struct` · `pyrefly_types::function::PropertyMetadata`

```rust
struct PropertyMetadata
```

**Fields**: `role`, `getter`, `setter`, `has_deleter`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn from_components(role: PropertyRole, getter: &Type, setter: Option<&Type>, has_deleter: bool) -> Self
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
