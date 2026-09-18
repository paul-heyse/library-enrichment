# `sqlparser::ast::ddl::NullsDistinctOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.NullsDistinctOption.json).

<a id="op-181bf5e17796f72e5210fe0a"></a>
## NullsDistinctOption

`enum` · `sqlparser::ast::ddl::NullsDistinctOption` · sqlparser 0.62.0

```rust
enum NullsDistinctOption
```

Source: `src/ast/ddl.rs:1479`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[PostgreSQL] unique index nulls handling option: `[ NULLS [ NOT ] DISTINCT ]`

[PostgreSQL]: https://www.postgresql.org/docs/17/sql-altertable.html

<a id="op-22515ea08056928a271bc093"></a>
## Distinct

`variant` · `sqlparser::ast::ddl::NullsDistinctOption::Distinct` · sqlparser 0.62.0

```rust
Distinct
```

Source: `src/ast/ddl.rs:1483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

NULLS DISTINCT

<a id="op-2dc1a1098e243b621d0f2f30"></a>
## None

`variant` · `sqlparser::ast::ddl::NullsDistinctOption::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/ddl.rs:1481`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Not specified

<a id="op-72284ede6be1e3c0ff481d92"></a>
## NotDistinct

`variant` · `sqlparser::ast::ddl::NullsDistinctOption::NotDistinct` · sqlparser 0.62.0

```rust
NotDistinct
```

Source: `src/ast/ddl.rs:1485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

NULLS NOT DISTINCT

<a id="op-2b75d0ed3e88e5f70a39236b"></a>
## clone

`function` · `sqlparser::ast::ddl::NullsDistinctOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NullsDistinctOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 17], "end": [1476, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44da63117c177db427ffaca0"></a>
## cmp

`function` · `sqlparser::ast::ddl::NullsDistinctOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NullsDistinctOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 57], "end": [1476, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc33d15210360e9cb66617b0"></a>
## deserialize

`function` · `sqlparser::ast::ddl::NullsDistinctOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1477, 49], "end": [1477, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d98c4928d243009c749471ea"></a>
## eq

`function` · `sqlparser::ast::ddl::NullsDistinctOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NullsDistinctOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 30], "end": [1476, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32df1d6471e0ec04f7ac8acf"></a>
## fmt

`function` · `sqlparser::ast::ddl::NullsDistinctOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 10], "end": [1476, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b54c89d7da2c83907f82476a"></a>
## fmt

`function` · `sqlparser::ast::ddl::NullsDistinctOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1488, 1], "end": [1496, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db080195daa3854dc1df3938"></a>
## hash

`function` · `sqlparser::ast::ddl::NullsDistinctOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 62], "end": [1476, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd2c5fde1803ca3ffb92b951"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::NullsDistinctOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NullsDistinctOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 41], "end": [1476, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9fe10bf9a82cd0cf30f511c"></a>
## serialize

`function` · `sqlparser::ast::ddl::NullsDistinctOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1477, 38], "end": [1477, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88d1542da651b3806593b043"></a>
## visit

`function` · `sqlparser::ast::ddl::NullsDistinctOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1478, 40], "end": [1478, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a915b437353bd393dbfe908b"></a>
## visit

`function` · `sqlparser::ast::ddl::NullsDistinctOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::NullsDistinctOption", "path": "NullsDistinctOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1478, 47], "end": [1478, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
