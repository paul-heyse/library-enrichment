# `sqlparser::tokenizer::TokenWithSpan`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.TokenWithSpan.json).

<a id="op-8f18f1407940a3467d080b83"></a>
## TokenWithSpan

`struct` · `sqlparser::tokenizer::TokenWithSpan` · sqlparser 0.62.0

```rust
struct TokenWithSpan
```

Source: `src/tokenizer.rs:741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [Token](../operations/sqlparser.tokenizer.Token.md#op-8dd5a6484792ec08e2eb8600) with [Span](../operations/sqlparser.tokenizer.Span.md#op-c744756cf28c8276d407a53a) attached to it

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

<a id="op-11b79ed56a8011f517fc34bf"></a>
## at

`function` · `sqlparser::tokenizer::TokenWithSpan::at` · sqlparser 0.62.0

```rust
fn at(token: Token, start: Location, end: Location) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 1], "end": [768, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Wrap a token with a location from `start` to `end`

<a id="op-b1b873c83b759c33badaf398"></a>
## clone

`function` · `sqlparser::tokenizer::TokenWithSpan::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TokenWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 17], "end": [737, 22], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tokenizer.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c932915a6dd8dca356319896"></a>
## cmp

`function` · `sqlparser::tokenizer::TokenWithSpan::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TokenWithSpan) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 30], "end": [737, 33], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/tokenizer.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-910b9eea3151696a8ba3fefb"></a>
## deserialize

`function` · `sqlparser::tokenizer::TokenWithSpan::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [738, 49], "end": [738, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/tokenizer.rs:738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5227c94e0377fd0d5323ec34"></a>
## eq

`function` · `sqlparser::tokenizer::TokenWithSpan::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Token) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [770, 1], "end": [774, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Token", "path": "Token"}}}], "constraints": []}}, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:771`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9126654f50880f95a7440fed"></a>
## eq

`function` · `sqlparser::tokenizer::TokenWithSpan::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TokenWithSpan) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 51], "end": [737, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-087195162157be87ada2c180"></a>
## fmt

`function` · `sqlparser::tokenizer::TokenWithSpan::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [782, 1], "end": [786, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tokenizer.rs:783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17c98ee2ef4aee7a2225227c"></a>
## fmt

`function` · `sqlparser::tokenizer::TokenWithSpan::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 10], "end": [737, 15], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tokenizer.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58dcd0eb46f227a9af22a724"></a>
## from

`function` · `sqlparser::tokenizer::TokenWithSpan::from` · sqlparser 0.62.0

```rust
fn from(value: AttachedToken) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "crate::tokenizer::TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [136, 2], "filename": "src/ast/helpers/attached_token.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::attached_token::AttachedToken", "path": "AttachedToken"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/helpers/attached_token.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dab43cd0ce4691bfa2a2ffe3"></a>
## hash

`function` · `sqlparser::tokenizer::TokenWithSpan::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 24], "end": [737, 28], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/tokenizer.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9486a2d1fda8f118203f7cd"></a>
## new

`function` · `sqlparser::tokenizer::TokenWithSpan::new` · sqlparser 0.62.0

```rust
fn new(token: Token, span: Span) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 1], "end": [768, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new [`TokenWithSpan`](../operations/sqlparser.tokenizer.TokenWithSpan.md#op-8f18f1407940a3467d080b83) from a [`Token`](../operations/sqlparser.tokenizer.Token.md#op-8dd5a6484792ec08e2eb8600) and a [`Span`](../operations/sqlparser.tokenizer.Span.md#op-c744756cf28c8276d407a53a)

<a id="op-7a96b8ab2c169cb457551bc9"></a>
## new_eof

`function` · `sqlparser::tokenizer::TokenWithSpan::new_eof` · sqlparser 0.62.0

```rust
fn new_eof() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 1], "end": [768, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:765`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return an EOF token with no location

<a id="op-ae0bf10207459108c0d95c9f"></a>
## partial_cmp

`function` · `sqlparser::tokenizer::TokenWithSpan::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TokenWithSpan) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 35], "end": [737, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/tokenizer.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c365e5f7f76a163627e12470"></a>
## serialize

`function` · `sqlparser::tokenizer::TokenWithSpan::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [738, 38], "end": [738, 47], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/tokenizer.rs:738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-241738537f16eed753beec5a"></a>
## span

`struct_field` · `sqlparser::tokenizer::TokenWithSpan::span` · sqlparser 0.62.0

```rust
span: Span
```

Source: `src/tokenizer.rs:745`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The span covering the token in the input.

<a id="op-5bef0046e80d997d8ab0998e"></a>
## span

`function` · `sqlparser::tokenizer::TokenWithSpan::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "crate::tokenizer::TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [107, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8394da804d2c0da446201ca"></a>
## token

`struct_field` · `sqlparser::tokenizer::TokenWithSpan::token` · sqlparser 0.62.0

```rust
token: Token
```

Source: `src/tokenizer.rs:743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The token value.

<a id="op-a3b51b3c579f705eb2d8342a"></a>
## visit

`function` · `sqlparser::tokenizer::TokenWithSpan::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [739, 47], "end": [739, 55], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/tokenizer.rs:739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5de80164fb954e5d6c541fa"></a>
## visit

`function` · `sqlparser::tokenizer::TokenWithSpan::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [739, 40], "end": [739, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/tokenizer.rs:739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3eea51c9e69e187fa85533a3"></a>
## wrap

`function` · `sqlparser::tokenizer::TokenWithSpan::wrap` · sqlparser 0.62.0

```rust
fn wrap(token: Token) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::TokenWithSpan", "path": "TokenWithSpan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 1], "end": [768, 2], "filename": "src/tokenizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/tokenizer.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Wrap a token with an empty span
