# `ruff_python_ast::expression`

Crate `ruff_python_ast` · 4 public items · structured records in [`model/ruff_python_ast.expression.json`](../model/ruff_python_ast.expression.json)

## LiteralExpressionRef

`enum` · `ruff_python_ast::expression::LiteralExpressionRef`

Also reachable as `ruff_python_ast::LiteralExpressionRef`

```rust
enum LiteralExpressionRef<'a>
```

**Variants**: `StringLiteral`, `BytesLiteral`, `NumberLiteral`, `BooleanLiteral`, `NoneLiteral`, `EllipsisLiteral`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (31)

```rust
fn as_boolean_literal(&self) -> Option<&&'a ast::ExprBooleanLiteral>
fn as_bytes_literal(&self) -> Option<&&'a ast::ExprBytesLiteral>
fn as_ellipsis_literal(&self) -> Option<&&'a ast::ExprEllipsisLiteral>
fn as_mut_boolean_literal(&mut self) -> Option<&mut &'a ast::ExprBooleanLiteral>
fn as_mut_bytes_literal(&mut self) -> Option<&mut &'a ast::ExprBytesLiteral>
fn as_mut_ellipsis_literal(&mut self) -> Option<&mut &'a ast::ExprEllipsisLiteral>
fn as_mut_none_literal(&mut self) -> Option<&mut &'a ast::ExprNoneLiteral>
fn as_mut_number_literal(&mut self) -> Option<&mut &'a ast::ExprNumberLiteral>
fn as_mut_string_literal(&mut self) -> Option<&mut &'a ast::ExprStringLiteral>
fn as_none_literal(&self) -> Option<&&'a ast::ExprNoneLiteral>
fn as_number_literal(&self) -> Option<&&'a ast::ExprNumberLiteral>
fn as_string_literal(&self) -> Option<&&'a ast::ExprStringLiteral>
fn boolean_literal(self) -> Option<&'a ast::ExprBooleanLiteral>
fn bytes_literal(self) -> Option<&'a ast::ExprBytesLiteral>
fn ellipsis_literal(self) -> Option<&'a ast::ExprEllipsisLiteral>
fn expect_boolean_literal(self) -> &'a ast::ExprBooleanLiteral where Self: ::std::fmt::Debug
fn expect_bytes_literal(self) -> &'a ast::ExprBytesLiteral where Self: ::std::fmt::Debug
fn expect_ellipsis_literal(self) -> &'a ast::ExprEllipsisLiteral where Self: ::std::fmt::Debug
fn expect_none_literal(self) -> &'a ast::ExprNoneLiteral where Self: ::std::fmt::Debug
fn expect_number_literal(self) -> &'a ast::ExprNumberLiteral where Self: ::std::fmt::Debug
fn expect_string_literal(self) -> &'a ast::ExprStringLiteral where Self: ::std::fmt::Debug
const fn is_boolean_literal(&self) -> bool
const fn is_bytes_literal(&self) -> bool
const fn is_ellipsis_literal(&self) -> bool
fn is_implicit_concatenated(&self) -> bool
const fn is_none_literal(&self) -> bool
const fn is_number_literal(&self) -> bool
const fn is_string_literal(&self) -> bool
fn none_literal(self) -> Option<&'a ast::ExprNoneLiteral>
fn number_literal(self) -> Option<&'a ast::ExprNumberLiteral>
fn string_literal(self) -> Option<&'a ast::ExprStringLiteral>
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Unowned pendant to all the literal variants of [`ast::Expr`] that stores a
reference instead of an owned value.

---

## StringLike

`enum` · `ruff_python_ast::expression::StringLike`

Also reachable as `ruff_python_ast::StringLike`

```rust
enum StringLike<'a>
```

**Variants**: `String`, `Bytes`, `FString`, `TString`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
const fn as_expression_ref(self) -> ExprRef<'a>
fn is_implicit_concatenated(self) -> bool
const fn is_interpolated_string(self) -> bool
fn parts(&self) -> StringLikePartIter<'a>
```

**via `core::convert::From`**

```rust
fn from(value: &'a ast::ExprStringLiteral) -> Self
fn from(value: &'a ast::ExprTString) -> Self
fn from(value: &'a ast::ExprFString) -> Self
fn from(value: &'a ast::ExprBytesLiteral) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &'a Expr) -> Result<Self, Self::Error>
fn try_from(value: AnyNodeRef<'a>) -> Result<Self, Self::Error>
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

An enum that holds a reference to a string-like expression from the AST. This includes string
literals, bytes literals, f-strings, and t-strings.

---

## StringLikePart

`enum` · `ruff_python_ast::expression::StringLikePart`

Also reachable as `ruff_python_ast::StringLikePart`

```rust
enum StringLikePart<'a>
```

**Variants**: `String`, `Bytes`, `FString`, `TString`

**Implements**: `core::convert::From`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
const fn as_string_literal(self) -> Option<&'a ast::StringLiteral>
fn content_range(self) -> TextRange
fn flags(&self) -> AnyStringFlags
const fn is_interpolated_string(self) -> bool
const fn is_string_literal(self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: &'a ast::StringLiteral) -> Self
fn from(value: &'a ast::TString) -> Self
fn from(value: &'a ast::FString) -> Self
fn from(value: &'a ast::BytesLiteral) -> Self
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

An enum that holds a reference to an individual part of a string-like expression.

---

## StringLikePartIter

`enum` · `ruff_python_ast::expression::StringLikePartIter`

Also reachable as `ruff_python_ast::StringLikePartIter`

```rust
enum StringLikePartIter<'a>
```

**Variants**: `String`, `Bytes`, `FString`, `TString`

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

An iterator over all the [`StringLikePart`] of a string-like expression.

This is created by the [`StringLike::parts`] method.

---
