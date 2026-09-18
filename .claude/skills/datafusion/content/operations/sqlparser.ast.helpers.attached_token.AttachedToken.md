# `sqlparser::ast::helpers::attached_token::AttachedToken`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.attached_token.AttachedToken.json).

<a id="op-201bf85729211851d2722737"></a>
## AttachedToken

`struct` · `sqlparser::ast::helpers::attached_token::AttachedToken` · sqlparser 0.62.0

```rust
struct AttachedToken
```

Source: `src/ast/helpers/attached_token.rs:83`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A wrapper over [`TokenWithSpan`](../operations/sqlparser.tokenizer.TokenWithSpan.md#op-8f18f1407940a3467d080b83)s that ignores the token and source
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

<a id="op-b5b417fc183166528e0b6091"></a>
## 0

`struct_field` · `sqlparser::ast::helpers::attached_token::AttachedToken::0` · sqlparser 0.62.0

```rust
0: tokenizer::TokenWithSpan
```

Source: `src/ast/helpers/attached_token.rs:83`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c92261df94ff78be4f140f34"></a>
## clone

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AttachedToken
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 10], "end": [80, 15], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/attached_token.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f185009fa46bc36c9f980c43"></a>
## cmp

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, _: &Self) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [118, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/attached_token.rs:115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ad3990a63464b07fdf3d227"></a>
## deserialize

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 49], "end": [81, 60], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/attached_token.rs:81`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b51ad9a27c811a5955097493"></a>
## empty

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::empty` · sqlparser 0.62.0

```rust
fn empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [90, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/attached_token.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return a new Empty AttachedToken

<a id="op-507f0db8acfde2e7f497674e"></a>
## eq

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::eq` · sqlparser 0.62.0

```rust
fn eq(&self, _: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [104, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/attached_token.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a252df07c4c962348dcfe9a4"></a>
## fmt

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [97, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/attached_token.rs:94`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-240fa861f90ac0746659b775"></a>
## from

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::from` · sqlparser 0.62.0

```rust
fn from(value: TokenWithSpan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [130, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/helpers/attached_token.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85b3053e125c983f504c8c31"></a>
## hash

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::hash` · sqlparser 0.62.0

```rust
fn hash<H: Hasher>(&self, _state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [124, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/attached_token.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4a026f80c4958e5191c4eb7"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [112, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/attached_token.rs:109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9bd1982cbaea6c1ebae7487"></a>
## serialize

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 38], "end": [81, 47], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/attached_token.rs:81`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71fe026b74da6a0333c16151"></a>
## visit

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 40], "end": [82, 45], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/attached_token.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd3617570cc35b989bfa638a"></a>
## visit

`function` · `sqlparser::ast::helpers::attached_token::AttachedToken::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 47], "end": [82, 55], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/attached_token.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
