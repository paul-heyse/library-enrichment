# `ra_ap_syntax::ast::generated::tokens`

Crate `ra_ap_syntax` · 10 public items · structured records in [`model/ra_ap_syntax.ast.generated.tokens.json`](../model/ra_ap_syntax.ast.generated.tokens.json)

## Byte

`struct` · `ra_ap_syntax::ast::generated::tokens::Byte`

Also reachable as `ra_ap_syntax::ast::Byte`

```rust
struct Byte
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn value(&self) -> Result<u8, EscapeError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

---

## ByteString

`struct` · `ra_ap_syntax::ast::generated::tokens::ByteString`

Also reachable as `ra_ap_syntax::ast::ByteString`

```rust
struct ByteString
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`, `ra_ap_syntax::ast::token_ext::IsString`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn value(&self) -> Result<Cow<'_, [u8]>, EscapeError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

**via `ra_ap_syntax::ast::token_ext::IsString`**

```rust
fn raw_prefix(&self) -> &'static str
fn unescape(&self, s: &str, callback: impl FnMut(Range<usize>, Result<char, EscapeError>))
```

---

## CString

`struct` · `ra_ap_syntax::ast::generated::tokens::CString`

Also reachable as `ra_ap_syntax::ast::CString`

```rust
struct CString
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`, `ra_ap_syntax::ast::token_ext::IsString`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn value(&self) -> Result<Cow<'_, [u8]>, EscapeError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

**via `ra_ap_syntax::ast::token_ext::IsString`**

```rust
fn raw_prefix(&self) -> &'static str
fn unescape(&self, s: &str, callback: impl FnMut(Range<usize>, Result<char, EscapeError>))
```

---

## Char

`struct` · `ra_ap_syntax::ast::generated::tokens::Char`

Also reachable as `ra_ap_syntax::ast::Char`

```rust
struct Char
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn value(&self) -> Result<char, EscapeError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

---

## Comment

`struct` · `ra_ap_syntax::ast::generated::tokens::Comment`

Also reachable as `ra_ap_syntax::ast::Comment`

```rust
struct Comment
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (2)

```rust
fn shape(&self) -> CommentShape
fn text_without_markers(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

---

## FloatNumber

`struct` · `ra_ap_syntax::ast::generated::tokens::FloatNumber`

Also reachable as `ra_ap_syntax::ast::FloatNumber`

```rust
struct FloatNumber
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (3)

```rust
fn split_into_parts(&self) -> (&str, &str)
fn suffix(&self) -> Option<&str>
fn value_string(&self) -> String
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

---

## Ident

`struct` · `ra_ap_syntax::ast::generated::tokens::Ident`

Also reachable as `ra_ap_syntax::ast::Ident`

```rust
struct Ident
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

---

## IntNumber

`struct` · `ra_ap_syntax::ast::generated::tokens::IntNumber`

Also reachable as `ra_ap_syntax::ast::IntNumber`

```rust
struct IntNumber
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (5)

```rust
fn radix(&self) -> Radix
fn split_into_parts(&self) -> (&str, &str, &str)
fn suffix(&self) -> Option<&str>
fn value(&self) -> Result<u128, ParseIntError>
fn value_string(&self) -> String
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

---

## String

`struct` · `ra_ap_syntax::ast::generated::tokens::String`

Also reachable as `ra_ap_syntax::ast::String`

```rust
struct String
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`, `ra_ap_syntax::ast::token_ext::IsString`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn value(&self) -> Result<Cow<'_, str>, EscapeError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

**via `ra_ap_syntax::ast::token_ext::IsString`**

```rust
fn raw_prefix(&self) -> &'static str
fn unescape(&self, s: &str, cb: impl FnMut(Range<usize>, Result<char, EscapeError>))
```

---

## Whitespace

`struct` · `ra_ap_syntax::ast::generated::tokens::Whitespace`

Also reachable as `ra_ap_syntax::ast::Whitespace`

```rust
struct Whitespace
```

**Implements**: `core::fmt::Display`, `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (1)

```rust
fn spans_multiple_lines(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

---
