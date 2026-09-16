# `pyrefly::binding::scope`

Crate `pyrefly` · 19 public items · structured records in [`model/pyrefly.binding.scope.json`](../model/pyrefly.binding.scope.json)

## Exportable

`enum` · `pyrefly::binding::scope::Exportable`

```rust
enum Exportable
```

**Variants**: `Initialized`, `Uninitialized`

**Derives**: Debug

A name defined in a module, which needs to be convertible to an export.

---

## FlowStyle

`enum` · `pyrefly::binding::scope::FlowStyle`

```rust
enum FlowStyle
```

**Variants**: `Other`, `ClassField`, `MergeableImport`, `Import`, `ImportAs`, `FunctionDef`, `ClassDef`, `PossiblyUninitialized`, `MaybeInitialized`, `Uninitialized`, `LoopRecursion`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn assume_initialized(self) -> Self
fn canonical_class_receiver_idx(&self) -> Option<Idx<Key>>
```

---

## LoopExit

`enum` · `pyrefly::binding::scope::LoopExit`

```rust
enum LoopExit
```

**Variants**: `Break`, `Continue`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## MutableCaptureError

`enum` · `pyrefly::binding::scope::MutableCaptureError`

```rust
enum MutableCaptureError
```

**Variants**: `NotFound`, `NonlocalScope`, `AssignedBeforeNonlocal`, `AssignedBeforeGlobal`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn message(&self, name: &Identifier) -> String
```

---

## NameReadInfo

`enum` · `pyrefly::binding::scope::NameReadInfo`

```rust
enum NameReadInfo
```

**Variants**: `Flow`, `Anywhere`, `ImplicitBuiltin`, `OuterClassTypeParameter`, `NotFound`

**Derives**: Debug

The result of looking up a name in the current scope stack for a read
operation.

---

## TerminationKind

`enum` · `pyrefly::binding::scope::TerminationKind`

```rust
enum TerminationKind
```

**Variants**: `Raise`, `Jump`, `StaticTest`

**Derives**: Clone, Copy, Debug

How control flow left a point in the program. Only an exception can be swallowed
by an enclosing `with`: `__exit__` also runs for `return`/`break`/`continue`, but
its return value is ignored, so those always leave the `with`.

---

## ClassIndices

`struct` · `pyrefly::binding::scope::ClassIndices`

```rust
struct ClassIndices
```

**Fields**: `def_index`, `class_idx`, `class_object_idx`, `base_type_idx`, `metadata_idx`, `mro_idx`, `disjoint_base_idx`, `synthesized_fields_idx`, `variance_idx`, `class_checks_idx`, `abstract_class_check_idx`, `subscript_symmetry_idx`

**Derives**: Clone, Debug

Because of complications related both to recursion in the binding graph and to
the need for efficient representations, Pyrefly relies on multiple different integer
indexes used to refer to classes and retrieve different kinds of binding information.

This struct type captures the requirement that a class must always have all of these
indexes available, and provides a convenient way to pass them.

This is used in bindings code, but the solver depends on the invariant that all these
indexes, which get stored in various Binding nodes, must be valid.

---

## Flow

`struct` · `pyrefly::binding::scope::Flow`

```rust
struct Flow
```

**Derives**: Clone, Debug, Default

Flow-sensitive information about a name.

---

## Fork

`struct` · `pyrefly::binding::scope::Fork`

```rust
struct Fork
```

**Derives**: Clone, Debug

Represents forks in control flow that contain branches. Used to
control how the final flow from merging branches behaves.

---

## InstanceAttribute

`struct` · `pyrefly::binding::scope::InstanceAttribute`

```rust
struct InstanceAttribute
```

**Derives**: Clone, Debug

---

## NameWriteInfo

`struct` · `pyrefly::binding::scope::NameWriteInfo`

```rust
struct NameWriteInfo
```

**Fields**: `annotation`, `anywhere_range`

**Derives**: Debug

The result of a successful lookup of a name for a write operation.

---

## OuterCaptureInfo

`struct` · `pyrefly::binding::scope::OuterCaptureInfo`

```rust
struct OuterCaptureInfo
```

**Fields**: `value_idx`, `narrow_idx`

**Derives**: Default

Value and narrow information for a captured variable from an outer scope.
Returned by `Scopes::outer_capture_info`.

---

## Scope

`struct` · `pyrefly::binding::scope::Scope`

```rust
struct Scope
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn annotation(range: TextRange, class_scope: bool) -> Self
fn class_body(range: TextRange, indices: ClassIndices, name: Identifier, has_protocol_base: bool) -> Self
fn comprehension(range: TextRange, is_generator: bool) -> Self
fn function(range: TextRange, name: Identifier, is_async: bool) -> Self
fn lambda(range: TextRange, name: Identifier, is_async: bool) -> Self
fn method(range: TextRange, name: Identifier, is_async: bool) -> Self
fn type_alias(range: TextRange) -> Self
```

---

## ScopeTrace

`struct` · `pyrefly::binding::scope::ScopeTrace`

```rust
struct ScopeTrace
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn available_definitions(&self, table: &BindingTable, position: TextSize) -> SmallSet<Idx<Key>>
fn definition_at_position<'a>(&self, table: &'a BindingTable, position: TextSize) -> Option<&'a Key>
fn exportables(&self) -> SmallMap<Name, Exportable>
fn module_deletes(&self) -> &SmallSet<Name>
fn toplevel_scope(&self) -> &Scope
```

---

## Scopes

`struct` · `pyrefly::binding::scope::Scopes`

```rust
struct Scopes
```

**Derives**: Clone, Debug

**Methods** (106)

```rust
fn add_implicit_builtin_to_module_static(&mut self, name: Hashed<&Name>, module: ModuleName) -> Key
fn add_loop_exit(&mut self, exit: LoopExit) -> bool
fn add_lvalue_to_current_static(&mut self, x: &Expr)
fn add_name_to_current_static(&mut self, name: &Identifier)
fn add_parameter_to_current_static(&mut self, name: &Identifier, ann: Option<Idx<KeyAnnotation>>)
fn add_possible_legacy_tparam_to_current_static(&mut self, name: &Identifier)
fn add_scoped_type_parameter_to_current_static(&mut self, name: &Identifier)
fn as_special_export(&self, name: &Name, base_name: Option<&Name>, current_module: ModuleName, lookup: &dyn LookupExport) -> Option<SpecialExport>
fn binding_idx_for_name(&self, name: &Name) -> Option<(Idx<Key>, FlowStyle)>
fn clone_current_flow(&self) -> Flow
fn collect_module_unused_imports(&self) -> Vec<UnusedImport>
fn current_binding_is_module_binding(&self, name: &Name) -> bool
fn current_class_def_index(&self) -> Option<ClassDefIndex>
fn current_class_key(&self) -> Option<Idx<KeyClass>>
fn current_flow_idx(&self, name: &Name) -> Option<Idx<Key>>
fn current_flow_style(&self, name: &Name) -> Option<FlowStyle>
fn current_fork_base_idx(&self, name: &Name) -> Option<Idx<Key>>
fn current_method_and_class(&self) -> Option<(Identifier, Idx<KeyClass>)>
fn current_method_context(&self) -> Option<Idx<KeyClass>>
fn current_scope_range(&self) -> TextRange
fn current_static_contains(&self, name: &Name) -> bool
fn define_in_current_flow(&mut self, name: Hashed<&Name>, idx: Idx<Key>, style: FlowStyle) -> Option<NameWriteInfo>
fn define_in_enclosing_non_comprehension_scope(&mut self, name: Hashed<&Name>, idx: Idx<Key>, style: FlowStyle)
fn enclosing_class_name(&self) -> Option<&Identifier>
fn enclosing_class_object_idx(&self) -> Option<Idx<Key>>
fn enter_finally(&mut self)
fn enter_with(&mut self)
fn existing_module_import_at(&self, module_name: &Name) -> Option<Idx<Key>>
fn exit_finally(&mut self)
fn exit_with(&mut self)
fn finally_depth(&self) -> usize
fn finish(self) -> ScopeTrace
fn finish_class_and_get_field_definitions(&mut self) -> (SmallMap<Name, (ClassFieldDefinition, TextRange)>, SmallMap<Name, Vec<Expr>>)
fn flow_style_for_name(&self, name: &Name) -> Option<FlowStyle>
fn fold_suggestion_candidates<'b>(&self, search: &mut Search<'_>, trailing: impl Iterator<Item = Candidate<'b>>)
fn function_predecessor_indices(&self, name: &Name) -> Option<(Idx<Key>, Idx<KeyDecoratedFunction>)>
fn get_current_flow_idx(&self, name: &Name) -> Option<Idx<Key>>
fn get_global_declaration(&self, name: &str) -> Option<TextRange>
fn global_capture_self_defined(&self, name: Hashed<&Name>) -> bool
fn has_future_annotations(&self) -> bool
fn has_import_name(&self, name: &Name) -> bool
fn has_nonlocal_binding(&self, name: &str) -> bool
fn has_terminated(&self) -> bool
fn implicit_capture_names(&self) -> SmallSet<Name>
fn in_class_body(&self) -> bool
fn in_comprehension(&self) -> bool
fn in_finally(&self) -> bool
fn in_function_scope(&self) -> bool
fn in_generator_expression(&self) -> bool
fn in_module_or_class_top_level(&self) -> bool
fn in_sync_comprehension(&self) -> bool
fn in_type_alias(&self) -> bool
fn init_current_static(&mut self, x: &[Stmt], module_info: &pyrefly_python::module::Module, top_level: bool, lookup: &dyn LookupExport, sys_info: SysInfo, get_annotation_idx: &mut impl FnMut(ShortIdentifier) -> Idx<KeyAnnotation>)
fn is_bound_parameter(&self, name: &str) -> bool
fn is_defined_at_module_scope(&self, name: &Name) -> bool
fn is_definitely_unreachable(&self) -> bool
fn is_final_at_module_scope(&self, name: &Name) -> bool
fn is_final_in_current_scope(&self, name: &Name) -> bool
fn is_in_async_def(&self) -> bool
fn is_in_protocol_class(&self) -> bool
fn is_unreachable_from_static_test(&self) -> bool
fn last_stmt_expr(&self) -> Option<Idx<Key>>
fn legacy_tparam_shadows_enclosing_annotation_scope(&self, name: &Name) -> bool
fn look_up_name_for_read(&self, name: Hashed<&Name>, usage: &Usage, lookup: &dyn LookupExport, current_module: ModuleName) -> NameReadInfo
fn lookup_final_string_value(&self, name: &Name) -> Option<&str>
fn loop_depth(&self) -> usize
fn loop_protects_from_finally_exit(&self) -> bool
fn mark_as_deleted(&mut self, name: &Name)
fn mark_flow_termination(&mut self, kind: TerminationKind)
fn mark_has_yield_in_dead_code(&mut self)
fn mark_import_used(&mut self, name: &Name)
fn mark_parameter_used(&mut self, name: &Name)
fn mark_variable_used(&mut self, name: &Name)
fn method_that_sets_attr(&self, x: &ExprAttribute) -> Option<MethodThatSetsAttr>
fn module(range: TextRange, keep_scope_tree: bool, is_interface: bool) -> Self
fn module_shadowed_implicit_builtins(&self) -> Vec<(Name, ModuleName)>
fn name_shadows_enclosing_annotation_scope(&self, name: &Name) -> bool
fn narrow_in_current_flow(&mut self, name: Hashed<&Name>, idx: Idx<Key>)
fn nesting_context(&self) -> NestingContext
fn outer_capture_info(&self, name: Hashed<&Name>, inner_fn_range: TextRange) -> OuterCaptureInfo
fn pop(&mut self) -> Scope
fn pop_function_scope(&mut self) -> (YieldsAndReturns, Option<SelfAssignments>, Vec<UnusedParameter>, Vec<UnusedVariable>)
fn propagate_new_flow_entries_to_fork_base(&mut self)
fn propagate_new_flow_entries_to_loop_base(&mut self)
fn push(&mut self, scope: Scope)
fn push_function_scope(&mut self, range: TextRange, name: &Identifier, in_class: bool, is_async: bool)
fn record_nn_module_registration(&mut self, receiver: &Expr, name: Name, value: Expr)
fn record_or_reject_return(&mut self, ret: CurrentIdx, x: StmtReturn, is_unreachable: bool) -> Result<(), (CurrentIdx, StmtReturn)>
fn record_or_reject_yield(&mut self, idx: Idx<KeyYield>, x: ExprYield, is_unreachable: bool) -> Result<(), ExprYield>
fn record_or_reject_yield_from(&mut self, idx: Idx<KeyYieldFrom>, x: ExprYieldFrom, is_unreachable: bool) -> Result<(), ExprYieldFrom>
fn record_self_assignments_if_applicable(&mut self, self_assignments: Option<SelfAssignments>)
fn record_self_attr_assign(&mut self, x: &ExprAttribute, value: ExprOrBinding, annotation: Option<Idx<KeyAnnotation>>) -> bool
fn register_future_import(&mut self, name: &Identifier)
fn register_import(&mut self, name: &Identifier)
fn register_import_with_star(&mut self, name: &Identifier)
fn register_parameter(&mut self, name: &Identifier, allow_unused: bool)
fn register_reexport_import(&mut self, name: &Identifier)
fn register_variable(&mut self, name: &Identifier)
fn resume_after_with(&mut self, last_statement_key: Idx<Key>)
fn set_definitely_unreachable(&mut self, is_definitely_unreachable: bool)
fn set_has_future_annotations(&mut self)
fn set_last_stmt_expr(&mut self, key: Option<Idx<Key>>)
fn set_self_name_if_applicable(&mut self, self_name: Option<Identifier>, receiver_kind: MethodSelfKind)
fn swap_current_flow_with(&mut self, flow: &mut Flow)
fn terminated_by_raise(&self) -> bool
fn validate_mutable_capture_and_get_key(&self, name: Hashed<&Name>, kind: MutableCaptureKind) -> Result<(Key, Option<ModuleName>), MutableCaptureError>
```

Scopes keep track of the current stack of the scopes we are in.

---

## UnusedImport

`struct` · `pyrefly::binding::scope::UnusedImport`

```rust
struct UnusedImport
```

**Fields**: `name`, `range`

**Derives**: Clone, Debug

---

## UnusedParameter

`struct` · `pyrefly::binding::scope::UnusedParameter`

```rust
struct UnusedParameter
```

**Fields**: `name`, `range`

**Derives**: Clone, Debug

---

## UnusedVariable

`struct` · `pyrefly::binding::scope::UnusedVariable`

```rust
struct UnusedVariable
```

**Fields**: `name`, `range`

**Derives**: Clone, Debug

---

## YieldsAndReturns

`struct` · `pyrefly::binding::scope::YieldsAndReturns`

```rust
struct YieldsAndReturns
```

**Fields**: `returns`, `yields`, `yield_froms`, `is_generator`

**Derives**: Clone, Debug, Default

Things we collect from inside a function.
The boolean flag is set when we know for sure the statement is definitely unreachable.

---
