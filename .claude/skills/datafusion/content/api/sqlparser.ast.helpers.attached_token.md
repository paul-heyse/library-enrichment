# `sqlparser::ast::helpers::attached_token`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.ast.helpers.attached_token.json`](../model/sqlparser.ast.helpers.attached_token.json)

## AttachedToken

`struct` · `sqlparser::ast::helpers::attached_token::AttachedToken`

```rust
struct AttachedToken
```

**Implements**: `core::convert::From`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (1)

```rust
fn empty() -> Self
```

**via `core::convert::From`**

```rust
fn from(value: TokenWithSpan) -> Self
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

A wrapper over [`TokenWithSpan`]s that ignores the token and source
location in comparisons and hashing.

This type is used when the token and location is not relevant for semantics,
but is still needed for accurate source location tracking, for example, in
the nodes in the [ast](crate::ast) module.

Note: **All** `AttachedTokens` are equal.

# Examples

Same token, different location are equal
```
# use sqlparser::ast::helpers::attached_token::AttachedToken;
# use sqlparser::tokenizer::{Location, Span, Token, TokenWithLocation};
// commas @ line 1, column 10
let tok1 = TokenWithLocation::new(
  Token::Comma,
  Span::new(Location::new(1, 10), Location::new(1, 11)),
);
// commas @ line 2, column 20
let tok2 = TokenWithLocation::new(
  Token::Comma,
  Span::new(Location::new(2, 20), Location::new(2, 21)),
);

assert_ne!(tok1, tok2); // token with locations are *not* equal
assert_eq!(AttachedToken(tok1), AttachedToken(tok2)); // attached tokens are
```

Different token, different location are equal 🤯

```
# use sqlparser::ast::helpers::attached_token::AttachedToken;
# use sqlparser::tokenizer::{Location, Span, Token, TokenWithLocation};
// commas @ line 1, column 10
let tok1 = TokenWithLocation::new(
  Token::Comma,
  Span::new(Location::new(1, 10), Location::new(1, 11)),
);
// period @ line 2, column 20
let tok2 = TokenWithLocation::new(
 Token::Period,
  Span::new(Location::new(2, 10), Location::new(2, 21)),
);

assert_ne!(tok1, tok2); // token with locations are *not* equal
assert_eq!(AttachedToken(tok1), AttachedToken(tok2)); // attached tokens are
```
// period @ line 2, column 20

---
