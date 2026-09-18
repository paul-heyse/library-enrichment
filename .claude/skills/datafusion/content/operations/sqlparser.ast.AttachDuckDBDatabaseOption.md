# `sqlparser::ast::AttachDuckDBDatabaseOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.AttachDuckDBDatabaseOption.json).

<a id="op-4a0a700c93b2aaca3a46978e"></a>
## AttachDuckDBDatabaseOption

`enum` · `sqlparser::ast::AttachDuckDBDatabaseOption` · sqlparser 0.62.0

```rust
enum AttachDuckDBDatabaseOption
```

Source: `src/ast/mod.rs:9018`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options supported by DuckDB for `ATTACH DATABASE`.

<a id="op-e5d9e3361835d4b7e39c12bf"></a>
## ReadOnly

`variant` · `sqlparser::ast::AttachDuckDBDatabaseOption::ReadOnly` · sqlparser 0.62.0

```rust
ReadOnly
```

Source: `src/ast/mod.rs:9020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

READ_ONLY option, optional boolean value.

<a id="op-da12c22f2ff2227e4a51f5e9"></a>
## Type

`variant` · `sqlparser::ast::AttachDuckDBDatabaseOption::Type` · sqlparser 0.62.0

```rust
Type
```

Source: `src/ast/mod.rs:9022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TYPE option specifying a database type identifier.

<a id="op-62875b3a38618204a0afcfbb"></a>
## clone

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AttachDuckDBDatabaseOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9014, 17], "end": [9014, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-054199f6dea38602ec94147a"></a>
## cmp

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AttachDuckDBDatabaseOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9014, 51], "end": [9014, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69a6ed1b6cef0f5c14ec9669"></a>
## deserialize

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9015, 49], "end": [9015, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-312bf7a5bf211e5000a381e6"></a>
## eq

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AttachDuckDBDatabaseOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9014, 24], "end": [9014, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50655711cab1fb219261fd2a"></a>
## fmt

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9014, 10], "end": [9014, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9967b2a6ad61bc354b25a17"></a>
## fmt

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9025, 1], "end": [9034, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9026`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-891120482f5fcda5ad2df591"></a>
## hash

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9014, 56], "end": [9014, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa069d89091c583dd28bc7a0"></a>
## partial_cmp

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AttachDuckDBDatabaseOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9014, 35], "end": [9014, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cc793d0cd6b32e8fb29e8d9"></a>
## serialize

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9015, 38], "end": [9015, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-508669b281e6ebfa99881782"></a>
## visit

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9016, 40], "end": [9016, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6572805a17d6965ee9edacc"></a>
## visit

`function` · `sqlparser::ast::AttachDuckDBDatabaseOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::AttachDuckDBDatabaseOption", "path": "AttachDuckDBDatabaseOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9016, 47], "end": [9016, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
