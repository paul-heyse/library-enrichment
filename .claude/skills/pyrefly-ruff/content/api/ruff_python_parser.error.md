# `ruff_python_parser::error`

Crate `ruff_python_parser` · 12 public items · structured records in [`model/ruff_python_parser.error.json`](../model/ruff_python_parser.error.json)

## ComprehensionUnpackingKind

`enum` · `ruff_python_parser::error::ComprehensionUnpackingKind`

```rust
enum ComprehensionUnpackingKind
```

**Variants**: `IterableInList`, `IterableInSet`, `IterableInGenerator`, `DictInDict`

The type of PEP 798 unpacking-comprehension error for
[`UnsupportedSyntaxErrorKind::UnpackingInComprehension`].

---

## FStringKind

`enum` · `ruff_python_parser::error::FStringKind`

```rust
enum FStringKind
```

**Variants**: `Backslash`, `Comment`, `LineBreak`, `NestedQuote`

The type of PEP 701 f-string error for [`UnsupportedSyntaxErrorKind::Pep701FString`].

---

## InterpolatedStringErrorType

`enum` · `ruff_python_parser::error::InterpolatedStringErrorType`

Also reachable as `ruff_python_parser::InterpolatedStringErrorType`

```rust
enum InterpolatedStringErrorType
```

**Variants**: `UnclosedLbrace`, `InvalidConversionFlag`, `SingleRbrace`, `UnterminatedString`, `UnterminatedTripleQuotedString`, `LambdaWithoutParentheses`, `ConversionFlagNotImmediatelyAfterExclamation`, `NewlineInFormatSpec`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Represents the different types of errors that can occur during parsing of an f-string or t-string.

---

## LexicalErrorType

`enum` · `ruff_python_parser::error::LexicalErrorType`

Also reachable as `ruff_python_parser::LexicalErrorType`

```rust
enum LexicalErrorType
```

**Variants**: `UnclosedStringError`, `UnicodeError`, `MissingUnicodeLbrace`, `MissingUnicodeRbrace`, `IndentationError`, `UnrecognizedToken`, `FStringError`, `TStringError`, `InvalidByteLiteral`, `LineContinuationError`, `Eof`, `OtherError`

**Implements**: `core::error::Error`, `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Represents the different types of errors that can occur during lexing.

---

## ParseErrorType

`enum` · `ruff_python_parser::error::ParseErrorType`

Also reachable as `ruff_python_parser::ParseErrorType`

```rust
enum ParseErrorType
```

**Variants**: `OtherError`, `StringAnnotationError`, `EmptySlice`, `EmptyGlobalNames`, `EmptyNonlocalNames`, `EmptyDeleteTargets`, `EmptyImportNames`, `EmptyTypeParams`, `UnparenthesizedNamedExpression`, `UnparenthesizedTupleExpression`, `UnparenthesizedGeneratorExpression`, `InvalidLambdaExpressionUsage`, `InvalidYieldExpressionUsage`, `InvalidStarredExpressionUsage`, `InvalidStarPatternUsage`, `InvalidMatchPatternTarget`, `ParamAfterVarKeywordParam`, `NonDefaultParamAfterDefaultParam`, `VarParameterWithDefault`, `InvalidAssignmentTarget`, `InvalidNamedAssignmentTarget`, `InvalidAnnotatedAssignmentTarget`, `InvalidAugmentedAssignmentTarget`, `InvalidDeleteTarget`, `PositionalAfterKeywordArgument`, `PositionalAfterKeywordUnpacking`, `InvalidArgumentUnpackingOrder`, `IterableUnpackingInComprehension`, `SimpleStatementsOnSameLine`, `SimpleAndCompoundStatementOnSameLine`, `ExpectedKeywordParam`, `ExpectedRealNumber`, `ExpectedImaginaryNumber`, `ExpectedExpression`, `ExpectedToken`, `UnexpectedIndentation`, `UnexpectedTokenAfterAsync`, `UnexpectedIpythonEscapeCommand`, `UnexpectedExpressionToken`, `FStringError`, `TStringError`, `Lexical`

**Implements**: `core::error::Error`, `core::fmt::Display`, `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

Represents the different types of errors that can occur during parsing.

---

## RelaxedDecoratorError

`enum` · `ruff_python_parser::error::RelaxedDecoratorError`

```rust
enum RelaxedDecoratorError
```

**Variants**: `CallExpression`, `Other`

---

## StarTupleKind

`enum` · `ruff_python_parser::error::StarTupleKind`

```rust
enum StarTupleKind
```

**Variants**: `Return`, `Yield`

The type of tuple unpacking for [`UnsupportedSyntaxErrorKind::StarTuple`].

---

## UnparenthesizedNamedExprKind

`enum` · `ruff_python_parser::error::UnparenthesizedNamedExprKind`

```rust
enum UnparenthesizedNamedExprKind
```

**Variants**: `SequenceIndex`, `SetLiteral`, `SetComprehension`

---

## UnsupportedSyntaxErrorKind

`enum` · `ruff_python_parser::error::UnsupportedSyntaxErrorKind`

Also reachable as `ruff_python_parser::UnsupportedSyntaxErrorKind`

```rust
enum UnsupportedSyntaxErrorKind
```

**Variants**: `Match`, `Walrus`, `ExceptStar`, `UnparenthesizedNamedExpr`, `ParenthesizedKeywordArgumentName`, `StarTuple`, `RelaxedDecorator`, `PositionalOnlyParameter`, `TypeParameterList`, `LazyImportStatement`, `TypeAliasStatement`, `TypeParamDefault`, `Pep701FString`, `ParenthesizedContextManager`, `StarExpressionInIndex`, `StarAnnotation`, `UnpackingInComprehension`, `UnparenthesizedUnpackInFor`, `UnparenthesizedExceptionTypes`, `TemplateStrings`, `UnaryPlusMatchPattern`

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

---

## LexicalError

`struct` · `ruff_python_parser::error::LexicalError`

```rust
struct LexicalError
```

Represents an error that occur during lexing and are
returned by the `parse_*` functions in the iterator in the
[lexer] implementation.

[lexer]: crate::lexer

---

## ParseError

`struct` · `ruff_python_parser::error::ParseError`

Also reachable as `ruff_python_parser::ParseError`

```rust
struct ParseError
```

**Fields**: `error`, `location`

**Implements**: `core::error::Error`, `core::fmt::Display`, `core::ops::deref::Deref`, `get_size2::GetSize`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn error(self) -> ParseErrorType
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Represents represent errors that occur during parsing and are
returned by the `parse_*` functions.

---

## UnsupportedSyntaxError

`struct` · `ruff_python_parser::error::UnsupportedSyntaxError`

Also reachable as `ruff_python_parser::UnsupportedSyntaxError`

```rust
struct UnsupportedSyntaxError
```

**Fields**: `kind`, `range`, `target_version`

**Implements**: `core::fmt::Display`, `get_size2::GetSize`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Represents a version-related syntax error detected during parsing.

An example of a version-related error is the use of a `match` statement before Python 3.10, when
it was first introduced. See [`UnsupportedSyntaxErrorKind`] for other kinds of errors.

---
