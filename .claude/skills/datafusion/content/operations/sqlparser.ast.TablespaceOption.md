# `sqlparser::ast::TablespaceOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TablespaceOption.json).

<a id="op-a7ed6a70832b1bbc612b47ef"></a>
## TablespaceOption

`struct` · `sqlparser::ast::TablespaceOption` · sqlparser 0.62.0

```rust
struct TablespaceOption
```

Source: `src/ast/mod.rs:8917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySql TableSpace option
<https://dev.mysql.com/doc/refman/8.4/en/create-table.html>

<a id="op-cf171436041625a59af21fe0"></a>
## clone

`function` · `sqlparser::ast::TablespaceOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TablespaceOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8912, 17], "end": [8912, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e111c681515d30ac906c8d36"></a>
## cmp

`function` · `sqlparser::ast::TablespaceOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TablespaceOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8912, 57], "end": [8912, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-884ae69ca1dfe0edaad29c71"></a>
## deserialize

`function` · `sqlparser::ast::TablespaceOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8913, 49], "end": [8913, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b8bfb22917cb1056c63398c"></a>
## eq

`function` · `sqlparser::ast::TablespaceOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TablespaceOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8912, 24], "end": [8912, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26989ef8e151ca1f4acf11c4"></a>
## fmt

`function` · `sqlparser::ast::TablespaceOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8912, 10], "end": [8912, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ee9bd55637ba6f3099f38a0"></a>
## hash

`function` · `sqlparser::ast::TablespaceOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8912, 39], "end": [8912, 43], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b332ada33b13c65773ce0119"></a>
## name

`struct_field` · `sqlparser::ast::TablespaceOption::name` · sqlparser 0.62.0

```rust
name: String
```

Source: `src/ast/mod.rs:8919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the tablespace.

<a id="op-7833a49d71dca7ef0073ba39"></a>
## partial_cmp

`function` · `sqlparser::ast::TablespaceOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TablespaceOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8912, 45], "end": [8912, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e34cf6f2f3f8913b35bf83a"></a>
## serialize

`function` · `sqlparser::ast::TablespaceOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8913, 38], "end": [8913, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7946be7e96724235e62e34e9"></a>
## storage

`struct_field` · `sqlparser::ast::TablespaceOption::storage` · sqlparser 0.62.0

```rust
storage: Option<StorageType>
```

Source: `src/ast/mod.rs:8921`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage type for the tablespace.

<a id="op-b8b2edcbc8646e4ddd56d07b"></a>
## visit

`function` · `sqlparser::ast::TablespaceOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8914, 47], "end": [8914, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc1a53da915ab56c84bdbde3"></a>
## visit

`function` · `sqlparser::ast::TablespaceOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TablespaceOption", "path": "TablespaceOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8914, 40], "end": [8914, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
