# `sqlparser::ast::query::Setting`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Setting.json).

<a id="op-04d95c28d5d98fea00d356a5"></a>
## Setting

`struct` · `sqlparser::ast::query::Setting` · sqlparser 0.62.0

```rust
struct Setting
```

Source: `src/ast/query.rs:1291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single setting key-value pair.

<a id="op-6d052def9e92e4e2e814fe44"></a>
## clone

`function` · `sqlparser::ast::query::Setting::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Setting
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 17], "end": [1287, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-908fe733f65ae02a23ffb6a0"></a>
## cmp

`function` · `sqlparser::ast::query::Setting::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Setting) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 51], "end": [1287, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b58e75856f5cdfb67a9e2d7"></a>
## deserialize

`function` · `sqlparser::ast::query::Setting::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 49], "end": [1288, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-112abe13201240f6cdcb3573"></a>
## eq

`function` · `sqlparser::ast::query::Setting::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Setting) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 24], "end": [1287, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21f356f9133227ae50bcfdcb"></a>
## fmt

`function` · `sqlparser::ast::query::Setting::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1298, 1], "end": [1302, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efb88d1cf300103c6eab1bb0"></a>
## fmt

`function` · `sqlparser::ast::query::Setting::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 10], "end": [1287, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57b589bf60e8cb84ac6ab103"></a>
## hash

`function` · `sqlparser::ast::query::Setting::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 56], "end": [1287, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08f34539329c2925761b0fe3"></a>
## key

`struct_field` · `sqlparser::ast::query::Setting::key` · sqlparser 0.62.0

```rust
key: Ident
```

Source: `src/ast/query.rs:1293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Setting name/key.

<a id="op-bad9e53d10efa44c96f41a88"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Setting::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Setting) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 35], "end": [1287, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dde470ae9a304ebe6ee528ad"></a>
## serialize

`function` · `sqlparser::ast::query::Setting::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 38], "end": [1288, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5143a80659029050ffd25b1d"></a>
## value

`struct_field` · `sqlparser::ast::query::Setting::value` · sqlparser 0.62.0

```rust
value: Expr
```

Source: `src/ast/query.rs:1295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value expression assigned to the setting.

<a id="op-f0e19c52ed8bac5768ba8b9d"></a>
## visit

`function` · `sqlparser::ast::query::Setting::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1289, 40], "end": [1289, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1d7ea76523e94c19d8a4e9c"></a>
## visit

`function` · `sqlparser::ast::query::Setting::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Setting", "path": "Setting"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1289, 47], "end": [1289, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
