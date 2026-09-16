# `ruff_python_ast::node_index`

Crate `ruff_python_ast` · 7 public items · structured records in [`model/ruff_python_ast.node_index.json`](../model/ruff_python_ast.node_index.json)

## MAX_REAL_INDEX

`constant` · `ruff_python_ast::node_index::MAX_REAL_INDEX`

Also reachable as `ruff_python_ast::MAX_REAL_INDEX`

```rust
const MAX_REAL_INDEX: u32 = _
```

---

## NodeIndexError

`enum` · `ruff_python_ast::node_index::NodeIndexError`

Also reachable as `ruff_python_ast::NodeIndexError`

```rust
enum NodeIndexError
```

**Variants**: `NoParent`, `TooNested`, `ExhaustedSubIndices`, `ExhaustedSubSubIndices`, `OverflowedIndices`, `OverflowedSubIndices`

**Derives**: Clone, Copy, Debug

---

## sub_ast_level

`function` · `ruff_python_ast::node_index::sub_ast_level`

Also reachable as `ruff_python_ast::sub_ast_level`

```rust
fn sub_ast_level(index: u32) -> u32
```

sub-AST level is stored in the top two bits

---

## sub_indices

`function` · `ruff_python_ast::node_index::sub_indices`

Also reachable as `ruff_python_ast::sub_indices`

```rust
fn sub_indices(index: u32) -> Result<(u32, u32), NodeIndexError>
```

Get the first and last index of the sub-AST of the input

---

## AtomicNodeIndex

`struct` · `ruff_python_ast::node_index::AtomicNodeIndex`

Also reachable as `ruff_python_ast::AtomicNodeIndex`

```rust
struct AtomicNodeIndex
```

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (2)

```rust
fn load(&self) -> NodeIndex
fn set(&self, index: NodeIndex)
```

A unique index for a node within an AST.

This type is interiorly mutable to allow assigning node indices
on-demand after parsing.

---

## NodeIndex

`struct` · `ruff_python_ast::node_index::NodeIndex`

Also reachable as `ruff_python_ast::NodeIndex`

```rust
struct NodeIndex
```

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn as_u32(self) -> Option<u32>
```

**via `core::convert::From`**

```rust
fn from(value: u32) -> Self
```

A unique index for a node within an AST.

Our encoding of 32-bit AST node indices is as follows:

* `u32::MAX`     (1111...1) is reserved as a forbidden value (mapped to 0 for `NonZero`)
* `u32::MAX - 1` (1111...0) is reserved for `NodeIndex::NONE`
* The top two bits encode the sub-AST level:
  * 00 is top-level AST
  * 01 is sub-AST (string annotation)
  * 10 is sub-sub-AST (string annotation in string annotation)
  * 11 is forbidden (well, it only appears in the above reserved values)
* The remaining 30 bits are the real (sub)-AST node index

To get the first sub-index of a node's sub-AST we:

* increment the sub-AST level in the high-bits
* at level 1, multiply the real index by 256
* at level 2, multiply the real index by 8

The multiplication gives each node a reserved space of 256 nodes for its sub-AST
to work with ("should be enough for anybody"), and 8 nodes for a sub-sub-AST
(enough for an identifier and maybe some simple unions).

Here are some implications:

* We have 2^30 top-level AST nodes (1 billion)
* To have a string annotation, the parent node needs to be multiplied by 256 without
  overflowing 30 bits, so string annotations cannot be used after 2^22 nodes (4 million),
  which would be like, a million lines of code.
* To have a sub-string annotation, the top-level node needs to be multiplied
  by 256 * 8, so sub-string annotations cannot be used after 2^19 nodes (500 thousand),
  or about 100k lines of code.

This feels like a pretty reasonable compromise that will work well in practice,
although it creates some very wonky boundary conditions that will be very unpleasant
if someone runs into them.

That said, string annotations are in many regards "legacy" and so new code ideally
doesn't have to use them, and there's never a real reason to use sub-annotation
let-alone a sub-sub-annotation.

---

## HasNodeIndex

`trait` · `ruff_python_ast::node_index::HasNodeIndex`

Also reachable as `ruff_python_ast::HasNodeIndex`

```rust
trait HasNodeIndex
```

**Implementors** (110)

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

**Methods** (1)

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

An AST node that has an index.

---
