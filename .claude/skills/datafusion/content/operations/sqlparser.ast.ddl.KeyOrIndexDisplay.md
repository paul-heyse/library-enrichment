# `sqlparser::ast::ddl::KeyOrIndexDisplay`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.KeyOrIndexDisplay.json).

<a id="op-201b4828b99ea33d82c07136"></a>
## KeyOrIndexDisplay

`enum` · `sqlparser::ast::ddl::KeyOrIndexDisplay` · sqlparser 0.62.0

```rust
enum KeyOrIndexDisplay
```

Source: `src/ast/ddl.rs:1363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Representation whether a definition can can contains the KEY or INDEX keywords with the same
meaning.

This enum initially is directed to `FULLTEXT`,`SPATIAL`, and `UNIQUE` indexes on create table
statements of `MySQL` [(1)].

[1]: https://dev.mysql.com/doc/refman/8.0/en/create-table.html

<a id="op-f5f86b3ce8d51272eb7bde48"></a>
## Index

`variant` · `sqlparser::ast::ddl::KeyOrIndexDisplay::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/ddl.rs:1369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Display the INDEX keyword

<a id="op-96c675cd43c4f5600a334632"></a>
## Key

`variant` · `sqlparser::ast::ddl::KeyOrIndexDisplay::Key` · sqlparser 0.62.0

```rust
Key
```

Source: `src/ast/ddl.rs:1367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Display the KEY keyword

<a id="op-b4ebee04a3a21c13d6323c0a"></a>
## None

`variant` · `sqlparser::ast::ddl::KeyOrIndexDisplay::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/ddl.rs:1365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Nothing to display

<a id="op-2c50fc29ba01023b0ff42cd5"></a>
## clone

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> KeyOrIndexDisplay
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1360, 23], "end": [1360, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a21dccf12b7b3952318a3e24"></a>
## cmp

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &KeyOrIndexDisplay) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1360, 57], "end": [1360, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8261e852269edbfef46ddbba"></a>
## deserialize

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 49], "end": [1361, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-646eea8e3007273baa1b7d1d"></a>
## eq

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &KeyOrIndexDisplay) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1360, 30], "end": [1360, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-213dc6a42e06a7857ecb2146"></a>
## fmt

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1379, 1], "end": [1399, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53e383f582f1286982265d67"></a>
## fmt

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1360, 10], "end": [1360, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3b3e956a5d6ed8f928751db"></a>
## hash

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1360, 62], "end": [1360, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34422d77f69944ee4d659086"></a>
## is_none

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::is_none` · sqlparser 0.62.0

```rust
fn is_none(self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1372, 1], "end": [1377, 2], "filename": "src/ast/ddl.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/ddl.rs:1374`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Check if this is the `None` variant.

<a id="op-8de70cba45d1d184034f6799"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &KeyOrIndexDisplay) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1360, 41], "end": [1360, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-835eb0166de803d3113c7377"></a>
## serialize

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1361, 38], "end": [1361, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24f8afc1624bce80d9394028"></a>
## visit

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1362, 47], "end": [1362, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a223e6e9bc03df73c585ae72"></a>
## visit

`function` · `sqlparser::ast::ddl::KeyOrIndexDisplay::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::KeyOrIndexDisplay", "path": "KeyOrIndexDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1362, 40], "end": [1362, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
