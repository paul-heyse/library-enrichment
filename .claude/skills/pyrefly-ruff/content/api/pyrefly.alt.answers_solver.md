# `pyrefly::alt::answers_solver`

Crate `pyrefly` · 11 public items · structured records in [`model/pyrefly.alt.answers_solver.json`](../model/pyrefly.alt.answers_solver.json)

## SccNodeState

`enum` · `pyrefly::alt::answers_solver::SccNodeState`

```rust
enum SccNodeState
```

**Variants**: `Fresh`, `InProgress`, `HasPlaceholder`, `Done`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn kind(&self, has_previous_answer: bool) -> SccNodeStateKind
```

Tracks the state of a node within an active SCC.

This replaces the previous stack-based tracking (recursion_stack, unwind_stack)
with explicit state tracking. The state transitions are:
- Fresh → InProgress (when we first encounter the node as a Participant)
- InProgress → HasPlaceholder (when a placeholder is recorded for cycle breaking)
- InProgress/HasPlaceholder → Done (when the node's calculation completes)

The variants are ordered by "advancement" (Fresh < InProgress < HasPlaceholder < Done).
The `advancement_rank()` method encodes this ordering for use during SCC merge.

---

## SccNodeStateKind

`enum` · `pyrefly::alt::answers_solver::SccNodeStateKind`

```rust
enum SccNodeStateKind
```

**Variants**: `Fresh`, `InProgressWithPreviousAnswer`, `InProgressWithPlaceholder`, `InProgressCold`, `Done`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Lightweight summary of an `SccNodeState` for borrow-safe read-then-act
patterns.

Reading the full `SccNodeState` requires borrowing the SCC, but we
often need to drop that borrow before mutating. This enum captures just
enough information to decide what action to take.

---

## AnswerScope

`struct` · `pyrefly::alt::answers_solver::AnswerScope`

```rust
struct AnswerScope
```

Retains SCC answer generations for the lifetime of one solver view.

Generations are appended at most once and never removed, so every answer
reference returned through the associated solver is backed by an `Rc` for
the full lifetime of the scope, independently of SCC stack mutations.
During normal solving, the active SCC on the calculation stack should also
retain the generation. The scope owns an independent `Rc` so memory safety
does not rely on that stack invariant or require extending a reference with
`unsafe`.

Cross-module answer providers are also retained for the scope lifetime. The
transaction owns every referenced module allocation, so its `ArcId` remains
unique while it is used as the cache key.

---

## AnswersSolver

`struct` · `pyrefly::alt::answers_solver::AnswersSolver`

```rust
struct AnswersSolver<'ctx, 'answer, Ans: LookupAnswer>
```

**Fields**: `exports`, `uniques`, `recurser`, `stdlib`, `heap`

**Methods** (352)

```rust
fn add_implicit_any_error(errors: &ErrorCollector, range: TextRange, generic_entity: String, tparam_name: Option<&str>)
fn add_specialization_errors(&self, specialization_errors: Vec1<TypeVarSpecializationError>, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>)
fn apply_special_form(&self, special_form: SpecialForm, arguments: &Expr, range: TextRange, type_form_context: TypeFormContext<'_>, errors: &ErrorCollector) -> Type
fn as_bool(&self, ty: &Type, range: TextRange, errors: &ErrorCollector) -> Option<bool>
fn as_call_target(&self, ty: Type) -> CallTargetLookup
fn as_call_target_or_error(&self, ty: Type, call_style: CallStyle<'_>, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> CallTarget
fn as_class_info(&self, ty: Type) -> Vec<Type>
fn as_class_type_unchecked(&self, class: &Class) -> ClassType
fn as_enum_member(&self, field: &ClassField, enum_cls: &Class) -> Option<Lit>
fn as_param(&self, field: &ClassField, name: &Name, default: bool, kw_only: bool, strict: bool, converter_param: Option<Type>, param_type_transform: &dyn Fn(Type) -> Type, errors: &ErrorCollector) -> Param
fn as_superclass(&self, class: &ClassType, want: &Class) -> Option<ClassType>
fn as_tuple(&self, cls: &ClassType) -> Option<Tuple>
fn as_typed_dict_unchecked(&self, class: &Class) -> TypedDict
fn async_iterate(&self, iterable: &Type, range: TextRange, errors: &ErrorCollector) -> Vec<Iterable>
fn atomic_narrow_for_facet(&self, base: &Type, facet: &FacetKind, op: &AtomicNarrowOp, allow_never_collapse: bool, range: TextRange, errors: &ErrorCollector) -> Option<Type>
fn attr_infer(&self, base: &TypeInfo, attr_name: &Name, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> TypeInfo
fn attr_infer_for_type(&self, base: &Type, attr_name: &Name, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn attribute_expr_infer(&self, x: &Expr, annotation: Option<&Annotation>, name: &Name, errors: &ErrorCollector) -> Type
fn augassign_infer(&self, ann: Option<Idx<KeyAnnotation>>, x: &StmtAugAssign, errors: &ErrorCollector) -> Type
fn base_errors(&self) -> &ErrorCollector
fn behaves_like_any(&self, ty: &Type) -> bool
fn bind_boundmethod(&self, m: &BoundMethod, is_subset: &mut dyn FnMut(&Type, &Type) -> bool) -> Option<Type>
fn bind_dunder_init(&self, t: Type, cls: &ClassType) -> Type
fn bind_dunder_new(&self, t: &Type, cls: ClassType) -> Option<Type>
fn bindings(&self) -> &'answer Bindings
fn binop_infer(&self, x: &ExprBinOp, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector, type_form_context: Option<TypeFormContext<'_>>) -> Type
fn build_pydantic_lax_conversion_table(&self, field_types: &[Type]) -> ConverterMap
fn calculate_abstract_members(&self, cls: &Class) -> AbstractClassMembers
fn calculate_class_disjoint_base(&self, cls: &Class, errors: &ErrorCollector) -> ClassDisjointBase
fn calculate_class_field(&self, class: &Class, name: &Name, range: TextRange, field_definition: &ClassFieldDefinition, functional_class_def: bool, errors: &ErrorCollector) -> ClassField
fn calculate_class_mro(&self, cls: &Class, errors: &ErrorCollector) -> ClassMro
fn calculate_class_tparams(&self, name: &Identifier, scoped_type_params: Option<&TypeParams>, generic_bases: &[BaseClassGeneric], legacy: &[Idx<KeyLegacyTypeParam>], errors: &ErrorCollector) -> TParams
fn calculate_class_tparams_no_legacy(&self, name: &Identifier, scoped_type_params: Option<&TypeParams>, errors: &ErrorCollector) -> TParams
fn calculate_subscript_symmetry(&self, cls: &Class) -> bool
fn calculate_typed_dict_field(&self, metadata: &ClassMetadata, name: &Name, range: TextRange, field_definition: &ClassFieldDefinition, errors: &ErrorCollector) -> ClassField
fn call_assert_shape(&self, args: &[Expr], keywords: &[Keyword], range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn call_assert_type(&self, args: &[Expr], keywords: &[Keyword], range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn call_dataclasses_asdict(&self, asdict_ty: &Type, args: &[CallArg<'_>], kws: &[CallKeyword<'_>], callee_range: TextRange, arg_range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn call_dataclasses_replace(&self, kind: ReplaceKind, replace_ty: &Type, args: &[CallArg<'_>], kws: &[CallKeyword<'_>], callee_range: TextRange, arg_range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn call_descriptor_getter(&self, getter_method: Type, base: DescriptorBase, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn call_descriptor_setter(&self, setter_method: Type, base: DescriptorBase, got: CallArg<'_>, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn call_functools_partial(&self, partial_ty: &Type, args: &[CallArg<'_>], kws: &[CallKeyword<'_>], callee_range: TextRange, arg_range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn call_getattr_or_delattr(&self, getattr_ty: Type, attr_name: Name, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn call_infer(&self, call_target: CallTarget, args: &[CallArg<'_>], keywords: &[CallKeyword<'_>], arguments_range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>, hint: Option<HintRef<'_, '_>>, ctor_targs: Option<&mut TArgs>) -> Type
fn call_isinstance(&self, obj: &Expr, class_or_tuple: &Expr, errors: &ErrorCollector) -> Type
fn call_issubclass(&self, cls: &Expr, class_or_tuple: &Expr, errors: &ErrorCollector) -> Type
fn call_len(&self, args: &[CallArg<'_>], callee_ty: Type, keywords: &[CallKeyword<'_>], func_range: TextRange, arguments_range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn call_magic_dunder_method(&self, ty: &Type, method_name: &Name, range: TextRange, args: &[CallArg<'_>], keywords: &[CallKeyword<'_>], errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Option<Type>
fn call_method_or_error(&self, ty: &Type, method_name: &Name, range: TextRange, args: &[CallArg<'_>], keywords: &[CallKeyword<'_>], errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn call_overloads(&self, overloads: Vec1<TargetWithTParams<Function>>, metadata: &FuncMetadata, shape_transform: Option<&ShapeTransform>, self_obj: Option<Type>, args: &[CallArg<'_>], keywords: &[CallKeyword<'_>], arguments_range: TextRange, errors: &ErrorCollector, return_errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>, hint: Option<HintRef<'_, '_>>, ctor_targs: Option<&mut TArgs>) -> (Type, Callable)
fn call_property_getter(&self, getter_method: Type, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn call_property_setter(&self, setter_method: Type, got: CallArg<'_>, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn call_reveal_type(&self, args: &[Expr], keywords: &[Keyword], range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn call_setattr(&self, setattr_ty: Type, arg: CallArg<'_>, attr_name: Name, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn call_typeform(&self, args: &[Expr], keywords: &[Keyword], range: TextRange, errors: &ErrorCollector) -> Type
fn call_typing_cast(&self, args: &[Expr], keywords: &[Keyword], range: TextRange, errors: &ErrorCollector) -> Type
fn callable_infer(&self, callable: Callable, callable_name: Option<&FunctionKind>, shape_transform: Option<&ShapeTransform>, tparams: Option<&TParams>, self_obj: Option<Type>, args: &[CallArg<'_>], keywords: &[CallKeyword<'_>], arguments_range: TextRange, arg_errors: &ErrorCollector, call_errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>, hint: Option<HintRef<'_, '_>>, ctor_targs: Option<&mut TArgs>) -> (Type, Vec<TypeVarSpecializationError>, Vec<ReturnTypeResolutionError>, ArgMap)
fn callable_infer_with_hint<R>(&self, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector, inner: impl FnMut(Option<&Type>, &ErrorCollector) -> R, result_type: impl Fn(&R) -> &Type) -> R
fn canonicalize_all_class_types(&self, ty: Type, range: TextRange, errors: &ErrorCollector) -> Type
fn check_and_return_type(&self, got: Type, want: &Type, loc: TextRange, errors: &ErrorCollector, tcc: &dyn Fn() -> TypeCheckContext) -> Type
fn check_assign_to_attribute_and_infer_narrow(&self, base: &Type, name: &Name, got: &ExprOrBinding, allow_assign_to_final: bool, range: TextRange, errors: &ErrorCollector) -> Option<Type>
fn check_attr_delete(&self, base: &TypeInfo, attr_name: &Name, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>, todo_ctx: &str)
fn check_class_attr_delete(&self, class_attr: ClassAttribute, attr_name: &Name, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>)
fn check_class_attr_set_and_infer_narrow(&self, class_attr: ClassAttribute, instance_class: Option<&ClassType>, class_base: Option<&ClassBase>, attr_name: &Name, got: TypeOrExpr<'_>, allow_assign_to_final: bool, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>, should_narrow: &mut bool, narrowed_types: &mut Vec<Type>)
fn check_consistent_multiple_inheritance(&self, cls: &Class, errors: &ErrorCollector)
fn check_consistent_override_for_field(&self, cls: &Class, field_name: &Name, class_field: &ClassField, bases: &ClassBases, errors: &ErrorCollector)
fn check_dict_items_against_typed_dict(&self, dict_items: &Vec<&DictItem>, typed_dict: &TypedDict, is_update: bool, range: TextRange, check_errors: &ErrorCollector, item_errors: &ErrorCollector)
fn check_dunder_bool_is_callable(&self, type_of_term_used_as_bool: &Type, range: TextRange, errors: &ErrorCollector)
fn check_final_reassignment(&self, annot: &AnnotationWithTarget, range: TextRange, errors: &ErrorCollector)
fn check_implicit_bool(&self, condition_type: &Type, range: TextRange, errors: &ErrorCollector)
fn check_match_case_reachability(&self, subject_idx: &Idx<Key>, narrowing_subject: Option<&NarrowingSubject>, narrow_ops_for_case: &(Box<NarrowOp>, TextRange), case_range: &TextRange, errors: &ErrorCollector)
fn check_match_exhaustiveness(&self, subject_idx: &Idx<Key>, narrowing_subject: Option<&NarrowingSubject>, narrow_ops_for_fall_through: &(Box<NarrowOp>, TextRange), subject_range: &TextRange, show_subject_expr: bool, errors: &ErrorCollector)
fn check_pydantic_argument_range_constraints(&self, cls: &Class, dataclass: &DataclassMetadata, args: &[CallArg<'_>], keywords: &[CallKeyword<'_>], errors: &ErrorCollector)
fn check_pydantic_range_constraints(&self, field_name: &Name, field_ty: &Type, keywords: &DataclassFieldKeywords, range: TextRange, errors: &ErrorCollector)
fn check_redundant_condition(&self, condition_type: &Type, range: TextRange, errors: &ErrorCollector)
fn check_set_read_write_and_infer_narrow(&self, attr_ty: Type, attr_name: &Name, got: TypeOrExpr<'_>, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>, should_narrow: bool, narrowed_types: &mut Vec<Type>)
fn check_type(&self, got: &Type, want: &Type, loc: TextRange, errors: &ErrorCollector, tcc: &dyn Fn() -> TypeCheckContext) -> bool
fn check_type_as_call_argument(&self, got: &Type, want: &Type, loc: TextRange, errors: &ErrorCollector, tcc: &dyn Fn() -> TypeCheckContext) -> bool
fn check_type_with_options(&self, got: &Type, want: &Type, loc: TextRange, options: TypeCheckOptions<'_, '_>) -> Option<SubsetError>
fn check_variance_violations(&self, class: &Class, class_bases: &ClassBases, field_map: &SmallMap<Name, &ClassField>) -> Vec<VarianceViolation>
fn class_bases_of(&self, cls: &Class, bases: &[BaseClass], is_new_type: bool, errors: &ErrorCollector) -> ClassBases
fn class_definition(&self, def_index: ClassDefIndex, x: &ClassDefData, parent: &NestingContext, is_protocol: bool, tparams_require_binding: bool, errors: &ErrorCollector) -> Class
fn class_instances_always_truthy(&self, cls: &Class) -> bool
fn class_metadata_of(&self, cls: &Class, bases: &[BaseClass], raw_keywords: &[Keyword], decorators: &[Idx<KeyDecorator>], is_new_type: bool, pydantic_config_dict: &PydanticConfigDict, pydantic_before_validator_fields: &[Name], django_field_info: &DjangoFieldInfo, capture_init: Option<&[Name]>, shaped_array_metadata: Option<&ShapedArrayDecoratorMetadata>, errors: &ErrorCollector) -> ClassMetadata
fn class_overrides_tuple_getitem(&self, cls: &ClassType) -> bool
fn class_self_param(&self, cls: &Class, posonly: bool) -> Param
fn collect_jaxtyping_tparams(&self, callable: &impl Visit<Type>, tparams: &Arc<TParams>, name_range: TextRange, errors: &ErrorCollector) -> Arc<TParams>
fn compare_infer(&self, x: &ExprCompare, errors: &ErrorCollector) -> Type
fn completions(&self, base: Type, expected_attribute_name: Option<&Name>, include_types: bool) -> Vec<AttrInfo>
fn compute_dataclass_field_initialization(&self, call: &ExprCall, field_name: &Name, annotated_field_ty: Option<&Type>, dm: &DataclassMetadata) -> Option<DataclassFieldKeywords>
fn compute_kw_only_fields_by_class(&self, cls: &Class) -> SmallMap<Class, SmallSet<Name>>
fn compute_variance(&self, class: &Class) -> VarianceMap
fn constructor_to_callable(&self, cls: &ClassType) -> Type
fn constructor_to_callable_for_class_def(&self, cls: &Class) -> Type
fn create_recursive(&self, binding: &Binding) -> Var
fn create_type_alias_params_recursive(&self, tparams: &TypeAliasParams, anchor: TextRange) -> Arc<TParams>
fn current(&self) -> &'answer Answers
fn dataclass_field_keywords(&self, func: &Type, field_name: &Name, args: &Arguments, annotated_field_ty: Option<&Type>, dataclass_metadata: &DataclassMetadata, errors: &ErrorCollector) -> DataclassFieldKeywords
fn decompose_async_generator(&self, ty: &Type) -> Option<(Type, Type)>
fn decompose_dict(&self, hint: &Type) -> (Option<Type>, Option<Type>)
fn decompose_generator(&self, ty: &Type) -> Option<(Type, Type, Type)>
fn decompose_hint<'b, D>(&self, hint: HintRef<'_, 'b>, decompose: impl Fn(&'b Type) -> Option<D>) -> Vec<D>
fn decompose_set(&self, hint: &Type) -> Option<Type>
fn decompose_tuple(&self, hint: &Type) -> Option<Type>
fn decorated_function_type(&self, def: &UndecoratedFunction, stmt: &FunctionDefData, errors: &ErrorCollector) -> Type
fn disjoint_base(&self, t: &Type) -> Class
fn distribute_over_union(&self, ty: &Type, f: impl FnMut(&Type) -> Type) -> Type
fn django_reverse_relations_index(&self) -> &'answer DjangoReverseRelationIndex
fn erase_tuple_type(&self, tuple: Tuple) -> ClassType
fn error(&self, errors: &ErrorCollector, range: TextRange, kind: ErrorKind, msg: String) -> Type
fn error_collector(&self) -> ErrorCollector
fn error_swallower(&self) -> ErrorCollector
fn error_with_context(&self, errors: &ErrorCollector, range: TextRange, kind: ErrorKind, msg: String, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn expand_mut(&self, ty: &mut Type)
fn expr_call_infer(&self, x: &ExprCall, callee_ty: Type, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn expr_check(&self, x: &Expr, check: Option<(&Type, &dyn Fn() -> TypeCheckContext)>, errors: &ErrorCollector) -> Type
fn expr_class_keyword(&self, x: &Expr, errors: &ErrorCollector) -> Annotation
fn expr_infer(&self, x: &Expr, errors: &ErrorCollector) -> Type
fn expr_infer_with_hint(&self, x: &Expr, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn expr_untype(&self, x: &Expr, type_form_context: TypeFormContext<'_>, errors: &ErrorCollector) -> Type
fn expr_with_options(&self, x: &Expr, options: ExprOptions<'_, '_, '_>) -> TypeInfo
fn extends_any(&self, cls: &Class) -> bool
fn extract_pydantic_field_from_annotation(&self, annot: Idx<KeyAnnotation>, field_name: &Name, metadata: &ClassMetadata) -> Option<DataclassFieldKeywords>
fn extract_root_model_inner_type(&self, ty: &Type) -> Option<Type>
fn field_defining_class_matches(&self, cls: &Class, field: &Name, predicate: impl FnOnce(&Class) -> bool) -> bool
fn field_is_inherited_from(&self, cls: &Class, field: &Name, ancestor: (&str, &str)) -> bool
fn finish_quantified(&self, vs: QuantifiedHandle, infer_with_first_use: bool) -> Result<(), Vec1<TypeVarSpecializationError>>
fn for_display(&self, t: Type) -> Type
fn force_for_narrowing(&self, ty: &Type, range: TextRange, errors: &ErrorCollector) -> Type
fn freeform_call_infer(&self, ty: Type, args: &[CallArg<'_>], kws: &[CallKeyword<'_>], callee_range: TextRange, arg_range: TextRange, hint: Option<HintRef<'_, '_>>, errors: &ErrorCollector) -> Type
fn functional_class_definition(&self, def_index: ClassDefIndex, name: &Identifier, parent: &NestingContext) -> Class
fn get<K: Solve<Ans>>(&self, k: &K) -> &'answer K::Answer where AnswerTable: TableKeyed<K, Value = AnswerEntry<K>>, BindingTable: TableKeyed<K, Value = BindingEntry<K>>, SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_abstract_members_for_class(&self, cls: &Class) -> &AbstractClassMembers
fn get_annotated_metadata(&self, expr: &Expr, type_form_context: TypeFormContext<'_>, errors: &ErrorCollector) -> Vec<Expr>
fn get_base_types_for_class(&self, cls: &Class) -> &ClassBases
fn get_bounded_quantified_attribute(&self, quantified: Quantified, upper_bound: &ClassType, name: &Name) -> Option<ClassAttribute>
fn get_bounded_quantified_class_attribute(&self, quantified: Quantified, class: &ClassType, name: &Name) -> Option<ClassAttribute>
fn get_class_attribute(&self, cls: &ClassBase, name: &Name) -> Option<ClassAttribute>
fn get_class_field_map(&self, cls: &Class) -> SmallMap<Name, &ClassField>
fn get_class_fields(&self, cls: &Class) -> Option<&ClassFields>
fn get_class_tparams<'b>(&'b self, class: &'b Class) -> Option<&'b Arc<TParams>>
fn get_dataclass_fields(&self, cls: &Class, bases_with_metadata: &[(Class, &ClassMetadata)], kind: &DataclassKind) -> SmallSet<Name>
fn get_dataclass_member(&self, cls: &Class, name: &Name) -> DataclassMember<'_>
fn get_dataclass_synthesized_fields(&self, cls: &Class, errors: &ErrorCollector) -> Option<ClassSynthesizedFields>
fn get_disjoint_base_for_class(&self, cls: &Class) -> &ClassDisjointBase
fn get_django_enum_synthesized_fields(&self, cls: &Class) -> Option<ClassSynthesizedFields>
fn get_django_field_type(&self, ty: &Type, class: &Class, field_name: Option<&Name>, initial_value_expr: Option<&Expr>) -> Option<Type>
fn get_django_model_synthesized_fields(&self, cls: &Class) -> Option<ClassSynthesizedFields>
fn get_dunder_init(&self, cls: &ClassType, get_object_init: bool) -> Option<Type>
fn get_dunder_new(&self, cls: &ClassType, preserve_self: bool) -> Option<Type>
fn get_enum_class_field_type(&self, class: &Class, name: &Name, direct_annotation: Option<&Annotation>, ty: &Type, field_definition: &ClassFieldDefinition, is_descriptor: bool, range: TextRange, errors: &ErrorCollector) -> Option<Type>
fn get_enum_from_class(&self, cls: &Class) -> Option<EnumMetadata>
fn get_enum_literal_or_instance_attribute(&self, lit: &LitEnum, metadata: &ClassMetadata, attr_name: &Name) -> Option<ClassAttribute>
fn get_enum_member(&self, cls: &Class, name: &Name) -> Option<Lit>
fn get_enum_member_count(&self, cls: &Class) -> Option<usize>
fn get_enum_members(&self, cls: &Class) -> SmallSet<Lit>
fn get_enum_or_instance_attribute(&self, class: &ClassType, metadata: &ClassMetadata, attr_name: &Name) -> Option<ClassAttribute>
fn get_factory_boy_synthesized_fields(&self, cls: &Class) -> Option<ClassSynthesizedFields>
fn get_field_from_current_class_only(&self, cls: &Class, name: &Name) -> Option<&ClassField>
fn get_from_class<K: Solve<Ans> + Exported>(&self, cls: &Class, k: &K) -> Option<&'answer K::Answer> where AnswerTable: TableKeyed<K, Value = AnswerEntry<K>>, BindingTable: TableKeyed<K, Value = BindingEntry<K>>, SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_from_export(&self, module: ModuleName, path: Option<&ModulePath>, k: &KeyExport) -> &'answer Type
fn get_hashed<K: Solve<Ans>>(&self, k: Hashed<&K>) -> &'answer K::Answer where AnswerTable: TableKeyed<K, Value = AnswerEntry<K>>, BindingTable: TableKeyed<K, Value = BindingEntry<K>>, SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_hashed_opt<K: Solve<Ans>>(&self, k: Hashed<&K>) -> Option<&'answer K::Answer> where AnswerTable: TableKeyed<K, Value = AnswerEntry<K>>, BindingTable: TableKeyed<K, Value = BindingEntry<K>>, SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_idx<K: Solve<Ans>>(&self, idx: Idx<K>) -> &'answer K::Answer where AnswerTable: TableKeyed<K, Value = AnswerEntry<K>>, BindingTable: TableKeyed<K, Value = BindingEntry<K>>, SolutionsTable: TableKeyed<K, Value = SolutionsEntry<K>>
fn get_instance_attribute(&self, cls: &ClassType, name: &Name) -> Option<ClassAttribute>
fn get_literal_string_attribute(&self, name: &Name) -> Option<ClassAttribute>
fn get_metaclass_attribute(&self, cls: &ClassBase, metaclass: &ClassType, name: &Name) -> Option<ClassAttribute>
fn get_metaclass_dunder_call(&self, cls: &ClassType) -> Option<Type>
fn get_metadata_for_class(&self, cls: &Class) -> &ClassMetadata
fn get_mro_for_class(&self, cls: &Class) -> &ClassMro
fn get_named_tuple_elements(&self, cls: &Class, errors: &ErrorCollector) -> SmallSet<Name>
fn get_named_tuple_synthesized_fields(&self, cls: &Class) -> Option<ClassSynthesizedFields>
fn get_new_type_synthesized_fields(&self, cls: &Class) -> Option<ClassSynthesizedFields>
fn get_nn_module_synthesized_fields(&self, cls: &Class, registrations: &SmallMap<Name, Vec<Expr>>) -> Option<ClassSynthesizedFields>
fn get_non_synthesized_class_member(&self, cls: &Class, name: &Name) -> Option<Cow<'_, ClassField>>
fn get_non_synthesized_class_member_and_defining_class(&self, cls: &Class, name: &Name) -> Option<WithDefiningClass<Cow<'_, ClassField>>>
fn get_non_synthesized_field_from_current_class_only(&self, cls: &Class, name: &Name) -> Option<&ClassField>
fn get_or_create_jaxtyping_dimension(&self, name: Name, kind: QuantifiedKind) -> Quantified
fn get_or_create_jaxtyping_variadic_shape(&self, name: Name, kind: QuantifiedKind) -> Quantified
fn get_produced_type(&self, iterables: Vec<Iterable>) -> Type
fn get_protocol_attribute(&self, cls: &ClassType, self_type: Type, name: &Name) -> Option<ClassAttribute>
fn get_pydantic_root_model_class_field_type(&self, cls: &Class, attr_name: &Name) -> Option<Type>
fn get_pydantic_root_model_init(&self, cls: &Class, root_model_type: Type, has_strict: bool) -> ClassSynthesizedField
fn get_pydantic_root_model_type_via_mro(&self, class: &Class, metadata: &ClassMetadata) -> Option<(Type, bool)>
fn get_self_attribute(&self, cls: &ClassType, name: &Name) -> Option<ClassAttribute>
fn get_shaped_array_attribute(&self, shaped_array: &ShapedArrayType, name: &Name) -> Option<ClassAttribute>
fn get_special_decorator<'a>(&'a self, decorator: &'a Decorator) -> Option<SpecialDecorator<'a>>
fn get_subscript_symmetry_for_class(&self, cls: &Class) -> bool
fn get_super_attribute(&self, start_lookup_cls: &ClassType, super_obj: &SuperObj, name: &Name) -> Option<ClassAttribute>
fn get_super_class_member(&self, cls: &Class, start_lookup_cls: Option<&ClassType>, name: &Name) -> Option<WithDefiningClass<Cow<'_, ClassField>>>
fn get_total_ordering_synthesized_fields(&self, errors: &ErrorCollector, cls: &Class) -> Option<ClassSynthesizedFields>
fn get_type_alias<'b>(&self, data: &'b TypeAliasData) -> Cow<'b, TypeAlias> where 'answer: 'b
fn get_typed_dict_attribute(&self, td: &TypedDictInner, name: &Name) -> Option<ClassAttribute>
fn get_typed_dict_dunder_init(&self, td: &TypedDictInner) -> Type
fn get_typed_dict_synthesized_fields(&self, cls: &Class) -> Option<ClassSynthesizedFields>
fn get_typed_dict_value_type(&self, typed_dict: &TypedDict) -> Type
fn get_typed_dict_value_type_as_builtins_dict(&self, typed_dict: &TypedDict) -> Option<Type>
fn has_attr(&self, base: &Type, attr_name: &Name) -> bool
fn has_django_field_choices(&self, call_expr: &ExprCall) -> bool
fn has_static_attr(&self, base: &Type, attr_name: &Name) -> bool
fn has_superclass(&self, class: &Class, want: &Class) -> bool
fn hasattr_narrow_type(&self, base: &Type, attr_name: &Name, range: TextRange, errors: &ErrorCollector) -> Option<Type>
fn infer_int_tuple_subscript(&self, int_tuple: &IntTuple, index: &Expr, index_ty: Option<&Type>, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn infer_tuple_subscript(&self, tuple: Tuple, index: &Expr, index_ty: Option<&Type>, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn infer_variance_ignoring_declared(&self, class: &Class) -> VarianceMap
fn infer_with_decomposed_hint<D>(&self, hint: Option<HintRef<'_, '_>>, decompose: impl Fn(&Type) -> Option<D>, infer: impl Fn(Option<D>, Option<HintRef<'_, '_>>) -> Type) -> Type
fn instance_as_dunder_call(&self, cls: &ClassType) -> Option<Type>
fn instantiate(&self, cls: &Class) -> Type
fn instantiate_fresh_callable(&self, tparams: &TParams, c: Callable) -> (QuantifiedHandle, Callable)
fn instantiate_fresh_class(&self, cls: &Class) -> (QuantifiedHandle, Type)
fn instantiate_fresh_forall(&self, forall: Forall<Forallable>) -> (QuantifiedHandle, Type)
fn instantiate_fresh_function(&self, tparams: &TParams, func: Function) -> (QuantifiedHandle, Function)
fn instantiate_type_var_tuple(&self) -> (TParams, Type)
fn instantiate_unbounded_tuple(&self) -> (TParams, Type)
fn intersects(&self, ts: &[Type]) -> Type
fn is_class_attribute_subset(&self, got: &ClassAttribute, want: &ClassAttribute, allow_property_override: bool, is_subset: &mut dyn FnMut(&Type, &Type) -> Result<(), SubsetError>) -> Result<(), Box<AttrSubsetError>>
fn is_compatible_constructor_return(&self, ty: &Type, class: &Class) -> bool
fn is_consistent(&self, got: &Type, want: &Type) -> bool
fn is_coroutine(&self, ty: &Type) -> bool
fn is_debug(&self) -> bool
fn is_dict_like(&self, ty: &Type) -> bool
fn is_equivalent(&self, got: &Type, want: &Type) -> bool
fn is_foreign_key_like_field(&self, field: &Class) -> bool
fn is_jaxtyping_quantified(&self, q: &Quantified) -> bool
fn is_jaxtyping_wrapper_expr(&self, expr: &Expr) -> bool
fn is_many_to_many_field(&self, field: &Class) -> bool
fn is_protocol_subset_at_attr(&self, got: &Type, protocol: &ClassType, attr_name: &Name, is_subset: &mut dyn FnMut(&Type, &Type) -> Result<(), SubsetError>) -> Result<(), SubsetError>
fn is_provably_disjoint(&self, left: &Type, right: &Type) -> bool
fn is_pydantic_strict_metadata(&self, ty: &Type) -> bool
fn is_sequence_for_pattern(&self, ty: &Type) -> bool
fn is_subclassable(&self, class: &Class) -> bool
fn is_subset_eq(&self, got: &Type, want: &Type) -> bool
fn is_subset_eq_with_reason(&self, got: &Type, want: &Type) -> Result<(), SubsetError>
fn iter_fields(&self, cls: &Class, dataclass: &DataclassMetadata, include_initvar: bool, kw_only_fields_by_class: &SmallMap<Class, SmallSet<Name>>) -> Vec<(Name, ClassField, DataclassFieldKeywords)>
fn iterate(&self, iterable: &Type, range: TextRange, errors: &ErrorCollector, orig_context: Option<&dyn Fn() -> ErrorContext>) -> Vec<Iterable>
fn literal_typed_dict_key_name(&self, ty: &Type) -> Option<Name>
fn map_over_union(&self, ty: &Type, f: impl FnMut(&Type))
fn maybe_apply_function_decorator(&self, callee: &Type, args: &[CallArg<'_>], kws: &[CallKeyword<'_>], errors: &ErrorCollector) -> Option<Type>
fn module(&self) -> &pyrefly_python::module::Module
fn narrow(&self, type_info: &TypeInfo, op: &NarrowOp, range: TextRange, errors: &ErrorCollector) -> TypeInfo
fn narrowable_for_attr(&self, base: &Type, attr_name: &Name, range: TextRange, errors: &ErrorCollector) -> Type
fn paramspec_from_call(&self, name: Identifier, x: &ExprCall, errors: &ErrorCollector) -> ParamSpec
fn parse_assert_shape_expr(&self, expr: &Expr, errors: &ErrorCollector) -> Option<IntTuple>
fn parse_jaxtyping_type_form(&self, value: &Expr, slice: &Expr, range: TextRange, errors: &ErrorCollector) -> Option<Type>
fn polars_column_name(&self, expr: &Expr) -> Option<Name>
fn polars_dataframe_annotated_type(&self, inner: &Type, metadata: &[Type]) -> Option<Type>
fn polars_select_columns(&self, schema: &DataFrameSchema, elts: &[Expr], errors: &ErrorCollector) -> Option<Type>
fn promote(&self, cls: &Class, range: TextRange, errors: &ErrorCollector) -> Type
fn promote_forall(&self, forall: Forall<Forallable>, range: TextRange, errors: &ErrorCollector) -> Type
fn promote_nontypeddict_silently_to_classtype(&self, cls: &Class) -> ClassType
fn promote_silently(&self, cls: &Class) -> Type
fn pydantic_config(&self, bases_with_metadata: &[(Class, &ClassMetadata)], pydantic_config_dict: &PydanticConfigDict, keywords: &[(Name, Annotation)], decorators: &[(&Decorator, TextRange)], errors: &ErrorCollector, range: TextRange) -> Option<PydanticConfig>
fn quantified_instance_as_dunder_call(&self, quantified: Quantified, upper_bound: &ClassType) -> Option<Type>
fn record_attribute_definition_index(&self, base: &Type, attribute_name: &Name, attribute_reference_range: TextRange, reference_kind: AttributeReferenceKind)
fn record_property_getter(&self, loc: TextRange, getter_ty: &Type)
fn record_recursive(&self, ty: Type, recursive: Var) -> Type
fn record_resolved_trace(&self, loc: TextRange, ty: &Type)
fn record_type_trace(&self, loc: TextRange, ty: &Type)
fn recurse<'a>(&'a self, var: Var) -> Option<Guard<'a, Var>>
fn report_forbidden_dataclass_target(&self, name: &Name, is_protocol: bool, is_enum: bool, is_typed_dict: bool, is_named_tuple: bool, range: TextRange, errors: &ErrorCollector) -> bool
fn resolve_facet_chain(&self, unresolved: UnresolvedFacetChain) -> Option<FacetChain>
fn resolve_facet_kind(&self, unresolved: UnresolvedFacetKind) -> Option<FacetKind>
fn resolve_get_class_attr(&self, attr_name: &Name, class_attr: ClassAttribute, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>) -> Result<Type, NoAccessReason>
fn resolve_named_tuple_element(&self, cls: &ClassType, name: &Name) -> Option<Type>
fn scoped_type_params(&self, x: Option<&TypeParams>, errors: &ErrorCollector) -> Vec<Quantified>
fn self_as_dunder_call(&self, cls: &ClassType) -> Option<Type>
fn sentinel_from_call(&self, assignment_name: Identifier, nesting_context: NestingContext, x: &ExprCall, errors: &ErrorCollector) -> Sentinel
fn set_debug(&self, value: bool)
fn set_flag_from_special_decorator(&self, flags: &mut FuncFlags, decorator: &SpecialDecorator<'_>) -> bool
fn shaped_array_shape_for_class(&self, cls: &Class) -> Option<Quantified>
fn shaped_array_shape_for_class_type(&self, cls: &ClassType) -> Option<Quantified>
fn show_binding(&self, binding: &Binding) -> String
fn show_binding_for<K: Keyed>(&self, idx: Idx<K>) -> String where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn show_binding_for_with<K: Keyed>(&self, idx: Idx<K>, bindings: &Bindings) -> String where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn show_binding_generic<K>(&self, binding: &K::Value) -> String where K: Keyed, BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn show_current_binding(&self) -> String
fn show_current_cycle(&self) -> impl Iterator<Item = String>
fn show_current_idx(&self) -> String
fn show_current_module(&self) -> String
fn show_current_stack(&self) -> impl Iterator<Item = String>
fn show_idx<K>(&self, idx: Idx<K>) -> String where K: Keyed, BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn show_idx_with<K>(&self, idx: Idx<K>, bindings: &Bindings) -> String where K: Keyed, BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn solve_abstract_members(&self, cls: &Class, errors: &ErrorCollector) -> AbstractClassMembers
fn solve_annotation(&self, binding: &BindingAnnotation, errors: &ErrorCollector) -> AnnotationWithTarget
fn solve_binding(&self, binding: &Binding, range: TextRange, errors: &ErrorCollector) -> TypeInfo
fn solve_class(&self, cls: &BindingClass, errors: &ErrorCollector) -> NoneIfRecursive<Class>
fn solve_class_base_type(&self, binding: &BindingClassBaseType, errors: &ErrorCollector) -> ClassBases
fn solve_class_checks(&self, binding: &BindingClassChecks, errors: &ErrorCollector) -> EmptyAnswer
fn solve_class_disjoint_base(&self, binding: &BindingClassDisjointBase, errors: &ErrorCollector) -> ClassDisjointBase
fn solve_class_field(&self, field: &BindingClassField, errors: &ErrorCollector) -> ClassField
fn solve_class_metadata(&self, binding: &BindingClassMetadata, errors: &ErrorCollector) -> ClassMetadata
fn solve_class_mro(&self, binding: &BindingClassMro, errors: &ErrorCollector) -> ClassMro
fn solve_class_synthesized_fields(&self, errors: &ErrorCollector, binding: &BindingClassSynthesizedFields) -> ClassSynthesizedFields
fn solve_decorated_function(&self, x: &BindingDecoratedFunction, errors: &ErrorCollector) -> Type
fn solve_decorator(&self, x: &BindingDecorator, errors: &ErrorCollector) -> Decorator
fn solve_django_reverse_relations(&self, binding: &BindingDjangoRelations, _range: TextRange, _errors: &ErrorCollector) -> DjangoReverseRelationIndex
fn solve_expectation(&self, binding: &BindingExpect, range: TextRange, errors: &ErrorCollector) -> EmptyAnswer
fn solve_function_binding(&self, idx: Idx<KeyDecoratedFunction>, predecessor: &mut Option<Idx<Key>>, errors: &ErrorCollector) -> Type
fn solve_legacy_tparam(&self, binding: &BindingLegacyTypeParam, scope_anchor: TextRange) -> LegacyTypeParameterLookup
fn solve_tparams(&self, binding: &BindingTParams, errors: &ErrorCollector) -> TParams
fn solve_type_alias(&self, binding: &BindingTypeAlias, errors: &ErrorCollector) -> TypeAlias
fn solve_undecorated_function(&self, x: &BindingUndecoratedFunction, errors: &ErrorCollector) -> UndecoratedFunction
fn solve_variance_binding(&self, variance_info: &BindingVariance, _errors: &ErrorCollector) -> VarianceMap
fn solve_yield(&self, x: &BindingYield, errors: &ErrorCollector) -> YieldResult
fn solve_yield_from(&self, x: &BindingYieldFrom, errors: &ErrorCollector) -> YieldFromResult
fn solver(&self) -> &Solver
fn specialize(&self, cls: &Class, targs: Vec<Type>, range: TextRange, errors: &ErrorCollector) -> Type
fn specialize_forall(&self, forall: Forall<Forallable>, targs: Vec<Type>, range: TextRange, errors: &ErrorCollector) -> Type
fn specialize_forall_in_base_class(&self, forall: Forall<Forallable>, targs: Vec<Type>, range: TextRange, errors: &ErrorCollector) -> Type
fn specialize_in_base_class(&self, cls: &Class, targs: Vec<Type>, range: TextRange, errors: &ErrorCollector) -> Type
fn stack(&self) -> &CalcStack
fn subscript_bytes_literal(&self, bytes: &[u8], index_expr: &Expr, errors: &ErrorCollector, range: TextRange, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn subscript_infer(&self, base: &TypeInfo, slice: &Expr, range: TextRange, type_form_context: TypeFormContext<'_>, errors: &ErrorCollector) -> TypeInfo
fn subscript_infer_for_type(&self, base: &Type, slice: &Expr, range: TextRange, errors: &ErrorCollector) -> Type
fn subscript_str_literal(&self, value: &str, base_type: &Type, index_expr: &Expr, errors: &ErrorCollector, range: TextRange, context: Option<&dyn Fn() -> ErrorContext>) -> Type
fn suggest_enum_member_for_value(&self, got: &Type, want: &Type) -> Option<String>
fn try_get_from_export(&self, module: ModuleName, attr: Name) -> Option<&'answer Type>
fn try_nn_module_dict_attr(&self, class: &ClassType, attr_name: &Name) -> Option<ClassAttribute>
fn try_nn_module_dict_index(&self, cls: &ClassType, base: &Type, slice: &Expr, range: TextRange, errors: &ErrorCollector) -> Type
fn try_nn_sequential_chain_forward(&self, cls: &ClassType, input_ty: Type, range: TextRange, errors: &ErrorCollector) -> Option<Type>
fn try_string_literal_as_typeform(&self, x: &Expr, hint: &Type, range: TextRange, errors: &ErrorCollector, tcc: &dyn Fn() -> TypeCheckContext, call_context: &CallContext<'_>) -> Option<Type>
fn type_of(&self, ty: Type) -> Type
fn type_of_attr_get(&self, base: &Type, attr_name: &Name, range: TextRange, errors: &ErrorCollector, error_kind: ErrorKind, context: Option<&dyn Fn() -> ErrorContext>, todo_ctx: &str) -> Type
fn type_of_magic_dunder_attr(&self, base: &Type, attr_name: &Name, range: TextRange, errors: &ErrorCollector, context: Option<&dyn Fn() -> ErrorContext>, todo_ctx: &str, allow_getattr_fallback: bool) -> Option<Type>
fn type_order(&self) -> TypeOrder<'_, Ans>
fn typed_dict_extra_items(&self, typed_dict: &TypedDict) -> ExtraItems
fn typed_dict_field(&self, typed_dict: &TypedDict, name: &Name) -> Option<TypedDictField>
fn typed_dict_fields(&self, typed_dict: &TypedDict) -> SmallMap<Name, TypedDictField>
fn typed_dict_kw_param_info(&self, typed_dict: &TypedDict) -> Vec<(Name, Type, Required)>
fn typevar_from_call(&self, name: Identifier, x: &ExprCall, kind: QuantifiedKind, errors: &ErrorCollector) -> TypeVar
fn typevartuple_from_call(&self, name: Identifier, x: &ExprCall, errors: &ErrorCollector) -> TypeVarTuple
fn undecorated_function(&self, def: &FunctionDefData, def_index: FuncDefIndex, is_in_type_checking_block: bool, body_kind: BodyKind, is_return_inferred: bool, calls_super_method: bool, class_key: Option<&Idx<KeyClass>>, decorators: &[Idx<KeyDecorator>], legacy_tparams: &[Idx<KeyLegacyTypeParam>], parent: &NestingContext, shape_dsl_def: Option<Arc<ShapeDslFunction>>, type_shape_dsl_def: Option<Arc<ParsedTypeShapeDslFunction>>, uses_shape_dsl_ir_name: Option<ShortIdentifier>, errors: &ErrorCollector) -> UndecoratedFunction
fn union(&self, x: Type, y: Type) -> Type
fn unions(&self, xs: Vec<Type>) -> Type
fn unop_infer(&self, x: &ExprUnaryOp, errors: &ErrorCollector) -> Type
fn untype(&self, ty: Type, range: TextRange, errors: &ErrorCollector) -> Type
fn untype_alias(&self, ta: &TypeAliasData) -> Type
fn untype_opt(&self, ty: Type, range: TextRange, errors: &ErrorCollector) -> Option<Type>
fn unwrap_async_iterable(&self, ty: &Type) -> Option<Type>
fn unwrap_awaitable(&self, ty: &Type) -> Option<Type>
fn unwrap_class_object_silently(&self, ty: &Type) -> Option<(TParams, Type)>
fn unwrap_coroutine(&self, ty: &Type) -> Option<(Type, Type, Type)>
fn unwrap_generator(&self, ty: &Type) -> Option<(Type, Type, Type)>
fn unwrap_iterable(&self, ty: &Type) -> Option<Type>
fn unwrap_mapping(&self, ty: &Type) -> Option<(Type, Type)>
fn uses_builtin_shaped_array_indexing(&self, cls: &Class) -> bool
fn validate_final_thread_state(&self)
fn validate_frozen_dataclass_inheritance(&self, cls: &Class, dataclass_metadata: &DataclassMetadata, bases_with_metadata: &[(Class, &ClassMetadata)], is_from_dataclass_transform: bool, errors: &ErrorCollector)
fn validate_post_init(&self, cls: &Class, dataclass_metadata: &DataclassMetadata, post_init: Type, range: TextRange, errors: &ErrorCollector)
fn validate_type_form(&self, ty: Type, range: TextRange, type_form_context: TypeFormContext<'_>, errors: &ErrorCollector) -> Type
fn validate_type_var_default(&self, name: &Name, kind: QuantifiedKind, default: &Type, range: TextRange, restriction: &Restriction, errors: &ErrorCollector) -> Type
fn validated_tparams(&self, range: TextRange, tparams: Vec<Quantified>, source: TParamsSource, errors: &ErrorCollector) -> TParams
```

`'ctx` covers the context a solver runs against: the standard library, the
type heap, the unique factory, the recursion guard, the cross-module export
and answer lookups, and the error collector and caches belonging to this
solve. A caller assembles all of it and hands it to `new`. The lifetime is
here because the solver holds that context by reference.

`'answer` is the lifetime the solver's API is built around. It bounds how
long an answer reference stays usable, which is not the same as how long the
answer lives: the `current` `Answers` table normally lasts for the whole
transaction epoch and is shortened to `'answer`, while provisional answers
retained by the `AnswerScope` last exactly this long. Taking the shorter of
the two lets one lifetime describe a borrow of either.

---

## CalcId

`struct` · `pyrefly::alt::answers_solver::CalcId`

```rust
struct CalcId
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Compactly represents the identity of a binding, for the purposes of
understanding the calculation stack.

---

## CalcStack

`struct` · `pyrefly::alt::answers_solver::CalcStack`

```rust
struct CalcStack
```

**Methods** (5)

```rust
fn current_cycle(&self) -> Option<Vec1<CalcId>>
fn into_vec(&self) -> Vec<CalcId>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn peek(&self) -> Option<CalcId>
```

Represent a stack of in-progress calculations in an `AnswersSolver`.

This is useful for debugging, particularly for debugging scc handling.

The stack is per-thread; we create a new `AnswersSolver` every time
we change modules when resolving exports, but the stack is passed
down because sccs can cross module boundaries.

---

## ReservedSlot

`struct` · `pyrefly::alt::answers_solver::ReservedSlot`

```rust
struct ReservedSlot<'a, 'ctx, 'answer, Ans: LookupAnswer>
```

**Implements**: `core::ops::drop::Drop`

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

Proof that this SCC owns the pending result slot for this calculation.
Dropping the proof rolls back the reservation if it is still pending.

---

## Scc

`struct` · `pyrefly::alt::answers_solver::Scc`

```rust
struct Scc
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Represent an SCC (Strongly Connected Component) we are currently solving.

This simplified model tracks SCC participants with explicit state rather than
using separate recursion and unwind stacks. The Rust call stack naturally
enforces LIFO ordering, so we only need to track the state of each
participant (Fresh/InProgress/Done).

---

## SccIterationState

`struct` · `pyrefly::alt::answers_solver::SccIterationState`

```rust
struct SccIterationState
```

**Fields**: `iteration`, `has_changed`, `recursion_breaks`

**Derives**: Debug

Per-SCC iteration state for iterative fixpoint solving.

This tracks the current iteration number, current and warm-start answer
generations, per-node progress, and convergence state.

Iteration state is SCC-scoped so that disjoint SCCs can iterate
independently.

---

## ThreadState

`struct` · `pyrefly::alt::answers_solver::ThreadState`

```rust
struct ThreadState
```

**Methods** (1)

```rust
fn new(recursion_limit_config: Option<RecursionLimitConfig>) -> Self
```

Represents thread-local state for the current `AnswersSolver` and any
`AnswersSolver`s waiting for the results that we are currently computing.

This state is initially created by some top-level `AnswersSolver` - when
we're calculating results for bindings, we started at either:
- a solver that is type-checking some module end-to-end, or
- an ad-hoc solver (used in some LSP functionality) solving one specific binding

We'll create a new `AnswersSolver` will change every time we switch modules,
which happens as we resolve types of imported names, but when this happens
we always pass the current `ThreadState`.

---

## TypeCheckOptions

`struct` · `pyrefly::alt::answers_solver::TypeCheckOptions`

```rust
struct TypeCheckOptions<'a, 'subset>
```

**Methods** (2)

```rust
fn new(errors: &'a ErrorCollector, context: &'a dyn Fn() -> TypeCheckContext) -> Self
fn with_call_context(self, call_context: &'a CallContext<'subset>) -> Self
```

---
