# `pyrefly::solver::type_order`

Crate `pyrefly` · 1 public items · structured records in [`model/pyrefly.solver.type_order.json`](../model/pyrefly.solver.type_order.json)

## TypeOrder

`struct` · `pyrefly::solver::type_order::TypeOrder`

```rust
struct TypeOrder<'solver, Ans: LookupAnswer>
```

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy

**Methods** (35)

```rust
fn args_expander(self, posargs: Vec<CallArg<'solver>>, keywords: Vec<CallKeyword<'solver>>) -> ArgsExpander<'solver, Ans>
fn as_class_type_unchecked(self, class: &Class) -> ClassType
fn as_superclass(self, class: &ClassType, want: &Class) -> Option<ClassType>
fn as_tuple_type(self, cls: &ClassType) -> Option<Type>
fn bind_boundmethod(self, m: &BoundMethod, is_subset: &mut dyn FnMut(&Type, &Type) -> bool) -> Option<Type>
fn coinductive_assumptions_used(self) -> bool
fn constructor_to_callable(self, cls: &ClassType) -> Type
fn constructor_to_callable_for_class_def(self, cls: &Class) -> Type
fn error_swallower(self) -> ErrorCollector
fn extends_any(self, cls: &Class) -> bool
fn get_enum_member_count(self, cls: &Class) -> Option<usize>
fn get_protocol_member_names(self, cls: &Class) -> SmallSet<Name>
fn get_type_alias<'b>(self, ta: &'b TypeAliasData) -> Cow<'b, TypeAlias> where 'solver: 'b
fn get_typed_dict_value_type(self, typed_dict: &TypedDict) -> Type
fn get_typed_dict_value_type_as_builtins_dict(self, typed_dict: &TypedDict) -> Option<Type>
fn get_variance_from_class(self, cls: &Class) -> &'solver VarianceMap
fn has_active_scc(self) -> bool
fn has_metaclass(self, cls: &Class, metaclass: &ClassType) -> bool
fn has_superclass(self, got: &Class, want: &Class) -> bool
fn instance_as_dunder_call(self, class_type: &ClassType) -> Option<Type>
fn instantiate_fresh_forall(self, forall: Forall<Forallable>) -> (QuantifiedHandle, Type)
fn is_debug(self) -> bool
fn is_new_type(self, cls: &Class) -> bool
fn is_protocol(self, cls: &Class) -> bool
fn is_protocol_subset_at_attr(self, got: &Type, protocol: &ClassType, name: &Name, is_subset: &mut dyn FnMut(&Type, &Type) -> Result<(), SubsetError>) -> Result<(), SubsetError>
fn is_subclassable(self, cls: &Class) -> bool
fn new(solver: &'solver AnswersSolver<'solver, 'solver, Ans>) -> Self
fn promote_silently(self, cls: &Class) -> Type
fn set_coinductive_assumptions_used(self, value: bool)
fn shaped_array_shape_for_class_type(self, cls: &ClassType) -> Option<Quantified>
fn stdlib(self) -> &'solver Stdlib
fn typed_dict_extra_items(self, typed_dict: &TypedDict) -> ExtraItems
fn typed_dict_fields(self, typed_dict: &TypedDict) -> SmallMap<Name, TypedDictField>
fn typed_dict_kw_param_info(self, typed_dict: &TypedDict) -> Vec<(Name, Type, Required)>
fn untype_alias(self, ta: &TypeAliasData) -> Type
```

`TypeOrder` provides a minimal API allowing `Subset` to request additional
information about types that may be required for solving bindings

This is needed for cases like the nominal type order and structural types where
the `Type` object itself does not contain enough information to determine
subset relations.

---
