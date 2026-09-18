# `sqlparser::ast::JsonPathElem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.JsonPathElem.json).

<a id="op-3e3952d4d8571ea9fbd72fc5"></a>
## JsonPathElem

`enum` · `sqlparser::ast::JsonPathElem` · sqlparser 0.62.0

```rust
enum JsonPathElem
```

Source: `src/ast/mod.rs:676`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An element of a JSON path.

<a id="op-236eb14d39ab89acfb341df8"></a>
## Bracket

`variant` · `sqlparser::ast::JsonPathElem::Bracket` · sqlparser 0.62.0

```rust
Bracket
```

Source: `src/ast/mod.rs:690`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Accesses an object field or array element using bracket notation,
e.g. `obj['foo']`.

See <https://docs.snowflake.com/en/user-guide/querying-semistructured#bracket-notation>.

<a id="op-ee509491f891427c3433ae47"></a>
## ColonBracket

`variant` · `sqlparser::ast::JsonPathElem::ColonBracket` · sqlparser 0.62.0

```rust
ColonBracket
```

Source: `src/ast/mod.rs:698`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Access an object field using colon bracket notation
e.g. `obj:['foo']`

See <https://docs.databricks.com/en/sql/language-manual/functions/colonsign.html>

<a id="op-e19d7eba513c7260fc08fa98"></a>
## Dot

`variant` · `sqlparser::ast::JsonPathElem::Dot` · sqlparser 0.62.0

```rust
Dot
```

Source: `src/ast/mod.rs:680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Accesses an object field using dot notation, e.g. `obj:foo.bar.baz`.

See <https://docs.snowflake.com/en/user-guide/querying-semistructured#dot-notation>.

<a id="op-940743bb5a57bfd009d45b50"></a>
## clone

`function` · `sqlparser::ast::JsonPathElem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonPathElem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 17], "end": [673, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7697903e16923947038bca85"></a>
## cmp

`function` · `sqlparser::ast::JsonPathElem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonPathElem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 51], "end": [673, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8266af28e6e2e98121f01b76"></a>
## deserialize

`function` · `sqlparser::ast::JsonPathElem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 49], "end": [674, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ac9ccda46253dc6d9959a9d"></a>
## eq

`function` · `sqlparser::ast::JsonPathElem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonPathElem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 24], "end": [673, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bbfd886448d35eb29fdcdbf"></a>
## fmt

`function` · `sqlparser::ast::JsonPathElem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 10], "end": [673, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66f863625ec18d34ff8656d7"></a>
## hash

`function` · `sqlparser::ast::JsonPathElem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 56], "end": [673, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e728f24ddfcb6f799db9083"></a>
## partial_cmp

`function` · `sqlparser::ast::JsonPathElem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonPathElem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [673, 35], "end": [673, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb4e2bb21bf54f7fef1b2ba2"></a>
## serialize

`function` · `sqlparser::ast::JsonPathElem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 38], "end": [674, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a41bf9da91f682ce991cac87"></a>
## span

`function` · `sqlparser::ast::JsonPathElem::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "super::JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1816, 1], "end": [1824, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fec5fed1513f3b34d922afb"></a>
## visit

`function` · `sqlparser::ast::JsonPathElem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 40], "end": [675, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:675`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4e8d18423cc5c8491cfc13e"></a>
## visit

`function` · `sqlparser::ast::JsonPathElem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonPathElem", "path": "JsonPathElem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [675, 47], "end": [675, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:675`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
