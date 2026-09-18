# `sqlparser::ast::table_constraints::IndexConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.IndexConstraint.json).

<a id="op-cdd5d7dbb936ee372dc0ecec"></a>
## IndexConstraint

`struct` · `sqlparser::ast::table_constraints::IndexConstraint` · sqlparser 0.62.0

```rust
struct IndexConstraint
```

Source: `src/ast/table_constraints.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQLs [index definition][1] for index creation. Not present on ANSI so, for now, the usage
is restricted to MySQL, as no other dialects that support this syntax were found.

`{INDEX | KEY} [index_name] [index_type] (key_part,...) [index_option]...`

[1]: https://dev.mysql.com/doc/refman/8.0/en/create-table.html

<a id="op-ecf815370b40ebaf8bcd30fc"></a>
## clone

`function` · `sqlparser::ast::table_constraints::IndexConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IndexConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 17], "end": [361, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c648565c26ebe75eb42495d3"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::IndexConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IndexConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 51], "end": [361, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c2fab255482d836d15afcdc"></a>
## columns

`struct_field` · `sqlparser::ast::table_constraints::IndexConstraint::columns` · sqlparser 0.62.0

```rust
columns: Vec<ast::IndexColumn>
```

Source: `src/ast/table_constraints.rs:374`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Referred column identifier list.

<a id="op-e116bcb78d0e15df9f0ca7ba"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::IndexConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 49], "end": [362, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-553f0d6a935538c943c39bc2"></a>
## display_as_key

`struct_field` · `sqlparser::ast::table_constraints::IndexConstraint::display_as_key` · sqlparser 0.62.0

```rust
display_as_key: bool
```

Source: `src/ast/table_constraints.rs:366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether this index starts with KEY (true) or INDEX (false), to maintain the same syntax.

<a id="op-9ec2e47f1a2c9f4c144fbb16"></a>
## eq

`function` · `sqlparser::ast::table_constraints::IndexConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IndexConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 24], "end": [361, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-230a78a0c9a46d7e15493f2e"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::IndexConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 10], "end": [361, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e11fba59a35f020cbdbf8671"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::IndexConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [380, 1], "end": [395, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/table_constraints.rs:381`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdbb37ef873143463ce990ea"></a>
## hash

`function` · `sqlparser::ast::table_constraints::IndexConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 56], "end": [361, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73c932d9297381cecd0cd9d6"></a>
## index_options

`struct_field` · `sqlparser::ast::table_constraints::IndexConstraint::index_options` · sqlparser 0.62.0

```rust
index_options: Vec<ast::IndexOption>
```

Source: `src/ast/table_constraints.rs:377`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional index options such as `USING`; see [`IndexOption`](../operations/sqlparser.ast.ddl.IndexOption.md#op-0dc5af5d56d2665aea0a2488).
Options applied to the index (e.g., `COMMENT`, `WITH` options).

<a id="op-f4a914ad69cb54fe1502d05e"></a>
## index_type

`struct_field` · `sqlparser::ast::table_constraints::IndexConstraint::index_type` · sqlparser 0.62.0

```rust
index_type: Option<ast::IndexType>
```

Source: `src/ast/table_constraints.rs:372`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional [index type][1].

[1]: IndexType

<a id="op-721cca1a2c23cf70a94efe80"></a>
## name

`struct_field` · `sqlparser::ast::table_constraints::IndexConstraint::name` · sqlparser 0.62.0

```rust
name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index name.

<a id="op-ed387b34318ed2bc9bfc2353"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::IndexConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IndexConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [361, 35], "end": [361, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6ec86c6039bcc5957decfb1"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::IndexConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [362, 38], "end": [362, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f9d6a1c64523e42003ee0a5"></a>
## span

`function` · `sqlparser::ast::table_constraints::IndexConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [410, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/table_constraints.rs:398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27a5f8a18fbb992c928fd66e"></a>
## visit

`function` · `sqlparser::ast::table_constraints::IndexConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [363, 47], "end": [363, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c69e6180d4d46f2cbe2cab2c"></a>
## visit

`function` · `sqlparser::ast::table_constraints::IndexConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [363, 40], "end": [363, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
