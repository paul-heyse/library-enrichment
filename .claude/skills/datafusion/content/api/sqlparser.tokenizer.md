# `sqlparser::tokenizer`

Crate `sqlparser` · 9 public items · structured records in [`model/sqlparser.tokenizer.json`](../model/sqlparser.tokenizer.json)

## Token

`enum` · `sqlparser::tokenizer::Token`

```rust
enum Token
```

**Variants**: `EOF`, `Word`, `Number`, `Char`, `SingleQuotedString`, `DoubleQuotedString`, `TripleSingleQuotedString`, `TripleDoubleQuotedString`, `DollarQuotedString`, `SingleQuotedByteStringLiteral`, `DoubleQuotedByteStringLiteral`, `TripleSingleQuotedByteStringLiteral`, `TripleDoubleQuotedByteStringLiteral`, `SingleQuotedRawStringLiteral`, `DoubleQuotedRawStringLiteral`, `TripleSingleQuotedRawStringLiteral`, `TripleDoubleQuotedRawStringLiteral`, `NationalStringLiteral`, `QuoteDelimitedStringLiteral`, `NationalQuoteDelimitedStringLiteral`, `EscapedStringLiteral`, `UnicodeStringLiteral`, `HexStringLiteral`, `Comma`, `Whitespace`, `DoubleEq`, `Eq`, `Neq`, `Lt`, `Gt`, `LtEq`, `GtEq`, `Spaceship`, `Plus`, `Minus`, `Mul`, `Div`, `DuckIntDiv`, `Mod`, `StringConcat`, `LParen`, `RParen`, `Period`, `Colon`, `DoubleColon`, `Assignment`, `SemiColon`, `Backslash`, `LBracket`, `RBracket`, `Ampersand`, `Pipe`, `Caret`, `LBrace`, `RBrace`, `RArrow`, `Sharp`, `DoubleSharp`, `Tilde`, `TildeAsterisk`, `ExclamationMarkTilde`, `ExclamationMarkTildeAsterisk`, `DoubleTilde`, `DoubleTildeAsterisk`, `ExclamationMarkDoubleTilde`, `ExclamationMarkDoubleTildeAsterisk`, `ShiftLeft`, `ShiftRight`, `Overlap`, `ExclamationMark`, `DoubleExclamationMark`, `AtSign`, `CaretAt`, `PGSquareRoot`, `PGCubeRoot`, `Placeholder`, `Arrow`, `LongArrow`, `HashArrow`, `AtDashAt`, `QuestionMarkDash`, `AmpersandLeftAngleBracket`, `AmpersandRightAngleBracket`, `AmpersandLeftAngleBracketVerticalBar`, `VerticalBarAmpersandRightAngleBracket`, `TwoWayArrow`, `LeftAngleBracketCaret`, `RightAngleBracketCaret`, `QuestionMarkSharp`, `QuestionMarkDashVerticalBar`, `QuestionMarkDoubleVerticalBar`, `TildeEqual`, `ShiftLeftVerticalBar`, `VerticalBarShiftRight`, `VerticalBarRightAngleBracket`, `HashLongArrow`, `AtArrow`, `ArrowAt`, `HashMinus`, `AtQuestion`, `AtAt`, `Question`, `QuestionAnd`, `QuestionPipe`, `CustomBinaryOperator`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn make_keyword(keyword: &str) -> Self
fn make_word(word: &str, quote_style: Option<char>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

SQL Token enumeration

---

## Whitespace

`enum` · `sqlparser::tokenizer::Whitespace`

```rust
enum Whitespace
```

**Variants**: `Space`, `Newline`, `Tab`, `SingleLineComment`, `MultiLineComment`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Represents whitespace in the input: spaces, newlines, tabs and comments.

---

## Location

`struct` · `sqlparser::tokenizer::Location`

```rust
struct Location
```

**Fields**: `line`, `column`

**Implements**: `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn empty() -> Self
fn new(line: u64, column: u64) -> Self
fn of(line: u64, column: u64) -> Self
fn span_to(self, end: Self) -> Span
```

**via `core::convert::From`**

```rust
fn from((line, column): (u64, u64)) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Location in input string

# Create an "empty" (unknown) `Location`
```
# use sqlparser::tokenizer::Location;
let location = Location::empty();
```

# Create a `Location` from a line and column
```
# use sqlparser::tokenizer::Location;
let location = Location::new(1, 1);
```

# Create a `Location` from a pair
```
# use sqlparser::tokenizer::Location;
let location = Location::from((1, 1));
```

---

## Span

`struct` · `sqlparser::tokenizer::Span`

```rust
struct Span
```

**Fields**: `start`, `end`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (5)

```rust
const fn empty() -> Span
fn new(start: Location, end: Location) -> Span
fn union(&self, other: &Span) -> Span
fn union_iter<I: IntoIterator<Item = Span>>(iter: I) -> Span
fn union_opt(&self, other: &Option<Span>) -> Span
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

A span represents a linear portion of the input string (start, end)

See [Spanned](crate::ast::Spanned) for more information.

---

## TokenWithSpan

`struct` · `sqlparser::tokenizer::TokenWithSpan`

```rust
struct TokenWithSpan
```

**Fields**: `token`, `span`

**Implements**: `core::convert::From`, `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::spans::Spanned`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn at(token: Token, start: Location, end: Location) -> Self
fn new(token: Token, span: Span) -> Self
fn new_eof() -> Self
fn wrap(token: Token) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: AttachedToken) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

A [Token] with [Span] attached to it

This is used to track the location of a token in the input string

# Examples
```
# use sqlparser::tokenizer::{Location, Span, Token, TokenWithSpan};
// commas @ line 1, column 10
let tok1 = TokenWithSpan::new(
  Token::Comma,
  Span::new(Location::new(1, 10), Location::new(1, 11)),
);
assert_eq!(tok1, Token::Comma); // can compare the token

// commas @ line 2, column 20
let tok2 = TokenWithSpan::new(
  Token::Comma,
  Span::new(Location::new(2, 20), Location::new(2, 21)),
);
// same token but different locations are not equal
assert_ne!(tok1, tok2);
```
A `Token` together with its `Span` (location in the source).

---

## Tokenizer

`struct` · `sqlparser::tokenizer::Tokenizer`

```rust
struct Tokenizer<'a>
```

**Methods** (6)

```rust
fn new(dialect: &'a dyn Dialect, query: &'a str) -> Self
fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError>
fn tokenize_with_location(&mut self) -> Result<Vec<TokenWithSpan>, TokenizerError>
fn tokenize_with_location_into_buf(&mut self, buf: &mut Vec<TokenWithSpan>) -> Result<(), TokenizerError>
fn tokenize_with_location_into_buf_with_mapper(&mut self, buf: &mut Vec<TokenWithSpan>, mapper: impl FnMut(TokenWithSpan) -> TokenWithSpan) -> Result<(), TokenizerError>
fn with_unescape(self, unescape: bool) -> Self
```

SQL Tokenizer

---

## TokenizerError

`struct` · `sqlparser::tokenizer::TokenizerError`

```rust
struct TokenizerError
```

**Fields**: `message`, `location`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

An error reported by the tokenizer, with a human-readable `message` and a `location`.

---

## Word

`struct` · `sqlparser::tokenizer::Word`

```rust
struct Word
```

**Fields**: `value`, `quote_style`, `keyword`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn into_ident(self, span: Span) -> Ident
fn to_ident(&self, span: Span) -> Ident
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

A keyword (like SELECT) or an optionally quoted SQL identifier

---

## TokenWithLocation

`type_alias` · `sqlparser::tokenizer::TokenWithLocation`

> **Deprecated** — since 0.53.0: please use `TokenWithSpan` instead

```rust
type TokenWithLocation = TokenWithSpan
```

Backwards compatibility struct for [`TokenWithSpan`]

---
