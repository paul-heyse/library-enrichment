# `sqlparser::ast::RaisErrorOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.RaisErrorOption.json).

<a id="op-baf912fdd244ce048109d275"></a>
## RaisErrorOption

`enum` · `sqlparser::ast::RaisErrorOption` · sqlparser 0.62.0

```rust
enum RaisErrorOption
```

Source: `src/ast/mod.rs:4989`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RAISERROR` options
See <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/raiserror-transact-sql?view=sql-server-ver16#options>

<a id="op-31f09316cedbf1d5cde487f4"></a>
## Log

`variant` · `sqlparser::ast::RaisErrorOption::Log` · sqlparser 0.62.0

```rust
Log
```

Source: `src/ast/mod.rs:4991`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Log the error.

<a id="op-76f0114bdcf449a2a47b41cc"></a>
## NoWait

`variant` · `sqlparser::ast::RaisErrorOption::NoWait` · sqlparser 0.62.0

```rust
NoWait
```

Source: `src/ast/mod.rs:4993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Do not wait for completion.

<a id="op-d56b12006e6d71bd5a4ada0d"></a>
## SetError

`variant` · `sqlparser::ast::RaisErrorOption::SetError` · sqlparser 0.62.0

```rust
SetError
```

Source: `src/ast/mod.rs:4995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the error state.

<a id="op-7a644804d2c5cb6e7d99a49f"></a>
## clone

`function` · `sqlparser::ast::RaisErrorOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RaisErrorOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 17], "end": [4984, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c6534020606440f76c699ab"></a>
## cmp

`function` · `sqlparser::ast::RaisErrorOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RaisErrorOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 51], "end": [4984, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-832412966459f291e8daa790"></a>
## deserialize

`function` · `sqlparser::ast::RaisErrorOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4985, 49], "end": [4985, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:4985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a95df03dd5ef4f025500ed39"></a>
## eq

`function` · `sqlparser::ast::RaisErrorOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RaisErrorOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 24], "end": [4984, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97f692e775f0924f0518f5c3"></a>
## fmt

`function` · `sqlparser::ast::RaisErrorOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 10], "end": [4984, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5bfeb158062edc9462d3edb"></a>
## fmt

`function` · `sqlparser::ast::RaisErrorOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4998, 1], "end": [5006, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:4999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c3946500a006939563217f6"></a>
## hash

`function` · `sqlparser::ast::RaisErrorOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 56], "end": [4984, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fc10465c1fc7c04c8397f8a"></a>
## partial_cmp

`function` · `sqlparser::ast::RaisErrorOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RaisErrorOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4984, 35], "end": [4984, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:4984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-294603cbefc2009b340c5557"></a>
## serialize

`function` · `sqlparser::ast::RaisErrorOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4985, 38], "end": [4985, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:4985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89be45ac0250c993e8f49b9a"></a>
## visit

`function` · `sqlparser::ast::RaisErrorOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4986, 47], "end": [4986, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:4986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b03c9b96e7447dc142e97b01"></a>
## visit

`function` · `sqlparser::ast::RaisErrorOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaisErrorOption", "path": "RaisErrorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4986, 40], "end": [4986, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:4986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
