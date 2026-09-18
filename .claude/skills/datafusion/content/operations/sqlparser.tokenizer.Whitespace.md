# `sqlparser::tokenizer::Whitespace`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.tokenizer.Whitespace.json).

<a id="op-9b1e319052822d002cfd74e3"></a>
## Whitespace

`enum` · `sqlparser::tokenizer::Whitespace` · sqlparser 0.62.0

```rust
enum Whitespace
```

Source: `src/tokenizer.rs:498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents whitespace in the input: spaces, newlines, tabs and comments.

<a id="op-06b3c006c8401689eda8c924"></a>
## MultiLineComment

`variant` · `sqlparser::tokenizer::Whitespace::MultiLineComment` · sqlparser 0.62.0

```rust
MultiLineComment
```

Source: `src/tokenizer.rs:515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A multi-line comment (without the `/* ... */` delimiters).

<a id="op-dc2702b66854d81846451167"></a>
## Newline

`variant` · `sqlparser::tokenizer::Whitespace::Newline` · sqlparser 0.62.0

```rust
Newline
```

Source: `src/tokenizer.rs:502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A newline character.

<a id="op-4cb57dcb6df3b35d58fb1f67"></a>
## SingleLineComment

`variant` · `sqlparser::tokenizer::Whitespace::SingleLineComment` · sqlparser 0.62.0

```rust
SingleLineComment
```

Source: `src/tokenizer.rs:507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single-line comment (e.g. `-- comment` or `# comment`).
The `comment` field contains the text, and `prefix` contains the comment prefix.

<a id="op-14b1d641125f625731846262"></a>
## Space

`variant` · `sqlparser::tokenizer::Whitespace::Space` · sqlparser 0.62.0

```rust
Space
```

Source: `src/tokenizer.rs:500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single space character.

<a id="op-b24d101727cdd3e6c3c6fbbe"></a>
## Tab

`variant` · `sqlparser::tokenizer::Whitespace::Tab` · sqlparser 0.62.0

```rust
Tab
```

Source: `src/tokenizer.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A tab character.

<a id="op-94b3ce9e393f690bbfeb89cc"></a>
## clone

`function` · `sqlparser::tokenizer::Whitespace::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Whitespace
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 17], "end": [495, 22], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/tokenizer.rs:495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c29e3277bde86b7324f81b15"></a>
## cmp

`function` · `sqlparser::tokenizer::Whitespace::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Whitespace) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 51], "end": [495, 54], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/tokenizer.rs:495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be4b50f56f12468c3ad162ed"></a>
## deserialize

`function` · `sqlparser::tokenizer::Whitespace::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 49], "end": [496, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/tokenizer.rs:496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca87897826cc04f92dfefd09"></a>
## eq

`function` · `sqlparser::tokenizer::Whitespace::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Whitespace) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 24], "end": [495, 33], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tokenizer.rs:495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfc23eda9cdf8343abb1d20e"></a>
## fmt

`function` · `sqlparser::tokenizer::Whitespace::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 10], "end": [495, 15], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tokenizer.rs:495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec3ae3ce144af9b6698c8e35"></a>
## fmt

`function` · `sqlparser::tokenizer::Whitespace::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 1], "end": [528, 2], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/tokenizer.rs:519`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc76200f115baca4e035c386"></a>
## hash

`function` · `sqlparser::tokenizer::Whitespace::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 56], "end": [495, 60], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/tokenizer.rs:495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c35ac45f88495f863bb811d"></a>
## partial_cmp

`function` · `sqlparser::tokenizer::Whitespace::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Whitespace) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [495, 35], "end": [495, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/tokenizer.rs:495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-399c8ad2b6e281219a05c479"></a>
## serialize

`function` · `sqlparser::tokenizer::Whitespace::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 38], "end": [496, 47], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/tokenizer.rs:496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3df4e258746e33625441f50b"></a>
## visit

`function` · `sqlparser::tokenizer::Whitespace::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 40], "end": [497, 45], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/tokenizer.rs:497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a2c48fb2e15a89788771338"></a>
## visit

`function` · `sqlparser::tokenizer::Whitespace::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::tokenizer::Whitespace", "path": "Whitespace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 47], "end": [497, 55], "filename": "src/tokenizer.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/tokenizer.rs:497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
