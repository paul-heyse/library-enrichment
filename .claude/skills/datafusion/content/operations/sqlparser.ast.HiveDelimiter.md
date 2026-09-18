# `sqlparser::ast::HiveDelimiter`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveDelimiter.json).

<a id="op-5299d8853b7fa5b089246bf6"></a>
## HiveDelimiter

`enum` · `sqlparser::ast::HiveDelimiter` · sqlparser 0.62.0

```rust
enum HiveDelimiter
```

Source: `src/ast/mod.rs:8608`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Kind of delimiter used in Hive `ROW FORMAT` definitions.

<a id="op-35d88c78a43dd1b634f03e8c"></a>
## CollectionItemsTerminatedBy

`variant` · `sqlparser::ast::HiveDelimiter::CollectionItemsTerminatedBy` · sqlparser 0.62.0

```rust
CollectionItemsTerminatedBy
```

Source: `src/ast/mod.rs:8614`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Collection items terminated by a delimiter.

<a id="op-f44c1a6079870c3be64e4d5b"></a>
## FieldsEscapedBy

`variant` · `sqlparser::ast::HiveDelimiter::FieldsEscapedBy` · sqlparser 0.62.0

```rust
FieldsEscapedBy
```

Source: `src/ast/mod.rs:8612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fields escaped by a character.

<a id="op-5e831d3900353dd595dbeaeb"></a>
## FieldsTerminatedBy

`variant` · `sqlparser::ast::HiveDelimiter::FieldsTerminatedBy` · sqlparser 0.62.0

```rust
FieldsTerminatedBy
```

Source: `src/ast/mod.rs:8610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fields terminated by a delimiter.

<a id="op-545d28a03d59f1112e4a4466"></a>
## LinesTerminatedBy

`variant` · `sqlparser::ast::HiveDelimiter::LinesTerminatedBy` · sqlparser 0.62.0

```rust
LinesTerminatedBy
```

Source: `src/ast/mod.rs:8618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Lines terminated by a delimiter.

<a id="op-7059dca92f744649cb8db426"></a>
## MapKeysTerminatedBy

`variant` · `sqlparser::ast::HiveDelimiter::MapKeysTerminatedBy` · sqlparser 0.62.0

```rust
MapKeysTerminatedBy
```

Source: `src/ast/mod.rs:8616`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Map keys terminated by a delimiter.

<a id="op-c573e21bdd5bc0674b3601e0"></a>
## NullDefinedAs

`variant` · `sqlparser::ast::HiveDelimiter::NullDefinedAs` · sqlparser 0.62.0

```rust
NullDefinedAs
```

Source: `src/ast/mod.rs:8620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Null represented by a specific token.

<a id="op-754aa99f15fbdf32b78655f1"></a>
## clone

`function` · `sqlparser::ast::HiveDelimiter::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveDelimiter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8604, 23], "end": [8604, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7391b211f4cf4898c94bc84"></a>
## cmp

`function` · `sqlparser::ast::HiveDelimiter::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveDelimiter) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8604, 57], "end": [8604, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35959280d584664d0e7f81aa"></a>
## deserialize

`function` · `sqlparser::ast::HiveDelimiter::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8605, 49], "end": [8605, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8605`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9d06f86a2872cd42320da98"></a>
## eq

`function` · `sqlparser::ast::HiveDelimiter::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveDelimiter) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8604, 30], "end": [8604, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df8f9676831a8c2dc39b8ddd"></a>
## fmt

`function` · `sqlparser::ast::HiveDelimiter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8604, 10], "end": [8604, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f15e1f3d0c31a4cf4e731264"></a>
## fmt

`function` · `sqlparser::ast::HiveDelimiter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8623, 1], "end": [8635, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8624`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1c99ba39c8bbc7c846c0eaf"></a>
## hash

`function` · `sqlparser::ast::HiveDelimiter::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8604, 62], "end": [8604, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ea9fd6732010a02fb778e60"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveDelimiter::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveDelimiter) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8604, 41], "end": [8604, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0988b294441e0d36bc089014"></a>
## serialize

`function` · `sqlparser::ast::HiveDelimiter::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8605, 38], "end": [8605, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8605`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1bfe33fe9ee4152a4b08d9c"></a>
## visit

`function` · `sqlparser::ast::HiveDelimiter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8606, 40], "end": [8606, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8606`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d70d5c54e4caae07d6ca98ed"></a>
## visit

`function` · `sqlparser::ast::HiveDelimiter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDelimiter", "path": "HiveDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8606, 47], "end": [8606, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8606`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
