# `sqlparser::ast::dml::MultiTableInsertType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MultiTableInsertType.json).

<a id="op-7a2fb139f06ebc3e6ad7762f"></a>
## MultiTableInsertType

`enum` · `sqlparser::ast::dml::MultiTableInsertType` · sqlparser 0.62.0

```rust
enum MultiTableInsertType
```

Source: `src/ast/dml.rs:899`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of multi-table INSERT statement(Snowflake).

See: <https://docs.snowflake.com/en/sql-reference/sql/insert-multi-table>

<a id="op-e0c770cd272bea5c2224f947"></a>
## All

`variant` · `sqlparser::ast::dml::MultiTableInsertType::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/dml.rs:901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INSERT ALL` - all matching WHEN clauses are executed

<a id="op-aaf2a559355141d7eede42af"></a>
## First

`variant` · `sqlparser::ast::dml::MultiTableInsertType::First` · sqlparser 0.62.0

```rust
First
```

Source: `src/ast/dml.rs:903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INSERT FIRST` - only the first matching WHEN clause is executed

<a id="op-b04d9d77e82c7247a70a6a75"></a>
## clone

`function` · `sqlparser::ast::dml::MultiTableInsertType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MultiTableInsertType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [896, 17], "end": [896, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb76494f51cfd6252e8bbc43"></a>
## cmp

`function` · `sqlparser::ast::dml::MultiTableInsertType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MultiTableInsertType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [896, 51], "end": [896, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5a4b7de5164e6cfe95e6ba9"></a>
## deserialize

`function` · `sqlparser::ast::dml::MultiTableInsertType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 49], "end": [897, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1b1db9019f7981885964665"></a>
## eq

`function` · `sqlparser::ast::dml::MultiTableInsertType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MultiTableInsertType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [896, 24], "end": [896, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69c5a1bf94feb1205cd24381"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [906, 1], "end": [913, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da3f1b05ef798d2ff0cb034e"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [896, 10], "end": [896, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79aed5f963ac024059a29b00"></a>
## hash

`function` · `sqlparser::ast::dml::MultiTableInsertType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [896, 56], "end": [896, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-361841cec83a8aeea2ef83a5"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MultiTableInsertType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MultiTableInsertType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [896, 35], "end": [896, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33a3cbd16aa9d0812adb7f64"></a>
## serialize

`function` · `sqlparser::ast::dml::MultiTableInsertType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [897, 38], "end": [897, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2c1d29eb902adcf63af97a2"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [898, 47], "end": [898, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6ef8cb1fbbbc46a1f0dcb1a"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertType", "path": "MultiTableInsertType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [898, 40], "end": [898, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
