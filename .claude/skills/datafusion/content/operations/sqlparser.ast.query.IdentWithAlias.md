# `sqlparser::ast::query::IdentWithAlias`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.IdentWithAlias.json).

<a id="op-05512a11092aba3db5c806fd"></a>
## IdentWithAlias

`struct` · `sqlparser::ast::query::IdentWithAlias` · sqlparser 0.62.0

```rust
struct IdentWithAlias
```

Source: `src/ast/query.rs:911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single aliased identifier

# Syntax
```plaintext
<ident> AS <alias>
```

<a id="op-ef301d87b999250bcb01074c"></a>
## alias

`struct_field` · `sqlparser::ast::query::IdentWithAlias::alias` · sqlparser 0.62.0

```rust
alias: Ident
```

Source: `src/ast/query.rs:915`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The alias to apply to `ident`.

<a id="op-c9d7c1f284141e29708d97a7"></a>
## clone

`function` · `sqlparser::ast::query::IdentWithAlias::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IdentWithAlias
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [908, 17], "end": [908, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-491fc09a80d8912013138665"></a>
## cmp

`function` · `sqlparser::ast::query::IdentWithAlias::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IdentWithAlias) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [908, 51], "end": [908, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41e37f6b4f213a7203e0bffc"></a>
## deserialize

`function` · `sqlparser::ast::query::IdentWithAlias::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [909, 49], "end": [909, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd1427400ec2ff203baa0de4"></a>
## eq

`function` · `sqlparser::ast::query::IdentWithAlias::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IdentWithAlias) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [908, 24], "end": [908, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7285af757b07cb9ef7013457"></a>
## fmt

`function` · `sqlparser::ast::query::IdentWithAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [918, 1], "end": [922, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bb3263bfb64572b8b9fe047"></a>
## fmt

`function` · `sqlparser::ast::query::IdentWithAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [908, 10], "end": [908, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a944f23b070391ce57a9b42f"></a>
## hash

`function` · `sqlparser::ast::query::IdentWithAlias::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [908, 56], "end": [908, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82469c7e0a6f4ef5c854f7d2"></a>
## ident

`struct_field` · `sqlparser::ast::query::IdentWithAlias::ident` · sqlparser 0.62.0

```rust
ident: Ident
```

Source: `src/ast/query.rs:913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The identifier being aliased.

<a id="op-e6ed73d2113aa43eda784f58"></a>
## partial_cmp

`function` · `sqlparser::ast::query::IdentWithAlias::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IdentWithAlias) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [908, 35], "end": [908, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37dadb4f81bdf528d4c02247"></a>
## serialize

`function` · `sqlparser::ast::query::IdentWithAlias::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [909, 38], "end": [909, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3447dfdee4989f842505b123"></a>
## visit

`function` · `sqlparser::ast::query::IdentWithAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 47], "end": [910, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:910`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7902726dabffe581859715f"></a>
## visit

`function` · `sqlparser::ast::query::IdentWithAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::IdentWithAlias", "path": "IdentWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [910, 40], "end": [910, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:910`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
