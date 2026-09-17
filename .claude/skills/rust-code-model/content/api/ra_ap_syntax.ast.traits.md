# `ra_ap_syntax::ast::traits`

Crate `ra_ap_syntax` · 12 public items · structured records in [`model/ra_ap_syntax.ast.traits.json`](../model/ra_ap_syntax.ast.traits.json)

## attrs_including_inner

`function` · `ra_ap_syntax::ast::traits::attrs_including_inner`

Also reachable as `ra_ap_syntax::ast::attrs_including_inner`

```rust
fn attrs_including_inner(owner: &dyn HasAttrs) -> impl Iterator<Item = ast::Attr> + Clone
```

---

## attrs_with_doc_including_inner

`function` · `ra_ap_syntax::ast::traits::attrs_with_doc_including_inner`

Also reachable as `ra_ap_syntax::ast::attrs_with_doc_including_inner`

```rust
fn attrs_with_doc_including_inner(owner: &dyn HasAttrs) -> impl Iterator<Item = ast::AnyAttr> + Clone
```

Returns all attributes of this node, including inner attributes that may not be directly under this node
but under a child.

---

## AttrsIter

`struct` · `ra_ap_syntax::ast::traits::AttrsIter`

Also reachable as `ra_ap_syntax::ast::AttrsIter`

```rust
struct AttrsIter<I>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone

**Methods** (1)

```rust
fn new(inner: I) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

---

## HasArgList

`trait` · `ra_ap_syntax::ast::traits::HasArgList`

Also reachable as `ra_ap_syntax::ast::HasArgList`

```rust
trait HasArgList: AstNode
```

**Implementors** (4)

- `ra_ap_syntax::ast::expr_ext::CallableExpr`
- `ra_ap_syntax::ast::generated::nodes::AnyHasArgList`
- `ra_ap_syntax::ast::generated::nodes::CallExpr`
- `ra_ap_syntax::ast::generated::nodes::MethodCallExpr`

**Methods** (1)

```rust
fn arg_list(&self) -> Option<ast::ArgList>
```

---

## HasAttrs

`trait` · `ra_ap_syntax::ast::traits::HasAttrs`

Also reachable as `ra_ap_syntax::ast::HasAttrs`

```rust
trait HasAttrs: AstNode
```

**Implementors** (87)

- `either::Either`
- `ra_ap_syntax::ast::expr_ext::CallableExpr`
- `ra_ap_syntax::ast::generated::nodes::Adt`
- `ra_ap_syntax::ast::generated::nodes::AnyHasAttrs`
- `ra_ap_syntax::ast::generated::nodes::ArrayExpr`
- `ra_ap_syntax::ast::generated::nodes::AsmClobberAbi`
- `ra_ap_syntax::ast::generated::nodes::AsmExpr`
- `ra_ap_syntax::ast::generated::nodes::AsmOperandNamed`
- `ra_ap_syntax::ast::generated::nodes::AsmOptions`
- `ra_ap_syntax::ast::generated::nodes::AsmPiece`
- `ra_ap_syntax::ast::generated::nodes::AssocItem`
- `ra_ap_syntax::ast::generated::nodes::AssocItemList`
- `ra_ap_syntax::ast::generated::nodes::AwaitExpr`
- `ra_ap_syntax::ast::generated::nodes::BecomeExpr`
- `ra_ap_syntax::ast::generated::nodes::BinExpr`
- `ra_ap_syntax::ast::generated::nodes::BlockExpr`
- `ra_ap_syntax::ast::generated::nodes::BreakExpr`
- `ra_ap_syntax::ast::generated::nodes::CallExpr`
- `ra_ap_syntax::ast::generated::nodes::CastExpr`
- `ra_ap_syntax::ast::generated::nodes::ClosureExpr`
- `ra_ap_syntax::ast::generated::nodes::Const`
- `ra_ap_syntax::ast::generated::nodes::ConstParam`
- `ra_ap_syntax::ast::generated::nodes::ContinueExpr`
- `ra_ap_syntax::ast::generated::nodes::Enum`
- `ra_ap_syntax::ast::generated::nodes::Expr`
- `ra_ap_syntax::ast::generated::nodes::ExternBlock`
- `ra_ap_syntax::ast::generated::nodes::ExternCrate`
- `ra_ap_syntax::ast::generated::nodes::ExternItem`
- `ra_ap_syntax::ast::generated::nodes::ExternItemList`
- `ra_ap_syntax::ast::generated::nodes::FieldExpr`
- `ra_ap_syntax::ast::generated::nodes::Fn`
- `ra_ap_syntax::ast::generated::nodes::ForExpr`
- `ra_ap_syntax::ast::generated::nodes::FormatArgsExpr`
- `ra_ap_syntax::ast::generated::nodes::GenericParam`
- `ra_ap_syntax::ast::generated::nodes::IdentPat`
- `ra_ap_syntax::ast::generated::nodes::IfExpr`
- `ra_ap_syntax::ast::generated::nodes::Impl`
- `ra_ap_syntax::ast::generated::nodes::IndexExpr`
- `ra_ap_syntax::ast::generated::nodes::Item`
- `ra_ap_syntax::ast::generated::nodes::ItemList`
- `ra_ap_syntax::ast::generated::nodes::LetExpr`
- `ra_ap_syntax::ast::generated::nodes::LetStmt`
- `ra_ap_syntax::ast::generated::nodes::LifetimeParam`
- `ra_ap_syntax::ast::generated::nodes::Literal`
- `ra_ap_syntax::ast::generated::nodes::LoopExpr`
- `ra_ap_syntax::ast::generated::nodes::MacroCall`
- `ra_ap_syntax::ast::generated::nodes::MacroDef`
- `ra_ap_syntax::ast::generated::nodes::MacroRules`
- `ra_ap_syntax::ast::generated::nodes::MatchArm`
- `ra_ap_syntax::ast::generated::nodes::MatchArmList`
- `ra_ap_syntax::ast::generated::nodes::MatchExpr`
- `ra_ap_syntax::ast::generated::nodes::MethodCallExpr`
- `ra_ap_syntax::ast::generated::nodes::Module`
- `ra_ap_syntax::ast::generated::nodes::OffsetOfExpr`
- `ra_ap_syntax::ast::generated::nodes::Param`
- `ra_ap_syntax::ast::generated::nodes::ParenExpr`
- `ra_ap_syntax::ast::generated::nodes::PathExpr`
- `ra_ap_syntax::ast::generated::nodes::PrefixExpr`
- `ra_ap_syntax::ast::generated::nodes::RangeExpr`
- `ra_ap_syntax::ast::generated::nodes::RecordExprField`
- `ra_ap_syntax::ast::generated::nodes::RecordExprFieldList`
- `ra_ap_syntax::ast::generated::nodes::RecordField`
- `ra_ap_syntax::ast::generated::nodes::RecordPatField`
- `ra_ap_syntax::ast::generated::nodes::RefExpr`
- `ra_ap_syntax::ast::generated::nodes::RestPat`
- `ra_ap_syntax::ast::generated::nodes::ReturnExpr`
- `ra_ap_syntax::ast::generated::nodes::SelfParam`
- `ra_ap_syntax::ast::generated::nodes::SourceFile`
- `ra_ap_syntax::ast::generated::nodes::Static`
- `ra_ap_syntax::ast::generated::nodes::StmtList`
- `ra_ap_syntax::ast::generated::nodes::Struct`
- `ra_ap_syntax::ast::generated::nodes::Trait`
- `ra_ap_syntax::ast::generated::nodes::TryExpr`
- `ra_ap_syntax::ast::generated::nodes::TupleExpr`
- `ra_ap_syntax::ast::generated::nodes::TupleField`
- `ra_ap_syntax::ast::generated::nodes::TypeAlias`
- `ra_ap_syntax::ast::generated::nodes::TypeParam`
- `ra_ap_syntax::ast::generated::nodes::UnderscoreExpr`
- `ra_ap_syntax::ast::generated::nodes::Union`
- `ra_ap_syntax::ast::generated::nodes::Use`
- `ra_ap_syntax::ast::generated::nodes::Variant`
- `ra_ap_syntax::ast::generated::nodes::VariantDef`
- `ra_ap_syntax::ast::generated::nodes::WhileExpr`
- `ra_ap_syntax::ast::generated::nodes::YeetExpr`
- `ra_ap_syntax::ast::generated::nodes::YieldExpr`
- `ra_ap_syntax::ast::node_ext::Macro`
- `ra_ap_syntax::ast::node_ext::TypeOrConstParam`

**Methods** (4)

```rust
fn attrs(&self) -> AstChildren<ast::Attr>
fn attrs_with_doc(&self) -> AstChildren<ast::AnyAttr>
fn doc_comments(&self) -> AstChildren<ast::DocComment>
fn inner_attributes_node(&self) -> Option<SyntaxNode>
```

---

## HasGenericArgs

`trait` · `ra_ap_syntax::ast::traits::HasGenericArgs`

Also reachable as `ra_ap_syntax::ast::HasGenericArgs`

```rust
trait HasGenericArgs: AstNode
```

**Implementors** (4)

- `ra_ap_syntax::ast::generated::nodes::AnyHasGenericArgs`
- `ra_ap_syntax::ast::generated::nodes::AssocTypeArg`
- `ra_ap_syntax::ast::generated::nodes::MethodCallExpr`
- `ra_ap_syntax::ast::generated::nodes::PathSegment`

**Methods** (1)

```rust
fn generic_arg_list(&self) -> Option<ast::GenericArgList>
```

---

## HasGenericParams

`trait` · `ra_ap_syntax::ast::traits::HasGenericParams`

Also reachable as `ra_ap_syntax::ast::HasGenericParams`

```rust
trait HasGenericParams: AstNode
```

**Implementors** (10)

- `ra_ap_syntax::ast::generated::nodes::Adt`
- `ra_ap_syntax::ast::generated::nodes::AnyHasGenericParams`
- `ra_ap_syntax::ast::generated::nodes::Const`
- `ra_ap_syntax::ast::generated::nodes::Enum`
- `ra_ap_syntax::ast::generated::nodes::Fn`
- `ra_ap_syntax::ast::generated::nodes::Impl`
- `ra_ap_syntax::ast::generated::nodes::Struct`
- `ra_ap_syntax::ast::generated::nodes::Trait`
- `ra_ap_syntax::ast::generated::nodes::TypeAlias`
- `ra_ap_syntax::ast::generated::nodes::Union`

**Methods** (2)

```rust
fn generic_param_list(&self) -> Option<ast::GenericParamList>
fn where_clause(&self) -> Option<ast::WhereClause>
```

---

## HasLoopBody

`trait` · `ra_ap_syntax::ast::traits::HasLoopBody`

Also reachable as `ra_ap_syntax::ast::HasLoopBody`

```rust
trait HasLoopBody: AstNode
```

**Implementors** (4)

- `ra_ap_syntax::ast::generated::nodes::AnyHasLoopBody`
- `ra_ap_syntax::ast::generated::nodes::ForExpr`
- `ra_ap_syntax::ast::generated::nodes::LoopExpr`
- `ra_ap_syntax::ast::generated::nodes::WhileExpr`

**Methods** (2)

```rust
fn label(&self) -> Option<ast::Label>
fn loop_body(&self) -> Option<ast::BlockExpr>
```

---

## HasModuleItem

`trait` · `ra_ap_syntax::ast::traits::HasModuleItem`

Also reachable as `ra_ap_syntax::ast::HasModuleItem`

```rust
trait HasModuleItem: AstNode
```

**Implementors** (5)

- `ra_ap_syntax::ast::generated::nodes::AnyHasModuleItem`
- `ra_ap_syntax::ast::generated::nodes::ItemList`
- `ra_ap_syntax::ast::generated::nodes::MacroItems`
- `ra_ap_syntax::ast::generated::nodes::SourceFile`
- `ra_ap_syntax::ast::generated::nodes::StmtList`

**Methods** (1)

```rust
fn items(&self) -> AstChildren<ast::Item>
```

---

## HasName

`trait` · `ra_ap_syntax::ast::traits::HasName`

Also reachable as `ra_ap_syntax::ast::HasName`

```rust
trait HasName: AstNode
```

**Implementors** (25)

- `either::Either`
- `ra_ap_syntax::ast::generated::nodes::Adt`
- `ra_ap_syntax::ast::generated::nodes::AnyHasName`
- `ra_ap_syntax::ast::generated::nodes::AsmOperandNamed`
- `ra_ap_syntax::ast::generated::nodes::Const`
- `ra_ap_syntax::ast::generated::nodes::ConstParam`
- `ra_ap_syntax::ast::generated::nodes::Enum`
- `ra_ap_syntax::ast::generated::nodes::Fn`
- `ra_ap_syntax::ast::generated::nodes::FormatArgsArg`
- `ra_ap_syntax::ast::generated::nodes::IdentPat`
- `ra_ap_syntax::ast::generated::nodes::MacroDef`
- `ra_ap_syntax::ast::generated::nodes::MacroRules`
- `ra_ap_syntax::ast::generated::nodes::Module`
- `ra_ap_syntax::ast::generated::nodes::RecordField`
- `ra_ap_syntax::ast::generated::nodes::Rename`
- `ra_ap_syntax::ast::generated::nodes::SelfParam`
- `ra_ap_syntax::ast::generated::nodes::Static`
- `ra_ap_syntax::ast::generated::nodes::Struct`
- `ra_ap_syntax::ast::generated::nodes::Trait`
- `ra_ap_syntax::ast::generated::nodes::TypeAlias`
- `ra_ap_syntax::ast::generated::nodes::TypeParam`
- `ra_ap_syntax::ast::generated::nodes::Union`
- `ra_ap_syntax::ast::generated::nodes::Variant`
- `ra_ap_syntax::ast::generated::nodes::VariantDef`
- `ra_ap_syntax::ast::node_ext::Macro`

**Methods** (1)

```rust
fn name(&self) -> Option<ast::Name>
```

---

## HasTypeBounds

`trait` · `ra_ap_syntax::ast::traits::HasTypeBounds`

Also reachable as `ra_ap_syntax::ast::HasTypeBounds`

```rust
trait HasTypeBounds: AstNode
```

**Implementors** (7)

- `ra_ap_syntax::ast::generated::nodes::AnyHasTypeBounds`
- `ra_ap_syntax::ast::generated::nodes::AssocTypeArg`
- `ra_ap_syntax::ast::generated::nodes::LifetimeParam`
- `ra_ap_syntax::ast::generated::nodes::Trait`
- `ra_ap_syntax::ast::generated::nodes::TypeAlias`
- `ra_ap_syntax::ast::generated::nodes::TypeParam`
- `ra_ap_syntax::ast::generated::nodes::WherePred`

**Methods** (2)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn type_bound_list(&self) -> Option<ast::TypeBoundList>
```

---

## HasVisibility

`trait` · `ra_ap_syntax::ast::traits::HasVisibility`

Also reachable as `ra_ap_syntax::ast::HasVisibility`

```rust
trait HasVisibility: AstNode
```

**Implementors** (20)

- `ra_ap_syntax::ast::generated::nodes::Adt`
- `ra_ap_syntax::ast::generated::nodes::AnyHasVisibility`
- `ra_ap_syntax::ast::generated::nodes::Const`
- `ra_ap_syntax::ast::generated::nodes::Enum`
- `ra_ap_syntax::ast::generated::nodes::ExternCrate`
- `ra_ap_syntax::ast::generated::nodes::Fn`
- `ra_ap_syntax::ast::generated::nodes::Impl`
- `ra_ap_syntax::ast::generated::nodes::MacroDef`
- `ra_ap_syntax::ast::generated::nodes::MacroRules`
- `ra_ap_syntax::ast::generated::nodes::Module`
- `ra_ap_syntax::ast::generated::nodes::RecordField`
- `ra_ap_syntax::ast::generated::nodes::Static`
- `ra_ap_syntax::ast::generated::nodes::Struct`
- `ra_ap_syntax::ast::generated::nodes::Trait`
- `ra_ap_syntax::ast::generated::nodes::TupleField`
- `ra_ap_syntax::ast::generated::nodes::TypeAlias`
- `ra_ap_syntax::ast::generated::nodes::Union`
- `ra_ap_syntax::ast::generated::nodes::Use`
- `ra_ap_syntax::ast::generated::nodes::Variant`
- `ra_ap_syntax::ast::generated::nodes::VariantDef`

**Methods** (1)

```rust
fn visibility(&self) -> Option<ast::Visibility>
```

---
