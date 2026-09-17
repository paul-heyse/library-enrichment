# `ra_ap_hir::semantics`

Crate `ra_ap_hir` · 9 public items · structured records in [`model/ra_ap_hir.semantics.json`](../model/ra_ap_hir.semantics.json)

## LintAttr

`enum` · `ra_ap_hir::semantics::LintAttr`

Also reachable as `ra_ap_hir::LintAttr`

```rust
enum LintAttr
```

**Variants**: `Allow`, `Expect`, `Warn`, `Deny`, `Forbid`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PathResolution

`enum` · `ra_ap_hir::semantics::PathResolution`

Also reachable as `ra_ap_hir::PathResolution`

```rust
enum PathResolution<'db>
```

**Variants**: `Def`, `Local`, `TypeParam`, `ConstParam`, `SelfType`, `BuiltinAttr`, `ToolModule`, `DeriveHelper`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## PathResolutionPerNs

`struct` · `ra_ap_hir::semantics::PathResolutionPerNs`

Also reachable as `ra_ap_hir::PathResolutionPerNs`

```rust
struct PathResolutionPerNs<'db>
```

**Fields**: `type_ns`, `value_ns`, `macro_ns`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn any(&self) -> Option<PathResolution<'db>>
fn new(type_ns: Option<PathResolution<'db>>, value_ns: Option<PathResolution<'db>>, macro_ns: Option<PathResolution<'db>>) -> Self
```

---

## Semantics

`struct` · `ra_ap_hir::semantics::Semantics`

Also reachable as `ra_ap_hir::Semantics`, `ra_ap_ide::Semantics`

```rust
struct Semantics<'db, DB: ?Sized>
```

**Fields**: `db`

**Implements**: `core::ops::deref::Deref`

**Derives**: Debug

**Methods** (35)

```rust
fn file_to_module_def(&self, file: impl Into<FileId>) -> Option<Module>
fn file_to_module_defs(&self, file: impl Into<FileId>) -> impl Iterator<Item = Module>
fn find_namelike_at_offset_with_descend<'slf>(&'slf self, node: &SyntaxNode, offset: TextSize) -> impl Iterator<Item = ast::NameLike> + 'slf
fn find_node_at_offset_with_descend<N: AstNode>(&self, node: &SyntaxNode, offset: TextSize) -> Option<N>
fn find_node_at_offset_with_macros<N: AstNode>(&self, node: &SyntaxNode, offset: TextSize) -> Option<N>
fn find_nodes_at_offset_with_descend<'slf, N: AstNode + 'slf>(&'slf self, node: &SyntaxNode, offset: TextSize) -> impl Iterator<Item = N> + 'slf
fn hir_file_for(&self, syntax_node: &SyntaxNode) -> HirFileId
fn hir_file_to_module_def(&self, file: impl Into<HirFileId>) -> Option<Module>
fn hir_file_to_module_defs(&self, file: impl Into<HirFileId>) -> impl Iterator<Item = Module>
fn is_nightly(&self, krate: Crate) -> bool
fn lint_attrs(&self, file_id: FileId, krate: Crate, item: ast::AnyHasAttrs) -> impl DoubleEndedIterator<Item = (LintAttr, SmolStr)>
fn new(db: &DB) -> Semantics<'_, DB>
fn new_dyn(db: &dyn HirDatabase) -> Semantics<'_, dyn HirDatabase>
fn resolve_await_to_poll(&self, await_expr: &ast::AwaitExpr) -> Option<Function>
fn resolve_bin_expr(&self, bin_expr: &ast::BinExpr) -> Option<Function>
fn resolve_index_expr(&self, index_expr: &ast::IndexExpr) -> Option<Function>
fn resolve_prefix_expr(&self, prefix_expr: &ast::PrefixExpr) -> Option<Function>
fn resolve_range_expr(&self, range_expr: &ast::RangeExpr) -> Option<Struct>
fn resolve_range_pat(&self, range_pat: &ast::RangePat) -> Option<Struct>
fn resolve_try_expr(&self, try_expr: &ast::TryExpr) -> Option<Function>
fn resolve_variant(&self, record_lit: ast::RecordExpr) -> Option<Variant>
fn to_adt_def(&self, a: &ast::Adt) -> Option<Adt>
fn to_const_def(&self, c: &ast::Const) -> Option<Const>
fn to_enum_def(&self, e: &ast::Enum) -> Option<Enum>
fn to_enum_variant_def(&self, v: &ast::Variant) -> Option<EnumVariant>
fn to_fn_def(&self, f: &ast::Fn) -> Option<Function>
fn to_impl_def(&self, i: &ast::Impl) -> Option<Impl>
fn to_macro_def(&self, m: &ast::Macro) -> Option<Macro>
fn to_module_def(&self, m: &ast::Module) -> Option<Module>
fn to_static_def(&self, s: &ast::Static) -> Option<Static>
fn to_struct_def(&self, s: &ast::Struct) -> Option<Struct>
fn to_trait_def(&self, t: &ast::Trait) -> Option<Trait>
fn to_type_alias_def(&self, t: &ast::TypeAlias) -> Option<TypeAlias>
fn to_union_def(&self, u: &ast::Union) -> Option<Union>
fn token_ancestors_with_macros(&self, token: SyntaxToken) -> impl Iterator<Item = SyntaxNode> + '_
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Primary API to get semantic information, like types, from syntax trees.

---

## SemanticsImpl

`struct` · `ra_ap_hir::semantics::SemanticsImpl`

Also reachable as `ra_ap_hir::SemanticsImpl`

```rust
struct SemanticsImpl<'db>
```

**Fields**: `db`

**Methods** (108)

```rust
fn adjust_edition(&self, file_id: HirFileId) -> HirFileId
fn ancestors_at_offset_with_macros(&self, node: &SyntaxNode, offset: TextSize) -> impl Iterator<Item = SyntaxNode> + '_
fn ancestors_with_macros(&self, node: SyntaxNode) -> impl Iterator<Item = SyntaxNode> + Clone + '_
fn ancestors_with_macros_file(&self, node: InFile<SyntaxNode>) -> impl Iterator<Item = InFile<SyntaxNode>> + Clone + '_
fn as_format_args_parts(&self, string: &ast::String) -> Option<Vec<(TextRange, Option<Either<PathResolution<'db>, InlineAsmOperand>>)>>
fn assert_contains_node(&self, node: &SyntaxNode)
fn attach_first_edition(&self, file: FileId) -> EditionedFileId
fn attach_first_edition_opt(&self, file: FileId) -> Option<EditionedFileId>
fn binding_mode_of_pat(&self, pat: &ast::IdentPat) -> Option<BindingMode>
fn check_for_format_args_template(&self, original_token: SyntaxToken, offset: TextSize) -> Option<(TextRange, HirFileRange, ast::String, Option<Either<PathResolution<'db>, InlineAsmOperand>>)>
fn check_for_format_args_template_with_file(&self, original_token: InFile<ast::String>, offset: TextSize) -> Option<(TextRange, HirFileRange, ast::String, Option<Either<PathResolution<'db>, InlineAsmOperand>>)>
fn debug_hir_at(&self, token: SyntaxToken) -> Option<String>
fn derive_helper(&self, attr: &ast::Attr) -> Option<Vec<(Macro, MacroCallId)>>
fn derive_helpers_in_scope(&self, adt: &ast::Adt) -> Option<Vec<(Symbol, Symbol)>>
fn descend_into_macros(&self, token: SyntaxToken) -> SmallVec<[SyntaxToken; 1]>
fn descend_into_macros_breakable<T>(&self, token: InFile<SyntaxToken>, cb: impl FnMut(InFile<SyntaxToken>, SyntaxContext) -> ControlFlow<T>) -> Option<T>
fn descend_into_macros_cb(&self, token: SyntaxToken, cb: impl FnMut(InFile<SyntaxToken>, SyntaxContext))
fn descend_into_macros_exact(&self, token: SyntaxToken) -> SmallVec<[SyntaxToken; 1]>
fn descend_into_macros_exact_with_file(&self, token: SyntaxToken) -> SmallVec<[InFile<SyntaxToken>; 1]>
fn descend_into_macros_no_opaque(&self, token: SyntaxToken, always_descend_into_derives: bool) -> SmallVec<[InFile<SyntaxToken>; 1]>
fn descend_into_macros_single_exact(&self, token: SyntaxToken) -> SyntaxToken
fn descend_node_into_attributes<N: AstNode>(&self, node: N) -> SmallVec<[N; 1]>
fn descend_token_into_include_expansion(&self, tok: InRealFile<SyntaxToken>) -> InFile<SyntaxToken>
fn diagnostics_display_range(&self, src: InFile<SyntaxNodePtr>) -> FileRangeWrapper<FileId>
fn diagnostics_display_range_for_range(&self, src: InFile<TextRange>) -> FileRangeWrapper<FileId>
fn evaluate_where_clause_at(&self, node: &SyntaxNode, offset: TextSize, where_clause: ast::WhereClause) -> PredicateEvaluationResult
fn expand(&self, file_id: MacroCallId) -> ExpandResult<SyntaxNode>
fn expand_allowed_builtins(&self, macro_call: &ast::MacroCall) -> Option<ExpandResult<SyntaxNode>>
fn expand_attr_macro(&self, item: &ast::Item) -> Option<ExpandResult<InFile<SyntaxNode>>>
fn expand_derive_as_pseudo_attr_macro(&self, attr: &ast::Meta) -> Option<SyntaxNode>
fn expand_derive_macro(&self, attr: &ast::Meta) -> Option<Vec<Option<ExpandResult<SyntaxNode>>>>
fn expand_macro_call(&self, macro_call: &ast::MacroCall) -> Option<InFile<SyntaxNode>>
fn expr_adjustments(&self, expr: &ast::Expr) -> Option<Vec<Adjustment<'db>>>
fn expr_is_diverging(&self, expr: &ast::Expr) -> bool
fn find_parent_file(&self, file_id: HirFileId) -> Option<InFile<SyntaxNode>>
fn first_crate(&self, file: FileId) -> Option<Crate>
fn fn_return_points(&self, func: Function) -> Vec<InFile<ast::ReturnExpr>>
fn get_failed_obligations(&self, token: SyntaxToken) -> Option<String>
fn get_unsafe_ops(&self, def: ExpressionStoreOwner) -> FxHashSet<ExprOrPatSource>
fn get_unsafe_ops_for_unsafe_block(&self, block: ast::BlockExpr) -> Vec<ExprOrPatSource>
fn impl_generated_from_derive(&self, impl_: Impl) -> Option<Adt>
fn is_attr_macro_call(&self, item: InFile<&ast::Item>) -> bool
fn is_derive_annotated(&self, adt: InFile<&ast::Adt>) -> bool
fn is_inside_macro_call(&self, token: InFile<&SyntaxToken>) -> bool
fn is_inside_unsafe(&self, expr: &ast::Expr) -> bool
fn is_proc_macro_call(&self, macro_call: InFile<&ast::MacroCall>) -> bool
fn is_unsafe_macro_call(&self, macro_call: &ast::MacroCall) -> bool
fn locals_used(&self, element: Either<&ast::Expr, &ast::StmtList>, text_range: TextRange) -> Option<FxIndexSet<Local<'db>>>
fn module_definition_node(&self, module: Module) -> InFile<SyntaxNode>
fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>
fn original_range(&self, node: &SyntaxNode) -> FileRange
fn original_range_opt(&self, node: &SyntaxNode) -> Option<FileRange>
fn original_syntax_node_rooted(&self, node: &SyntaxNode) -> Option<SyntaxNode>
fn parse(&self, file_id: EditionedFileId) -> ast::SourceFile
fn parse_guess_edition(&self, file_id: FileId) -> ast::SourceFile
fn parse_or_expand(&self, file_id: HirFileId) -> SyntaxNode
fn pattern_adjustments(&self, pat: &ast::Pat) -> SmallVec<[Type<'db>; 1]>
fn record_literal_matched_fields(&self, literal: &ast::RecordExpr) -> Vec<(Field, Type<'db>)>
fn record_literal_missing_fields(&self, literal: &ast::RecordExpr) -> Vec<(Field, Type<'db>)>
fn record_pattern_matched_fields(&self, pattern: &ast::RecordPat) -> Vec<(Field, Type<'db>)>
fn record_pattern_missing_fields(&self, pattern: &ast::RecordPat) -> Vec<(Field, Type<'db>)>
fn rename_conflicts<'a>(&self, to_be_renamed: &Local<'a>, new_name: &Name) -> Vec<Local<'a>>
fn resolve_attr_macro_call(&self, item: &ast::Item) -> Option<Macro>
fn resolve_bind_pat_to_const(&self, pat: &ast::IdentPat) -> Option<ModuleDef>
fn resolve_derive_macro(&self, attr: &ast::Meta) -> Option<Vec<Option<Macro>>>
fn resolve_expr_as_callable(&self, call: &ast::Expr) -> Option<Callable<'db>>
fn resolve_field(&self, field: &ast::FieldExpr) -> Option<Either<Field, TupleField<'db>>>
fn resolve_field_fallback(&self, field: &ast::FieldExpr) -> Option<(Either<Either<Field, TupleField<'db>>, Function>, Option<GenericSubstitution<'db>>)>
fn resolve_label(&self, label: &ast::Lifetime) -> Option<Label>
fn resolve_lifetime_param(&self, lifetime: &ast::Lifetime) -> Option<LifetimeParam>
fn resolve_macro_call(&self, macro_call: &ast::MacroCall) -> Option<Macro>
fn resolve_macro_call2(&self, macro_call: InFile<&ast::MacroCall>) -> Option<Macro>
fn resolve_macro_call_arm(&self, macro_call: &ast::MacroCall) -> Option<u32>
fn resolve_method_call(&self, call: &ast::MethodCallExpr) -> Option<Function>
fn resolve_method_call_as_callable(&self, call: &ast::MethodCallExpr) -> Option<Callable<'db>>
fn resolve_method_call_fallback(&self, call: &ast::MethodCallExpr) -> Option<(Either<Function, Field>, Option<GenericSubstitution<'db>>)>
fn resolve_mod_path(&self, scope: &SyntaxNode, path: &ModPath) -> Option<impl Iterator<Item = ItemInNs>>
fn resolve_offset_of_field(&self, name_ref: &ast::NameRef) -> Option<(Either<EnumVariant, Field>, GenericSubstitution<'db>)>
fn resolve_path(&self, path: &ast::Path) -> Option<PathResolution<'db>>
fn resolve_path_per_ns(&self, path: &ast::Path) -> Option<PathResolutionPerNs<'db>>
fn resolve_path_with_subst(&self, path: &ast::Path) -> Option<(PathResolution<'db>, Option<GenericSubstitution<'db>>)>
fn resolve_record_field(&self, field: &ast::RecordExprField) -> Option<(Field, Option<Local<'db>>, Type<'db>)>
fn resolve_record_field_with_substitution(&self, field: &ast::RecordExprField) -> Option<(Field, Option<Local<'db>>, Type<'db>, GenericSubstitution<'db>)>
fn resolve_record_pat_field(&self, field: &ast::RecordPatField) -> Option<(Field, Type<'db>)>
fn resolve_record_pat_field_with_subst(&self, field: &ast::RecordPatField) -> Option<(Field, Type<'db>, GenericSubstitution<'db>)>
fn resolve_trait(&self, path: &ast::Path) -> Option<Trait>
fn resolve_trait_impl_method(&self, env: Type<'db>, trait_: Trait, func: Function, subst: impl IntoIterator<Item = Type<'db>>) -> Option<Function>
fn resolve_tuple_struct_pat_fields(&self, tuple_struct_pat: &ast::TupleStructPat) -> Option<Vec<(Field, Type<'db>)>>
fn resolve_type(&self, ty: &ast::Type) -> Option<Type<'db>>
fn resolve_use_type_arg(&self, name: &ast::NameRef) -> Option<TypeParam>
fn scope(&self, node: &SyntaxNode) -> Option<SemanticsScope<'db>>
fn scope_at_offset(&self, node: &SyntaxNode, offset: TextSize) -> Option<SemanticsScope<'db>>
fn source<Def: HasSource>(&self, def: Def) -> Option<InFile<Def::Ast>>
fn source_with_range<Def: HasSource>(&self, def: Def) -> Option<InFile<(TextRange, Option<Def::Ast>)>>
fn speculative_expand_attr_macro(&self, actual_macro_call: &ast::Item, speculative_args: &ast::Item, token_to_map: SyntaxToken) -> Option<(SyntaxNode, Vec<(SyntaxToken, u8)>)>
fn speculative_expand_derive_as_pseudo_attr_macro(&self, actual_macro_call: &ast::Attr, speculative_args: &ast::Attr, token_to_map: SyntaxToken) -> Option<(SyntaxNode, Vec<(SyntaxToken, u8)>)>
fn speculative_expand_macro_call(&self, actual_macro_call: &ast::MacroCall, speculative_args: &ast::TokenTree, token_to_map: SyntaxToken) -> Option<(SyntaxNode, Vec<(SyntaxToken, u8)>)>
fn speculative_expand_raw(&self, macro_file: MacroCallId, speculative_args: &SyntaxNode, token_to_map: SyntaxToken) -> Option<(SyntaxNode, Vec<(SyntaxToken, u8)>)>
fn store_owner_for(&self, node: InFile<&SyntaxNode>) -> Option<ExpressionStoreOwner>
fn to_def<T: ToDef<'db>>(&self, src: &T) -> Option<T::Def>
fn to_def2<T: ToDef<'db>>(&self, src: InFile<&T>) -> Option<T::Def>
fn to_node<N: AstNode>(&self, ptr: InFile<AstPtr<N>>) -> N
fn to_node_syntax(&self, ptr: InFile<SyntaxNodePtr>) -> SyntaxNode
fn try_expr_returned_type(&self, try_expr: &ast::TryExpr) -> Option<Type<'db>>
fn type_of_binding_in_pat(&self, pat: &ast::IdentPat) -> Option<Type<'db>>
fn type_of_expr(&self, expr: &ast::Expr) -> Option<TypeInfo<'db>>
fn type_of_pat(&self, pat: &ast::Pat) -> Option<TypeInfo<'db>>
fn type_of_self(&self, param: &ast::SelfParam) -> Option<Type<'db>>
```

---

## SemanticsScope

`struct` · `ra_ap_hir::semantics::SemanticsScope`

Also reachable as `ra_ap_hir::SemanticsScope`

```rust
struct SemanticsScope<'db>
```

**Fields**: `db`

**Derives**: Debug

**Methods** (15)

```rust
fn assoc_type_shorthand_candidates(&self, resolution: &PathResolution<'db>, cb: impl FnMut(TypeAlias))
fn can_use_trait_methods(&self, t: Trait) -> bool
fn containing_function(&self) -> Option<Function>
fn expression_store_owner(&self) -> Option<ExpressionStoreOwner>
fn extern_crate_decls(&self) -> impl Iterator<Item = Name> + '_
fn extern_crates(&self) -> impl Iterator<Item = (Name, Module)> + '_
fn file_id(&self) -> HirFileId
fn generic_def(&self) -> Option<GenericDef>
fn has_same_self_type(&self, other: &SemanticsScope<'_>) -> bool
fn krate(&self) -> Crate
fn module(&self) -> Module
fn process_all_names(&self, f: &mut dyn FnMut(Name, ScopeDef<'db>))
fn resolve_mod_path(&self, path: &ModPath) -> impl Iterator<Item = ItemInNs> + use<>
fn speculative_resolve(&self, ast_path: &ast::Path) -> Option<PathResolution<'db>>
fn visible_traits(&self) -> VisibleTraits
```

`SemanticsScope` encapsulates the notion of a scope (the set of visible
names) at a particular program point.

It is a bit tricky, as scopes do not really exist inside the compiler.
Rather, the compiler directly computes for each reference the definition it
refers to. It might transiently compute the explicit scope map while doing
so, but, generally, this is not something left after the analysis.

However, we do very much need explicit scopes for IDE purposes --
completion, at its core, lists the contents of the current scope. The notion
of scope is also useful to answer questions like "what would be the meaning
of this piece of code if we inserted it into this position?".

So `SemanticsScope` is constructed from a specific program point (a syntax
node or just a raw offset) and provides access to the set of visible names
on a somewhat best-effort basis.

Note that if you are wondering "what does this specific existing name mean?",
you'd better use the `resolve_` family of methods.

---

## TypeInfo

`struct` · `ra_ap_hir::semantics::TypeInfo`

Also reachable as `ra_ap_hir::TypeInfo`

```rust
struct TypeInfo<'db>
```

**Fields**: `original`, `adjusted`

**Derives**: Debug

**Methods** (3)

```rust
fn adjusted(self) -> Type<'db>
fn has_adjustment(&self) -> bool
fn original(self) -> Type<'db>
```

---

## VisibleTraits

`struct` · `ra_ap_hir::semantics::VisibleTraits`

Also reachable as `ra_ap_hir::VisibleTraits`

```rust
struct VisibleTraits
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Debug

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

---

## ToDef

`trait` · `ra_ap_hir::semantics::ToDef`

```rust
trait ToDef<'db>: AstNode + Clone
```

**Methods** (1)

```rust
fn to_def(sema: &SemanticsImpl<'db>, src: InFile<&Self>) -> Option<Self::Def>
```

---
