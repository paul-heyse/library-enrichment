# `ra_ap_syntax::ast::make`

Crate `ra_ap_syntax` · 140 public items · structured records in [`model/ra_ap_syntax.ast.make.json`](../model/ra_ap_syntax.ast.make.json)

## arg_list

`function` · `ra_ap_syntax::ast::make::arg_list`

```rust
fn arg_list(args: impl IntoIterator<Item = ast::Expr>) -> ast::ArgList
```

---

## assoc_item_list

`function` · `ra_ap_syntax::ast::make::assoc_item_list`

```rust
fn assoc_item_list(body: Option<Vec<ast::AssocItem>>) -> ast::AssocItemList
```

---

## async_move_block_expr

`function` · `ra_ap_syntax::ast::make::async_move_block_expr`

```rust
fn async_move_block_expr(stmts: impl IntoIterator<Item = ast::Stmt>, tail_expr: Option<ast::Expr>) -> ast::BlockExpr
```

---

## attr_inner

`function` · `ra_ap_syntax::ast::make::attr_inner`

```rust
fn attr_inner(meta: ast::Meta) -> ast::Attr
```

---

## attr_outer

`function` · `ra_ap_syntax::ast::make::attr_outer`

```rust
fn attr_outer(meta: ast::Meta) -> ast::Attr
```

---

## block_expr

`function` · `ra_ap_syntax::ast::make::block_expr`

```rust
fn block_expr(stmts: impl IntoIterator<Item = ast::Stmt>, tail_expr: Option<ast::Expr>) -> ast::BlockExpr
```

---

## box_pat

`function` · `ra_ap_syntax::ast::make::box_pat`

```rust
fn box_pat(pat: ast::Pat) -> ast::BoxPat
```

---

## cfg_attr_meta

`function` · `ra_ap_syntax::ast::make::cfg_attr_meta`

```rust
fn cfg_attr_meta(predicate: ast::CfgPredicate, inner: impl IntoIterator<Item = ast::Meta>) -> ast::CfgAttrMeta
```

---

## cfg_flag

`function` · `ra_ap_syntax::ast::make::cfg_flag`

```rust
fn cfg_flag(flag: &str) -> ast::CfgPredicate
```

---

## const_param

`function` · `ra_ap_syntax::ast::make::const_param`

```rust
fn const_param(name: ast::Name, ty: ast::Type) -> ast::ConstParam
```

---

## deref_pat

`function` · `ra_ap_syntax::ast::make::deref_pat`

```rust
fn deref_pat(pat: ast::Pat) -> ast::Pat
```

---

## enum_

`function` · `ra_ap_syntax::ast::make::enum_`

```rust
fn enum_(attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, enum_name: ast::Name, generic_param_list: Option<ast::GenericParamList>, where_clause: Option<ast::WhereClause>, variant_list: ast::VariantList) -> ast::Enum
```

---

## expr_assignment

`function` · `ra_ap_syntax::ast::make::expr_assignment`

```rust
fn expr_assignment(lhs: ast::Expr, rhs: ast::Expr) -> ast::BinExpr
```

---

## expr_await

`function` · `ra_ap_syntax::ast::make::expr_await`

```rust
fn expr_await(expr: ast::Expr) -> ast::Expr
```

---

## expr_bin_op

`function` · `ra_ap_syntax::ast::make::expr_bin_op`

```rust
fn expr_bin_op(lhs: ast::Expr, op: ast::BinaryOp, rhs: ast::Expr) -> ast::Expr
```

---

## expr_break

`function` · `ra_ap_syntax::ast::make::expr_break`

```rust
fn expr_break(label: Option<ast::Lifetime>, expr: Option<ast::Expr>) -> ast::Expr
```

---

## expr_call

`function` · `ra_ap_syntax::ast::make::expr_call`

```rust
fn expr_call(f: ast::Expr, arg_list: ast::ArgList) -> ast::CallExpr
```

---

## expr_closure

`function` · `ra_ap_syntax::ast::make::expr_closure`

```rust
fn expr_closure(pats: impl IntoIterator<Item = ast::Param>, expr: ast::Expr) -> ast::ClosureExpr
```

---

## expr_const_value

`function` · `ra_ap_syntax::ast::make::expr_const_value`

```rust
fn expr_const_value(text: &str) -> ast::ConstArg
```

---

## expr_continue

`function` · `ra_ap_syntax::ast::make::expr_continue`

```rust
fn expr_continue(label: Option<ast::Lifetime>) -> ast::Expr
```

---

## expr_empty_block

`function` · `ra_ap_syntax::ast::make::expr_empty_block`

```rust
fn expr_empty_block() -> ast::BlockExpr
```

---

## expr_field

`function` · `ra_ap_syntax::ast::make::expr_field`

```rust
fn expr_field(receiver: ast::Expr, field: &str) -> ast::Expr
```

---

## expr_for_loop

`function` · `ra_ap_syntax::ast::make::expr_for_loop`

```rust
fn expr_for_loop(pat: ast::Pat, expr: ast::Expr, block: ast::BlockExpr) -> ast::ForExpr
```

---

## expr_if

`function` · `ra_ap_syntax::ast::make::expr_if`

```rust
fn expr_if(condition: ast::Expr, then_branch: ast::BlockExpr, else_branch: Option<ast::ElseBranch>) -> ast::IfExpr
```

---

## expr_let

`function` · `ra_ap_syntax::ast::make::expr_let`

```rust
fn expr_let(pattern: ast::Pat, expr: ast::Expr) -> ast::LetExpr
```

---

## expr_literal

`function` · `ra_ap_syntax::ast::make::expr_literal`

```rust
fn expr_literal(text: &str) -> ast::Literal
```

---

## expr_loop

`function` · `ra_ap_syntax::ast::make::expr_loop`

```rust
fn expr_loop(block: ast::BlockExpr) -> ast::Expr
```

---

## expr_macro

`function` · `ra_ap_syntax::ast::make::expr_macro`

```rust
fn expr_macro(path: ast::Path, tt: ast::TokenTree) -> ast::MacroExpr
```

---

## expr_match

`function` · `ra_ap_syntax::ast::make::expr_match`

```rust
fn expr_match(expr: ast::Expr, match_arm_list: ast::MatchArmList) -> ast::MatchExpr
```

---

## expr_method_call

`function` · `ra_ap_syntax::ast::make::expr_method_call`

```rust
fn expr_method_call(receiver: ast::Expr, method: ast::NameRef, arg_list: ast::ArgList) -> ast::MethodCallExpr
```

---

## expr_paren

`function` · `ra_ap_syntax::ast::make::expr_paren`

```rust
fn expr_paren(expr: ast::Expr) -> ast::ParenExpr
```

---

## expr_path

`function` · `ra_ap_syntax::ast::make::expr_path`

```rust
fn expr_path(path: ast::Path) -> ast::Expr
```

---

## expr_prefix

`function` · `ra_ap_syntax::ast::make::expr_prefix`

```rust
fn expr_prefix(op: SyntaxKind, expr: ast::Expr) -> ast::PrefixExpr
```

---

## expr_raw_ref

`function` · `ra_ap_syntax::ast::make::expr_raw_ref`

```rust
fn expr_raw_ref(expr: ast::Expr, exclusive: bool) -> ast::Expr
```

---

## expr_reborrow

`function` · `ra_ap_syntax::ast::make::expr_reborrow`

```rust
fn expr_reborrow(expr: ast::Expr) -> ast::Expr
```

---

## expr_ref

`function` · `ra_ap_syntax::ast::make::expr_ref`

```rust
fn expr_ref(expr: ast::Expr, exclusive: bool) -> ast::Expr
```

---

## expr_return

`function` · `ra_ap_syntax::ast::make::expr_return`

```rust
fn expr_return(expr: Option<ast::Expr>) -> ast::Expr
```

---

## expr_stmt

`function` · `ra_ap_syntax::ast::make::expr_stmt`

```rust
fn expr_stmt(expr: ast::Expr) -> ast::ExprStmt
```

---

## expr_try

`function` · `ra_ap_syntax::ast::make::expr_try`

```rust
fn expr_try(expr: ast::Expr) -> ast::Expr
```

---

## expr_tuple

`function` · `ra_ap_syntax::ast::make::expr_tuple`

```rust
fn expr_tuple(elements: impl IntoIterator<Item = ast::Expr>) -> ast::TupleExpr
```

---

## expr_while_loop

`function` · `ra_ap_syntax::ast::make::expr_while_loop`

```rust
fn expr_while_loop(condition: ast::Expr, block: ast::BlockExpr) -> ast::WhileExpr
```

---

## fn_

`function` · `ra_ap_syntax::ast::make::fn_`

```rust
fn fn_(attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, fn_name: ast::Name, type_params: Option<ast::GenericParamList>, where_clause: Option<ast::WhereClause>, params: ast::ParamList, body: ast::BlockExpr, ret_type: Option<ast::RetType>, is_async: bool, is_const: bool, is_unsafe: bool, is_gen: bool) -> ast::Fn
```

---

## generic_arg_list

`function` · `ra_ap_syntax::ast::make::generic_arg_list`

```rust
fn generic_arg_list(args: impl IntoIterator<Item = ast::GenericArg>) -> ast::GenericArgList
```

---

## generic_param_list

`function` · `ra_ap_syntax::ast::make::generic_param_list`

```rust
fn generic_param_list(pats: impl IntoIterator<Item = ast::GenericParam>) -> ast::GenericParamList
```

---

## generic_ty_path_segment

`function` · `ra_ap_syntax::ast::make::generic_ty_path_segment`

```rust
fn generic_ty_path_segment(name_ref: ast::NameRef, generic_args: impl IntoIterator<Item = ast::GenericArg>) -> ast::PathSegment
```

Type and expressions/patterns path differ in whether they require `::` before generic arguments.
Type paths allow them but they are often omitted, while expression/pattern paths require them.

---

## hacky_block_expr

`function` · `ra_ap_syntax::ast::make::hacky_block_expr`

```rust
fn hacky_block_expr(elements: impl IntoIterator<Item = SyntaxElement>, tail_expr: Option<ast::Expr>) -> ast::BlockExpr
```

Ideally this function wouldn't exist since it involves manual indenting.
It differs from `make::block_expr` by also supporting comments and whitespace.

FIXME: replace usages of this with the mutable syntax tree API

---

## ident_pat

`function` · `ra_ap_syntax::ast::make::ident_pat`

```rust
fn ident_pat(ref_: bool, mut_: bool, name: ast::Name) -> ast::IdentPat
```

---

## impl_

`function` · `ra_ap_syntax::ast::make::impl_`

```rust
fn impl_(attrs: impl IntoIterator<Item = ast::Attr>, generic_params: Option<ast::GenericParamList>, generic_args: Option<ast::GenericArgList>, path_type: ast::Type, where_clause: Option<ast::WhereClause>, body: Option<ast::AssocItemList>) -> ast::Impl
```

---

## impl_trait

`function` · `ra_ap_syntax::ast::make::impl_trait`

```rust
fn impl_trait(attrs: impl IntoIterator<Item = ast::Attr>, is_unsafe: bool, trait_gen_params: Option<ast::GenericParamList>, trait_gen_args: Option<ast::GenericArgList>, type_gen_params: Option<ast::GenericParamList>, type_gen_args: Option<ast::GenericArgList>, is_negative: bool, path_type: ast::Type, ty: ast::Type, trait_where_clause: Option<ast::WhereClause>, ty_where_clause: Option<ast::WhereClause>, body: Option<ast::AssocItemList>) -> ast::Impl
```

---

## impl_trait_type

`function` · `ra_ap_syntax::ast::make::impl_trait_type`

```rust
fn impl_trait_type(bounds: ast::TypeBoundList) -> ast::ImplTraitType
```

---

## item_const

`function` · `ra_ap_syntax::ast::make::item_const`

```rust
fn item_const(attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, name: ast::Name, ty: ast::Type, expr: ast::Expr) -> ast::Const
```

---

## item_list

`function` · `ra_ap_syntax::ast::make::item_list`

```rust
fn item_list(body: Option<Vec<ast::Item>>) -> ast::ItemList
```

---

## item_static

`function` · `ra_ap_syntax::ast::make::item_static`

```rust
fn item_static(visibility: Option<ast::Visibility>, is_unsafe: bool, is_mut: bool, name: ast::Name, ty: ast::Type, expr: Option<ast::Expr>) -> ast::Static
```

---

## join_paths

`function` · `ra_ap_syntax::ast::make::join_paths`

```rust
fn join_paths(paths: impl IntoIterator<Item = ast::Path>) -> ast::Path
```

---

## let_else_stmt

`function` · `ra_ap_syntax::ast::make::let_else_stmt`

```rust
fn let_else_stmt(pattern: ast::Pat, ty: Option<ast::Type>, expr: ast::Expr, diverging: ast::BlockExpr) -> ast::LetStmt
```

---

## let_stmt

`function` · `ra_ap_syntax::ast::make::let_stmt`

```rust
fn let_stmt(pattern: ast::Pat, ty: Option<ast::Type>, initializer: Option<ast::Expr>) -> ast::LetStmt
```

---

## lifetime

`function` · `ra_ap_syntax::ast::make::lifetime`

```rust
fn lifetime(text: &str) -> ast::Lifetime
```

---

## lifetime_arg

`function` · `ra_ap_syntax::ast::make::lifetime_arg`

```rust
fn lifetime_arg(lifetime: ast::Lifetime) -> ast::LifetimeArg
```

---

## lifetime_param

`function` · `ra_ap_syntax::ast::make::lifetime_param`

```rust
fn lifetime_param(lifetime: ast::Lifetime) -> ast::LifetimeParam
```

---

## literal_pat

`function` · `ra_ap_syntax::ast::make::literal_pat`

```rust
fn literal_pat(lit: &str) -> ast::LiteralPat
```

---

## match_arm

`function` · `ra_ap_syntax::ast::make::match_arm`

```rust
fn match_arm(pat: ast::Pat, guard: Option<ast::MatchGuard>, expr: ast::Expr) -> ast::MatchArm
```

---

## match_arm_list

`function` · `ra_ap_syntax::ast::make::match_arm_list`

```rust
fn match_arm_list(arms: impl IntoIterator<Item = ast::MatchArm>) -> ast::MatchArmList
```

---

## match_guard

`function` · `ra_ap_syntax::ast::make::match_guard`

```rust
fn match_guard(condition: ast::Expr) -> ast::MatchGuard
```

---

## meta_expr

`function` · `ra_ap_syntax::ast::make::meta_expr`

```rust
fn meta_expr(path: ast::Path, expr: ast::Expr) -> ast::Meta
```

---

## meta_path

`function` · `ra_ap_syntax::ast::make::meta_path`

```rust
fn meta_path(path: ast::Path) -> ast::Meta
```

---

## meta_token_tree

`function` · `ra_ap_syntax::ast::make::meta_token_tree`

```rust
fn meta_token_tree(path: ast::Path, tt: ast::TokenTree) -> ast::Meta
```

---

## mod_

`function` · `ra_ap_syntax::ast::make::mod_`

```rust
fn mod_(name: ast::Name, body: Option<ast::ItemList>) -> ast::Module
```

---

## mut_self_param

`function` · `ra_ap_syntax::ast::make::mut_self_param`

```rust
fn mut_self_param() -> ast::SelfParam
```

---

## name

`function` · `ra_ap_syntax::ast::make::name`

```rust
fn name(name: &str) -> ast::Name
```

---

## name_ref

`function` · `ra_ap_syntax::ast::make::name_ref`

```rust
fn name_ref(name_ref: &str) -> ast::NameRef
```

---

## name_ref_self_ty

`function` · `ra_ap_syntax::ast::make::name_ref_self_ty`

```rust
fn name_ref_self_ty() -> ast::NameRef
```

---

## or_pat

`function` · `ra_ap_syntax::ast::make::or_pat`

```rust
fn or_pat(pats: impl IntoIterator<Item = ast::Pat>, leading_pipe: bool) -> ast::OrPat
```

Returns a `Pat` if the path has just one segment, an `OrPat` otherwise.

Invariant: `pats` must be length > 1.

---

## param

`function` · `ra_ap_syntax::ast::make::param`

```rust
fn param(pat: ast::Pat, ty: ast::Type) -> ast::Param
```

---

## param_list

`function` · `ra_ap_syntax::ast::make::param_list`

```rust
fn param_list(self_param: Option<ast::SelfParam>, pats: impl IntoIterator<Item = ast::Param>) -> ast::ParamList
```

---

## paren_pat

`function` · `ra_ap_syntax::ast::make::paren_pat`

```rust
fn paren_pat(pat: ast::Pat) -> ast::ParenPat
```

---

## path_concat

`function` · `ra_ap_syntax::ast::make::path_concat`

```rust
fn path_concat(first: ast::Path, second: ast::Path) -> ast::Path
```

---

## path_from_segments

`function` · `ra_ap_syntax::ast::make::path_from_segments`

```rust
fn path_from_segments(segments: impl IntoIterator<Item = ast::PathSegment>, is_abs: bool) -> ast::Path
```

---

## path_from_text

`function` · `ra_ap_syntax::ast::make::path_from_text`

```rust
fn path_from_text(text: &str) -> ast::Path
```

---

## path_from_text_with_edition

`function` · `ra_ap_syntax::ast::make::path_from_text_with_edition`

```rust
fn path_from_text_with_edition(text: &str, edition: parser::Edition) -> ast::Path
```

---

## path_pat

`function` · `ra_ap_syntax::ast::make::path_pat`

```rust
fn path_pat(path: ast::Path) -> ast::Pat
```

Returns a `IdentPat` if the path has just one segment, a `PathPat` otherwise.

---

## path_qualified

`function` · `ra_ap_syntax::ast::make::path_qualified`

```rust
fn path_qualified(qual: ast::Path, segment: ast::PathSegment) -> ast::Path
```

---

## path_segment

`function` · `ra_ap_syntax::ast::make::path_segment`

```rust
fn path_segment(name_ref: ast::NameRef) -> ast::PathSegment
```

---

## path_segment_crate

`function` · `ra_ap_syntax::ast::make::path_segment_crate`

```rust
fn path_segment_crate() -> ast::PathSegment
```

---

## path_segment_self

`function` · `ra_ap_syntax::ast::make::path_segment_self`

```rust
fn path_segment_self() -> ast::PathSegment
```

---

## path_segment_super

`function` · `ra_ap_syntax::ast::make::path_segment_super`

```rust
fn path_segment_super() -> ast::PathSegment
```

---

## path_segment_ty

`function` · `ra_ap_syntax::ast::make::path_segment_ty`

```rust
fn path_segment_ty(type_ref: ast::Type, trait_ref: Option<ast::PathType>) -> ast::PathSegment
```

---

## path_unqualified

`function` · `ra_ap_syntax::ast::make::path_unqualified`

```rust
fn path_unqualified(segment: ast::PathSegment) -> ast::Path
```

---

## range_pat

`function` · `ra_ap_syntax::ast::make::range_pat`

```rust
fn range_pat(start: Option<ast::Pat>, end: Option<ast::Pat>) -> ast::RangePat
```

---

## record_expr

`function` · `ra_ap_syntax::ast::make::record_expr`

```rust
fn record_expr(path: ast::Path, fields: ast::RecordExprFieldList) -> ast::RecordExpr
```

---

## record_expr_field

`function` · `ra_ap_syntax::ast::make::record_expr_field`

```rust
fn record_expr_field(name: ast::NameRef, expr: Option<ast::Expr>) -> ast::RecordExprField
```

---

## record_expr_field_list

`function` · `ra_ap_syntax::ast::make::record_expr_field_list`

```rust
fn record_expr_field_list(fields: impl IntoIterator<Item = ast::RecordExprField>) -> ast::RecordExprFieldList
```

---

## record_field

`function` · `ra_ap_syntax::ast::make::record_field`

```rust
fn record_field(visibility: Option<ast::Visibility>, name: ast::Name, ty: ast::Type) -> ast::RecordField
```

---

## record_field_list

`function` · `ra_ap_syntax::ast::make::record_field_list`

```rust
fn record_field_list(fields: impl IntoIterator<Item = ast::RecordField>) -> ast::RecordFieldList
```

---

## record_pat

`function` · `ra_ap_syntax::ast::make::record_pat`

```rust
fn record_pat(path: ast::Path, pats: impl IntoIterator<Item = ast::Pat>) -> ast::RecordPat
```

---

## record_pat_field

`function` · `ra_ap_syntax::ast::make::record_pat_field`

```rust
fn record_pat_field(name_ref: ast::NameRef, pat: ast::Pat) -> ast::RecordPatField
```

---

## record_pat_field_list

`function` · `ra_ap_syntax::ast::make::record_pat_field_list`

```rust
fn record_pat_field_list(fields: impl IntoIterator<Item = ast::RecordPatField>, rest_pat: Option<ast::RestPat>) -> ast::RecordPatFieldList
```

---

## record_pat_field_shorthand

`function` · `ra_ap_syntax::ast::make::record_pat_field_shorthand`

```rust
fn record_pat_field_shorthand(pat: ast::Pat) -> ast::RecordPatField
```

---

## record_pat_with_fields

`function` · `ra_ap_syntax::ast::make::record_pat_with_fields`

```rust
fn record_pat_with_fields(path: ast::Path, fields: ast::RecordPatFieldList) -> ast::RecordPat
```

---

## ref_pat

`function` · `ra_ap_syntax::ast::make::ref_pat`

```rust
fn ref_pat(pat: ast::Pat) -> ast::RefPat
```

---

## rest_pat

`function` · `ra_ap_syntax::ast::make::rest_pat`

```rust
fn rest_pat() -> ast::RestPat
```

---

## ret_type

`function` · `ra_ap_syntax::ast::make::ret_type`

```rust
fn ret_type(ty: ast::Type) -> ast::RetType
```

---

## self_param

`function` · `ra_ap_syntax::ast::make::self_param`

```rust
fn self_param() -> ast::SelfParam
```

---

## slice_pat

`function` · `ra_ap_syntax::ast::make::slice_pat`

```rust
fn slice_pat(pats: impl IntoIterator<Item = ast::Pat>) -> ast::SlicePat
```

---

## struct_

`function` · `ra_ap_syntax::ast::make::struct_`

```rust
fn struct_(visibility: Option<ast::Visibility>, strukt_name: ast::Name, generic_param_list: Option<ast::GenericParamList>, field_list: ast::FieldList) -> ast::Struct
```

---

## tail_only_block_expr

`function` · `ra_ap_syntax::ast::make::tail_only_block_expr`

```rust
fn tail_only_block_expr(tail_expr: ast::Expr) -> ast::BlockExpr
```

---

## token

`function` · `ra_ap_syntax::ast::make::token`

```rust
fn token(kind: SyntaxKind) -> SyntaxToken
```

---

## token_tree

`function` · `ra_ap_syntax::ast::make::token_tree`

```rust
fn token_tree(delimiter: SyntaxKind, tt: impl IntoIterator<Item = rowan::NodeOrToken<ast::TokenTree, SyntaxToken>>) -> ast::TokenTree
```

---

## trait_

`function` · `ra_ap_syntax::ast::make::trait_`

```rust
fn trait_(is_unsafe: bool, ident: &str, gen_params: Option<ast::GenericParamList>, where_clause: Option<ast::WhereClause>, assoc_items: ast::AssocItemList) -> ast::Trait
```

---

## tuple_field

`function` · `ra_ap_syntax::ast::make::tuple_field`

```rust
fn tuple_field(visibility: Option<ast::Visibility>, ty: ast::Type) -> ast::TupleField
```

---

## tuple_field_list

`function` · `ra_ap_syntax::ast::make::tuple_field_list`

```rust
fn tuple_field_list(fields: impl IntoIterator<Item = ast::TupleField>) -> ast::TupleFieldList
```

---

## tuple_pat

`function` · `ra_ap_syntax::ast::make::tuple_pat`

```rust
fn tuple_pat(pats: impl IntoIterator<Item = ast::Pat>) -> ast::TuplePat
```

Creates a tuple of patterns from an iterator of patterns.

Invariant: `pats` must be length > 0

---

## tuple_struct_pat

`function` · `ra_ap_syntax::ast::make::tuple_struct_pat`

```rust
fn tuple_struct_pat(path: ast::Path, pats: impl IntoIterator<Item = ast::Pat>) -> ast::TupleStructPat
```

---

## turbofish_generic_arg_list

`function` · `ra_ap_syntax::ast::make::turbofish_generic_arg_list`

```rust
fn turbofish_generic_arg_list(args: impl IntoIterator<Item = ast::GenericArg>) -> ast::GenericArgList
```

---

## ty

`function` · `ra_ap_syntax::ast::make::ty`

```rust
fn ty(text: &str) -> ast::Type
```

---

## ty_alias

`function` · `ra_ap_syntax::ast::make::ty_alias`

```rust
fn ty_alias(attrs: impl IntoIterator<Item = ast::Attr>, ident: &str, generic_param_list: Option<ast::GenericParamList>, type_param_bounds: Option<ast::TypeParam>, where_clause: Option<ast::WhereClause>, assignment: Option<(ast::Type, Option<ast::WhereClause>)>) -> ast::TypeAlias
```

---

## ty_fn_ptr

`function` · `ra_ap_syntax::ast::make::ty_fn_ptr`

```rust
fn ty_fn_ptr<I: Iterator<Item = ast::Param>>(is_unsafe: bool, abi: Option<ast::Abi>, params: I, ret_type: Option<ast::RetType>) -> ast::FnPtrType
```

---

## ty_paren

`function` · `ra_ap_syntax::ast::make::ty_paren`

```rust
fn ty_paren(ty: ast::Type) -> ast::Type
```

---

## ty_path

`function` · `ra_ap_syntax::ast::make::ty_path`

```rust
fn ty_path(path: ast::Path) -> ast::Type
```

---

## ty_placeholder

`function` · `ra_ap_syntax::ast::make::ty_placeholder`

```rust
fn ty_placeholder() -> ast::Type
```

---

## ty_ref

`function` · `ra_ap_syntax::ast::make::ty_ref`

```rust
fn ty_ref(target: ast::Type, exclusive: bool) -> ast::Type
```

---

## ty_tuple

`function` · `ra_ap_syntax::ast::make::ty_tuple`

```rust
fn ty_tuple(types: impl IntoIterator<Item = ast::Type>) -> ast::Type
```

---

## ty_unit

`function` · `ra_ap_syntax::ast::make::ty_unit`

```rust
fn ty_unit() -> ast::Type
```

---

## type_arg

`function` · `ra_ap_syntax::ast::make::type_arg`

```rust
fn type_arg(ty: ast::Type) -> ast::TypeArg
```

---

## type_bound

`function` · `ra_ap_syntax::ast::make::type_bound`

```rust
fn type_bound(bound: ast::Type) -> ast::TypeBound
```

---

## type_bound_list

`function` · `ra_ap_syntax::ast::make::type_bound_list`

```rust
fn type_bound_list(bounds: impl IntoIterator<Item = ast::TypeBound>) -> Option<ast::TypeBoundList>
```

---

## type_bound_text

`function` · `ra_ap_syntax::ast::make::type_bound_text`

```rust
fn type_bound_text(bound: &str) -> ast::TypeBound
```

---

## type_param

`function` · `ra_ap_syntax::ast::make::type_param`

```rust
fn type_param(name: ast::Name, bounds: Option<ast::TypeBoundList>) -> ast::TypeParam
```

---

## unnamed_param

`function` · `ra_ap_syntax::ast::make::unnamed_param`

```rust
fn unnamed_param(ty: ast::Type) -> ast::Param
```

---

## untyped_param

`function` · `ra_ap_syntax::ast::make::untyped_param`

```rust
fn untyped_param(pat: ast::Pat) -> ast::Param
```

---

## use_

`function` · `ra_ap_syntax::ast::make::use_`

```rust
fn use_(attrs: impl IntoIterator<Item = ast::Attr>, visibility: Option<ast::Visibility>, use_tree: ast::UseTree) -> ast::Use
```

---

## use_tree

`function` · `ra_ap_syntax::ast::make::use_tree`

```rust
fn use_tree(path: ast::Path, use_tree_list: Option<ast::UseTreeList>, alias: Option<ast::Rename>, add_star: bool) -> ast::UseTree
```

---

## use_tree_glob

`function` · `ra_ap_syntax::ast::make::use_tree_glob`

```rust
fn use_tree_glob() -> ast::UseTree
```

---

## use_tree_list

`function` · `ra_ap_syntax::ast::make::use_tree_list`

```rust
fn use_tree_list(use_trees: impl IntoIterator<Item = ast::UseTree>) -> ast::UseTreeList
```

---

## variant

`function` · `ra_ap_syntax::ast::make::variant`

```rust
fn variant(visibility: Option<ast::Visibility>, name: ast::Name, field_list: Option<ast::FieldList>, discriminant: Option<ast::Expr>) -> ast::Variant
```

---

## variant_list

`function` · `ra_ap_syntax::ast::make::variant_list`

```rust
fn variant_list(variants: impl IntoIterator<Item = ast::Variant>) -> ast::VariantList
```

---

## visibility_pub

`function` · `ra_ap_syntax::ast::make::visibility_pub`

```rust
fn visibility_pub() -> ast::Visibility
```

---

## visibility_pub_crate

`function` · `ra_ap_syntax::ast::make::visibility_pub_crate`

```rust
fn visibility_pub_crate() -> ast::Visibility
```

---

## where_clause

`function` · `ra_ap_syntax::ast::make::where_clause`

```rust
fn where_clause(preds: impl IntoIterator<Item = ast::WherePred>) -> ast::WhereClause
```

---

## where_pred

`function` · `ra_ap_syntax::ast::make::where_pred`

```rust
fn where_pred(path: either::Either<ast::Lifetime, ast::Type>, bounds: impl IntoIterator<Item = ast::TypeBound>) -> ast::WherePred
```

---

## wildcard_pat

`function` · `ra_ap_syntax::ast::make::wildcard_pat`

```rust
fn wildcard_pat() -> ast::WildcardPat
```

---
