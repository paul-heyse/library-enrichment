# `ruff_python_semantic::model`

Crate `ruff_python_semantic` · 9 public items · structured records in [`model/ruff_python_semantic.model.json`](../model/ruff_python_semantic.model.json)

## ReadResult

`enum` · `ruff_python_semantic::model::ReadResult`

Also reachable as `ruff_python_semantic::ReadResult`

```rust
enum ReadResult
```

**Variants**: `Resolved`, `ImplicitGlobal`, `WildcardImport`, `UnboundLocal`, `NotFound`

**Derives**: Debug

---

## Symbol

`enum` · `ruff_python_semantic::model::Symbol`

Also reachable as `ruff_python_semantic::Symbol`

```rust
enum Symbol
```

**Variants**: `Binding`, `Builtin`, `Unbound`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn binding_id(self) -> Option<BindingId>
const fn is_bound(self) -> bool
```

The result of looking up a symbol in a [`SemanticModel`].

---

## TypingOnlyBindingsStatus

`enum` · `ruff_python_semantic::model::TypingOnlyBindingsStatus`

Also reachable as `ruff_python_semantic::TypingOnlyBindingsStatus`

```rust
enum TypingOnlyBindingsStatus
```

**Variants**: `Allowed`, `Disallowed`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn is_allowed(self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: bool) -> Self
```

---

## ImportedName

`struct` · `ruff_python_semantic::model::ImportedName`

Also reachable as `ruff_python_semantic::ImportedName`

```rust
struct ImportedName
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Debug

**Methods** (3)

```rust
const fn context(&self) -> ExecutionContext
fn into_name(self) -> String
fn statement<'a>(&self, semantic: &SemanticModel<'a>) -> &'a Stmt
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## Modules

`struct` · `ruff_python_semantic::model::Modules`

Also reachable as `ruff_python_semantic::Modules`

```rust
struct Modules
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Debug

**Methods** (22)

```rust
const fn all() -> Self
const fn bits(&self) -> u32
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: u32) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: u32) -> Self
const fn from_bits_truncate(bits: u32) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<Modules>
const fn iter_names(&self) -> iter::IterNames<Modules>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> Modules
fn bits(&self) -> u32
fn from_bits_retain(bits: u32) -> Modules
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: Modules) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

A select list of Python modules that the semantic model can explicitly track.

---

## SemanticModel

`struct` · `ruff_python_semantic::model::SemanticModel`

Also reachable as `ruff_python_semantic::SemanticModel`

```rust
struct SemanticModel<'a>
```

**Fields**: `scopes`, `scope_id`, `definitions`, `definition_id`, `bindings`, `shadowed_bindings`, `flags`, `seen`, `handled_exceptions`

**Methods** (124)

```rust
fn add_delayed_annotation(&mut self, binding_id: BindingId, annotation_id: BindingId)
fn add_global_reference(&mut self, binding_id: BindingId, ctx: ExprContext, range: TextRange)
fn add_local_reference(&mut self, binding_id: BindingId, ctx: ExprContext, range: TextRange)
fn add_module(&mut self, module: &str)
fn add_rebinding_scope(&mut self, binding_id: BindingId, scope_id: ScopeId)
fn at_top_level(&self) -> bool
fn binding(&self, id: BindingId) -> &Binding<'a>
fn current_expression(&self) -> Option<&'a Expr>
fn current_expression_grandparent(&self) -> Option<&'a Expr>
fn current_expression_parent(&self) -> Option<&'a Expr>
fn current_expressions(&self) -> impl Iterator<Item = &'a Expr> + '_
fn current_scope(&self) -> &Scope<'a>
fn current_scope_ids(&self) -> impl Iterator<Item = ScopeId> + '_
fn current_scope_mut(&mut self) -> &mut Scope<'a>
fn current_scopes(&self) -> impl Iterator<Item = &Scope<'a>>
fn current_statement(&self) -> &'a Stmt
fn current_statement_id(&self) -> Option<NodeId>
fn current_statement_ids(&self) -> impl Iterator<Item = NodeId> + '_
fn current_statement_parent(&self) -> Option<&'a Stmt>
fn current_statement_parent_id(&self) -> Option<NodeId>
fn current_statements(&self) -> impl Iterator<Item = &'a Stmt> + '_
fn delayed_annotations(&self, binding_id: BindingId) -> Option<&[BindingId]>
fn dominates(&self, dominator: NodeId, node: NodeId) -> bool
const fn execution_context(&self) -> ExecutionContext
fn expression(&self, node_id: NodeId) -> Option<&'a Expr>
fn expressions(&self, node_id: NodeId) -> impl Iterator<Item = &'a Expr> + '_
fn extract_dunder_all_names<'expr>(&self, stmt: &'expr Stmt) -> (Vec<DunderAllName<'expr>>, DunderAllFlags)
fn first_non_type_parent_scope(&self, scope: &Scope<'_>) -> Option<&Scope<'a>>
fn first_non_type_parent_scope_id(&self, scope_id: ScopeId) -> Option<ScopeId>
fn function_scope(&self, function_def: &ast::StmtFunctionDef) -> Option<&Scope<'_>>
const fn future_annotations_or_stub(&self) -> bool
fn global(&self, name: &str) -> Option<TextRange>
fn global_binding(&mut self, symbol: &'a str) -> Option<BindingId>
fn global_scope(&self) -> &Scope<'a>
fn has_builtin_binding(&self, member: &str) -> bool
fn has_builtin_binding_in_scope(&self, member: &str, scope: ScopeId) -> bool
const fn in_annotated_type_alias_value(&self) -> bool
const fn in_annotation(&self) -> bool
const fn in_assert_statement(&self) -> bool
fn in_async_context(&self) -> bool
const fn in_attribute_docstring(&self) -> bool
const fn in_boolean_test(&self) -> bool
const fn in_class_base(&self) -> bool
const fn in_complex_string_type_definition(&self) -> bool
const fn in_deferred_class_base(&self) -> bool
const fn in_deferred_type_alias_value(&self) -> bool
const fn in_deferred_type_definition(&self) -> bool
const fn in_dunder_all_definition(&self) -> bool
const fn in_exception_handler(&self) -> bool
const fn in_f_string(&self) -> bool
const fn in_interpolated_string(&self) -> bool
const fn in_interpolated_string_replacement_field(&self) -> bool
const fn in_named_expression_assignment(&self) -> bool
fn in_nested_literal(&self) -> bool
fn in_nested_union(&self) -> bool
const fn in_no_type_check(&self) -> bool
const fn in_orelse(&self) -> bool
const fn in_pep_257_docstring(&self) -> bool
fn in_protocol_or_abstract_method(&self) -> bool
const fn in_runtime_evaluated_annotation(&self) -> bool
const fn in_runtime_required_annotation(&self) -> bool
const fn in_simple_string_type_definition(&self) -> bool
const fn in_string_type_definition(&self) -> bool
const fn in_stub_file(&self) -> bool
const fn in_subscript(&self) -> bool
const fn in_type_alias_value(&self) -> bool
const fn in_type_checking_block(&self) -> bool
const fn in_type_definition(&self) -> bool
const fn in_typing_literal(&self) -> bool
const fn in_typing_only_annotation(&self) -> bool
fn inside_optional(&self) -> bool
fn is_available(&self, member: &str) -> bool
fn is_available_in_scope(&self, member: &str, scope_id: ScopeId) -> bool
fn is_current_scope(&self, scope_id: ScopeId) -> bool
fn is_unused(&self, expr: &Expr) -> bool
fn lookup_attribute(&self, value: &Expr) -> Option<BindingId>
fn lookup_attribute_in_scope(&self, value: &Expr, scope_id: ScopeId) -> Option<BindingId>
fn lookup_binding(&mut self, symbol: &'a str) -> Option<BindingId>
fn lookup_symbol(&self, symbol: &str) -> Symbol
fn lookup_symbol_in_scope(&self, symbol: &str, scope_id: ScopeId, in_forward_reference: bool) -> Symbol
fn match_builtin_expr(&self, expr: &Expr, symbol: &str) -> bool
fn match_typing_expr(&self, expr: &Expr, target: &str) -> bool
fn match_typing_qualified_name(&self, qualified_name: &QualifiedName<'_>, target: &str) -> bool
fn new(typing_modules: &'a [String], custom_builtins: &'a [String], target_version: PythonVersion, source_type: PySourceType, path: &Path, module: Module<'a>) -> Self
fn node(&self, node_id: NodeId) -> &NodeRef<'a>
fn nonlocal(&self, name: &str) -> Option<(ScopeId, BindingId)>
fn only_binding(&self, name: &ast::ExprName) -> Option<BindingId>
fn parent_expression_id(&self, node_id: NodeId) -> Option<NodeId>
fn parent_statement(&self, node_id: NodeId) -> Option<&'a Stmt>
fn parent_statement_id(&self, node_id: NodeId) -> Option<NodeId>
fn pop_branch(&mut self)
fn pop_definition(&mut self)
fn pop_node(&mut self)
fn pop_scope(&mut self)
fn push_binding(&mut self, name: &'a str, range: TextRange, kind: BindingKind<'a>, flags: BindingFlags) -> BindingId
fn push_branch(&mut self) -> Option<BranchId>
fn push_definition(&mut self, definition: Member<'a>)
fn push_node<T: Into<NodeRef<'a>>>(&mut self, node: T)
fn push_scope(&mut self, kind: ScopeKind<'a>)
fn rebinding_scopes(&self, binding_id: BindingId) -> Option<&[ScopeId]>
fn reference(&self, id: ResolvedReferenceId) -> &ResolvedReference
fn resolve_builtin_symbol<'expr>(&'a self, expr: &'expr Expr) -> Option<&'a str> where 'expr: 'a
fn resolve_del(&mut self, symbol: &'a str, range: TextRange)
fn resolve_load(&mut self, name: &'a ast::ExprName) -> ReadResult
fn resolve_name(&self, name: &ast::ExprName) -> Option<BindingId>
fn resolve_qualified_import_name(&self, module: &str, member: &str) -> Option<ImportedName>
fn resolve_qualified_name<'name, 'expr: 'name>(&self, value: &'expr Expr) -> Option<QualifiedName<'name>> where 'a: 'name
fn restore(&mut self, snapshot: Snapshot)
fn same_branch(&self, left: NodeId, right: NodeId) -> bool
const fn seen_import_boundary(&self) -> bool
fn seen_module(&self, module: Modules) -> bool
const fn seen_module_docstring_boundary(&self) -> bool
fn seen_typing(&self) -> bool
fn set_branch(&mut self, branch_id: Option<BranchId>)
fn set_globals(&mut self, globals: Globals<'a>)
fn shadowed_binding(&self, binding_id: BindingId) -> Option<BindingId>
fn shadowed_bindings(&self, scope_id: ScopeId, binding_id: BindingId) -> impl Iterator<Item = ShadowedBinding> + '_
fn simulate_runtime_load(&self, name: &ast::ExprName, typing_only_bindings_status: TypingOnlyBindingsStatus) -> Option<BindingId>
fn simulate_runtime_load_at_location_in_scope(&self, symbol: &str, symbol_range: TextRange, scope_id: ScopeId, typing_only_bindings_status: TypingOnlyBindingsStatus) -> Option<BindingId>
fn snapshot(&self) -> Snapshot
fn statement(&self, node_id: NodeId) -> &'a Stmt
fn statements(&self, node_id: NodeId) -> impl Iterator<Item = &'a Stmt> + '_
fn typing_modules(&self) -> impl Iterator<Item = &'a str>
fn unresolved_references(&self) -> impl Iterator<Item = &UnresolvedReference>
```

A semantic model for a Python module, to enable querying the module's semantic information.

---

## SemanticModelFlags

`struct` · `ruff_python_semantic::model::SemanticModelFlags`

Also reachable as `ruff_python_semantic::SemanticModelFlags`

```rust
struct SemanticModelFlags
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (22)

```rust
const fn all() -> Self
const fn bits(&self) -> u32
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: u32) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: u32) -> Self
const fn from_bits_truncate(bits: u32) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<SemanticModelFlags>
const fn iter_names(&self) -> iter::IterNames<SemanticModelFlags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> SemanticModelFlags
fn bits(&self) -> u32
fn from_bits_retain(bits: u32) -> SemanticModelFlags
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: SemanticModelFlags) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

Flags indicating the current model state.

---

## ShadowedBinding

`struct` · `ruff_python_semantic::model::ShadowedBinding`

Also reachable as `ruff_python_semantic::ShadowedBinding`

```rust
struct ShadowedBinding
```

**Methods** (3)

```rust
const fn binding_id(&self) -> BindingId
const fn same_scope(&self) -> bool
const fn shadowed_id(&self) -> BindingId
```

---

## Snapshot

`struct` · `ruff_python_semantic::model::Snapshot`

Also reachable as `ruff_python_semantic::Snapshot`

```rust
struct Snapshot
```

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

A snapshot of the [`SemanticModel`] at a given point in the AST traversal.

---
