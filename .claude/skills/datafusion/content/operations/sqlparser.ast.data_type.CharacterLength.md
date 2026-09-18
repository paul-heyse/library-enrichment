# `sqlparser::ast::data_type::CharacterLength`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.CharacterLength.json).

<a id="op-72cccc5f1a5ede8608ac222f"></a>
## CharacterLength

`enum` · `sqlparser::ast::data_type::CharacterLength` · sqlparser 0.62.0

```rust
enum CharacterLength
```

Source: `src/ast/data_type.rs:1048`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Information about [character length][1], including length and possibly unit.

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-length

<a id="op-53a064ae6a3c7ea350800268"></a>
## IntegerLength

`variant` · `sqlparser::ast::data_type::CharacterLength::IntegerLength` · sqlparser 0.62.0

```rust
IntegerLength
```

Source: `src/ast/data_type.rs:1050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Integer length with optional unit (e.g. `CHAR(10)` or `VARCHAR(10 CHARACTERS)`).

<a id="op-a8f5d905ed1f2e75a51ea554"></a>
## Max

`variant` · `sqlparser::ast::data_type::CharacterLength::Max` · sqlparser 0.62.0

```rust
Max
```

Source: `src/ast/data_type.rs:1057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

VARCHAR(MAX) or NVARCHAR(MAX), used in T-SQL (Microsoft SQL Server).

<a id="op-e71adde22965cc3111581987"></a>
## clone

`function` · `sqlparser::ast::data_type::CharacterLength::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CharacterLength
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 23], "end": [1045, 28], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:1045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f8d6e8acd21805c61878242"></a>
## cmp

`function` · `sqlparser::ast::data_type::CharacterLength::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CharacterLength) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 57], "end": [1045, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:1045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27e8e8e39874400b7cc64945"></a>
## deserialize

`function` · `sqlparser::ast::data_type::CharacterLength::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1046, 49], "end": [1046, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:1046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe359e4f702ecf4ab6ba1055"></a>
## eq

`function` · `sqlparser::ast::data_type::CharacterLength::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CharacterLength) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 30], "end": [1045, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:1045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10be9f1c0f7168d198040043"></a>
## fmt

`function` · `sqlparser::ast::data_type::CharacterLength::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1060, 1], "end": [1075, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8312caa53eb05999212ef845"></a>
## fmt

`function` · `sqlparser::ast::data_type::CharacterLength::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 10], "end": [1045, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:1045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-403986bc2b10914894c09144"></a>
## hash

`function` · `sqlparser::ast::data_type::CharacterLength::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 62], "end": [1045, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:1045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77ab4727713324a3a2a2abb0"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::CharacterLength::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CharacterLength) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 41], "end": [1045, 51], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:1045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-698cc605e8b9325e78eed981"></a>
## serialize

`function` · `sqlparser::ast::data_type::CharacterLength::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1046, 38], "end": [1046, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:1046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ef90d6a9cb284ffd922673a"></a>
## visit

`function` · `sqlparser::ast::data_type::CharacterLength::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1047, 40], "end": [1047, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:1047`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a85394f529307b4573785bd"></a>
## visit

`function` · `sqlparser::ast::data_type::CharacterLength::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::CharacterLength", "path": "CharacterLength"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1047, 47], "end": [1047, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:1047`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
