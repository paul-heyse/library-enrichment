# `sqlparser::tokenizer::Word`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.Word.json).

<a id="op-4bfe123fe128658670006b17"></a>
## Word

`struct` · `sqlparser::tokenizer::Word` · sqlparser 0.62.0

```rust
struct Word
```

Source: `src/tokenizer.rs:458`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A keyword (like SELECT) or an optionally quoted SQL identifier

<a id="op-26af71c8c6d9621ed65cf006"></a>
## clone

`function` · `sqlparser::tokenizer::Word::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Word
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 17], "end": [455, 22], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tokenizer.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41ee7b1488b60b8ba2b3a86f"></a>
## cmp

`function` · `sqlparser::tokenizer::Word::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Word) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 51], "end": [455, 54], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/tokenizer.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30a0cee8f7e3cceb372f48f0"></a>
## deserialize

`function` · `sqlparser::tokenizer::Word::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 49], "end": [456, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/tokenizer.rs:456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-429fe7dc1cb8e8dc28caad28"></a>
## eq

`function` · `sqlparser::tokenizer::Word::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Word) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 24], "end": [455, 33], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b3aa0303da56fafb1762e5e"></a>
## fmt

`function` · `sqlparser::tokenizer::Word::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [481, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tokenizer.rs:472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9211531421341ba8a8de7148"></a>
## fmt

`function` · `sqlparser::tokenizer::Word::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 10], "end": [455, 15], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tokenizer.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55aa17889f48f1a4ed485a7c"></a>
## hash

`function` · `sqlparser::tokenizer::Word::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 56], "end": [455, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/tokenizer.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8ecdcd415c75587ebda39e1"></a>
## into_ident

`function` · `sqlparser::tokenizer::Word::into_ident` · sqlparser 0.62.0

```rust
fn into_ident(self, span: Span) -> Ident
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20575, 1], "end": [20600, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:20593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Convert this word into an [`Ident`](../operations/sqlparser.ast.Ident.md#op-84514077950174f9df2ced53) identifier, consuming the `Word`.

This avoids cloning the string value. If you need to keep the original
`Word`, use [`to_ident`](Self::to_ident) instead.

<a id="op-79d3c057b2a476dff8097f1a"></a>
## keyword

`struct_field` · `sqlparser::tokenizer::Word::keyword` · sqlparser 0.62.0

```rust
keyword: keywords::Keyword
```

Source: `src/tokenizer.rs:468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the word was not quoted and it matched one of the known keywords,
this will have one of the values from dialect::keywords, otherwise empty

<a id="op-14c167a52b883b42fa30d52d"></a>
## partial_cmp

`function` · `sqlparser::tokenizer::Word::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Word) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 35], "end": [455, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/tokenizer.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3523bddd50169386fb263fab"></a>
## quote_style

`struct_field` · `sqlparser::tokenizer::Word::quote_style` · sqlparser 0.62.0

```rust
quote_style: Option<char>
```

Source: `src/tokenizer.rs:465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An identifier can be "quoted" (&lt;delimited identifier> in ANSI parlance).
The standard and most implementations allow using double quotes for this,
but some implementations support other quoting styles as well (e.g. \[MS SQL])

<a id="op-67651e433eced5a497e84d77"></a>
## serialize

`function` · `sqlparser::tokenizer::Word::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [456, 38], "end": [456, 47], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/tokenizer.rs:456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59a39dd7bdba6b12a13bf08f"></a>
## to_ident

`function` · `sqlparser::tokenizer::Word::to_ident` · sqlparser 0.62.0

```rust
fn to_ident(&self, span: Span) -> Ident
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20575, 1], "end": [20600, 2], "filename": "src/parser/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser/mod.rs:20581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Convert a reference to this word into an [`Ident`](../operations/sqlparser.ast.Ident.md#op-84514077950174f9df2ced53) by cloning the value.

Use this method when you need to keep the original `Word` around.
If you can consume the `Word`, prefer [`into_ident`](Self::into_ident) instead
to avoid cloning.

<a id="op-3f6caab8958bbd11982985c7"></a>
## value

`struct_field` · `sqlparser::tokenizer::Word::value` · sqlparser 0.62.0

```rust
value: String
```

Source: `src/tokenizer.rs:461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value of the token, without the enclosing quotes, and with the
escape sequences (if any) processed (TODO: escapes are not handled)

<a id="op-2c17b798e69a67c53a0703c2"></a>
## visit

`function` · `sqlparser::tokenizer::Word::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 40], "end": [457, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/tokenizer.rs:457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1c4cb58274821869e8179bf"></a>
## visit

`function` · `sqlparser::tokenizer::Word::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Word", "path": "Word"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [457, 47], "end": [457, 55], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/tokenizer.rs:457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
