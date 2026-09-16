# `ruff_python_ast::comparable`

Crate `ruff_python_ast` · 104 public items · structured records in [`model/ruff_python_ast.comparable.json`](../model/ruff_python_ast.comparable.json)

## ComparableBoolOp

`enum` · `ruff_python_ast::comparable::ComparableBoolOp`

```rust
enum ComparableBoolOp
```

**Variants**: `And`, `Or`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(op: ast::BoolOp) -> Self
```

---

## ComparableCmpOp

`enum` · `ruff_python_ast::comparable::ComparableCmpOp`

```rust
enum ComparableCmpOp
```

**Variants**: `Eq`, `NotEq`, `Lt`, `LtE`, `Gt`, `GtE`, `Is`, `IsNot`, `In`, `NotIn`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(op: ast::CmpOp) -> Self
```

---

## ComparableExceptHandler

`enum` · `ruff_python_ast::comparable::ComparableExceptHandler`

```rust
enum ComparableExceptHandler<'a>
```

**Variants**: `ExceptHandler`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(except_handler: &'a ast::ExceptHandler) -> Self
```

---

## ComparableExpr

`enum` · `ruff_python_ast::comparable::ComparableExpr`

```rust
enum ComparableExpr<'a>
```

**Variants**: `BoolOp`, `NamedExpr`, `BinOp`, `UnaryOp`, `Lambda`, `IfExp`, `Dict`, `Set`, `ListComp`, `SetComp`, `DictComp`, `GeneratorExp`, `Await`, `Yield`, `YieldFrom`, `Compare`, `Call`, `FStringExpressionElement`, `FString`, `TStringInterpolationElement`, `TString`, `StringLiteral`, `BytesLiteral`, `NumberLiteral`, `BoolLiteral`, `NoneLiteral`, `EllipsisLiteral`, `Attribute`, `Subscript`, `Starred`, `Name`, `List`, `Tuple`, `Slice`, `IpyEscapeCommand`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(expr: &'a Box<ast::Expr>) -> Self
fn from(expr: &'a ast::Expr) -> Self
fn from(expr: &'a ast::ExprName) -> Self
```

---

## ComparableInterpolatedStringElement

`enum` · `ruff_python_ast::comparable::ComparableInterpolatedStringElement`

```rust
enum ComparableInterpolatedStringElement<'a>
```

**Variants**: `Literal`, `InterpolatedElement`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(interpolated_string_element: &'a ast::InterpolatedStringElement) -> Self
fn from(interpolated_element: &'a ast::InterpolatedElement) -> Self
```

---

## ComparableLiteral

`enum` · `ruff_python_ast::comparable::ComparableLiteral`

```rust
enum ComparableLiteral<'a>
```

**Variants**: `None`, `Ellipsis`, `Bool`, `Str`, `Bytes`, `Number`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(literal: ast::LiteralExpressionRef<'a>) -> Self
```

---

## ComparableMod

`enum` · `ruff_python_ast::comparable::ComparableMod`

```rust
enum ComparableMod<'a>
```

**Variants**: `Module`, `Expression`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(mod_: &'a ast::Mod) -> Self
```

---

## ComparableNumber

`enum` · `ruff_python_ast::comparable::ComparableNumber`

```rust
enum ComparableNumber<'a>
```

**Variants**: `Int`, `Float`, `Complex`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(number: &'a ast::Number) -> Self
```

---

## ComparableOperator

`enum` · `ruff_python_ast::comparable::ComparableOperator`

```rust
enum ComparableOperator
```

**Variants**: `Add`, `Sub`, `Mult`, `MatMult`, `Div`, `Mod`, `Pow`, `LShift`, `RShift`, `BitOr`, `BitXor`, `BitAnd`, `FloorDiv`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(op: ast::Operator) -> Self
```

---

## ComparablePattern

`enum` · `ruff_python_ast::comparable::ComparablePattern`

```rust
enum ComparablePattern<'a>
```

**Variants**: `MatchValue`, `MatchSingleton`, `MatchSequence`, `MatchMapping`, `MatchClass`, `MatchStar`, `MatchAs`, `MatchOr`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(pattern: &'a ast::Pattern) -> Self
```

---

## ComparableSingleton

`enum` · `ruff_python_ast::comparable::ComparableSingleton`

```rust
enum ComparableSingleton
```

**Variants**: `None`, `True`, `False`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(singleton: &ast::Singleton) -> Self
```

---

## ComparableStmt

`enum` · `ruff_python_ast::comparable::ComparableStmt`

```rust
enum ComparableStmt<'a>
```

**Variants**: `FunctionDef`, `ClassDef`, `Return`, `Delete`, `Assign`, `AugAssign`, `AnnAssign`, `For`, `While`, `If`, `With`, `Match`, `Raise`, `Try`, `TypeAlias`, `Assert`, `Import`, `ImportFrom`, `Global`, `Nonlocal`, `IpyEscapeCommand`, `Expr`, `Pass`, `Break`, `Continue`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(stmt: &'a ast::Stmt) -> Self
```

---

## ComparableTypeParam

`enum` · `ruff_python_ast::comparable::ComparableTypeParam`

```rust
enum ComparableTypeParam<'a>
```

**Variants**: `TypeVar`, `ParamSpec`, `TypeVarTuple`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(type_param: &'a ast::TypeParam) -> Self
```

---

## ComparableUnaryOp

`enum` · `ruff_python_ast::comparable::ComparableUnaryOp`

```rust
enum ComparableUnaryOp
```

**Variants**: `Invert`, `Not`, `UAdd`, `USub`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(op: ast::UnaryOp) -> Self
```

---

## ComparableAlias

`struct` · `ruff_python_ast::comparable::ComparableAlias`

```rust
struct ComparableAlias<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(alias: &'a ast::Alias) -> Self
```

---

## ComparableArguments

`struct` · `ruff_python_ast::comparable::ComparableArguments`

```rust
struct ComparableArguments<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(arguments: &'a Box<ast::Arguments>) -> Self
fn from(arguments: &'a ast::Arguments) -> Self
```

---

## ComparableBytesLiteral

`struct` · `ruff_python_ast::comparable::ComparableBytesLiteral`

```rust
struct ComparableBytesLiteral<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(bytes_literal: &'a ast::BytesLiteral) -> Self
```

---

## ComparableComprehension

`struct` · `ruff_python_ast::comparable::ComparableComprehension`

```rust
struct ComparableComprehension<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(comprehension: &'a ast::Comprehension) -> Self
```

---

## ComparableDebugText

`struct` · `ruff_python_ast::comparable::ComparableDebugText`

```rust
struct ComparableDebugText<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(debug_text: &'a ast::DebugText) -> Self
```

Comparable wrapper for [`ast::DebugText`].

Compares the full debug text (leading + expression source + trailing) rather than only the
expression source, because whitespace is part of the f-string's runtime output: `f"{x =}"`
produces `"x =<value>"` while `f"{x=}"` produces `"x=<value>"`, making them distinct
`Literal` types.

---

## ComparableDecorator

`struct` · `ruff_python_ast::comparable::ComparableDecorator`

```rust
struct ComparableDecorator<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(decorator: &'a ast::Decorator) -> Self
```

---

## ComparableDictItem

`struct` · `ruff_python_ast::comparable::ComparableDictItem`

```rust
struct ComparableDictItem<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(_: &'a ast::DictItem) -> Self
```

---

## ComparableElifElseClause

`struct` · `ruff_python_ast::comparable::ComparableElifElseClause`

```rust
struct ComparableElifElseClause<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(elif_else_clause: &'a ast::ElifElseClause) -> Self
```

---

## ComparableFString

`struct` · `ruff_python_ast::comparable::ComparableFString`

```rust
struct ComparableFString<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &'a ast::FStringValue) -> Self
```

---

## ComparableKeyword

`struct` · `ruff_python_ast::comparable::ComparableKeyword`

```rust
struct ComparableKeyword<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(keyword: &'a ast::Keyword) -> Self
```

---

## ComparableMatchCase

`struct` · `ruff_python_ast::comparable::ComparableMatchCase`

```rust
struct ComparableMatchCase<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(match_case: &'a ast::MatchCase) -> Self
```

---

## ComparableModExpression

`struct` · `ruff_python_ast::comparable::ComparableModExpression`

```rust
struct ComparableModExpression<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(expr: &'a ast::ModExpression) -> Self
```

---

## ComparableModModule

`struct` · `ruff_python_ast::comparable::ComparableModModule`

```rust
struct ComparableModModule<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(module: &'a ast::ModModule) -> Self
```

---

## ComparableParameter

`struct` · `ruff_python_ast::comparable::ComparableParameter`

```rust
struct ComparableParameter<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(arg: &'a Box<ast::Parameter>) -> Self
fn from(arg: &'a ast::Parameter) -> Self
```

---

## ComparableParameterWithDefault

`struct` · `ruff_python_ast::comparable::ComparableParameterWithDefault`

```rust
struct ComparableParameterWithDefault<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(arg: &'a ast::ParameterWithDefault) -> Self
```

---

## ComparableParameters

`struct` · `ruff_python_ast::comparable::ComparableParameters`

```rust
struct ComparableParameters<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(parameters: &'a ast::Parameters) -> Self
fn from(parameters: &'a Box<ast::Parameters>) -> Self
```

---

## ComparablePatternArguments

`struct` · `ruff_python_ast::comparable::ComparablePatternArguments`

```rust
struct ComparablePatternArguments<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(parameters: &'a ast::PatternArguments) -> Self
```

---

## ComparablePatternKeyword

`struct` · `ruff_python_ast::comparable::ComparablePatternKeyword`

```rust
struct ComparablePatternKeyword<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(keyword: &'a ast::PatternKeyword) -> Self
```

---

## ComparableStringLiteral

`struct` · `ruff_python_ast::comparable::ComparableStringLiteral`

```rust
struct ComparableStringLiteral<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(string_literal: &'a ast::StringLiteral) -> Self
```

---

## ComparableTString

`struct` · `ruff_python_ast::comparable::ComparableTString`

```rust
struct ComparableTString<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &'a ast::TStringValue) -> Self
```

---

## ComparableTypeParams

`struct` · `ruff_python_ast::comparable::ComparableTypeParams`

```rust
struct ComparableTypeParams<'a>
```

**Fields**: `type_params`

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(type_params: &'a ast::TypeParams) -> Self
fn from(type_params: &'a Box<ast::TypeParams>) -> Self
```

---

## ComparableWithItem

`struct` · `ruff_python_ast::comparable::ComparableWithItem`

```rust
struct ComparableWithItem<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(with_item: &'a ast::WithItem) -> Self
```

---

## ExceptHandlerExceptHandler

`struct` · `ruff_python_ast::comparable::ExceptHandlerExceptHandler`

```rust
struct ExceptHandlerExceptHandler<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprAttribute

`struct` · `ruff_python_ast::comparable::ExprAttribute`

```rust
struct ExprAttribute<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprAwait

`struct` · `ruff_python_ast::comparable::ExprAwait`

```rust
struct ExprAwait<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprBinOp

`struct` · `ruff_python_ast::comparable::ExprBinOp`

```rust
struct ExprBinOp<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprBoolLiteral

`struct` · `ruff_python_ast::comparable::ExprBoolLiteral`

```rust
struct ExprBoolLiteral
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprBoolOp

`struct` · `ruff_python_ast::comparable::ExprBoolOp`

```rust
struct ExprBoolOp<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprBytesLiteral

`struct` · `ruff_python_ast::comparable::ExprBytesLiteral`

```rust
struct ExprBytesLiteral<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprCall

`struct` · `ruff_python_ast::comparable::ExprCall`

```rust
struct ExprCall<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprCompare

`struct` · `ruff_python_ast::comparable::ExprCompare`

```rust
struct ExprCompare<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprDict

`struct` · `ruff_python_ast::comparable::ExprDict`

```rust
struct ExprDict<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprDictComp

`struct` · `ruff_python_ast::comparable::ExprDictComp`

```rust
struct ExprDictComp<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprFString

`struct` · `ruff_python_ast::comparable::ExprFString`

```rust
struct ExprFString<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprGenerator

`struct` · `ruff_python_ast::comparable::ExprGenerator`

```rust
struct ExprGenerator<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprIf

`struct` · `ruff_python_ast::comparable::ExprIf`

```rust
struct ExprIf<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprInterpolatedElement

`struct` · `ruff_python_ast::comparable::ExprInterpolatedElement`

```rust
struct ExprInterpolatedElement<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprIpyEscapeCommand

`struct` · `ruff_python_ast::comparable::ExprIpyEscapeCommand`

```rust
struct ExprIpyEscapeCommand<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprLambda

`struct` · `ruff_python_ast::comparable::ExprLambda`

```rust
struct ExprLambda<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprList

`struct` · `ruff_python_ast::comparable::ExprList`

```rust
struct ExprList<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprListComp

`struct` · `ruff_python_ast::comparable::ExprListComp`

```rust
struct ExprListComp<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprName

`struct` · `ruff_python_ast::comparable::ExprName`

```rust
struct ExprName<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprNamed

`struct` · `ruff_python_ast::comparable::ExprNamed`

```rust
struct ExprNamed<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprNumberLiteral

`struct` · `ruff_python_ast::comparable::ExprNumberLiteral`

```rust
struct ExprNumberLiteral<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprSet

`struct` · `ruff_python_ast::comparable::ExprSet`

```rust
struct ExprSet<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprSetComp

`struct` · `ruff_python_ast::comparable::ExprSetComp`

```rust
struct ExprSetComp<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprSlice

`struct` · `ruff_python_ast::comparable::ExprSlice`

```rust
struct ExprSlice<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprStarred

`struct` · `ruff_python_ast::comparable::ExprStarred`

```rust
struct ExprStarred<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprStringLiteral

`struct` · `ruff_python_ast::comparable::ExprStringLiteral`

```rust
struct ExprStringLiteral<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprSubscript

`struct` · `ruff_python_ast::comparable::ExprSubscript`

```rust
struct ExprSubscript<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprTString

`struct` · `ruff_python_ast::comparable::ExprTString`

```rust
struct ExprTString<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprTuple

`struct` · `ruff_python_ast::comparable::ExprTuple`

```rust
struct ExprTuple<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprUnaryOp

`struct` · `ruff_python_ast::comparable::ExprUnaryOp`

```rust
struct ExprUnaryOp<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprYield

`struct` · `ruff_python_ast::comparable::ExprYield`

```rust
struct ExprYield<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## ExprYieldFrom

`struct` · `ruff_python_ast::comparable::ExprYieldFrom`

```rust
struct ExprYieldFrom<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## HashableExpr

`struct` · `ruff_python_ast::comparable::HashableExpr`

```rust
struct HashableExpr<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(expr: &'a Expr) -> Self
```

Wrapper around [`Expr`] that implements [`Hash`] and [`PartialEq`] according to Python
semantics:

> Values that compare equal (such as 1, 1.0, and True) can be used interchangeably to index the
> same dictionary entry.

For example, considers `True`, `1`, and `1.0` to be equal, as they hash to the same value
in Python, along with `False`, `0`, and `0.0`.

See: <https://docs.python.org/3/library/stdtypes.html#mapping-types-dict>

---

## InterpolatedElement

`struct` · `ruff_python_ast::comparable::InterpolatedElement`

```rust
struct InterpolatedElement<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(interpolated_element: &'a ast::InterpolatedElement) -> Self
```

---

## PatternMatchAs

`struct` · `ruff_python_ast::comparable::PatternMatchAs`

```rust
struct PatternMatchAs<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PatternMatchClass

`struct` · `ruff_python_ast::comparable::PatternMatchClass`

```rust
struct PatternMatchClass<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PatternMatchMapping

`struct` · `ruff_python_ast::comparable::PatternMatchMapping`

```rust
struct PatternMatchMapping<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PatternMatchOr

`struct` · `ruff_python_ast::comparable::PatternMatchOr`

```rust
struct PatternMatchOr<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PatternMatchSequence

`struct` · `ruff_python_ast::comparable::PatternMatchSequence`

```rust
struct PatternMatchSequence<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PatternMatchSingleton

`struct` · `ruff_python_ast::comparable::PatternMatchSingleton`

```rust
struct PatternMatchSingleton
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PatternMatchStar

`struct` · `ruff_python_ast::comparable::PatternMatchStar`

```rust
struct PatternMatchStar<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## PatternMatchValue

`struct` · `ruff_python_ast::comparable::PatternMatchValue`

```rust
struct PatternMatchValue<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtAnnAssign

`struct` · `ruff_python_ast::comparable::StmtAnnAssign`

```rust
struct StmtAnnAssign<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtAssert

`struct` · `ruff_python_ast::comparable::StmtAssert`

```rust
struct StmtAssert<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtAssign

`struct` · `ruff_python_ast::comparable::StmtAssign`

```rust
struct StmtAssign<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtAugAssign

`struct` · `ruff_python_ast::comparable::StmtAugAssign`

```rust
struct StmtAugAssign<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtClassDef

`struct` · `ruff_python_ast::comparable::StmtClassDef`

```rust
struct StmtClassDef<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtDelete

`struct` · `ruff_python_ast::comparable::StmtDelete`

```rust
struct StmtDelete<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtExpr

`struct` · `ruff_python_ast::comparable::StmtExpr`

```rust
struct StmtExpr<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtFor

`struct` · `ruff_python_ast::comparable::StmtFor`

```rust
struct StmtFor<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtFunctionDef

`struct` · `ruff_python_ast::comparable::StmtFunctionDef`

```rust
struct StmtFunctionDef<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtGlobal

`struct` · `ruff_python_ast::comparable::StmtGlobal`

```rust
struct StmtGlobal<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtIf

`struct` · `ruff_python_ast::comparable::StmtIf`

```rust
struct StmtIf<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtImport

`struct` · `ruff_python_ast::comparable::StmtImport`

```rust
struct StmtImport<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtImportFrom

`struct` · `ruff_python_ast::comparable::StmtImportFrom`

```rust
struct StmtImportFrom<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtIpyEscapeCommand

`struct` · `ruff_python_ast::comparable::StmtIpyEscapeCommand`

```rust
struct StmtIpyEscapeCommand<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtMatch

`struct` · `ruff_python_ast::comparable::StmtMatch`

```rust
struct StmtMatch<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtNonlocal

`struct` · `ruff_python_ast::comparable::StmtNonlocal`

```rust
struct StmtNonlocal<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtRaise

`struct` · `ruff_python_ast::comparable::StmtRaise`

```rust
struct StmtRaise<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtReturn

`struct` · `ruff_python_ast::comparable::StmtReturn`

```rust
struct StmtReturn<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtTry

`struct` · `ruff_python_ast::comparable::StmtTry`

```rust
struct StmtTry<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtTypeAlias

`struct` · `ruff_python_ast::comparable::StmtTypeAlias`

```rust
struct StmtTypeAlias<'a>
```

**Fields**: `name`, `type_params`, `value`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtWhile

`struct` · `ruff_python_ast::comparable::StmtWhile`

```rust
struct StmtWhile<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## StmtWith

`struct` · `ruff_python_ast::comparable::StmtWith`

```rust
struct StmtWith<'a>
```

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## TypeParamParamSpec

`struct` · `ruff_python_ast::comparable::TypeParamParamSpec`

```rust
struct TypeParamParamSpec<'a>
```

**Fields**: `name`, `default`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## TypeParamTypeVar

`struct` · `ruff_python_ast::comparable::TypeParamTypeVar`

```rust
struct TypeParamTypeVar<'a>
```

**Fields**: `name`, `bound`, `default`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## TypeParamTypeVarTuple

`struct` · `ruff_python_ast::comparable::TypeParamTypeVarTuple`

```rust
struct TypeParamTypeVarTuple<'a>
```

**Fields**: `name`, `default`

**Derives**: Debug, Eq, Hash, PartialEq, StructuralPartialEq

---
