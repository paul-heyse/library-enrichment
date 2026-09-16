# `pyrefly_types::types`

Crate `pyrefly_types` · 18 public items · structured records in [`model/pyrefly_types.types.json`](../model/pyrefly_types.types.json)

## AnyStyle

`enum` · `pyrefly_types::types::AnyStyle`

```rust
enum AnyStyle
```

**Variants**: `Explicit`, `Implicit`, `Error`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn propagate(self) -> Type
```

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

The types of Any. Prefer later ones where we have multiple.

---

## BoundMethodType

`enum` · `pyrefly_types::types::BoundMethodType`

```rust
enum BoundMethodType
```

**Variants**: `Function`, `Forall`, `Overload`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_type(self) -> Type
fn metadata(&self) -> &FuncMetadata
fn strip_receiver(&self) -> Option<Self>
fn subst_self_type_mut(&mut self, replacement: &Type)
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

## CalleeKind

`enum` · `pyrefly_types::types::CalleeKind`

```rust
enum CalleeKind
```

**Variants**: `Callable`, `Function`, `Class`

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

## Forallable

`enum` · `pyrefly_types::types::Forallable`

```rust
enum Forallable
```

**Variants**: `TypeAlias`, `Function`, `Callable`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn as_type(self) -> Type
fn forall(self, tparams: Arc<TParams>) -> Type
fn name(&self) -> Cow<'_, Name>
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

These are things that can have Forall around them, so often you see `Forall<Forallable>`

---

## NeverStyle

`enum` · `pyrefly_types::types::NeverStyle`

```rust
enum NeverStyle
```

**Variants**: `NoReturn`, `Never`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

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

The types of Never. Prefer later ones where we have multiple.

---

## OverloadType

`enum` · `pyrefly_types::types::OverloadType`

```rust
enum OverloadType
```

**Variants**: `Function`, `Forall`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_type(&self) -> Type
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

## SuperObj

`enum` · `pyrefly_types::types::SuperObj`

```rust
enum SuperObj
```

**Variants**: `Instance`, `Class`

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

The second argument (implicit or explicit) to a super() call.
Either an instance of a class (inside an instance method) or a
class object (inside a classmethod or staticmethod)

---

## TParamsSource

`enum` · `pyrefly_types::types::TParamsSource`

```rust
enum TParamsSource
```

**Variants**: `Class`, `TypeAlias`, `Function`

**Implements**: `core::fmt::Display`

**Derives**: Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## Type

`enum` · `pyrefly_types::types::Type`

```rust
enum Type
```

**Variants**: `Literal`, `LiteralString`, `Callable`, `CallableResidual`, `TypeLevelDslCall`, `Function`, `BoundMethod`, `Overload`, `Union`, `Intersect`, `ClassDef`, `ClassType`, `TypedDict`, `PartialTypedDict`, `ShapedArray`, `IntTuple`, `NNModule`, `DataFrame`, `Series`, `Int`, `Tuple`, `Module`, `Forall`, `Var`, `Quantified`, `QuantifiedValue`, `ElementOfTypeVarTuple`, `TypeGuard`, `TypeIs`, `Annotated`, `Unpack`, `TypeVar`, `ParamSpec`, `TypeVarTuple`, `SpecialForm`, `Concatenate`, `ParamSpecValue`, `Args`, `Kwargs`, `ArgsValue`, `KwargsValue`, `Type`, `TypeForm`, `Ellipsis`, `Any`, `Never`, `TypeAlias`, `UntypedAlias`, `Sentinel`, `SuperInstance`, `SelfType`, `KwCall`, `Materialization`, `None`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (102)

```rust
fn any(&self, predicate: impl FnMut(&Type) -> bool) -> bool
fn any_error() -> Self
fn any_explicit() -> Self
fn any_implicit() -> Self
fn any_tuple() -> Self
fn arc_clone(Arc<self>) -> Self
fn as_bool(&self) -> Option<bool>
fn as_lsp_string(&self, mode: LspDisplayMode) -> String
fn as_lsp_string_with_options(&self, fallback_name: Option<&str>, mode: LspDisplayMode, expand_unions: bool, qualify_outside: Option<ModuleName>) -> String
fn as_module(&self) -> Option<&ModuleType>
fn as_quantified(&self) -> Option<(&Quantified, Option<&Type>)>
fn as_shape_literal(&self) -> Option<i64>
fn callable(params: Vec<Param>, ret: Type) -> Self
fn callable_concatenate(args: Box<[PrefixParam]>, param_spec: Type, ret: Type) -> Self
fn callable_ellipsis(ret: Type) -> Self
fn callable_first_param(&self, heap: &TypeHeap) -> Option<Type>
fn callable_param_spec(p: Type, ret: Type) -> Self
fn callable_residual_generic(quantified: Quantified) -> Type
fn callable_residual_overload(identity: OverloadResidualIdentity, branches: Vec<OverloadBranchProjection>) -> Type
fn callable_return_type(&self, heap: &TypeHeap) -> Option<Type>
fn callee_kind(&self) -> Option<CalleeKind>
fn canonicalize(self) -> Self
fn clean_var(self) -> Type
fn collect_all_vars(&self) -> Vec<Var>
fn collect_maybe_placeholder_vars(&self) -> Vec<Var>
fn collect_quantifieds<'a>(&'a self, acc: &mut SmallSet<&'a Quantified>)
fn collect_raw_legacy_type_variables(&self, acc: &mut Vec<Name>)
fn concrete_tuple(elts: Vec<Type>) -> Self
fn contains_overload_callable_residual(&self) -> bool
fn contains_type_variable(&self) -> bool
fn deterministic_printing(self) -> Self
fn explicit_any(self) -> Self
fn finalize_callable_residuals_at_boundary(self, heap: &TypeHeap, preserve_class_targs: bool) -> Type
fn finalize_type_level_dsl_at_boundary(&mut self) -> Vec<dimension::ShapeError>
fn flatten_overload_residual_markers(&mut self, heap: &TypeHeap)
fn flatten_residuals(self, heap: &TypeHeap) -> Type
fn for_each_free_quantified<'a>(&'a self, f: &mut impl FnMut(&'a Quantified))
fn for_each_quantified<'a>(&'a self, f: &mut impl FnMut(&'a Quantified))
fn function_deprecation(&self) -> Option<&Deprecation>
fn get_annotation_parts(&self, stdlib: Option<&Stdlib>) -> Vec<AnnotationPart>
fn get_types_with_locations(&self, stdlib: Option<&Stdlib>) -> Vec<(String, Option<TextRangeWithModule>)>
fn has_final_decoration(&self) -> bool
fn has_toplevel_func_metadata(&self) -> bool
fn into_unions(self) -> Vec<Type>
fn is_abstract_method(&self) -> bool
fn is_any(&self) -> bool
fn is_ellipsis_value(&self) -> bool
fn is_error(&self) -> bool
fn is_explicit_type_variable(&self) -> bool
fn is_implicit_literal(&self) -> bool
fn is_kind_param_spec(&self) -> bool
fn is_kind_type_var_tuple(&self) -> bool
fn is_literal_string(&self) -> bool
fn is_never(&self) -> bool
fn is_none(&self) -> bool
fn is_overload(&self) -> bool
fn is_override(&self) -> bool
fn is_property_getter(&self) -> bool
fn is_property_setter_with_getter(&self) -> Option<Type>
fn is_raw_legacy_type_variable(&self) -> bool
fn is_scalar(&self) -> bool
fn is_toplevel_callable(&self) -> bool
fn is_typed_dict(&self) -> bool
fn is_typeguard(&self) -> bool
fn is_typeis(&self) -> bool
fn is_union(&self) -> bool
fn is_unpack(&self) -> bool
fn lit_string_style(&self) -> Option<&LitStyle>
fn materialize(&self) -> Self
fn may_contain_placeholder_var(&self) -> bool
fn never() -> Self
fn optional(x: Self) -> Self
fn promote_implicit_literals(self, stdlib: &Stdlib) -> Type
fn promote_shallow_implicit_literals(self, stdlib: &Stdlib) -> Type
fn property_metadata(&self) -> Option<&PropertyMetadata>
fn qname(&self) -> Option<&QName>
fn set_property_metadata(&mut self, metadata: PropertyMetadata) -> bool
fn strip_library_schemas(self) -> Type
fn subst(self, mp: &SmallMap<&Quantified, &Type>) -> Self
fn subst_mut(&mut self, mp: &SmallMap<&Quantified, &Type>)
fn subst_mut_fn(&mut self, mp: &mut dyn FnMut(&Quantified) -> Option<Type>)
fn subst_self_special_form_mut(&mut self, self_type: &Type)
fn subst_self_type_mut(&mut self, replacement: &Type)
fn to_callable(self) -> Option<Callable>
fn to_func_kind(&self) -> Option<&FunctionKind>
fn toplevel_callable_signatures(&self) -> impl Iterator<Item = (&Callable, Option<&Arc<TParams>>)>
fn toplevel_func_metadata(&self) -> Option<&FuncMetadata>
fn toplevel_func_metadata_mut(&mut self) -> Option<&mut FuncMetadata>
fn transform(self, f: &mut dyn FnMut(&mut Type)) -> Self
fn transform_mut(&mut self, f: &mut dyn FnMut(&mut Type))
fn transform_toplevel_callable_signatures(&mut self, f: impl FnMut(&mut Callable, &mut Option<Arc<TParams>>))
fn transform_types_in_type_variable_positions(&mut self, f: &mut dyn FnMut(&mut Type))
fn truncate_class_nesting(self, max_depth: usize, max_inner_union_width: usize, any: &Type) -> Type
fn type_of(inner: Type) -> Self
fn unbounded_tuple(elt: Type) -> Self
fn union(members: Vec<Type>) -> Self
fn union_width(&self) -> usize
fn universe<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
fn unpacked_tuple(prefix: Vec<Type>, middle: Type, suffix: Vec<Type>) -> Self
fn unpacked_typed_dict(&self) -> Option<&TypedDict>
fn with_literal_style(self, style: LitStyle) -> Self
fn without_property_metadata(&self) -> Type
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
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Self))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Self))
```

---

## BoundMethod

`struct` · `pyrefly_types::types::BoundMethod`

```rust
struct BoundMethod
```

**Fields**: `obj`, `func`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn as_type(self) -> Type
fn with_bound_object(&self, obj: Type) -> Self
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

## Forall

`struct` · `pyrefly_types::types::Forall`

```rust
struct Forall<T>
```

**Fields**: `tparams`, `body`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn apply_targs(self, targs: TArgs) -> Type
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

## NNModuleType

`struct` · `pyrefly_types::types::NNModuleType`

```rust
struct NNModuleType
```

**Fields**: `class`, `fields`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(class: ClassType, fields: SmallMap<Name, Type>) -> Self
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

An nn.Module instance with captured constructor arguments.

Analogous to how `ShapedArrayType` wraps `ClassType` + shape info, `NNModuleType`
wraps `ClassType` + a field map of captured init args. This allows DSL forward
functions to access constructor parameters (e.g., `kernel_size`, `stride`)
directly from the type, without requiring every shape-relevant parameter to
be a generic type param on the class.

Created by init DSL functions during `construct_class`. When `forward` is
called on an NNModule instance, the fields are injected as `Val::Module`
into the DSL's bound_args.

---

## Overload

`struct` · `pyrefly_types::types::Overload`

```rust
struct Overload
```

**Fields**: `signatures`, `metadata`

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

## Substitution

`struct` · `pyrefly_types::types::Substitution`

```rust
struct Substitution<'a>
```

**Methods** (3)

```rust
fn for_prefix(tparams: &'a TParams, args: &'a [Type]) -> Self
fn substitute_into(&self, ty: Type) -> Type
fn substitute_into_mut(&self, ty: &mut Type)
```

---

## TArgs

`struct` · `pyrefly_types::types::TArgs`

```rust
struct TArgs
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (15)

```rust
fn as_mut(&mut self) -> &mut [Type]
fn as_slice(&self) -> &[Type]
fn display_count(&self) -> usize
fn is_empty(&self) -> bool
fn iter_paired(&self) -> impl ExactSizeIterator<Item = (&Quantified, &Type)>
fn iter_paired_mut(&mut self) -> impl ExactSizeIterator<Item = (&Quantified, &mut Type)>
fn len(&self) -> usize
fn new(tparams: Arc<TParams>, targs: Vec<Type>) -> Self
fn split_mut(&mut self) -> (&TParams, &mut [Type])
fn substitute_into(&self, ty: Type) -> Type
fn substitute_into_mut(&self, ty: &mut Type)
fn substitute_with(&self, substitution: &Substitution<'_>) -> Self
fn substitution<'a>(&'a self) -> Substitution<'a>
fn substitution_map(&self) -> SmallMap<&Quantified, &Type>
fn tparams(&self) -> &TParams
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

---

## TParams

`struct` · `pyrefly_types::types::TParams`

```rust
struct TParams
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (9)

```rust
fn as_vec(&self) -> &[Quantified]
fn empty() -> TParams
fn empty_ref() -> &'static Self
fn extend(&mut self, other: &TParams)
fn is_empty(&self) -> bool
fn iter(&self) -> impl ExactSizeIterator<Item = &Quantified>
fn len(&self) -> usize
fn new(tparams: Vec<Quantified>) -> TParams
fn truncate_recursive_targs(self) -> Self
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

Wraps a vector of type parameters.

---

## Union

`struct` · `pyrefly_types::types::Union`

```rust
struct Union
```

**Fields**: `members`, `display_name`

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

## Var

`struct` · `pyrefly_types::types::Var`

```rust
struct Var
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn new(uniques: &UniqueFactory) -> Self
fn to_type(self, heap: &TypeHeap) -> Type
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

An introduced synthetic variable to range over as yet unknown types.

---
