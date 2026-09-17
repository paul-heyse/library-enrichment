# `ra_ap_syntax::ast::syntax_factory`

Crate `ra_ap_syntax` · 1 public items · structured records in [`model/ra_ap_syntax.ast.syntax_factory.json`](../model/ra_ap_syntax.ast.syntax_factory.json)

## SyntaxFactory

`struct` · `ra_ap_syntax::ast::syntax_factory::SyntaxFactory`

```rust
struct SyntaxFactory
```

**Derives**: Debug, Default

**Methods** (154)

```rust
fn arg_list(&self, args: impl IntoIterator<Item = ast::Expr>) -> ast::ArgList
fn assoc_item_list(&self, items: impl IntoIterator<Item = ast::AssocItem>) -> ast::AssocItemList
fn assoc_item_list_empty(&self) -> ast::AssocItemList
fn async_move_block_expr(&self, statements: impl IntoIterator<Item = ast::Stmt>, tail_expr: Option<ast::Expr>) -> ast::BlockExpr
fn attr_inner(&self, meta: ast::Meta) -> ast::Attr
fn attr_outer(&self, meta: ast::Meta) -> ast::Attr
fn block_expr(&self, statements: impl IntoIterator<Item = ast::Stmt>, tail_expr: Option<ast::Expr>) -> ast::BlockExpr
fn box_pat(&self, pat: ast::Pat) -> ast::BoxPat
fn cfg_attr_meta(&self, predicate: ast::CfgPredicate, inner: impl IntoIterator<Item = ast::Meta>) -> ast::CfgAttrMeta
fn cfg_flag(&self, flag: &str) -> ast::CfgPredicate
fn const_param(&self, name: ast::Name, ty: ast::Type) -> ast::ConstParam
fn deref_pat(&self, pat: ast::Pat) -> ast::Pat
fn expr_assignment(&self, lhs: ast::Expr, rhs: ast::Expr) -> ast::BinExpr
fn expr_await(&self, expr: ast::Expr) -> ast::AwaitExpr
fn expr_bin(&self, lhs: ast::Expr, op: ast::BinaryOp, rhs: ast::Expr) -> ast::BinExpr
fn expr_bin_op(&self, lhs: ast::Expr, op: ast::BinaryOp, rhs: ast::Expr) -> ast::Expr
fn expr_break(&self, label: Option<Lifetime>, expr: Option<ast::Expr>) -> ast::BreakExpr
fn expr_call(&self, expr: ast::Expr, arg_list: ast::ArgList) -> ast::CallExpr
fn expr_closure(&self, pats: impl IntoIterator<Item = ast::Param>, expr: ast::Expr) -> ast::ClosureExpr
fn expr_const_value(&self, text: &str) -> ast::ConstArg
fn expr_continue(&self, label: Option<Lifetime>) -> ast::ContinueExpr
fn expr_empty_block(&self) -> ast::BlockExpr
fn expr_field(&self, receiver: ast::Expr, field: &str) -> ast::FieldExpr
fn expr_for_loop(&self, pat: ast::Pat, iterable: ast::Expr, body: ast::BlockExpr) -> ast::ForExpr
fn expr_if(&self, condition: ast::Expr, then_branch: ast::BlockExpr, else_branch: Option<ast::ElseBranch>) -> ast::IfExpr
fn expr_let(&self, pattern: ast::Pat, expr: ast::Expr) -> ast::LetExpr
fn expr_literal(&self, text: &str) -> ast::Literal
fn expr_loop(&self, body: ast::BlockExpr) -> ast::LoopExpr
fn expr_macro(&self, path: ast::Path, tt: ast::TokenTree) -> ast::MacroExpr
fn expr_match(&self, expr: ast::Expr, match_arm_list: ast::MatchArmList) -> ast::MatchExpr
fn expr_method_call(&self, receiver: ast::Expr, method: ast::NameRef, arg_list: ast::ArgList) -> ast::MethodCallExpr
fn expr_paren(&self, expr: ast::Expr) -> ast::ParenExpr
fn expr_path(&self, path: ast::Path) -> ast::Expr
fn expr_prefix(&self, op: SyntaxKind, expr: ast::Expr) -> ast::PrefixExpr
fn expr_raw_ref(&self, expr: ast::Expr, exclusive: bool) -> ast::Expr
fn expr_reborrow(&self, expr: ast::Expr) -> ast::Expr
fn expr_ref(&self, expr: ast::Expr, exclusive: bool) -> ast::Expr
fn expr_return(&self, expr: Option<ast::Expr>) -> ast::ReturnExpr
fn expr_self(&self) -> ast::Expr
fn expr_stmt(&self, expr: ast::Expr) -> ast::ExprStmt
fn expr_todo(&self) -> ast::Expr
fn expr_try(&self, expr: ast::Expr) -> ast::Expr
fn expr_tuple(&self, fields: impl IntoIterator<Item = ast::Expr>) -> ast::TupleExpr
fn expr_underscore(&self) -> ast::UnderscoreExpr
fn expr_unit(&self) -> ast::Expr
fn expr_while_loop(&self, condition: ast::Expr, body: ast::BlockExpr) -> ast::WhileExpr
fn field_from_idents<'a>(&self, parts: impl std::iter::IntoIterator<Item = &'a str>) -> Option<ast::Expr>
fn fn_(&self, attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, fn_name: ast::Name, type_params: Option<ast::GenericParamList>, where_clause: Option<ast::WhereClause>, params: ast::ParamList, body: ast::BlockExpr, ret_type: Option<ast::RetType>, is_async: bool, is_const: bool, is_unsafe: bool, is_gen: bool) -> ast::Fn
fn generic_arg_list(&self, generic_args: impl IntoIterator<Item = ast::GenericArg>, is_turbo: bool) -> ast::GenericArgList
fn generic_param_list(&self, params: impl IntoIterator<Item = ast::GenericParam>) -> ast::GenericParamList
fn generic_ty_path_segment(&self, name_ref: ast::NameRef, generic_args: impl IntoIterator<Item = ast::GenericArg>) -> ast::PathSegment
fn hacky_block_expr(&self, elements: impl IntoIterator<Item = SyntaxElement>, tail_expr: Option<ast::Expr>) -> ast::BlockExpr
fn ident(&self, text: &str) -> SyntaxToken
fn ident_pat(&self, ref_: bool, mut_: bool, name: ast::Name) -> ast::IdentPat
fn ident_path(&self, ident: &str) -> ast::Path
fn impl_(&self, attrs: impl IntoIterator<Item = ast::Attr>, generic_params: Option<ast::GenericParamList>, generic_args: Option<ast::GenericArgList>, path_type: ast::Type, where_clause: Option<ast::WhereClause>, body: Option<ast::AssocItemList>) -> ast::Impl
fn impl_trait(&self, attrs: impl IntoIterator<Item = ast::Attr>, is_unsafe: bool, trait_gen_params: Option<ast::GenericParamList>, trait_gen_args: Option<ast::GenericArgList>, type_gen_params: Option<ast::GenericParamList>, type_gen_args: Option<ast::GenericArgList>, is_negative: bool, path_type: ast::Type, ty: ast::Type, trait_where_clause: Option<ast::WhereClause>, ty_where_clause: Option<ast::WhereClause>, body: Option<ast::AssocItemList>) -> ast::Impl
fn impl_trait_type(&self, bounds: ast::TypeBoundList) -> ast::ImplTraitType
fn item_const(&self, attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, name: ast::Name, ty: ast::Type, expr: ast::Expr) -> ast::Const
fn item_enum(&self, attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, name: ast::Name, generic_param_list: Option<ast::GenericParamList>, where_clause: Option<ast::WhereClause>, variant_list: ast::VariantList) -> ast::Enum
fn item_list(&self, items: impl IntoIterator<Item = ast::Item>) -> ast::ItemList
fn item_static(&self, visibility: Option<ast::Visibility>, is_unsafe: bool, is_mut: bool, name: ast::Name, ty: ast::Type, expr: Option<ast::Expr>) -> ast::Static
fn let_else_stmt(&self, pattern: ast::Pat, ty: Option<ast::Type>, initializer: ast::Expr, diverging: ast::BlockExpr) -> ast::LetStmt
fn let_stmt(&self, pattern: ast::Pat, ty: Option<ast::Type>, initializer: Option<ast::Expr>) -> ast::LetStmt
fn lifetime(&self, text: &str) -> ast::Lifetime
fn lifetime_arg(&self, lifetime: ast::Lifetime) -> ast::LifetimeArg
fn lifetime_param(&self, lifetime: ast::Lifetime) -> ast::LifetimeParam
fn literal_pat(&self, text: &str) -> ast::LiteralPat
fn match_arm(&self, pat: ast::Pat, guard: Option<ast::MatchGuard>, expr: ast::Expr) -> ast::MatchArm
fn match_arm_list(&self, match_arms: impl IntoIterator<Item = ast::MatchArm>) -> ast::MatchArmList
fn match_guard(&self, condition: ast::Expr) -> ast::MatchGuard
fn meta_token_tree(&self, path: ast::Path, tt: ast::TokenTree) -> ast::Meta
fn mod_(&self, name: ast::Name, body: Option<ast::ItemList>) -> ast::Module
fn mut_self_param(&self) -> ast::SelfParam
fn name(&self, name: &str) -> ast::Name
fn name_ref(&self, name: &str) -> ast::NameRef
fn name_ref_self_ty(&self) -> ast::NameRef
fn or_pat(&self, pats: impl IntoIterator<Item = ast::Pat>, leading_pipe: bool) -> ast::OrPat
fn param(&self, pat: ast::Pat, ty: ast::Type) -> ast::Param
fn param_list(&self, self_param: Option<ast::SelfParam>, params: impl IntoIterator<Item = ast::Param>) -> ast::ParamList
fn paren_pat(&self, pat: ast::Pat) -> ast::ParenPat
fn path_concat(&self, first: ast::Path, second: ast::Path) -> ast::Path
fn path_from_idents<'a>(&self, parts: impl IntoIterator<Item = &'a str>) -> Option<ast::Path>
fn path_from_segments(&self, segments: impl IntoIterator<Item = ast::PathSegment>, is_abs: bool) -> ast::Path
fn path_from_text(&self, text: &str) -> ast::Path
fn path_from_text_with_edition(&self, text: &str, edition: Edition) -> ast::Path
fn path_pat(&self, path: ast::Path) -> ast::Pat
fn path_qualified(&self, qual: ast::Path, segment: ast::PathSegment) -> ast::Path
fn path_segment(&self, name_ref: ast::NameRef) -> ast::PathSegment
fn path_segment_crate(&self) -> ast::PathSegment
fn path_segment_generics(&self, name_ref: ast::NameRef, generic_arg_list: ast::GenericArgList) -> ast::PathSegment
fn path_segment_self(&self) -> ast::PathSegment
fn path_segment_super(&self) -> ast::PathSegment
fn path_unqualified(&self, segment: ast::PathSegment) -> ast::Path
fn range_pat(&self, start: Option<ast::Pat>, end: Option<ast::Pat>) -> ast::RangePat
fn record_expr(&self, path: ast::Path, fields: ast::RecordExprFieldList) -> ast::RecordExpr
fn record_expr_field(&self, name: ast::NameRef, expr: Option<ast::Expr>) -> ast::RecordExprField
fn record_expr_field_list(&self, fields: impl IntoIterator<Item = ast::RecordExprField>) -> ast::RecordExprFieldList
fn record_field(&self, visibility: Option<ast::Visibility>, name: ast::Name, ty: ast::Type) -> ast::RecordField
fn record_field_list(&self, fields: impl IntoIterator<Item = ast::RecordField>) -> ast::RecordFieldList
fn record_pat_field(&self, name_ref: ast::NameRef, pat: ast::Pat) -> ast::RecordPatField
fn record_pat_field_list(&self, fields: impl IntoIterator<Item = ast::RecordPatField>, rest_pat: Option<ast::RestPat>) -> ast::RecordPatFieldList
fn record_pat_field_shorthand(&self, pat: ast::Pat) -> ast::RecordPatField
fn record_pat_with_fields(&self, path: ast::Path, fields: ast::RecordPatFieldList) -> ast::RecordPat
fn ref_pat(&self, pat: ast::Pat) -> ast::RefPat
fn rest_pat(&self) -> ast::RestPat
fn ret_type(&self, ty: ast::Type) -> ast::RetType
fn self_param(&self) -> ast::SelfParam
fn simple_ident_pat(&self, name: ast::Name) -> ast::IdentPat
fn slice_pat(&self, pats: impl IntoIterator<Item = ast::Pat>) -> ast::SlicePat
fn struct_(&self, visibility: Option<ast::Visibility>, strukt_name: ast::Name, generic_param_list: Option<ast::GenericParamList>, field_list: ast::FieldList) -> ast::Struct
fn tail_only_block_expr(&self, tail_expr: ast::Expr) -> ast::BlockExpr
fn token(&self, kind: SyntaxKind) -> SyntaxToken
fn token_tree(&self, delimiter: SyntaxKind, tt: impl IntoIterator<Item = NodeOrToken<ast::TokenTree, SyntaxToken>>) -> ast::TokenTree
fn token_tree_from_node(&self, node: &SyntaxNode) -> ast::TokenTree
fn trait_(&self, is_unsafe: bool, ident: &str, generic_param_list: Option<ast::GenericParamList>, where_clause: Option<ast::WhereClause>, assoc_items: ast::AssocItemList) -> ast::Trait
fn tuple_field(&self, visibility: Option<ast::Visibility>, ty: ast::Type) -> ast::TupleField
fn tuple_field_list(&self, fields: impl IntoIterator<Item = ast::TupleField>) -> ast::TupleFieldList
fn tuple_pat(&self, pats: impl IntoIterator<Item = ast::Pat>) -> ast::TuplePat
fn tuple_struct_pat(&self, path: ast::Path, fields: impl IntoIterator<Item = ast::Pat>) -> ast::TupleStructPat
fn ty(&self, text: &str) -> ast::Type
fn ty_alias(&self, attrs: impl IntoIterator<Item = ast::Attr>, ident: &str, generic_param_list: Option<ast::GenericParamList>, type_param_bounds: Option<ast::TypeParam>, where_clause: Option<ast::WhereClause>, assignment: Option<(ast::Type, Option<ast::WhereClause>)>) -> ast::TypeAlias
fn ty_fn_ptr<I: Iterator<Item = Param>>(&self, is_unsafe: bool, abi: Option<ast::Abi>, params: I, ret_type: Option<ast::RetType>) -> ast::FnPtrType
fn ty_infer(&self) -> ast::InferType
fn ty_name(&self, name: ast::Name) -> ast::Type
fn ty_option(&self, t: ast::Type) -> ast::PathType
fn ty_paren(&self, ty: ast::Type) -> ast::Type
fn ty_path(&self, path: ast::Path) -> ast::PathType
fn ty_path_from_segments(&self, segments: impl IntoIterator<Item = ast::PathSegment>, is_abs: bool) -> ast::Type
fn ty_placeholder(&self) -> ast::Type
fn ty_ref(&self, ty: ast::Type, is_mut: bool) -> ast::Type
fn ty_result(&self, t: ast::Type, e: ast::Type) -> ast::PathType
fn ty_tuple(&self, types: impl IntoIterator<Item = ast::Type>) -> ast::Type
fn ty_unit(&self) -> ast::Type
fn type_arg(&self, ty: ast::Type) -> ast::TypeArg
fn type_bound(&self, bound: ast::Type) -> ast::TypeBound
fn type_bound_list(&self, bounds: impl IntoIterator<Item = ast::TypeBound>) -> Option<ast::TypeBoundList>
fn type_bound_text(&self, bound: &str) -> ast::TypeBound
fn type_param(&self, name: ast::Name, bounds: Option<ast::TypeBoundList>) -> ast::TypeParam
fn unnamed_param(&self, ty: ast::Type) -> ast::Param
fn untyped_param(&self, pat: ast::Pat) -> ast::Param
fn use_(&self, attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, use_tree: ast::UseTree) -> ast::Use
fn use_tree(&self, path: ast::Path, use_tree_list: Option<ast::UseTreeList>, alias: Option<ast::Rename>, add_star: bool) -> ast::UseTree
fn use_tree_glob(&self) -> ast::UseTree
fn use_tree_list(&self, use_trees: impl IntoIterator<Item = ast::UseTree>) -> ast::UseTreeList
fn variant(&self, visibility: Option<ast::Visibility>, name: ast::Name, field_list: Option<ast::FieldList>, discriminant: Option<ast::Expr>) -> ast::Variant
fn variant_list(&self, variants: impl IntoIterator<Item = ast::Variant>) -> ast::VariantList
fn visibility_pub(&self) -> ast::Visibility
fn visibility_pub_crate(&self) -> ast::Visibility
fn where_clause(&self, predicates: impl IntoIterator<Item = ast::WherePred>) -> ast::WhereClause
fn where_pred(&self, path: Either<ast::Lifetime, ast::Type>, bounds: impl IntoIterator<Item = ast::TypeBound>) -> ast::WherePred
fn whitespace(&self, text: &str) -> SyntaxToken
fn wildcard_pat(&self) -> ast::WildcardPat
fn without_mappings() -> Self
```

---
