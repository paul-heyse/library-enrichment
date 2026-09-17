# `ra_ap_syntax::ast::generated::nodes`

Crate `ra_ap_syntax` · 197 public items · structured records in [`model/ra_ap_syntax.ast.generated.nodes.json`](../model/ra_ap_syntax.ast.generated.nodes.json)

## Adt

`enum` · `ra_ap_syntax::ast::generated::nodes::Adt`

Also reachable as `ra_ap_syntax::ast::Adt`

```rust
enum Adt
```

**Variants**: `Enum`, `Struct`, `Union`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: Struct) -> Adt
fn from(node: Enum) -> Adt
fn from(node: Union) -> Adt
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyAttr

`enum` · `ra_ap_syntax::ast::generated::nodes::AnyAttr`

Also reachable as `ra_ap_syntax::ast::AnyAttr`

```rust
enum AnyAttr
```

**Variants**: `Attr`, `DocComment`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn kind(&self) -> AttrKind
```

**via `core::convert::From`**

```rust
fn from(node: DocComment) -> AnyAttr
fn from(node: Attr) -> AnyAttr
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AsmOperand

`enum` · `ra_ap_syntax::ast::generated::nodes::AsmOperand`

Also reachable as `ra_ap_syntax::ast::AsmOperand`

```rust
enum AsmOperand
```

**Variants**: `AsmConst`, `AsmLabel`, `AsmRegOperand`, `AsmSym`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: AsmRegOperand) -> AsmOperand
fn from(node: AsmLabel) -> AsmOperand
fn from(node: AsmConst) -> AsmOperand
fn from(node: AsmSym) -> AsmOperand
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AsmPiece

`enum` · `ra_ap_syntax::ast::generated::nodes::AsmPiece`

Also reachable as `ra_ap_syntax::ast::AsmPiece`

```rust
enum AsmPiece
```

**Variants**: `AsmClobberAbi`, `AsmOperandNamed`, `AsmOptions`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: AsmClobberAbi) -> AsmPiece
fn from(node: AsmOperandNamed) -> AsmPiece
fn from(node: AsmOptions) -> AsmPiece
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AssocItem

`enum` · `ra_ap_syntax::ast::generated::nodes::AssocItem`

Also reachable as `ra_ap_syntax::ast::AssocItem`

```rust
enum AssocItem
```

**Variants**: `Const`, `Fn`, `MacroCall`, `TypeAlias`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: MacroCall) -> AssocItem
fn from(node: TypeAlias) -> AssocItem
fn from(node: Const) -> AssocItem
fn from(node: Fn) -> AssocItem
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## CfgPredicate

`enum` · `ra_ap_syntax::ast::generated::nodes::CfgPredicate`

Also reachable as `ra_ap_syntax::ast::CfgPredicate`

```rust
enum CfgPredicate
```

**Variants**: `CfgAtom`, `CfgComposite`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: CfgAtom) -> CfgPredicate
fn from(node: CfgComposite) -> CfgPredicate
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## Expr

`enum` · `ra_ap_syntax::ast::generated::nodes::Expr`

Also reachable as `ra_ap_syntax::ast::Expr`

```rust
enum Expr
```

**Variants**: `ArrayExpr`, `AsmExpr`, `AwaitExpr`, `BecomeExpr`, `BinExpr`, `BlockExpr`, `BreakExpr`, `CallExpr`, `CastExpr`, `ClosureExpr`, `ContinueExpr`, `FieldExpr`, `ForExpr`, `FormatArgsExpr`, `IfExpr`, `IncludeBytesExpr`, `IndexExpr`, `LetExpr`, `Literal`, `LoopExpr`, `MacroExpr`, `MatchExpr`, `MethodCallExpr`, `OffsetOfExpr`, `ParenExpr`, `PathExpr`, `PrefixExpr`, `RangeExpr`, `RecordExpr`, `RefExpr`, `ReturnExpr`, `TryExpr`, `TupleExpr`, `UnderscoreExpr`, `WhileExpr`, `YeetExpr`, `YieldExpr`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn is_block_like(&self) -> bool
fn is_ret_like_with_no_value(&self) -> bool
fn needs_parens_in(&self, parent: &SyntaxNode) -> bool
fn needs_parens_in_place_of(&self, parent: &SyntaxNode, place_of: &SyntaxNode) -> bool
fn parse(text: &str, edition: Edition) -> Parse<ast::Expr>
fn precedence(&self) -> ExprPrecedence
```

**via `core::convert::From`**

```rust
fn from(node: TupleExpr) -> Expr
fn from(node: PathExpr) -> Expr
fn from(node: IncludeBytesExpr) -> Expr
fn from(node: ContinueExpr) -> Expr
fn from(node: ArrayExpr) -> Expr
fn from(node: RecordExpr) -> Expr
fn from(node: IfExpr) -> Expr
fn from(node: FormatArgsExpr) -> Expr
fn from(node: BecomeExpr) -> Expr
fn from(node: TryExpr) -> Expr
fn from(node: YeetExpr) -> Expr
fn from(node: PrefixExpr) -> Expr
fn from(node: LetExpr) -> Expr
fn from(node: WhileExpr) -> Expr
fn from(node: ReturnExpr) -> Expr
fn from(node: BinExpr) -> Expr
fn from(node: UnderscoreExpr) -> Expr
fn from(node: MatchExpr) -> Expr
fn from(node: MacroExpr) -> Expr
fn from(node: ParenExpr) -> Expr
fn from(node: LoopExpr) -> Expr
fn from(node: CastExpr) -> Expr
fn from(node: AsmExpr) -> Expr
fn from(node: BreakExpr) -> Expr
fn from(node: OffsetOfExpr) -> Expr
fn from(node: CallExpr) -> Expr
fn from(node: YieldExpr) -> Expr
fn from(node: RefExpr) -> Expr
fn from(node: AwaitExpr) -> Expr
fn from(node: ForExpr) -> Expr
fn from(node: RangeExpr) -> Expr
fn from(node: IndexExpr) -> Expr
fn from(node: FieldExpr) -> Expr
fn from(node: ClosureExpr) -> Expr
fn from(node: MethodCallExpr) -> Expr
fn from(node: Literal) -> Expr
fn from(node: BlockExpr) -> Expr
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## ExternItem

`enum` · `ra_ap_syntax::ast::generated::nodes::ExternItem`

Also reachable as `ra_ap_syntax::ast::ExternItem`

```rust
enum ExternItem
```

**Variants**: `Fn`, `MacroCall`, `Static`, `TypeAlias`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: MacroCall) -> ExternItem
fn from(node: TypeAlias) -> ExternItem
fn from(node: Fn) -> ExternItem
fn from(node: Static) -> ExternItem
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## FieldList

`enum` · `ra_ap_syntax::ast::generated::nodes::FieldList`

Also reachable as `ra_ap_syntax::ast::FieldList`

```rust
enum FieldList
```

**Variants**: `RecordFieldList`, `TupleFieldList`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: RecordFieldList) -> FieldList
fn from(node: TupleFieldList) -> FieldList
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## GenericArg

`enum` · `ra_ap_syntax::ast::generated::nodes::GenericArg`

Also reachable as `ra_ap_syntax::ast::GenericArg`

```rust
enum GenericArg
```

**Variants**: `AssocTypeArg`, `ConstArg`, `LifetimeArg`, `TypeArg`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: ConstArg) -> GenericArg
fn from(node: LifetimeArg) -> GenericArg
fn from(node: TypeArg) -> GenericArg
fn from(node: AssocTypeArg) -> GenericArg
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## GenericParam

`enum` · `ra_ap_syntax::ast::generated::nodes::GenericParam`

Also reachable as `ra_ap_syntax::ast::GenericParam`

```rust
enum GenericParam
```

**Variants**: `ConstParam`, `LifetimeParam`, `TypeParam`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: TypeOrConstParam) -> Self
fn from(node: LifetimeParam) -> GenericParam
fn from(node: TypeParam) -> GenericParam
fn from(node: ConstParam) -> GenericParam
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## Item

`enum` · `ra_ap_syntax::ast::generated::nodes::Item`

Also reachable as `ra_ap_syntax::ast::Item`

```rust
enum Item
```

**Variants**: `AsmExpr`, `Const`, `Enum`, `ExternBlock`, `ExternCrate`, `Fn`, `Impl`, `MacroCall`, `MacroDef`, `MacroRules`, `Module`, `Static`, `Struct`, `Trait`, `TypeAlias`, `Union`, `Use`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn generic_param_list(&self) -> Option<ast::GenericParamList>
```

**via `core::convert::From`**

```rust
fn from(node: Struct) -> Item
fn from(node: Impl) -> Item
fn from(node: Enum) -> Item
fn from(node: TypeAlias) -> Item
fn from(it: ast::Adt) -> Self
fn from(node: Union) -> Item
fn from(node: Fn) -> Item
fn from(node: MacroRules) -> Item
fn from(node: Module) -> Item
fn from(node: Const) -> Item
fn from(node: AsmExpr) -> Item
fn from(node: MacroDef) -> Item
fn from(node: Static) -> Item
fn from(assoc: ast::AssocItem) -> Self
fn from(extern_item: ast::ExternItem) -> Self
fn from(node: MacroCall) -> Item
fn from(node: ExternBlock) -> Item
fn from(node: Trait) -> Item
fn from(node: ExternCrate) -> Item
fn from(node: Use) -> Item
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## Meta

`enum` · `ra_ap_syntax::ast::generated::nodes::Meta`

Also reachable as `ra_ap_syntax::ast::Meta`

```rust
enum Meta
```

**Variants**: `CfgAttrMeta`, `CfgMeta`, `KeyValueMeta`, `PathMeta`, `TokenTreeMeta`, `UnsafeMeta`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn as_simple_atom(&self) -> Option<SmolStr>
fn as_simple_call(&self) -> Option<(SmolStr, ast::TokenTree)>
fn as_simple_path(&self) -> Option<ast::Path>
fn parent_attr(&self) -> Option<ast::Attr>
fn path(&self) -> Option<ast::Path>
fn simple_name(&self) -> Option<SmolStr>
fn skip_cfg_attrs(self) -> SmallVec<[ast::Meta; 1]>
```

**via `core::convert::From`**

```rust
fn from(node: TokenTreeMeta) -> Meta
fn from(node: CfgMeta) -> Meta
fn from(node: CfgAttrMeta) -> Meta
fn from(node: KeyValueMeta) -> Meta
fn from(node: UnsafeMeta) -> Meta
fn from(node: PathMeta) -> Meta
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## Pat

`enum` · `ra_ap_syntax::ast::generated::nodes::Pat`

Also reachable as `ra_ap_syntax::ast::Pat`

```rust
enum Pat
```

**Variants**: `BoxPat`, `ConstBlockPat`, `DerefPat`, `IdentPat`, `LiteralPat`, `MacroPat`, `NotNull`, `OrPat`, `ParenPat`, `PathPat`, `RangePat`, `RecordPat`, `RefPat`, `RestPat`, `SlicePat`, `TuplePat`, `TupleStructPat`, `WildcardPat`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: OrPat) -> Pat
fn from(node: BoxPat) -> Pat
fn from(node: WildcardPat) -> Pat
fn from(node: TupleStructPat) -> Pat
fn from(node: PathPat) -> Pat
fn from(node: DerefPat) -> Pat
fn from(node: TuplePat) -> Pat
fn from(node: SlicePat) -> Pat
fn from(node: ConstBlockPat) -> Pat
fn from(node: MacroPat) -> Pat
fn from(node: ParenPat) -> Pat
fn from(node: RangePat) -> Pat
fn from(node: LiteralPat) -> Pat
fn from(node: NotNull) -> Pat
fn from(node: RestPat) -> Pat
fn from(node: RefPat) -> Pat
fn from(node: RecordPat) -> Pat
fn from(node: IdentPat) -> Pat
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## Stmt

`enum` · `ra_ap_syntax::ast::generated::nodes::Stmt`

Also reachable as `ra_ap_syntax::ast::Stmt`

```rust
enum Stmt
```

**Variants**: `ExprStmt`, `Item`, `LetStmt`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: Item) -> Stmt
fn from(node: LetStmt) -> Stmt
fn from(node: ExprStmt) -> Stmt
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## Type

`enum` · `ra_ap_syntax::ast::generated::nodes::Type`

Also reachable as `ra_ap_syntax::ast::Type`

```rust
enum Type
```

**Variants**: `ArrayType`, `DynTraitType`, `FnPtrType`, `ForType`, `ImplTraitType`, `InferType`, `MacroType`, `NeverType`, `ParenType`, `PathType`, `PatternType`, `PtrType`, `RefType`, `SliceType`, `TupleType`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn generic_arg_list(&self) -> Option<ast::GenericArgList>
fn needs_angles_in_path(&self) -> bool
fn needs_parens_in(&self, parent: &SyntaxNode) -> bool
```

**via `core::convert::From`**

```rust
fn from(node: ParenType) -> Type
fn from(node: PathType) -> Type
fn from(node: MacroType) -> Type
fn from(node: InferType) -> Type
fn from(node: PatternType) -> Type
fn from(node: FnPtrType) -> Type
fn from(node: ForType) -> Type
fn from(node: RefType) -> Type
fn from(node: TupleType) -> Type
fn from(node: ArrayType) -> Type
fn from(node: ImplTraitType) -> Type
fn from(node: NeverType) -> Type
fn from(node: PtrType) -> Type
fn from(node: SliceType) -> Type
fn from(node: DynTraitType) -> Type
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## UseBoundGenericArg

`enum` · `ra_ap_syntax::ast::generated::nodes::UseBoundGenericArg`

Also reachable as `ra_ap_syntax::ast::UseBoundGenericArg`

```rust
enum UseBoundGenericArg
```

**Variants**: `Lifetime`, `NameRef`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: Lifetime) -> UseBoundGenericArg
fn from(node: NameRef) -> UseBoundGenericArg
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## VariantDef

`enum` · `ra_ap_syntax::ast::generated::nodes::VariantDef`

Also reachable as `ra_ap_syntax::ast::VariantDef`

```rust
enum VariantDef
```

**Variants**: `Struct`, `Union`, `Variant`

**Implements**: `core::convert::From`, `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(node: Struct) -> VariantDef
fn from(node: Union) -> VariantDef
fn from(node: Variant) -> VariantDef
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## Abi

`struct` · `ra_ap_syntax::ast::generated::nodes::Abi`

Also reachable as `ra_ap_syntax::ast::Abi`

```rust
struct Abi
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn abi_string(&self) -> Option<ast::String>
fn extern_token(&self) -> Option<SyntaxToken>
fn string_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasArgList

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasArgList`

Also reachable as `ra_ap_syntax::ast::AnyHasArgList`

```rust
struct AnyHasArgList
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasArgList`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasArgList>(node: T) -> AnyHasArgList
```

**via `core::convert::From`**

```rust
fn from(node: MethodCallExpr) -> AnyHasArgList
fn from(node: CallExpr) -> AnyHasArgList
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasAttrs

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasAttrs`

Also reachable as `ra_ap_syntax::ast::AnyHasAttrs`

```rust
struct AnyHasAttrs
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasAttrs>(node: T) -> AnyHasAttrs
```

**via `core::convert::From`**

```rust
fn from(node: RecordExprField) -> AnyHasAttrs
fn from(node: SourceFile) -> AnyHasAttrs
fn from(node: ReturnExpr) -> AnyHasAttrs
fn from(node: BinExpr) -> AnyHasAttrs
fn from(node: UnderscoreExpr) -> AnyHasAttrs
fn from(node: MatchExpr) -> AnyHasAttrs
fn from(node: MacroRules) -> AnyHasAttrs
fn from(node: RecordPatField) -> AnyHasAttrs
fn from(node: AsmClobberAbi) -> AnyHasAttrs
fn from(node: ParenExpr) -> AnyHasAttrs
fn from(node: ArrayExpr) -> AnyHasAttrs
fn from(node: Module) -> AnyHasAttrs
fn from(node: LoopExpr) -> AnyHasAttrs
fn from(node: CastExpr) -> AnyHasAttrs
fn from(node: Const) -> AnyHasAttrs
fn from(node: BreakExpr) -> AnyHasAttrs
fn from(node: AsmExpr) -> AnyHasAttrs
fn from(node: MacroDef) -> AnyHasAttrs
fn from(node: AsmOperandNamed) -> AnyHasAttrs
fn from(node: SelfParam) -> AnyHasAttrs
fn from(node: Static) -> AnyHasAttrs
fn from(node: OffsetOfExpr) -> AnyHasAttrs
fn from(node: MethodCallExpr) -> AnyHasAttrs
fn from(node: RecordField) -> AnyHasAttrs
fn from(node: ast::AssocItem) -> Self
fn from(node: YieldExpr) -> AnyHasAttrs
fn from(node: RefExpr) -> AnyHasAttrs
fn from(node: AwaitExpr) -> AnyHasAttrs
fn from(node: CallExpr) -> AnyHasAttrs
fn from(node: ForExpr) -> AnyHasAttrs
fn from(node: AssocItemList) -> AnyHasAttrs
fn from(node: MacroCall) -> AnyHasAttrs
fn from(node: ExternItemList) -> AnyHasAttrs
fn from(node: ast::Item) -> Self
fn from(node: ExternBlock) -> AnyHasAttrs
fn from(node: MatchArm) -> AnyHasAttrs
fn from(node: LifetimeParam) -> AnyHasAttrs
fn from(node: RestPat) -> AnyHasAttrs
fn from(node: RecordExprFieldList) -> AnyHasAttrs
fn from(node: Param) -> AnyHasAttrs
fn from(node: TypeParam) -> AnyHasAttrs
fn from(node: RangeExpr) -> AnyHasAttrs
fn from(node: AsmOptions) -> AnyHasAttrs
fn from(node: Trait) -> AnyHasAttrs
fn from(node: MatchArmList) -> AnyHasAttrs
fn from(node: IndexExpr) -> AnyHasAttrs
fn from(node: Union) -> AnyHasAttrs
fn from(node: Variant) -> AnyHasAttrs
fn from(node: FieldExpr) -> AnyHasAttrs
fn from(node: ClosureExpr) -> AnyHasAttrs
fn from(node: ExternCrate) -> AnyHasAttrs
fn from(node: Use) -> AnyHasAttrs
fn from(node: Literal) -> AnyHasAttrs
fn from(node: BlockExpr) -> AnyHasAttrs
fn from(node: IdentPat) -> AnyHasAttrs
fn from(node: ConstParam) -> AnyHasAttrs
fn from(node: Struct) -> AnyHasAttrs
fn from(node: TupleExpr) -> AnyHasAttrs
fn from(node: Impl) -> AnyHasAttrs
fn from(node: Enum) -> AnyHasAttrs
fn from(node: PathExpr) -> AnyHasAttrs
fn from(node: StmtList) -> AnyHasAttrs
fn from(node: LetStmt) -> AnyHasAttrs
fn from(node: ContinueExpr) -> AnyHasAttrs
fn from(node: ItemList) -> AnyHasAttrs
fn from(node: TupleField) -> AnyHasAttrs
fn from(node: IfExpr) -> AnyHasAttrs
fn from(node: FormatArgsExpr) -> AnyHasAttrs
fn from(node: TypeAlias) -> AnyHasAttrs
fn from(node: BecomeExpr) -> AnyHasAttrs
fn from(node: TryExpr) -> AnyHasAttrs
fn from(node: YeetExpr) -> AnyHasAttrs
fn from(node: PrefixExpr) -> AnyHasAttrs
fn from(node: LetExpr) -> AnyHasAttrs
fn from(node: Fn) -> AnyHasAttrs
fn from(node: WhileExpr) -> AnyHasAttrs
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasGenericArgs

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasGenericArgs`

Also reachable as `ra_ap_syntax::ast::AnyHasGenericArgs`

```rust
struct AnyHasGenericArgs
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasGenericArgs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasGenericArgs>(node: T) -> AnyHasGenericArgs
```

**via `core::convert::From`**

```rust
fn from(node: MethodCallExpr) -> AnyHasGenericArgs
fn from(node: PathSegment) -> AnyHasGenericArgs
fn from(node: AssocTypeArg) -> AnyHasGenericArgs
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasGenericParams

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasGenericParams`

Also reachable as `ra_ap_syntax::ast::AnyHasGenericParams`

```rust
struct AnyHasGenericParams
```

**Implements**: `core::convert::From`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasGenericParams`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasGenericParams>(node: T) -> AnyHasGenericParams
```

**via `core::convert::From`**

```rust
fn from(node: Const) -> AnyHasGenericParams
fn from(node: Trait) -> AnyHasGenericParams
fn from(node: Union) -> AnyHasGenericParams
fn from(node: Struct) -> AnyHasGenericParams
fn from(node: Impl) -> AnyHasGenericParams
fn from(node: Enum) -> AnyHasGenericParams
fn from(node: TypeAlias) -> AnyHasGenericParams
fn from(node: Fn) -> AnyHasGenericParams
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasLoopBody

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasLoopBody`

Also reachable as `ra_ap_syntax::ast::AnyHasLoopBody`

```rust
struct AnyHasLoopBody
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasLoopBody`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasLoopBody>(node: T) -> AnyHasLoopBody
```

**via `core::convert::From`**

```rust
fn from(node: ForExpr) -> AnyHasLoopBody
fn from(node: WhileExpr) -> AnyHasLoopBody
fn from(node: LoopExpr) -> AnyHasLoopBody
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasModuleItem

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasModuleItem`

Also reachable as `ra_ap_syntax::ast::AnyHasModuleItem`

```rust
struct AnyHasModuleItem
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasModuleItem`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasModuleItem>(node: T) -> AnyHasModuleItem
```

**via `core::convert::From`**

```rust
fn from(node: SourceFile) -> AnyHasModuleItem
fn from(node: MacroItems) -> AnyHasModuleItem
fn from(node: ItemList) -> AnyHasModuleItem
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasName

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasName`

Also reachable as `ra_ap_syntax::ast::AnyHasName`

```rust
struct AnyHasName
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasName>(node: T) -> AnyHasName
```

**via `core::convert::From`**

```rust
fn from(node: Union) -> AnyHasName
fn from(node: FormatArgsArg) -> AnyHasName
fn from(node: RecordField) -> AnyHasName
fn from(node: Struct) -> AnyHasName
fn from(node: Enum) -> AnyHasName
fn from(node: TypeAlias) -> AnyHasName
fn from(node: TypeParam) -> AnyHasName
fn from(node: AsmOperandNamed) -> AnyHasName
fn from(node: Variant) -> AnyHasName
fn from(node: Fn) -> AnyHasName
fn from(node: IdentPat) -> AnyHasName
fn from(node: ConstParam) -> AnyHasName
fn from(node: Rename) -> AnyHasName
fn from(node: MacroRules) -> AnyHasName
fn from(node: Module) -> AnyHasName
fn from(node: MacroDef) -> AnyHasName
fn from(node: Const) -> AnyHasName
fn from(node: Trait) -> AnyHasName
fn from(node: SelfParam) -> AnyHasName
fn from(node: Static) -> AnyHasName
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasTypeBounds

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasTypeBounds`

Also reachable as `ra_ap_syntax::ast::AnyHasTypeBounds`

```rust
struct AnyHasTypeBounds
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasTypeBounds`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasTypeBounds>(node: T) -> AnyHasTypeBounds
```

**via `core::convert::From`**

```rust
fn from(node: AssocTypeArg) -> AnyHasTypeBounds
fn from(node: WherePred) -> AnyHasTypeBounds
fn from(node: LifetimeParam) -> AnyHasTypeBounds
fn from(node: Trait) -> AnyHasTypeBounds
fn from(node: TypeAlias) -> AnyHasTypeBounds
fn from(node: TypeParam) -> AnyHasTypeBounds
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## AnyHasVisibility

`struct` · `ra_ap_syntax::ast::generated::nodes::AnyHasVisibility`

Also reachable as `ra_ap_syntax::ast::AnyHasVisibility`

```rust
struct AnyHasVisibility
```

**Implements**: `core::convert::From`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn new<T: ast::HasVisibility>(node: T) -> AnyHasVisibility
```

**via `core::convert::From`**

```rust
fn from(node: Variant) -> AnyHasVisibility
fn from(node: Fn) -> AnyHasVisibility
fn from(node: Impl) -> AnyHasVisibility
fn from(node: ExternCrate) -> AnyHasVisibility
fn from(node: Use) -> AnyHasVisibility
fn from(node: Trait) -> AnyHasVisibility
fn from(node: MacroRules) -> AnyHasVisibility
fn from(node: TupleField) -> AnyHasVisibility
fn from(node: Module) -> AnyHasVisibility
fn from(node: TypeAlias) -> AnyHasVisibility
fn from(node: MacroDef) -> AnyHasVisibility
fn from(node: Static) -> AnyHasVisibility
fn from(node: Union) -> AnyHasVisibility
fn from(node: RecordField) -> AnyHasVisibility
fn from(node: Struct) -> AnyHasVisibility
fn from(node: Enum) -> AnyHasVisibility
fn from(node: Const) -> AnyHasVisibility
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn syntax(&self) -> &SyntaxNode
```

---

## ArgList

`struct` · `ra_ap_syntax::ast::generated::nodes::ArgList`

Also reachable as `ra_ap_syntax::ast::ArgList`

```rust
struct ArgList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn args(&self) -> AstChildren<Expr>
fn args_maybe_empty(&self) -> impl Iterator<Item = Option<ast::Expr>>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ArrayExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::ArrayExpr`

Also reachable as `ra_ap_syntax::ast::ArrayExpr`

```rust
struct ArrayExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn expr(&self) -> Option<Expr>
fn exprs(&self) -> AstChildren<Expr>
fn kind(&self) -> ArrayExprKind
fn l_brack_token(&self) -> Option<SyntaxToken>
fn r_brack_token(&self) -> Option<SyntaxToken>
fn semicolon_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ArrayType

`struct` · `ra_ap_syntax::ast::generated::nodes::ArrayType`

Also reachable as `ra_ap_syntax::ast::ArrayType`

```rust
struct ArrayType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn const_arg(&self) -> Option<ConstArg>
fn l_brack_token(&self) -> Option<SyntaxToken>
fn r_brack_token(&self) -> Option<SyntaxToken>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmClobberAbi

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmClobberAbi`

Also reachable as `ra_ap_syntax::ast::AsmClobberAbi`

```rust
struct AsmClobberAbi
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn clobber_abi_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn string_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmConst

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmConst`

Also reachable as `ra_ap_syntax::ast::AsmConst`

```rust
struct AsmConst
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn const_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmDirSpec

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmDirSpec`

Also reachable as `ra_ap_syntax::ast::AsmDirSpec`

```rust
struct AsmDirSpec
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn in_token(&self) -> Option<SyntaxToken>
fn inlateout_token(&self) -> Option<SyntaxToken>
fn inout_token(&self) -> Option<SyntaxToken>
fn lateout_token(&self) -> Option<SyntaxToken>
fn out_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmExpr`

Also reachable as `ra_ap_syntax::ast::AsmExpr`

```rust
struct AsmExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (10)

```rust
fn asm_pieces(&self) -> AstChildren<AsmPiece>
fn asm_token(&self) -> Option<SyntaxToken>
fn builtin_token(&self) -> Option<SyntaxToken>
fn comma_token(&self) -> Option<SyntaxToken>
fn global_asm_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn naked_asm_token(&self) -> Option<SyntaxToken>
fn pound_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn template(&self) -> AstChildren<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmLabel

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmLabel`

Also reachable as `ra_ap_syntax::ast::AsmLabel`

```rust
struct AsmLabel
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn block_expr(&self) -> Option<BlockExpr>
fn label_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmOperandExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmOperandExpr`

Also reachable as `ra_ap_syntax::ast::AsmOperandExpr`

```rust
struct AsmOperandExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn fat_arrow_token(&self) -> Option<SyntaxToken>
fn in_expr(&self) -> Option<Expr>
fn out_expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmOperandNamed

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmOperandNamed`

Also reachable as `ra_ap_syntax::ast::AsmOperandNamed`

```rust
struct AsmOperandNamed
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn asm_operand(&self) -> Option<AsmOperand>
fn eq_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmOption

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmOption`

Also reachable as `ra_ap_syntax::ast::AsmOption`

```rust
struct AsmOption
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (9)

```rust
fn att_syntax_token(&self) -> Option<SyntaxToken>
fn may_unwind_token(&self) -> Option<SyntaxToken>
fn nomem_token(&self) -> Option<SyntaxToken>
fn noreturn_token(&self) -> Option<SyntaxToken>
fn nostack_token(&self) -> Option<SyntaxToken>
fn preserves_flags_token(&self) -> Option<SyntaxToken>
fn pure_token(&self) -> Option<SyntaxToken>
fn raw_token(&self) -> Option<SyntaxToken>
fn readonly_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmOptions

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmOptions`

Also reachable as `ra_ap_syntax::ast::AsmOptions`

```rust
struct AsmOptions
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn asm_options(&self) -> AstChildren<AsmOption>
fn comma_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn options_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmRegOperand

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmRegOperand`

Also reachable as `ra_ap_syntax::ast::AsmRegOperand`

```rust
struct AsmRegOperand
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn asm_dir_spec(&self) -> Option<AsmDirSpec>
fn asm_operand_expr(&self) -> Option<AsmOperandExpr>
fn asm_reg_spec(&self) -> Option<AsmRegSpec>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmRegSpec

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmRegSpec`

Also reachable as `ra_ap_syntax::ast::AsmRegSpec`

```rust
struct AsmRegSpec
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn name_ref(&self) -> Option<NameRef>
fn string_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AsmSym

`struct` · `ra_ap_syntax::ast::generated::nodes::AsmSym`

Also reachable as `ra_ap_syntax::ast::AsmSym`

```rust
struct AsmSym
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn path(&self) -> Option<Path>
fn sym_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AssocItemList

`struct` · `ra_ap_syntax::ast::generated::nodes::AssocItemList`

Also reachable as `ra_ap_syntax::ast::AssocItemList`

```rust
struct AssocItemList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn add_items(&self, editor: &SyntaxEditor, items: Vec<ast::AssocItem>)
fn assoc_items(&self) -> AstChildren<AssocItem>
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AssocTypeArg

`struct` · `ra_ap_syntax::ast::generated::nodes::AssocTypeArg`

Also reachable as `ra_ap_syntax::ast::AssocTypeArg`

```rust
struct AssocTypeArg
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasGenericArgs`, `ra_ap_syntax::ast::traits::HasTypeBounds`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn const_arg(&self) -> Option<ConstArg>
fn eq_token(&self) -> Option<SyntaxToken>
fn name_ref(&self) -> Option<NameRef>
fn param_list(&self) -> Option<ParamList>
fn ret_type(&self) -> Option<RetType>
fn return_type_syntax(&self) -> Option<ReturnTypeSyntax>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Attr

`struct` · `ra_ap_syntax::ast::generated::nodes::Attr`

Also reachable as `ra_ap_syntax::ast::Attr`

```rust
struct Attr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (12)

```rust
fn as_simple_atom(&self) -> Option<SmolStr>
fn as_simple_call(&self) -> Option<(SmolStr, ast::TokenTree)>
fn as_simple_path(&self) -> Option<ast::Path>
fn excl_token(&self) -> Option<SyntaxToken>
fn kind(&self) -> AttrKind
fn l_brack_token(&self) -> Option<SyntaxToken>
fn meta(&self) -> Option<Meta>
fn path(&self) -> Option<ast::Path>
fn pound_token(&self) -> Option<SyntaxToken>
fn r_brack_token(&self) -> Option<SyntaxToken>
fn simple_name(&self) -> Option<SmolStr>
fn skip_cfg_attrs(&self) -> SmallVec<[ast::Meta; 1]>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## AwaitExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::AwaitExpr`

Also reachable as `ra_ap_syntax::ast::AwaitExpr`

```rust
struct AwaitExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn await_token(&self) -> Option<SyntaxToken>
fn dot_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## BecomeExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::BecomeExpr`

Also reachable as `ra_ap_syntax::ast::BecomeExpr`

```rust
struct BecomeExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn become_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## BinExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::BinExpr`

Also reachable as `ra_ap_syntax::ast::BinExpr`

```rust
struct BinExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn lhs(&self) -> Option<ast::Expr>
fn op_details(&self) -> Option<(SyntaxToken, BinaryOp)>
fn op_kind(&self) -> Option<BinaryOp>
fn op_token(&self) -> Option<SyntaxToken>
fn rhs(&self) -> Option<ast::Expr>
fn sub_exprs(&self) -> (Option<ast::Expr>, Option<ast::Expr>)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## BlockExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::BlockExpr`

Also reachable as `ra_ap_syntax::ast::BlockExpr`

```rust
struct BlockExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (13)

```rust
fn async_token(&self) -> Option<SyntaxToken>
fn const_token(&self) -> Option<SyntaxToken>
fn gen_token(&self) -> Option<SyntaxToken>
fn is_standalone(&self) -> bool
fn label(&self) -> Option<Label>
fn may_carry_attributes(&self) -> bool
fn modifier(&self) -> Option<BlockModifier>
fn move_token(&self) -> Option<SyntaxToken>
fn statements(&self) -> impl Iterator<Item = ast::Stmt>
fn stmt_list(&self) -> Option<StmtList>
fn tail_expr(&self) -> Option<ast::Expr>
fn try_block_modifier(&self) -> Option<TryBlockModifier>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## BoxPat

`struct` · `ra_ap_syntax::ast::generated::nodes::BoxPat`

Also reachable as `ra_ap_syntax::ast::BoxPat`

```rust
struct BoxPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn box_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## BreakExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::BreakExpr`

Also reachable as `ra_ap_syntax::ast::BreakExpr`

```rust
struct BreakExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn break_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn lifetime(&self) -> Option<Lifetime>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## CallExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::CallExpr`

Also reachable as `ra_ap_syntax::ast::CallExpr`

```rust
struct CallExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasArgList`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## CastExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::CastExpr`

Also reachable as `ra_ap_syntax::ast::CastExpr`

```rust
struct CastExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn as_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## CfgAtom

`struct` · `ra_ap_syntax::ast::generated::nodes::CfgAtom`

Also reachable as `ra_ap_syntax::ast::CfgAtom`

```rust
struct CfgAtom
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn eq_token(&self) -> Option<SyntaxToken>
fn false_token(&self) -> Option<SyntaxToken>
fn ident_token(&self) -> Option<SyntaxToken>
fn key(&self) -> Option<CfgAtomKey>
fn string_token(&self) -> Option<SyntaxToken>
fn true_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## CfgAttrMeta

`struct` · `ra_ap_syntax::ast::generated::nodes::CfgAttrMeta`

Also reachable as `ra_ap_syntax::ast::CfgAttrMeta`

```rust
struct CfgAttrMeta
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn cfg_attr_token(&self) -> Option<SyntaxToken>
fn cfg_predicate(&self) -> Option<CfgPredicate>
fn comma_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn metas(&self) -> AstChildren<Meta>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## CfgComposite

`struct` · `ra_ap_syntax::ast::generated::nodes::CfgComposite`

Also reachable as `ra_ap_syntax::ast::CfgComposite`

```rust
struct CfgComposite
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn cfg_predicates(&self) -> AstChildren<CfgPredicate>
fn keyword(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## CfgMeta

`struct` · `ra_ap_syntax::ast::generated::nodes::CfgMeta`

Also reachable as `ra_ap_syntax::ast::CfgMeta`

```rust
struct CfgMeta
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn cfg_predicate(&self) -> Option<CfgPredicate>
fn cfg_token(&self) -> Option<SyntaxToken>
fn comma_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ClosureExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::ClosureExpr`

Also reachable as `ra_ap_syntax::ast::ClosureExpr`

```rust
struct ClosureExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (9)

```rust
fn async_token(&self) -> Option<SyntaxToken>
fn body(&self) -> Option<Expr>
fn const_token(&self) -> Option<SyntaxToken>
fn for_binder(&self) -> Option<ForBinder>
fn gen_token(&self) -> Option<SyntaxToken>
fn move_token(&self) -> Option<SyntaxToken>
fn param_list(&self) -> Option<ParamList>
fn ret_type(&self) -> Option<RetType>
fn static_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Const

`struct` · `ra_ap_syntax::ast::generated::nodes::Const`

Also reachable as `ra_ap_syntax::ast::Const`

```rust
struct Const
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (9)

```rust
fn body(&self) -> Option<Expr>
fn colon_token(&self) -> Option<SyntaxToken>
fn const_token(&self) -> Option<SyntaxToken>
fn default_token(&self) -> Option<SyntaxToken>
fn eq_token(&self) -> Option<SyntaxToken>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
fn type_token(&self) -> Option<SyntaxToken>
fn underscore_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ConstArg

`struct` · `ra_ap_syntax::ast::generated::nodes::ConstArg`

Also reachable as `ra_ap_syntax::ast::ConstArg`

```rust
struct ConstArg
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ConstBlockPat

`struct` · `ra_ap_syntax::ast::generated::nodes::ConstBlockPat`

Also reachable as `ra_ap_syntax::ast::ConstBlockPat`

```rust
struct ConstBlockPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn block_expr(&self) -> Option<BlockExpr>
fn const_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ConstParam

`struct` · `ra_ap_syntax::ast::generated::nodes::ConstParam`

Also reachable as `ra_ap_syntax::ast::ConstParam`

```rust
struct ConstParam
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn const_token(&self) -> Option<SyntaxToken>
fn default_val(&self) -> Option<ConstArg>
fn eq_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ContinueExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::ContinueExpr`

Also reachable as `ra_ap_syntax::ast::ContinueExpr`

```rust
struct ContinueExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn continue_token(&self) -> Option<SyntaxToken>
fn lifetime(&self) -> Option<Lifetime>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## DerefPat

`struct` · `ra_ap_syntax::ast::generated::nodes::DerefPat`

Also reachable as `ra_ap_syntax::ast::DerefPat`

```rust
struct DerefPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn builtin_token(&self) -> Option<SyntaxToken>
fn deref_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
fn pound_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## DocComment

`struct` · `ra_ap_syntax::ast::generated::nodes::DocComment`

Also reachable as `ra_ap_syntax::ast::DocComment`

```rust
struct DocComment
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn inner_doc_comment_token(&self) -> Option<SyntaxToken>
fn kind(&self) -> AttrKind
fn outer_doc_comment_token(&self) -> Option<SyntaxToken>
fn shape(&self) -> CommentShape
fn text(&self) -> &str
fn text_with_markers(&self) -> &str
fn token(&self) -> AnyComment
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## DynTraitType

`struct` · `ra_ap_syntax::ast::generated::nodes::DynTraitType`

Also reachable as `ra_ap_syntax::ast::DynTraitType`

```rust
struct DynTraitType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn dyn_token(&self) -> Option<SyntaxToken>
fn type_bound_list(&self) -> Option<TypeBoundList>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Enum

`struct` · `ra_ap_syntax::ast::generated::nodes::Enum`

Also reachable as `ra_ap_syntax::ast::Enum`

```rust
struct Enum
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`, `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn enum_token(&self) -> Option<SyntaxToken>
fn variant_list(&self) -> Option<VariantList>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`**

```rust
fn where_clause_position(&self) -> Option<Position>
```

---

## ExprStmt

`struct` · `ra_ap_syntax::ast::generated::nodes::ExprStmt`

Also reachable as `ra_ap_syntax::ast::ExprStmt`

```rust
struct ExprStmt
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn expr(&self) -> Option<Expr>
fn semicolon_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ExternBlock

`struct` · `ra_ap_syntax::ast::generated::nodes::ExternBlock`

Also reachable as `ra_ap_syntax::ast::ExternBlock`

```rust
struct ExternBlock
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn abi(&self) -> Option<Abi>
fn extern_item_list(&self) -> Option<ExternItemList>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ExternCrate

`struct` · `ra_ap_syntax::ast::generated::nodes::ExternCrate`

Also reachable as `ra_ap_syntax::ast::ExternCrate`

```rust
struct ExternCrate
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn crate_token(&self) -> Option<SyntaxToken>
fn extern_token(&self) -> Option<SyntaxToken>
fn name_ref(&self) -> Option<NameRef>
fn rename(&self) -> Option<Rename>
fn semicolon_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ExternItemList

`struct` · `ra_ap_syntax::ast::generated::nodes::ExternItemList`

Also reachable as `ra_ap_syntax::ast::ExternItemList`

```rust
struct ExternItemList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn extern_items(&self) -> AstChildren<ExternItem>
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## FieldExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::FieldExpr`

Also reachable as `ra_ap_syntax::ast::FieldExpr`

```rust
struct FieldExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn dot_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn field_access(&self) -> Option<FieldKind>
fn index_token(&self) -> Option<SyntaxToken>
fn name_ref(&self) -> Option<NameRef>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Fn

`struct` · `ra_ap_syntax::ast::generated::nodes::Fn`

Also reachable as `ra_ap_syntax::ast::Fn`

```rust
struct Fn
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`, `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (13)

```rust
fn abi(&self) -> Option<Abi>
fn async_token(&self) -> Option<SyntaxToken>
fn body(&self) -> Option<BlockExpr>
fn const_token(&self) -> Option<SyntaxToken>
fn default_token(&self) -> Option<SyntaxToken>
fn fn_token(&self) -> Option<SyntaxToken>
fn gen_token(&self) -> Option<SyntaxToken>
fn param_list(&self) -> Option<ParamList>
fn replace_or_insert_body(&self, editor: &SyntaxEditor, body: ast::BlockExpr)
fn ret_type(&self) -> Option<RetType>
fn safe_token(&self) -> Option<SyntaxToken>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`**

```rust
fn where_clause_position(&self) -> Option<Position>
```

---

## FnPtrType

`struct` · `ra_ap_syntax::ast::generated::nodes::FnPtrType`

Also reachable as `ra_ap_syntax::ast::FnPtrType`

```rust
struct FnPtrType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn abi(&self) -> Option<Abi>
fn async_token(&self) -> Option<SyntaxToken>
fn const_token(&self) -> Option<SyntaxToken>
fn fn_token(&self) -> Option<SyntaxToken>
fn param_list(&self) -> Option<ParamList>
fn ret_type(&self) -> Option<RetType>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ForBinder

`struct` · `ra_ap_syntax::ast::generated::nodes::ForBinder`

Also reachable as `ra_ap_syntax::ast::ForBinder`

```rust
struct ForBinder
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn for_token(&self) -> Option<SyntaxToken>
fn generic_param_list(&self) -> Option<GenericParamList>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ForExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::ForExpr`

Also reachable as `ra_ap_syntax::ast::ForExpr`

```rust
struct ForExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasLoopBody`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn for_token(&self) -> Option<SyntaxToken>
fn in_token(&self) -> Option<SyntaxToken>
fn iterable(&self) -> Option<ast::Expr>
fn pat(&self) -> Option<Pat>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::ast::traits::HasLoopBody`**

```rust
fn loop_body(&self) -> Option<ast::BlockExpr>
```

---

## ForType

`struct` · `ra_ap_syntax::ast::generated::nodes::ForType`

Also reachable as `ra_ap_syntax::ast::ForType`

```rust
struct ForType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn for_binder(&self) -> Option<ForBinder>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## FormatArgsArg

`struct` · `ra_ap_syntax::ast::generated::nodes::FormatArgsArg`

Also reachable as `ra_ap_syntax::ast::FormatArgsArg`

```rust
struct FormatArgsArg
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn eq_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## FormatArgsExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::FormatArgsExpr`

Also reachable as `ra_ap_syntax::ast::FormatArgsExpr`

```rust
struct FormatArgsExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (8)

```rust
fn args(&self) -> AstChildren<FormatArgsArg>
fn builtin_token(&self) -> Option<SyntaxToken>
fn comma_token(&self) -> Option<SyntaxToken>
fn format_args_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn pound_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn template(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## GenericArgList

`struct` · `ra_ap_syntax::ast::generated::nodes::GenericArgList`

Also reachable as `ra_ap_syntax::ast::GenericArgList`

```rust
struct GenericArgList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn coloncolon_token(&self) -> Option<SyntaxToken>
fn generic_args(&self) -> AstChildren<GenericArg>
fn l_angle_token(&self) -> Option<SyntaxToken>
fn lifetime_args(&self) -> impl Iterator<Item = ast::LifetimeArg>
fn r_angle_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## GenericParamList

`struct` · `ra_ap_syntax::ast::generated::nodes::GenericParamList`

Also reachable as `ra_ap_syntax::ast::GenericParamList`

```rust
struct GenericParamList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn generic_params(&self) -> AstChildren<GenericParam>
fn l_angle_token(&self) -> Option<SyntaxToken>
fn lifetime_params(&self) -> impl Iterator<Item = ast::LifetimeParam>
fn r_angle_token(&self) -> Option<SyntaxToken>
fn to_generic_args(&self, make: &SyntaxFactory) -> ast::GenericArgList
fn type_or_const_params(&self) -> impl Iterator<Item = ast::TypeOrConstParam> + use<>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## IdentPat

`struct` · `ra_ap_syntax::ast::generated::nodes::IdentPat`

Also reachable as `ra_ap_syntax::ast::IdentPat`

```rust
struct IdentPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn at_token(&self) -> Option<SyntaxToken>
fn is_simple_ident(&self) -> bool
fn mut_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
fn ref_token(&self) -> Option<SyntaxToken>
fn set_pat(&self, pat: Option<ast::Pat>, editor: &SyntaxEditor) -> ast::IdentPat
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## IfExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::IfExpr`

Also reachable as `ra_ap_syntax::ast::IfExpr`

```rust
struct IfExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn condition(&self) -> Option<ast::Expr>
fn else_branch(&self) -> Option<ElseBranch>
fn else_token(&self) -> Option<SyntaxToken>
fn if_token(&self) -> Option<SyntaxToken>
fn then_branch(&self) -> Option<ast::BlockExpr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Impl

`struct` · `ra_ap_syntax::ast::generated::nodes::Impl`

Also reachable as `ra_ap_syntax::ast::Impl`

```rust
struct Impl
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasVisibility`, `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (11)

```rust
fn assoc_item_list(&self) -> Option<AssocItemList>
fn const_token(&self) -> Option<SyntaxToken>
fn default_token(&self) -> Option<SyntaxToken>
fn excl_token(&self) -> Option<SyntaxToken>
fn for_token(&self) -> Option<SyntaxToken>
fn for_trait_name_ref(name_ref: &ast::NameRef) -> Option<ast::Impl>
fn get_or_create_assoc_item_list_with_editor(&self, editor: &SyntaxEditor) -> ast::AssocItemList
fn impl_token(&self) -> Option<SyntaxToken>
fn self_ty(&self) -> Option<ast::Type>
fn trait_(&self) -> Option<ast::Type>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`**

```rust
fn where_clause_position(&self) -> Option<Position>
```

---

## ImplRestriction

`struct` · `ra_ap_syntax::ast::generated::nodes::ImplRestriction`

Also reachable as `ra_ap_syntax::ast::ImplRestriction`

```rust
struct ImplRestriction
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn impl_token(&self) -> Option<SyntaxToken>
fn visibility_inner(&self) -> Option<VisibilityInner>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ImplTraitType

`struct` · `ra_ap_syntax::ast::generated::nodes::ImplTraitType`

Also reachable as `ra_ap_syntax::ast::ImplTraitType`

```rust
struct ImplTraitType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn impl_token(&self) -> Option<SyntaxToken>
fn type_bound_list(&self) -> Option<TypeBoundList>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## IncludeBytesExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::IncludeBytesExpr`

Also reachable as `ra_ap_syntax::ast::IncludeBytesExpr`

```rust
struct IncludeBytesExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn builtin_token(&self) -> Option<SyntaxToken>
fn include_bytes_token(&self) -> Option<SyntaxToken>
fn pound_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## IndexExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::IndexExpr`

Also reachable as `ra_ap_syntax::ast::IndexExpr`

```rust
struct IndexExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn base(&self) -> Option<ast::Expr>
fn index(&self) -> Option<ast::Expr>
fn l_brack_token(&self) -> Option<SyntaxToken>
fn r_brack_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## InferType

`struct` · `ra_ap_syntax::ast::generated::nodes::InferType`

Also reachable as `ra_ap_syntax::ast::InferType`

```rust
struct InferType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn underscore_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ItemList

`struct` · `ra_ap_syntax::ast::generated::nodes::ItemList`

Also reachable as `ra_ap_syntax::ast::ItemList`

```rust
struct ItemList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasModuleItem`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## KeyValueMeta

`struct` · `ra_ap_syntax::ast::generated::nodes::KeyValueMeta`

Also reachable as `ra_ap_syntax::ast::KeyValueMeta`

```rust
struct KeyValueMeta
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn eq_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn path(&self) -> Option<Path>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Label

`struct` · `ra_ap_syntax::ast::generated::nodes::Label`

Also reachable as `ra_ap_syntax::ast::Label`

```rust
struct Label
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn lifetime(&self) -> Option<Lifetime>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## LetElse

`struct` · `ra_ap_syntax::ast::generated::nodes::LetElse`

Also reachable as `ra_ap_syntax::ast::LetElse`

```rust
struct LetElse
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn block_expr(&self) -> Option<BlockExpr>
fn else_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## LetExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::LetExpr`

Also reachable as `ra_ap_syntax::ast::LetExpr`

```rust
struct LetExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn eq_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn let_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## LetStmt

`struct` · `ra_ap_syntax::ast::generated::nodes::LetStmt`

Also reachable as `ra_ap_syntax::ast::LetStmt`

```rust
struct LetStmt
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (9)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn eq_token(&self) -> Option<SyntaxToken>
fn initializer(&self) -> Option<Expr>
fn let_else(&self) -> Option<LetElse>
fn let_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn super_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Lifetime

`struct` · `ra_ap_syntax::ast::generated::nodes::Lifetime`

Also reachable as `ra_ap_syntax::ast::Lifetime`

```rust
struct Lifetime
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn lifetime_ident_token(&self) -> Option<SyntaxToken>
fn text(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## LifetimeArg

`struct` · `ra_ap_syntax::ast::generated::nodes::LifetimeArg`

Also reachable as `ra_ap_syntax::ast::LifetimeArg`

```rust
struct LifetimeArg
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn lifetime(&self) -> Option<Lifetime>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## LifetimeParam

`struct` · `ra_ap_syntax::ast::generated::nodes::LifetimeParam`

Also reachable as `ra_ap_syntax::ast::LifetimeParam`

```rust
struct LifetimeParam
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasTypeBounds`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn lifetime(&self) -> Option<Lifetime>
fn lifetime_bounds(&self) -> impl Iterator<Item = SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Literal

`struct` · `ra_ap_syntax::ast::generated::nodes::Literal`

Also reachable as `ra_ap_syntax::ast::Literal`

```rust
struct Literal
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn kind(&self) -> LiteralKind
fn token(&self) -> SyntaxToken
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## LiteralPat

`struct` · `ra_ap_syntax::ast::generated::nodes::LiteralPat`

Also reachable as `ra_ap_syntax::ast::LiteralPat`

```rust
struct LiteralPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn literal(&self) -> Option<Literal>
fn minus_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## LoopExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::LoopExpr`

Also reachable as `ra_ap_syntax::ast::LoopExpr`

```rust
struct LoopExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasLoopBody`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn loop_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroCall

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroCall`

Also reachable as `ra_ap_syntax::ast::MacroCall`

```rust
struct MacroCall
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn excl_token(&self) -> Option<SyntaxToken>
fn path(&self) -> Option<Path>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn token_tree(&self) -> Option<TokenTree>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroDef

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroDef`

Also reachable as `ra_ap_syntax::ast::MacroDef`

```rust
struct MacroDef
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn args(&self) -> Option<TokenTree>
fn body(&self) -> Option<TokenTree>
fn macro_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroExpr`

Also reachable as `ra_ap_syntax::ast::MacroExpr`

```rust
struct MacroExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn macro_call(&self) -> Option<MacroCall>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroItems

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroItems`

Also reachable as `ra_ap_syntax::ast::MacroItems`

```rust
struct MacroItems
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasModuleItem`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroPat

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroPat`

Also reachable as `ra_ap_syntax::ast::MacroPat`

```rust
struct MacroPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn macro_call(&self) -> Option<MacroCall>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroRules

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroRules`

Also reachable as `ra_ap_syntax::ast::MacroRules`

```rust
struct MacroRules
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn excl_token(&self) -> Option<SyntaxToken>
fn macro_rules_token(&self) -> Option<SyntaxToken>
fn token_tree(&self) -> Option<TokenTree>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroStmts

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroStmts`

Also reachable as `ra_ap_syntax::ast::MacroStmts`

```rust
struct MacroStmts
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn expr(&self) -> Option<Expr>
fn statements(&self) -> AstChildren<Stmt>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MacroType

`struct` · `ra_ap_syntax::ast::generated::nodes::MacroType`

Also reachable as `ra_ap_syntax::ast::MacroType`

```rust
struct MacroType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn macro_call(&self) -> Option<MacroCall>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MatchArm

`struct` · `ra_ap_syntax::ast::generated::nodes::MatchArm`

Also reachable as `ra_ap_syntax::ast::MatchArm`

```rust
struct MatchArm
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn comma_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn fat_arrow_token(&self) -> Option<SyntaxToken>
fn guard(&self) -> Option<MatchGuard>
fn parent_match(&self) -> ast::MatchExpr
fn pat(&self) -> Option<Pat>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MatchArmList

`struct` · `ra_ap_syntax::ast::generated::nodes::MatchArmList`

Also reachable as `ra_ap_syntax::ast::MatchArmList`

```rust
struct MatchArmList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn arms(&self) -> AstChildren<MatchArm>
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MatchExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::MatchExpr`

Also reachable as `ra_ap_syntax::ast::MatchExpr`

```rust
struct MatchExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn expr(&self) -> Option<Expr>
fn match_arm_list(&self) -> Option<MatchArmList>
fn match_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MatchGuard

`struct` · `ra_ap_syntax::ast::generated::nodes::MatchGuard`

Also reachable as `ra_ap_syntax::ast::MatchGuard`

```rust
struct MatchGuard
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn condition(&self) -> Option<ast::Expr>
fn if_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MethodCallExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::MethodCallExpr`

Also reachable as `ra_ap_syntax::ast::MethodCallExpr`

```rust
struct MethodCallExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasArgList`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericArgs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn dot_token(&self) -> Option<SyntaxToken>
fn name_ref(&self) -> Option<NameRef>
fn receiver(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Module

`struct` · `ra_ap_syntax::ast::generated::nodes::Module`

Also reachable as `ra_ap_syntax::ast::Module`

```rust
struct Module
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn item_list(&self) -> Option<ItemList>
fn mod_token(&self) -> Option<SyntaxToken>
fn parent(&self) -> Option<ast::Module>
fn semicolon_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## MutRestriction

`struct` · `ra_ap_syntax::ast::generated::nodes::MutRestriction`

Also reachable as `ra_ap_syntax::ast::MutRestriction`

```rust
struct MutRestriction
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn mut_token(&self) -> Option<SyntaxToken>
fn visibility_inner(&self) -> Option<VisibilityInner>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Name

`struct` · `ra_ap_syntax::ast::generated::nodes::Name`

Also reachable as `ra_ap_syntax::ast::Name`

```rust
struct Name
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn ident_token(&self) -> Option<SyntaxToken>
fn self_token(&self) -> Option<SyntaxToken>
fn text(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## NameRef

`struct` · `ra_ap_syntax::ast::generated::nodes::NameRef`

Also reachable as `ra_ap_syntax::ast::NameRef`

```rust
struct NameRef
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (9)

```rust
fn Self_token(&self) -> Option<SyntaxToken>
fn as_tuple_field(&self) -> Option<usize>
fn crate_token(&self) -> Option<SyntaxToken>
fn ident_token(&self) -> Option<SyntaxToken>
fn int_number_token(&self) -> Option<SyntaxToken>
fn self_token(&self) -> Option<SyntaxToken>
fn super_token(&self) -> Option<SyntaxToken>
fn text(&self) -> &str
fn token_kind(&self) -> SyntaxKind
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## NeverType

`struct` · `ra_ap_syntax::ast::generated::nodes::NeverType`

Also reachable as `ra_ap_syntax::ast::NeverType`

```rust
struct NeverType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn excl_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## NotNull

`struct` · `ra_ap_syntax::ast::generated::nodes::NotNull`

Also reachable as `ra_ap_syntax::ast::NotNull`

```rust
struct NotNull
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn excl_token(&self) -> Option<SyntaxToken>
fn null_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## OffsetOfExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::OffsetOfExpr`

Also reachable as `ra_ap_syntax::ast::OffsetOfExpr`

```rust
struct OffsetOfExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (8)

```rust
fn builtin_token(&self) -> Option<SyntaxToken>
fn comma_token(&self) -> Option<SyntaxToken>
fn fields(&self) -> AstChildren<NameRef>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn offset_of_token(&self) -> Option<SyntaxToken>
fn pound_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## OrPat

`struct` · `ra_ap_syntax::ast::generated::nodes::OrPat`

Also reachable as `ra_ap_syntax::ast::OrPat`

```rust
struct OrPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn leading_pipe(&self) -> Option<SyntaxToken>
fn pats(&self) -> AstChildren<Pat>
fn pipe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Param

`struct` · `ra_ap_syntax::ast::generated::nodes::Param`

Also reachable as `ra_ap_syntax::ast::Param`

```rust
struct Param
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn dotdotdot_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ParamList

`struct` · `ra_ap_syntax::ast::generated::nodes::ParamList`

Also reachable as `ra_ap_syntax::ast::ParamList`

```rust
struct ParamList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn comma_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn params(&self) -> AstChildren<Param>
fn pipe_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn self_param(&self) -> Option<SelfParam>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ParenExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::ParenExpr`

Also reachable as `ra_ap_syntax::ast::ParenExpr`

```rust
struct ParenExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn expr(&self) -> Option<Expr>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ParenPat

`struct` · `ra_ap_syntax::ast::generated::nodes::ParenPat`

Also reachable as `ra_ap_syntax::ast::ParenPat`

```rust
struct ParenPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn l_paren_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ParenType

`struct` · `ra_ap_syntax::ast::generated::nodes::ParenType`

Also reachable as `ra_ap_syntax::ast::ParenType`

```rust
struct ParenType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ParenthesizedArgList

`struct` · `ra_ap_syntax::ast::generated::nodes::ParenthesizedArgList`

Also reachable as `ra_ap_syntax::ast::ParenthesizedArgList`

```rust
struct ParenthesizedArgList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn coloncolon_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn type_args(&self) -> AstChildren<TypeArg>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Path

`struct` · `ra_ap_syntax::ast::generated::nodes::Path`

Also reachable as `ra_ap_syntax::ast::Path`

```rust
struct Path
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (12)

```rust
fn as_single_name_ref(&self) -> Option<ast::NameRef>
fn as_single_segment(&self) -> Option<ast::PathSegment>
fn coloncolon_token(&self) -> Option<SyntaxToken>
fn first_qualifier(&self) -> Option<ast::Path>
fn first_qualifier_or_self(&self) -> ast::Path
fn first_segment(&self) -> Option<ast::PathSegment>
fn parent_path(&self) -> Option<ast::Path>
fn qualifier(&self) -> Option<Path>
fn qualifiers(&self) -> impl Iterator<Item = ast::Path> + Clone
fn segment(&self) -> Option<PathSegment>
fn segments(&self) -> impl Iterator<Item = ast::PathSegment> + Clone
fn top_path(&self) -> ast::Path
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PathExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::PathExpr`

Also reachable as `ra_ap_syntax::ast::PathExpr`

```rust
struct PathExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn path(&self) -> Option<Path>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PathMeta

`struct` · `ra_ap_syntax::ast::generated::nodes::PathMeta`

Also reachable as `ra_ap_syntax::ast::PathMeta`

```rust
struct PathMeta
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn path(&self) -> Option<Path>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PathPat

`struct` · `ra_ap_syntax::ast::generated::nodes::PathPat`

Also reachable as `ra_ap_syntax::ast::PathPat`

```rust
struct PathPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn path(&self) -> Option<Path>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PathSegment

`struct` · `ra_ap_syntax::ast::generated::nodes::PathSegment`

Also reachable as `ra_ap_syntax::ast::PathSegment`

```rust
struct PathSegment
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasGenericArgs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (13)

```rust
fn coloncolon_token(&self) -> Option<SyntaxToken>
fn crate_token(&self) -> Option<SyntaxToken>
fn kind(&self) -> Option<PathSegmentKind>
fn name_ref(&self) -> Option<NameRef>
fn parent_path(&self) -> ast::Path
fn parenthesized_arg_list(&self) -> Option<ParenthesizedArgList>
fn qualifying_trait(&self) -> Option<ast::PathType>
fn ret_type(&self) -> Option<RetType>
fn return_type_syntax(&self) -> Option<ReturnTypeSyntax>
fn self_token(&self) -> Option<SyntaxToken>
fn self_type_token(&self) -> Option<SyntaxToken>
fn super_token(&self) -> Option<SyntaxToken>
fn type_anchor(&self) -> Option<TypeAnchor>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PathType

`struct` · `ra_ap_syntax::ast::generated::nodes::PathType`

Also reachable as `ra_ap_syntax::ast::PathType`

```rust
struct PathType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn path(&self) -> Option<Path>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PatternType

`struct` · `ra_ap_syntax::ast::generated::nodes::PatternType`

Also reachable as `ra_ap_syntax::ast::PatternType`

```rust
struct PatternType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (8)

```rust
fn builtin_token(&self) -> Option<SyntaxToken>
fn is_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
fn pattern_type_token(&self) -> Option<SyntaxToken>
fn pound_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PrefixExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::PrefixExpr`

Also reachable as `ra_ap_syntax::ast::PrefixExpr`

```rust
struct PrefixExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn expr(&self) -> Option<Expr>
fn op_kind(&self) -> Option<UnaryOp>
fn op_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## PtrType

`struct` · `ra_ap_syntax::ast::generated::nodes::PtrType`

Also reachable as `ra_ap_syntax::ast::PtrType`

```rust
struct PtrType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn const_token(&self) -> Option<SyntaxToken>
fn mut_token(&self) -> Option<SyntaxToken>
fn star_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RangeExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::RangeExpr`

Also reachable as `ra_ap_syntax::ast::RangeExpr`

```rust
struct RangeExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::RangeItem`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn is_range_full(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::ast::RangeItem`**

```rust
fn end(&self) -> Option<ast::Expr>
fn op_kind(&self) -> Option<RangeOp>
fn op_token(&self) -> Option<SyntaxToken>
fn start(&self) -> Option<ast::Expr>
```

---

## RangePat

`struct` · `ra_ap_syntax::ast::generated::nodes::RangePat`

Also reachable as `ra_ap_syntax::ast::RangePat`

```rust
struct RangePat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::RangeItem`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::ast::RangeItem`**

```rust
fn end(&self) -> Option<ast::Pat>
fn op_kind(&self) -> Option<RangeOp>
fn op_token(&self) -> Option<SyntaxToken>
fn start(&self) -> Option<ast::Pat>
```

---

## RecordExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordExpr`

Also reachable as `ra_ap_syntax::ast::RecordExpr`

```rust
struct RecordExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn path(&self) -> Option<Path>
fn record_expr_field_list(&self) -> Option<RecordExprFieldList>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RecordExprField

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordExprField`

Also reachable as `ra_ap_syntax::ast::RecordExprField`

```rust
struct RecordExprField
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (8)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn field_name(&self) -> Option<ast::NameRef>
fn for_field_name(field_name: &ast::NameRef) -> Option<ast::RecordExprField>
fn for_name_ref(name_ref: &ast::NameRef) -> Option<ast::RecordExprField>
fn name_ref(&self) -> Option<NameRef>
fn parent_record_lit(&self) -> ast::RecordExpr
fn replace_expr(&self, editor: &SyntaxEditor, expr: ast::Expr)
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RecordExprFieldList

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordExprFieldList`

Also reachable as `ra_ap_syntax::ast::RecordExprFieldList`

```rust
struct RecordExprFieldList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn add_fields(&self, editor: &SyntaxEditor, fields: impl IntoIterator<Item = ast::RecordExprField>)
fn dotdot_token(&self) -> Option<SyntaxToken>
fn fields(&self) -> AstChildren<RecordExprField>
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
fn spread(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RecordField

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordField`

Also reachable as `ra_ap_syntax::ast::RecordField`

```rust
struct RecordField
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn default_val(&self) -> Option<ConstArg>
fn eq_token(&self) -> Option<SyntaxToken>
fn mut_restriction(&self) -> Option<MutRestriction>
fn ty(&self) -> Option<Type>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RecordFieldList

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordFieldList`

Also reachable as `ra_ap_syntax::ast::RecordFieldList`

```rust
struct RecordFieldList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn fields(&self) -> AstChildren<RecordField>
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RecordPat

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordPat`

Also reachable as `ra_ap_syntax::ast::RecordPat`

```rust
struct RecordPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn path(&self) -> Option<Path>
fn record_pat_field_list(&self) -> Option<RecordPatFieldList>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RecordPatField

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordPatField`

Also reachable as `ra_ap_syntax::ast::RecordPatField`

```rust
struct RecordPatField
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn colon_token(&self) -> Option<SyntaxToken>
fn field_name(&self) -> Option<NameOrNameRef>
fn for_field_name(field_name: &ast::Name) -> Option<ast::RecordPatField>
fn for_field_name_ref(field_name: &ast::NameRef) -> Option<ast::RecordPatField>
fn name_ref(&self) -> Option<NameRef>
fn parent_record_pat(&self) -> ast::RecordPat
fn pat(&self) -> Option<Pat>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RecordPatFieldList

`struct` · `ra_ap_syntax::ast::generated::nodes::RecordPatFieldList`

Also reachable as `ra_ap_syntax::ast::RecordPatFieldList`

```rust
struct RecordPatFieldList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn add_fields(&self, editor: &SyntaxEditor, fields: impl IntoIterator<Item = ast::RecordPatField>)
fn fields(&self) -> AstChildren<RecordPatField>
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
fn rest_pat(&self) -> Option<RestPat>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RefExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::RefExpr`

Also reachable as `ra_ap_syntax::ast::RefExpr`

```rust
struct RefExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn amp_token(&self) -> Option<SyntaxToken>
fn const_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn mut_token(&self) -> Option<SyntaxToken>
fn raw_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RefPat

`struct` · `ra_ap_syntax::ast::generated::nodes::RefPat`

Also reachable as `ra_ap_syntax::ast::RefPat`

```rust
struct RefPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn amp_token(&self) -> Option<SyntaxToken>
fn mut_token(&self) -> Option<SyntaxToken>
fn pat(&self) -> Option<Pat>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RefType

`struct` · `ra_ap_syntax::ast::generated::nodes::RefType`

Also reachable as `ra_ap_syntax::ast::RefType`

```rust
struct RefType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn amp_token(&self) -> Option<SyntaxToken>
fn lifetime(&self) -> Option<Lifetime>
fn mut_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Rename

`struct` · `ra_ap_syntax::ast::generated::nodes::Rename`

Also reachable as `ra_ap_syntax::ast::Rename`

```rust
struct Rename
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn as_token(&self) -> Option<SyntaxToken>
fn underscore_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RestPat

`struct` · `ra_ap_syntax::ast::generated::nodes::RestPat`

Also reachable as `ra_ap_syntax::ast::RestPat`

```rust
struct RestPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn dotdot_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## RetType

`struct` · `ra_ap_syntax::ast::generated::nodes::RetType`

Also reachable as `ra_ap_syntax::ast::RetType`

```rust
struct RetType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn thin_arrow_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ReturnExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::ReturnExpr`

Also reachable as `ra_ap_syntax::ast::ReturnExpr`

```rust
struct ReturnExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn expr(&self) -> Option<Expr>
fn return_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## ReturnTypeSyntax

`struct` · `ra_ap_syntax::ast::generated::nodes::ReturnTypeSyntax`

Also reachable as `ra_ap_syntax::ast::ReturnTypeSyntax`

```rust
struct ReturnTypeSyntax
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn dotdot_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## SelfParam

`struct` · `ra_ap_syntax::ast::generated::nodes::SelfParam`

Also reachable as `ra_ap_syntax::ast::SelfParam`

```rust
struct SelfParam
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn amp_token(&self) -> Option<SyntaxToken>
fn colon_token(&self) -> Option<SyntaxToken>
fn kind(&self) -> SelfParamKind
fn lifetime(&self) -> Option<Lifetime>
fn mut_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## SlicePat

`struct` · `ra_ap_syntax::ast::generated::nodes::SlicePat`

Also reachable as `ra_ap_syntax::ast::SlicePat`

```rust
struct SlicePat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn components(&self) -> SlicePatComponents
fn l_brack_token(&self) -> Option<SyntaxToken>
fn pats(&self) -> AstChildren<Pat>
fn r_brack_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## SliceType

`struct` · `ra_ap_syntax::ast::generated::nodes::SliceType`

Also reachable as `ra_ap_syntax::ast::SliceType`

```rust
struct SliceType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn l_brack_token(&self) -> Option<SyntaxToken>
fn r_brack_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## SourceFile

`struct` · `ra_ap_syntax::ast::generated::nodes::SourceFile`

Also reachable as `ra_ap_syntax::SourceFile`, `ra_ap_syntax::ast::SourceFile`

```rust
struct SourceFile
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasModuleItem`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn frontmatter_token(&self) -> Option<SyntaxToken>
fn parse(text: &str, edition: Edition) -> Parse<SourceFile>
fn shebang_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Static

`struct` · `ra_ap_syntax::ast::generated::nodes::Static`

Also reachable as `ra_ap_syntax::ast::Static`

```rust
struct Static
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (9)

```rust
fn body(&self) -> Option<Expr>
fn colon_token(&self) -> Option<SyntaxToken>
fn eq_token(&self) -> Option<SyntaxToken>
fn mut_token(&self) -> Option<SyntaxToken>
fn safe_token(&self) -> Option<SyntaxToken>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn static_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## StmtList

`struct` · `ra_ap_syntax::ast::generated::nodes::StmtList`

Also reachable as `ra_ap_syntax::ast::StmtList`

```rust
struct StmtList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasModuleItem`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
fn statements(&self) -> AstChildren<Stmt>
fn tail_expr(&self) -> Option<Expr>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Struct

`struct` · `ra_ap_syntax::ast::generated::nodes::Struct`

Also reachable as `ra_ap_syntax::ast::Struct`

```rust
struct Struct
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`, `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn field_list(&self) -> Option<FieldList>
fn kind(&self) -> StructKind
fn semicolon_token(&self) -> Option<SyntaxToken>
fn struct_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`**

```rust
fn where_clause_position(&self) -> Option<Position>
```

---

## TokenTree

`struct` · `ra_ap_syntax::ast::generated::nodes::TokenTree`

Also reachable as `ra_ap_syntax::ast::TokenTree`

```rust
struct TokenTree
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (10)

```rust
fn l_brack_token(&self) -> Option<SyntaxToken>
fn l_curly_token(&self) -> Option<SyntaxToken>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn left_delimiter_token(&self) -> Option<SyntaxToken>
fn parent_meta(&self) -> Option<ast::Meta>
fn r_brack_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn right_delimiter_token(&self) -> Option<SyntaxToken>
fn token_trees_and_tokens(&self) -> impl Iterator<Item = NodeOrToken<ast::TokenTree, SyntaxToken>>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TokenTreeMeta

`struct` · `ra_ap_syntax::ast::generated::nodes::TokenTreeMeta`

Also reachable as `ra_ap_syntax::ast::TokenTreeMeta`

```rust
struct TokenTreeMeta
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn path(&self) -> Option<Path>
fn token_tree(&self) -> Option<TokenTree>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Trait

`struct` · `ra_ap_syntax::ast::generated::nodes::Trait`

Also reachable as `ra_ap_syntax::ast::Trait`

```rust
struct Trait
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasTypeBounds`, `ra_ap_syntax::ast::traits::HasVisibility`, `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn assoc_item_list(&self) -> Option<AssocItemList>
fn auto_token(&self) -> Option<SyntaxToken>
fn eq_token(&self) -> Option<SyntaxToken>
fn impl_restriction(&self) -> Option<ImplRestriction>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn trait_token(&self) -> Option<SyntaxToken>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`**

```rust
fn where_clause_position(&self) -> Option<Position>
```

---

## TryBlockModifier

`struct` · `ra_ap_syntax::ast::generated::nodes::TryBlockModifier`

Also reachable as `ra_ap_syntax::ast::TryBlockModifier`

```rust
struct TryBlockModifier
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn bikeshed_token(&self) -> Option<SyntaxToken>
fn try_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TryExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::TryExpr`

Also reachable as `ra_ap_syntax::ast::TryExpr`

```rust
struct TryExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn expr(&self) -> Option<Expr>
fn question_mark_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TupleExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::TupleExpr`

Also reachable as `ra_ap_syntax::ast::TupleExpr`

```rust
struct TupleExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn fields(&self) -> AstChildren<Expr>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TupleField

`struct` · `ra_ap_syntax::ast::generated::nodes::TupleField`

Also reachable as `ra_ap_syntax::ast::TupleField`

```rust
struct TupleField
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn mut_restriction(&self) -> Option<MutRestriction>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TupleFieldList

`struct` · `ra_ap_syntax::ast::generated::nodes::TupleFieldList`

Also reachable as `ra_ap_syntax::ast::TupleFieldList`

```rust
struct TupleFieldList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn fields(&self) -> AstChildren<TupleField>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TuplePat

`struct` · `ra_ap_syntax::ast::generated::nodes::TuplePat`

Also reachable as `ra_ap_syntax::ast::TuplePat`

```rust
struct TuplePat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn fields(&self) -> AstChildren<Pat>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TupleStructPat

`struct` · `ra_ap_syntax::ast::generated::nodes::TupleStructPat`

Also reachable as `ra_ap_syntax::ast::TupleStructPat`

```rust
struct TupleStructPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn fields(&self) -> AstChildren<Pat>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn path(&self) -> Option<Path>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TupleType

`struct` · `ra_ap_syntax::ast::generated::nodes::TupleType`

Also reachable as `ra_ap_syntax::ast::TupleType`

```rust
struct TupleType
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn fields(&self) -> AstChildren<Type>
fn l_paren_token(&self) -> Option<SyntaxToken>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TypeAlias

`struct` · `ra_ap_syntax::ast::generated::nodes::TypeAlias`

Also reachable as `ra_ap_syntax::ast::TypeAlias`

```rust
struct TypeAlias
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasTypeBounds`, `ra_ap_syntax::ast::traits::HasVisibility`, `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn default_token(&self) -> Option<SyntaxToken>
fn eq_token(&self) -> Option<SyntaxToken>
fn semicolon_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
fn type_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::GetOrCreateWhereClause`**

```rust
fn where_clause_position(&self) -> Option<Position>
```

---

## TypeAnchor

`struct` · `ra_ap_syntax::ast::generated::nodes::TypeAnchor`

Also reachable as `ra_ap_syntax::ast::TypeAnchor`

```rust
struct TypeAnchor
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn as_token(&self) -> Option<SyntaxToken>
fn l_angle_token(&self) -> Option<SyntaxToken>
fn path_type(&self) -> Option<PathType>
fn r_angle_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TypeArg

`struct` · `ra_ap_syntax::ast::generated::nodes::TypeArg`

Also reachable as `ra_ap_syntax::ast::TypeArg`

```rust
struct TypeArg
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TypeBound

`struct` · `ra_ap_syntax::ast::generated::nodes::TypeBound`

Also reachable as `ra_ap_syntax::ast::TypeBound`

```rust
struct TypeBound
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (12)

```rust
fn async_token(&self) -> Option<SyntaxToken>
fn const_token(&self) -> Option<SyntaxToken>
fn for_binder(&self) -> Option<ForBinder>
fn kind(&self) -> Option<TypeBoundKind>
fn l_brack_token(&self) -> Option<SyntaxToken>
fn lifetime(&self) -> Option<Lifetime>
fn question_mark_token(&self) -> Option<SyntaxToken>
fn r_brack_token(&self) -> Option<SyntaxToken>
fn tilde_token(&self) -> Option<SyntaxToken>
fn ty(&self) -> Option<Type>
fn use_bound_generic_args(&self) -> Option<UseBoundGenericArgs>
fn use_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## TypeBoundList

`struct` · `ra_ap_syntax::ast::generated::nodes::TypeBoundList`

Also reachable as `ra_ap_syntax::ast::TypeBoundList`

```rust
struct TypeBoundList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::syntax_editor::edits::Removable`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn bounds(&self) -> AstChildren<TypeBound>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::Removable`**

```rust
fn remove(&self, editor: &SyntaxEditor)
```

---

## TypeParam

`struct` · `ra_ap_syntax::ast::generated::nodes::TypeParam`

Also reachable as `ra_ap_syntax::ast::TypeParam`

```rust
struct TypeParam
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasTypeBounds`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn default_type(&self) -> Option<Type>
fn eq_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## UnderscoreExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::UnderscoreExpr`

Also reachable as `ra_ap_syntax::ast::UnderscoreExpr`

```rust
struct UnderscoreExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn underscore_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Union

`struct` · `ra_ap_syntax::ast::generated::nodes::Union`

Also reachable as `ra_ap_syntax::ast::Union`

```rust
struct Union
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasGenericParams`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn kind(&self) -> StructKind
fn record_field_list(&self) -> Option<RecordFieldList>
fn union_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## UnsafeMeta

`struct` · `ra_ap_syntax::ast::generated::nodes::UnsafeMeta`

Also reachable as `ra_ap_syntax::ast::UnsafeMeta`

```rust
struct UnsafeMeta
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn l_paren_token(&self) -> Option<SyntaxToken>
fn meta(&self) -> Option<Meta>
fn r_paren_token(&self) -> Option<SyntaxToken>
fn unsafe_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Use

`struct` · `ra_ap_syntax::ast::generated::nodes::Use`

Also reachable as `ra_ap_syntax::ast::Use`

```rust
struct Use
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasVisibility`, `ra_ap_syntax::syntax_editor::edits::Removable`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn is_simple_glob(&self) -> bool
fn semicolon_token(&self) -> Option<SyntaxToken>
fn use_token(&self) -> Option<SyntaxToken>
fn use_tree(&self) -> Option<UseTree>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::Removable`**

```rust
fn remove(&self, editor: &SyntaxEditor)
```

---

## UseBoundGenericArgs

`struct` · `ra_ap_syntax::ast::generated::nodes::UseBoundGenericArgs`

Also reachable as `ra_ap_syntax::ast::UseBoundGenericArgs`

```rust
struct UseBoundGenericArgs
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn l_angle_token(&self) -> Option<SyntaxToken>
fn r_angle_token(&self) -> Option<SyntaxToken>
fn use_bound_generic_args(&self) -> AstChildren<UseBoundGenericArg>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## UseTree

`struct` · `ra_ap_syntax::ast::generated::nodes::UseTree`

Also reachable as `ra_ap_syntax::ast::UseTree`

```rust
struct UseTree
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::syntax_editor::edits::Removable`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (11)

```rust
fn coloncolon_token(&self) -> Option<SyntaxToken>
fn is_simple_path(&self) -> bool
fn parent_use_tree_list(&self) -> Option<ast::UseTreeList>
fn path(&self) -> Option<Path>
fn remove_recursive(self, editor: &SyntaxEditor)
fn rename(&self) -> Option<Rename>
fn split_prefix_with_editor(&self, editor: &SyntaxEditor, prefix: &ast::Path)
fn star_token(&self) -> Option<SyntaxToken>
fn top_use_tree(&self) -> ast::UseTree
fn use_tree_list(&self) -> Option<UseTreeList>
fn wrap_in_tree_list_with_editor(&self) -> Option<ast::UseTree>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::syntax_editor::edits::Removable`**

```rust
fn remove(&self, editor: &SyntaxEditor)
```

---

## UseTreeList

`struct` · `ra_ap_syntax::ast::generated::nodes::UseTreeList`

Also reachable as `ra_ap_syntax::ast::UseTreeList`

```rust
struct UseTreeList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (7)

```rust
fn comma(&self) -> impl Iterator<Item = SyntaxToken>
fn has_inner_comment(&self) -> bool
fn l_curly_token(&self) -> Option<SyntaxToken>
fn parent_use_tree(&self) -> ast::UseTree
fn r_curly_token(&self) -> Option<SyntaxToken>
fn remove_unnecessary_braces(self, editor: &SyntaxEditor)
fn use_trees(&self) -> AstChildren<UseTree>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Variant

`struct` · `ra_ap_syntax::ast::generated::nodes::Variant`

Also reachable as `ra_ap_syntax::ast::Variant`

```rust
struct Variant
```

**Implements**: `core::fmt::Display`, `ra_ap_span::ast_id::AstIdNode`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasName`, `ra_ap_syntax::ast::traits::HasVisibility`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (6)

```rust
fn const_arg(&self) -> Option<ConstArg>
fn eq_token(&self) -> Option<SyntaxToken>
fn field_list(&self) -> Option<FieldList>
fn kind(&self) -> StructKind
fn parent_enum(&self) -> ast::Enum
fn underscore_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## VariantList

`struct` · `ra_ap_syntax::ast::generated::nodes::VariantList`

Also reachable as `ra_ap_syntax::ast::VariantList`

```rust
struct VariantList
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn add_variant(&self, editor: &SyntaxEditor, variant: &ast::Variant)
fn l_curly_token(&self) -> Option<SyntaxToken>
fn r_curly_token(&self) -> Option<SyntaxToken>
fn variants(&self) -> AstChildren<Variant>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## Visibility

`struct` · `ra_ap_syntax::ast::generated::nodes::Visibility`

Also reachable as `ra_ap_syntax::ast::Visibility`

```rust
struct Visibility
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn kind(&self) -> VisibilityKind
fn pub_token(&self) -> Option<SyntaxToken>
fn visibility_inner(&self) -> Option<VisibilityInner>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## VisibilityInner

`struct` · `ra_ap_syntax::ast::generated::nodes::VisibilityInner`

Also reachable as `ra_ap_syntax::ast::VisibilityInner`

```rust
struct VisibilityInner
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn in_token(&self) -> Option<SyntaxToken>
fn kind(&self) -> VisibilityKind
fn l_paren_token(&self) -> Option<SyntaxToken>
fn path(&self) -> Option<Path>
fn r_paren_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## WhereClause

`struct` · `ra_ap_syntax::ast::generated::nodes::WhereClause`

Also reachable as `ra_ap_syntax::ast::WhereClause`

```rust
struct WhereClause
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn predicates(&self) -> AstChildren<WherePred>
fn where_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## WherePred

`struct` · `ra_ap_syntax::ast::generated::nodes::WherePred`

Also reachable as `ra_ap_syntax::ast::WherePred`

```rust
struct WherePred
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasTypeBounds`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn for_binder(&self) -> Option<ForBinder>
fn lifetime(&self) -> Option<Lifetime>
fn ty(&self) -> Option<Type>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## WhileExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::WhileExpr`

Also reachable as `ra_ap_syntax::ast::WhileExpr`

```rust
struct WhileExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`, `ra_ap_syntax::ast::traits::HasLoopBody`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn condition(&self) -> Option<ast::Expr>
fn while_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

**via `ra_ap_syntax::ast::traits::HasLoopBody`**

```rust
fn loop_body(&self) -> Option<ast::BlockExpr>
```

---

## WildcardPat

`struct` · `ra_ap_syntax::ast::generated::nodes::WildcardPat`

Also reachable as `ra_ap_syntax::ast::WildcardPat`

```rust
struct WildcardPat
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn underscore_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## YeetExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::YeetExpr`

Also reachable as `ra_ap_syntax::ast::YeetExpr`

```rust
struct YeetExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn do_token(&self) -> Option<SyntaxToken>
fn expr(&self) -> Option<Expr>
fn yeet_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---

## YieldExpr

`struct` · `ra_ap_syntax::ast::generated::nodes::YieldExpr`

Also reachable as `ra_ap_syntax::ast::YieldExpr`

```rust
struct YieldExpr
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstNode`, `ra_ap_syntax::ast::traits::HasAttrs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn expr(&self) -> Option<Expr>
fn yield_token(&self) -> Option<SyntaxToken>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstNode`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxNode) -> Option<Self>
fn kind() -> SyntaxKind where Self: Sized
fn syntax(&self) -> &SyntaxNode
```

---
