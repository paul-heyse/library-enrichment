# `pyrefly_types::heap`

Crate `pyrefly_types` · 2 public items · structured records in [`model/pyrefly_types.heap.json`](../model/pyrefly_types.heap.json)

## TypeHeap

`struct` · `pyrefly_types::heap::TypeHeap`

```rust
struct TypeHeap
```

**Derives**: Debug

**Methods** (66)

```rust
fn mk(&self, ty: Type) -> Type
fn mk_any(&self, style: AnyStyle) -> Type
fn mk_any_error(&self) -> Type
fn mk_any_explicit(&self) -> Type
fn mk_any_implicit(&self) -> Type
fn mk_args(&self, quantified: Quantified) -> Type
fn mk_args_value(&self, quantified: Quantified) -> Type
fn mk_bound_method(&self, bound_method: BoundMethod) -> Type
fn mk_callable(&self, params: Params, ret: Type) -> Type
fn mk_callable_concatenate(&self, params: Box<[PrefixParam]>, param_spec: Type, ret: Type) -> Type
fn mk_callable_ellipsis(&self, ret: Type) -> Type
fn mk_callable_from(&self, callable: Callable) -> Type
fn mk_callable_from_vec(&self, params: Vec<Param>, ret: Type) -> Type
fn mk_callable_param_spec(&self, param_spec: Type, ret: Type) -> Type
fn mk_class_def(&self, class: Class) -> Type
fn mk_class_type(&self, class_type: ClassType) -> Type
fn mk_concatenate(&self, types: Box<[PrefixParam]>, param_spec: Type) -> Type
fn mk_concrete_tuple(&self, elts: Vec<Type>) -> Type
fn mk_element_of_type_var_tuple(&self, quantified: Quantified) -> Type
fn mk_forall(&self, forall: Forall<Forallable>) -> Type
fn mk_function(&self, func: Function) -> Type
fn mk_int(&self, symint: Int) -> Type
fn mk_int_tuple(&self, int_tuple: IntTuple) -> Type
fn mk_intersect(&self, members: Vec<Type>, fallback: Type) -> Type
fn mk_kw_call(&self, kw_call: KwCall) -> Type
fn mk_kwargs(&self, quantified: Quantified) -> Type
fn mk_kwargs_value(&self, quantified: Quantified) -> Type
fn mk_literal(&self, literal: Literal) -> Type
fn mk_literal_string(&self, style: LitStyle) -> Type
fn mk_materialization(&self) -> Type
fn mk_module(&self, module: ModuleType) -> Type
fn mk_never(&self) -> Type
fn mk_never_style(&self, style: NeverStyle) -> Type
fn mk_nn_module(&self, module: NNModuleType) -> Type
fn mk_none(&self) -> Type
fn mk_optional(&self, inner: Type) -> Type
fn mk_overload(&self, overload: Overload) -> Type
fn mk_param_spec(&self, param_spec: ParamSpec) -> Type
fn mk_param_spec_value(&self, params: ParamList) -> Type
fn mk_partial_typed_dict(&self, typed_dict: TypedDict) -> Type
fn mk_quantified(&self, quantified: Quantified) -> Type
fn mk_quantified_value(&self, quantified: Quantified) -> Type
fn mk_self_type(&self, class_type: ClassType) -> Type
fn mk_sentinel(&self, sentinel: Sentinel) -> Type
fn mk_shaped_array(&self, shaped_array: ShapedArrayType) -> Type
fn mk_special_form(&self, special_form: SpecialForm) -> Type
fn mk_super_instance(&self, lookup_cls: ClassType, obj: SuperObj) -> Type
fn mk_tuple(&self, tuple: Tuple) -> Type
fn mk_type_alias(&self, type_alias: TypeAliasData) -> Type
fn mk_type_guard(&self, inner: Type) -> Type
fn mk_type_is(&self, inner: Type) -> Type
fn mk_type_of(&self, inner: Type) -> Type
fn mk_type_var(&self, type_var: TypeVar) -> Type
fn mk_type_var_tuple(&self, type_var_tuple: TypeVarTuple) -> Type
fn mk_typed_dict(&self, typed_dict: TypedDict) -> Type
fn mk_typeform(&self, inner: Type) -> Type
fn mk_unbounded_tuple(&self, elem: Type) -> Type
fn mk_union(&self, members: Vec<Type>) -> Type
fn mk_union_with_name(&self, members: Vec<Type>, display_name: (ModuleName, Name)) -> Type
fn mk_unpack(&self, inner: Type) -> Type
fn mk_unpacked_tuple(&self, before: Vec<Type>, middle: Type, after: Vec<Type>) -> Type
fn mk_var(&self, var: Var) -> Type
fn new() -> Self
fn ptr(&self, ty: &Type) -> TypePtr
fn unique(&self) -> Unique
fn unptr<'t>(&'t self, type_ptr: &TypePtr) -> &'t Type
```

A factory for constructing types.

Currently returns boxed types; will be backed by an arena in the future.
Each TypeHeap has a unique identifier for debugging and tracking purposes.

---

## TypePtr

`struct` · `pyrefly_types::heap::TypePtr`

```rust
struct TypePtr
```

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug

A type reference with an erased lifetime.

Used in contexts where a lifetime parameter would be problematic,
such as structs that own both a `TypeHeap` and store types.
The `id` field records which `TypeHeap` created this pointer,
and `TypeHeap::unptr` verifies the match at runtime (panicking on mismatch).

---
