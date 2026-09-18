# `sqlparser::ast::ddl::Partition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.Partition.json).

<a id="op-1d05815f9934b3a35e191e25"></a>
## Partition

`enum` · `sqlparser::ast::ddl::Partition` · sqlparser 0.62.0

```rust
enum Partition
```

Source: `src/ast/ddl.rs:2726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PARTITION statement used in ALTER TABLE et al. such as in Hive and ClickHouse SQL.
For example, ClickHouse's OPTIMIZE TABLE supports syntax like PARTITION ID 'partition_id' and PARTITION expr.
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/optimize)

<a id="op-6082e02a2f21ddb9b5fa4ab6"></a>
## Expr

`variant` · `sqlparser::ast::ddl::Partition::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/ddl.rs:2730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse supports PARTITION expr syntax.

<a id="op-c67af8fecada574a2c760144"></a>
## Identifier

`variant` · `sqlparser::ast::ddl::Partition::Identifier` · sqlparser 0.62.0

```rust
Identifier
```

Source: `src/ast/ddl.rs:2728`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse supports PARTITION ID 'partition_id' syntax.

<a id="op-74e68141016ceacb7b64fed7"></a>
## Part

`variant` · `sqlparser::ast::ddl::Partition::Part` · sqlparser 0.62.0

```rust
Part
```

Source: `src/ast/ddl.rs:2733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse supports PART expr which represents physical partition in disk.
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/partition#attach-partitionpart)

<a id="op-054951a7d0a8cd8768c492ea"></a>
## Partitions

`variant` · `sqlparser::ast::ddl::Partition::Partitions` · sqlparser 0.62.0

```rust
Partitions
```

Source: `src/ast/ddl.rs:2735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive supports multiple partitions in PARTITION (part1, part2, ...) syntax.

<a id="op-3637ca0cb5feeffeadd2098c"></a>
## clone

`function` · `sqlparser::ast::ddl::Partition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Partition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 17], "end": [2723, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59228102f30385c757866a79"></a>
## cmp

`function` · `sqlparser::ast::ddl::Partition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Partition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 57], "end": [2723, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f02a8792c0ec964e6c18aba1"></a>
## deserialize

`function` · `sqlparser::ast::ddl::Partition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2724, 49], "end": [2724, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bd6a82d4aadbe38e4f2fb4e"></a>
## eq

`function` · `sqlparser::ast::ddl::Partition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Partition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 24], "end": [2723, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2e26a163b91dc5e9806d406"></a>
## fmt

`function` · `sqlparser::ast::ddl::Partition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 10], "end": [2723, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8c9a33ac7e42a070af2fe71"></a>
## fmt

`function` · `sqlparser::ast::ddl::Partition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2738, 1], "end": [2749, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da715e72d84a430b551ee0bf"></a>
## hash

`function` · `sqlparser::ast::ddl::Partition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 39], "end": [2723, 43], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8160c3037b17d0a36b5f2ace"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::Partition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Partition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2723, 45], "end": [2723, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39a7b9064147666d9cef5a68"></a>
## serialize

`function` · `sqlparser::ast::ddl::Partition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2724, 38], "end": [2724, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f1d2637981ffd3237cc90e3"></a>
## span

`function` · `sqlparser::ast::ddl::Partition::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "super::Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1236, 1], "end": [1245, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-581c2d3b348df15bd5b9085d"></a>
## visit

`function` · `sqlparser::ast::ddl::Partition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2725, 47], "end": [2725, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73ec3d5cc308631db349938b"></a>
## visit

`function` · `sqlparser::ast::ddl::Partition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Partition", "path": "Partition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2725, 40], "end": [2725, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
