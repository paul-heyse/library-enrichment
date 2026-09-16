# `pyrefly::binding::bindings`

Crate `pyrefly` · 10 public items · structured records in [`model/pyrefly.binding.bindings.json`](../model/pyrefly.binding.bindings.json)

## AwaitContext

`enum` · `pyrefly::binding::bindings::AwaitContext`

```rust
enum AwaitContext
```

**Variants**: `General`, `GeneratorElement`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

An enum tracking whether we are in a generator expression
like `(x for x in xs)` - used to allow `await` inside of generators
even when a function is not async, for example (await x for x in xs).

This is legal because the resulting AsyncGenerator does not actually
await until iterated (which can only be done in an `async def`).

In any other comprehension, `await` requires us to be in an `async def`.

---

## InitializedInFlow

`enum` · `pyrefly::binding::bindings::InitializedInFlow`

```rust
enum InitializedInFlow
```

**Variants**: `Yes`, `Conditionally`, `No`, `DeferredCheck`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn as_error_message(&self, name: &Name) -> Option<String>
fn deferred_termination_keys(&self) -> Option<&[Idx<Key>]>
```

---

## LegacyTParamId

`enum` · `pyrefly::binding::bindings::LegacyTParamId`

```rust
enum LegacyTParamId
```

**Variants**: `Name`, `Attr`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Debug

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## NameLookupResult

`enum` · `pyrefly::binding::bindings::NameLookupResult`

```rust
enum NameLookupResult
```

**Variants**: `Found`, `NotFound`

**Derives**: Debug

The result of looking up a name. Similar to `NameReadInfo`, but
differs because the `BindingsBuilder` layer is responsible for both
intercepting first-usage reads and for wrapping forward-reference `Key`s
in `Idx<Key>` by inserting them into the bindings table.

---

## BindingTable

`struct` · `pyrefly::binding::bindings::BindingTable`

```rust
struct BindingTable
```

**Fields**: `types`, `expectations`, `type_aliases`, `exports`, `decorators`, `decorated_functions`, `undecorated_functions`, `func_defs`, `classes`, `tparams`, `class_base_types`, `class_fields`, `class_synthesized_fields`, `variance`, `class_checks`, `annotations`, `class_metadata`, `django_relations`, `class_mros`, `class_disjoint_bases`, `abstract_class_check`, `class_subscript_symmetry`, `legacy_tparams`, `yields`, `yield_froms`

**Implements**: `pyrefly::binding::table::TableKeyed`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn insert<K: Keyed>(&mut self, key: K, value: K::Value) -> Idx<K> where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn insert_idx<K: Keyed>(&mut self, idx: Idx<K>, value: K::Value) -> Idx<K> where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
```

**via `pyrefly::binding::table::TableKeyed`**

```rust
fn get(&self) -> &Self::Value
fn get_mut(&mut self) -> &mut Self::Value
```

---

## Bindings

`struct` · `pyrefly::binding::bindings::Bindings`

```rust
struct Bindings
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`

**Derives**: Clone, Debug

**Methods** (30)

```rust
fn available_definitions(&self, position: TextSize) -> SmallSet<Idx<Key>>
fn class_def_index(&self, class_def: &StmtClassDef) -> Option<ClassDefIndex>
fn definition_at_position(&self, position: TextSize) -> Option<&Key>
fn display<K: Keyed>(&self, idx: Idx<K>) -> impl Display + '_ where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn enclosing_class(&self, range: TextRange) -> Option<Idx<KeyClass>>
fn function_def_range(&self, def_index: FuncDefIndex) -> Option<TextRange>
fn function_has_return_annotation(&self, name: &Identifier) -> bool
fn get<K: Keyed>(&self, idx: Idx<K>) -> &K::Value where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn get_class_fields(&self, idx: ClassDefIndex) -> Option<&ClassFields>
fn get_function_param(&self, name: &Identifier) -> &FunctionParameter
fn get_lambda_param_id(&self, name: &Identifier) -> LambdaParamId
fn idx_to_key<K: Keyed>(&self, idx: Idx<K>) -> &K where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn is_type_level_lambda_parameter(&self, name: &Identifier) -> bool
fn is_valid_key(&self, k: &Key) -> bool
fn key_to_idx<K: Keyed>(&self, k: &K) -> Idx<K> where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn key_to_idx_hashed<K: Keyed>(&self, k: Hashed<&K>) -> Idx<K> where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn key_to_idx_hashed_opt<K: Keyed>(&self, k: Hashed<&K>) -> Option<Idx<K>> where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn keys<K: Keyed>(&self) -> impl ExactSizeIterator<Item = Idx<K>> + '_ where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn lambda_yield_keys(&self, range: TextRange) -> (&[Idx<KeyYield>], &[Idx<KeyYieldFrom>])
fn metadata(&self) -> &Arc<BindingsMetadata>
fn module(&self) -> &pyrefly_python::module::Module
fn module_deletes(&self) -> &SmallSet<Name>
fn module_ranges(&self) -> &Arc<ModuleRanges>
fn new(x: ModModule, module_info: pyrefly_python::module::Module, exports: &Exports, solver: &Solver, lookup: &dyn LookupExport, sys_info: SysInfo, errors: &ErrorCollector, enable_trace: bool, check_unannotated_defs: bool, analyze_unannotated_for_ide: bool, infer_return_types: InferReturnTypes, treat_all_caps_as_final: bool) -> Self
fn should_promote_at_range(&self, range: TextRange) -> bool
fn subsequently_initialized(&self, ann: Idx<KeyAnnotation>) -> bool
fn sys_info(&self) -> &SysInfo
fn unused_imports(&self) -> &[UnusedImport]
fn unused_parameters(&self) -> &[UnusedParameter]
fn unused_variables(&self) -> &[UnusedVariable]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## BindingsBuilder

`struct` · `pyrefly::binding::bindings::BindingsBuilder`

```rust
struct BindingsBuilder<'a>
```

**Fields**: `module_info`, `lookup`, `sys_info`, `metadata`, `func_count`, `has_docstring`, `scopes`, `check_unannotated_defs`, `analyze_unannotated_for_ide`, `infer_return_types`, `treat_all_caps_as_final`, `class_scopes`, `adjacent_namedtuple_defaults`, `promote_ranges`, `type_checking_depth`

**Implements**: `ruff_python_parser::semantic_errors::SemanticSyntaxContext`

**Methods** (94)

```rust
fn abandon_branch(&mut self)
fn add_loop_exitpoint(&mut self, exit: LoopExit)
fn add_name_definitions(&mut self, legacy_tparams: &LegacyTParamCollector)
fn as_special_export(&self, e: &Expr) -> Option<SpecialExport>
fn base_class_of(&self, base_expr: Expr) -> BaseClass
fn bind_annotation(&mut self, name: &Identifier, annotation: &mut Expr, is_initialized: AnnAssignHasValue) -> Idx<KeyAnnotation>
fn bind_attr_assign(&mut self, attr: ExprAttribute, assigned: &mut Expr, make_assigned_value: impl FnOnce(&Expr, Option<Idx<KeyAnnotation>>) -> ExprOrBinding) -> ExprOrBinding
fn bind_attr_assign_impl(&mut self, attr: ExprAttribute, assigned: Option<&mut Expr>, make_assigned_value: impl FnOnce(Option<&Expr>, Option<Idx<KeyAnnotation>>) -> ExprOrBinding, ensure_assigned: bool) -> ExprOrBinding
fn bind_current(&mut self, name: &Name, current: &CurrentIdx, style: FlowStyle) -> Option<Idx<KeyAnnotation>>
fn bind_current_as(&mut self, name: &Identifier, current: CurrentIdx, binding: Binding, style: FlowStyle) -> Option<Idx<KeyAnnotation>>
fn bind_definition(&mut self, name: &Identifier, binding: Binding, style: FlowStyle) -> Option<Idx<KeyAnnotation>>
fn bind_function_param(&mut self, target: AnnotationTarget, x: &Parameter, undecorated_idx: Idx<KeyUndecoratedFunction>, class_key: Option<Idx<KeyClass>>, is_variadic: bool, ignore_annotation: bool)
fn bind_inline_functional_named_tuple(&mut self, call: &mut ExprCall, kind: SpecialExport) -> Option<Idx<Key>>
fn bind_lambda(&mut self, lambda: &mut ExprLambda, usage: &mut Usage, kind: LambdaKind)
fn bind_lambda_param(&mut self, name: &Identifier, kind: LambdaKind, usage: &Usage)
fn bind_name(&mut self, name: &Name, idx: Idx<Key>, style: FlowStyle) -> Option<Idx<KeyAnnotation>>
fn bind_narrow_ops(&mut self, narrow_ops: &NarrowOps, use_location: NarrowUseLocation, usage: &Usage)
fn bind_single_name_assign(&mut self, name: &Identifier, value: Box<Expr>, direct_ann: Option<(&Expr, Idx<KeyAnnotation>)>, ensure_assigned: bool) -> Option<Idx<KeyAnnotation>>
fn bind_subscript_assign(&mut self, subscript: ExprSubscript, assigned: &mut Expr, make_assigned_value: impl FnOnce(&Expr, Option<Idx<KeyAnnotation>>) -> ExprOrBinding)
fn bind_subscript_assign_impl(&mut self, subscript: ExprSubscript, assigned: Option<&mut Expr>, make_assigned_value: impl FnOnce(Option<&Expr>, Option<Idx<KeyAnnotation>>) -> ExprOrBinding, ensure_assigned: bool)
fn bind_target_no_expr(&mut self, target: &mut Expr, make_binding: &dyn Fn(Option<Idx<KeyAnnotation>>) -> Binding)
fn bind_target_with_expr(&mut self, target: &mut Expr, assigned: &mut Expr, make_binding: &dyn Fn(&Expr, Option<Idx<KeyAnnotation>>) -> Binding)
fn bind_targets_with_value(&mut self, targets: &mut [Expr], value: &mut Expr)
fn build_narrow_entries(&mut self, negated_prev_ops: &NarrowOps) -> Vec<(Idx<Key>, Box<NarrowOp>, TextRange)>
fn check_functional_definition_name(&mut self, name: &Name, arg: &Expr, error_kind: ErrorKind)
fn class_def(&mut self, x: StmtClassDef, parent: &NestingContext)
fn class_object_is_generic(&self, idx: Idx<Key>) -> bool
fn create_function_index(&mut self, function_identifier: &Identifier) -> (Idx<KeyDecoratedFunction>, Option<Idx<Key>>)
fn declare_current_idx(&mut self, key: Key) -> CurrentIdx
fn declare_mutable_capture(&mut self, name: &Identifier, kind: MutableCaptureKind)
fn defer_bound_name(&mut self, key: Key, lookup_result_idx: Idx<Key>, usage: &Usage, promote: bool) -> Idx<Key>
fn ensure_and_bind_decorators(&mut self, decorators: ThinVec<Decorator>, usage: &mut Usage) -> Vec<Idx<KeyDecorator>>
fn ensure_class_member_type(&mut self, x: &mut Expr, tparams_builder: Option<&mut LegacyTParamCollector>)
fn ensure_expr(&mut self, x: &mut Expr, usage: &mut Usage)
fn ensure_expr_name(&mut self, x: &ExprName, usage: &mut Usage) -> Idx<Key>
fn ensure_expr_opt(&mut self, x: Option<&mut Expr>, usage: &mut Usage)
fn ensure_type(&mut self, x: &mut Expr, tparams_builder: Option<&mut LegacyTParamCollector>)
fn ensure_type_opt(&mut self, x: Option<&mut Expr>, tparams_builder: Option<&mut LegacyTParamCollector>)
fn ensure_type_with_usage(&mut self, x: &mut Expr, tparams_builder: Option<&mut LegacyTParamCollector>, usage: &mut Usage)
fn error(&self, range: TextRange, kind: ErrorKind, msg: String)
fn error_with_detail(&self, range: TextRange, kind: ErrorKind, header: String, detail: String)
fn error_with_detail_from(&self, range: TextRange, kind: ErrorKind, header: String, detail: impl FnOnce() -> Option<String>)
fn extract_django_fields_from_class_body(&self, field_definitions: &SmallMap<Name, (ClassFieldDefinition, TextRange)>) -> DjangoFieldInfo
fn extract_field_validator_fields(&self, body: &[Stmt]) -> Vec<Name>
fn extract_pydantic_config_dict(&self, e: &Expr, name: &Hashed<Name>, pydantic_config_dict: &mut PydanticConfigDict)
fn finish_bool_op_fork(&mut self)
fn finish_branch(&mut self)
fn finish_exhaustive_fork(&mut self)
fn finish_match_or_fork(&mut self)
fn finish_non_exhaustive_fork(&mut self, negated_prev_ops: &NarrowOps, base_termination_key: Option<Idx<Key>>)
fn func_def_index(&mut self) -> FuncDefIndex
fn function_def(&mut self, x: StmtFunctionDef, parent: &NestingContext)
fn get_original_binding(&'a self, original_idx: Idx<Key>) -> Option<(Idx<Key>, Option<&'a Binding>)>
fn idx_for_promise<K>(&mut self, key: K) -> Idx<K> where K: Keyed, BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn idx_to_key<K>(&self, idx: Idx<K>) -> &K where K: Keyed, BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn infer_with_first_use(&self) -> bool
fn init_static_scope(&mut self, x: &[Stmt], top_level: bool)
fn insert_binding<K: Keyed>(&mut self, key: K, value: K::Value) -> Idx<K> where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn insert_binding_current(&mut self, current: CurrentIdx, value: Binding) -> Idx<Key>
fn insert_binding_idx<K: Keyed>(&mut self, idx: Idx<K>, value: K::Value) -> Idx<K> where BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn insert_binding_overwrite(&mut self, key: Key, value: Binding) -> Idx<Key>
fn insert_subsequently_initialized(&mut self, ann_idx: Idx<KeyAnnotation>)
fn intercept_lookup(&mut self, legacy_tparams: &mut LegacyTParamCollector, id: LegacyTParamId) -> NameLookupResult
fn last_statement_idx_for_implicit_return(&mut self, last: LastStmt, x: &Expr) -> Idx<Key>
fn lookup_name(&mut self, name: Hashed<&Name>, usage: &mut Usage) -> NameLookupResult
fn mark_does_not_pin_if_first_use(&mut self, def_idx: Idx<Key>)
fn next_branch(&mut self)
fn promised_idx<K>(&self, key: &K) -> Option<Idx<K>> where K: Keyed, BindingTable: TableKeyed<K, Value = BindingEntry<K>>
fn record_django_relation_class(&mut self, class_idx: Idx<KeyClass>, fields: Vec<Idx<KeyClassField>>)
fn record_lambda_yield_keys(&mut self, range: TextRange, yield_keys: Box<[Idx<KeyYield>]>, yield_from_keys: Box<[Idx<KeyYieldFrom>]>)
fn record_unused_imports(&mut self, unused: Vec<UnusedImport>)
fn record_unused_parameters(&mut self, unused: Vec<UnusedParameter>)
fn record_unused_variables(&mut self, unused: Vec<UnusedVariable>)
fn record_used_imports_from_dunder_all_names<T>(&mut self, dunder_all_names: T) where T: Iterator<Item = &'a Name>
fn seed_captured_variables(&mut self)
fn setup_loop(&mut self, range: TextRange, loop_header_targets: &SmallSet<Name>)
fn start_branch(&mut self)
fn start_fork(&mut self, range: TextRange)
fn start_fork_and_branch(&mut self, range: TextRange)
fn stmt(&mut self, x: Stmt, parent: &NestingContext)
fn stmt_match(&mut self, x: StmtMatch, parent: &NestingContext)
fn stmts(&mut self, xs: ThinVec<Stmt>, parent: &NestingContext)
fn suggest_similar_name(&self, missing: &Name) -> Option<Name>
fn synthesize_collections_named_tuple_def(&mut self, class_name: Identifier, parent: &NestingContext, func: &mut Expr, members: &mut [Expr], keywords: &mut [Keyword], bind_to_name: bool, adjacent_defaults: Option<Vec<Expr>>) -> Idx<KeyClass>
fn synthesize_enum_def(&mut self, name: &ExprName, parent: &NestingContext, func: &mut Expr, arg_name: &mut Expr, members: &mut [Expr])
fn synthesize_typed_dict_def(&mut self, name: &ExprName, parent: &NestingContext, func: &mut Expr, arg_name: &Expr, args: &mut [Expr], keywords: &mut [Keyword])
fn synthesize_typing_named_tuple_def(&mut self, class_name: Identifier, parent: &NestingContext, func: &mut Expr, members: &[Expr], bind_to_name: bool, adjacent_defaults: Option<Vec<Expr>>) -> Idx<KeyClass>
fn synthesize_typing_new_type(&mut self, name: &ExprName, parent: &NestingContext, func: &mut Expr, new_type_name: &mut Expr, base: &mut Expr)
fn teardown_loop(&mut self, range: TextRange, narrow_ops: &NarrowOps, orelse: ThinVec<Stmt>, parent: &NestingContext, is_while_true: bool, loop_definitely_runs: bool)
fn try_intercept_lookup(&mut self, legacy_tparams: &mut LegacyTParamCollector, id: &LegacyTParamId) -> Option<NameLookupResult>
fn type_alias_index(&mut self) -> TypeAliasIndex
fn type_params(&mut self, x: &mut TypeParams) -> SmallSet<Name>
fn type_params_with_owner(&mut self, x: &mut TypeParams, owner: Option<Name>) -> SmallSet<Name>
fn with_semantic_checker(&mut self, f: impl FnOnce(&mut SemanticSyntaxChecker, &Self))
```

**via `ruff_python_parser::semantic_errors::SemanticSyntaxContext`**

```rust
fn future_annotations_or_stub(&self) -> bool
fn global(&self, name: &str) -> Option<TextRange>
fn has_nonlocal_binding(&self, name: &str) -> bool
fn in_async_context(&self) -> bool
fn in_await_allowed_context(&self) -> bool
fn in_class_body_comprehension(&self) -> bool
fn in_function_scope(&self) -> bool
fn in_generator_context(&self) -> bool
fn in_loop_context(&self) -> bool
fn in_module_scope(&self) -> bool
fn in_notebook(&self) -> bool
fn in_sync_comprehension(&self) -> bool
fn in_yield_allowed_context(&self) -> bool
fn is_bound_parameter(&self, name: &str) -> bool
fn lazy_import_context(&self) -> Option<LazyImportContext>
fn python_version(&self) -> ruff_python_ast::PythonVersion
fn report_semantic_error(&self, error: SemanticSyntaxError)
fn source(&self) -> &str
```

---

## CurrentIdx

`struct` · `pyrefly::binding::bindings::CurrentIdx`

```rust
struct CurrentIdx
```

**Derives**: Debug

**Methods** (4)

```rust
fn idx(&self) -> Idx<Key>
fn into_idx(self) -> Idx<Key>
fn new(idx: Idx<Key>) -> Self
fn usage(&mut self) -> &mut Usage
```

An abstraction representing the `Idx<Key>` for a binding that we
are currently constructing, which can be used as a factory to create
usage values for `ensure_expr`.

Note that while it wraps a `Usage`, that usage is always `Usage::CurrentIdx`,
never some other variant.

The first_use_of tracking has been removed since deferred BoundName processing
now handles all first-use detection after AST traversal.

---

## LegacyTParamCollector

`struct` · `pyrefly::binding::bindings::LegacyTParamCollector`

```rust
struct LegacyTParamCollector
```

**Methods** (2)

```rust
fn lookup_keys(&self) -> Vec<Idx<KeyLegacyTypeParam>>
fn new(has_scoped_tparams: bool) -> Self
```

Handle intercepting names inside either function parameter/return
annotations or base class lists of classes, in order to check whether they
point at type variable declarations and need to be converted to type
parameters.

---

## BindingEntry

`type_alias` · `pyrefly::binding::bindings::BindingEntry`

```rust
type BindingEntry<K> = (pyrefly_graph::index::Index<K>, pyrefly_graph::index_map::IndexMap<K, <K as Keyed>::Value>)
```

---
