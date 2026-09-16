# HasNodeIndex

`ruff_python_ast::node_index::HasNodeIndex`

```rust
trait HasNodeIndex
```

Also reachable as `ruff_python_ast::HasNodeIndex`

Prose: [`api/ruff_python_ast.node_index.md`](../api/ruff_python_ast.node_index.md#hasnodeindex) · records: [`model/ruff_python_ast.node_index.json`](../model/ruff_python_ast.node_index.json)

## Required

Every implementation must supply these.

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

## Implementors (110)

Read one before writing your own.

- `ruff_python_ast::generated::AnyNodeRef`
- `ruff_python_ast::generated::AnyRootNodeRef`
- `ruff_python_ast::generated::ExceptHandler`
- `ruff_python_ast::generated::ExceptHandlerRef`
- `ruff_python_ast::generated::Expr`
- `ruff_python_ast::generated::ExprAttribute`
- `ruff_python_ast::generated::ExprAwait`
- `ruff_python_ast::generated::ExprBinOp`
- `ruff_python_ast::generated::ExprBoolOp`
- `ruff_python_ast::generated::ExprBooleanLiteral`
- `ruff_python_ast::generated::ExprBytesLiteral`
- `ruff_python_ast::generated::ExprCall`
- `ruff_python_ast::generated::ExprCompare`
- `ruff_python_ast::generated::ExprDict`
- `ruff_python_ast::generated::ExprDictComp`
- `ruff_python_ast::generated::ExprEllipsisLiteral`
- `ruff_python_ast::generated::ExprFString`
- `ruff_python_ast::generated::ExprGenerator`
- `ruff_python_ast::generated::ExprIf`
- `ruff_python_ast::generated::ExprIpyEscapeCommand`
- `ruff_python_ast::generated::ExprLambda`
- `ruff_python_ast::generated::ExprList`
- `ruff_python_ast::generated::ExprListComp`
- `ruff_python_ast::generated::ExprName`
- `ruff_python_ast::generated::ExprNamed`
- `ruff_python_ast::generated::ExprNoneLiteral`
- `ruff_python_ast::generated::ExprNumberLiteral`
- `ruff_python_ast::generated::ExprRef`
- `ruff_python_ast::generated::ExprSet`
- `ruff_python_ast::generated::ExprSetComp`
- `ruff_python_ast::generated::ExprSlice`
- `ruff_python_ast::generated::ExprStarred`
- `ruff_python_ast::generated::ExprStringLiteral`
- `ruff_python_ast::generated::ExprSubscript`
- `ruff_python_ast::generated::ExprTString`
- `ruff_python_ast::generated::ExprTuple`
- `ruff_python_ast::generated::ExprUnaryOp`
- `ruff_python_ast::generated::ExprYield`
- `ruff_python_ast::generated::ExprYieldFrom`
- `ruff_python_ast::generated::InterpolatedStringElement`
- `ruff_python_ast::generated::InterpolatedStringElementRef`
- `ruff_python_ast::generated::Mod`
- `ruff_python_ast::generated::ModExpression`
- `ruff_python_ast::generated::ModModule`
- `ruff_python_ast::generated::ModRef`
- `ruff_python_ast::generated::Pattern`
- `ruff_python_ast::generated::PatternMatchAs`
- `ruff_python_ast::generated::PatternMatchClass`
- `ruff_python_ast::generated::PatternMatchMapping`
- `ruff_python_ast::generated::PatternMatchOr`
- `ruff_python_ast::generated::PatternMatchSequence`
- `ruff_python_ast::generated::PatternMatchSingleton`
- `ruff_python_ast::generated::PatternMatchStar`
- `ruff_python_ast::generated::PatternMatchValue`
- `ruff_python_ast::generated::PatternRef`
- `ruff_python_ast::generated::Stmt`
- `ruff_python_ast::generated::StmtAnnAssign`
- `ruff_python_ast::generated::StmtAssert`
- `ruff_python_ast::generated::StmtAssign`
- `ruff_python_ast::generated::StmtAugAssign`
- `ruff_python_ast::generated::StmtBreak`
- `ruff_python_ast::generated::StmtClassDef`
- `ruff_python_ast::generated::StmtContinue`
- `ruff_python_ast::generated::StmtDelete`
- `ruff_python_ast::generated::StmtExpr`
- `ruff_python_ast::generated::StmtFor`
- `ruff_python_ast::generated::StmtFunctionDef`
- `ruff_python_ast::generated::StmtGlobal`
- `ruff_python_ast::generated::StmtIf`
- `ruff_python_ast::generated::StmtImport`
- `ruff_python_ast::generated::StmtImportFrom`
- `ruff_python_ast::generated::StmtIpyEscapeCommand`
- `ruff_python_ast::generated::StmtMatch`
- `ruff_python_ast::generated::StmtNonlocal`
- `ruff_python_ast::generated::StmtPass`
- `ruff_python_ast::generated::StmtRaise`
- `ruff_python_ast::generated::StmtRef`
- `ruff_python_ast::generated::StmtReturn`
- `ruff_python_ast::generated::StmtTry`
- `ruff_python_ast::generated::StmtTypeAlias`
- `ruff_python_ast::generated::StmtWhile`
- `ruff_python_ast::generated::StmtWith`
- `ruff_python_ast::generated::TypeParam`
- `ruff_python_ast::generated::TypeParamParamSpec`
- `ruff_python_ast::generated::TypeParamRef`
- `ruff_python_ast::generated::TypeParamTypeVar`
- `ruff_python_ast::generated::TypeParamTypeVarTuple`
- `ruff_python_ast::nodes::Alias`
- `ruff_python_ast::nodes::Arguments`
- `ruff_python_ast::nodes::BytesLiteral`
- `ruff_python_ast::nodes::Comprehension`
- `ruff_python_ast::nodes::Decorator`
- `ruff_python_ast::nodes::ElifElseClause`
- `ruff_python_ast::nodes::ExceptHandlerExceptHandler`
- `ruff_python_ast::nodes::FString`
- `ruff_python_ast::nodes::Identifier`
- `ruff_python_ast::nodes::InterpolatedElement`
- `ruff_python_ast::nodes::InterpolatedStringFormatSpec`
- `ruff_python_ast::nodes::InterpolatedStringLiteralElement`
- `ruff_python_ast::nodes::Keyword`
- `ruff_python_ast::nodes::MatchCase`
- `ruff_python_ast::nodes::Parameter`
- `ruff_python_ast::nodes::ParameterWithDefault`
- `ruff_python_ast::nodes::Parameters`
- `ruff_python_ast::nodes::PatternArguments`
- `ruff_python_ast::nodes::PatternKeyword`
- `ruff_python_ast::nodes::StringLiteral`
- `ruff_python_ast::nodes::TString`
- `ruff_python_ast::nodes::TypeParams`
- `ruff_python_ast::nodes::WithItem`

## Documentation

An AST node that has an index.
