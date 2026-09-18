# `sqlparser::ast::GranteeName`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.GranteeName.json).

<a id="op-c797c3eb63000c40457bd2ca"></a>
## GranteeName

`enum` · `sqlparser::ast::GranteeName` · sqlparser 0.62.0

```rust
enum GranteeName
```

Source: `src/ast/mod.rs:7483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Users/roles designated in a GRANT/REVOKE

<a id="op-b1a9d996de9e893c7651d231"></a>
## ObjectName

`variant` · `sqlparser::ast::GranteeName::ObjectName` · sqlparser 0.62.0

```rust
ObjectName
```

Source: `src/ast/mod.rs:7485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A bare identifier

<a id="op-6ec3593cf2655e108adb360b"></a>
## UserHost

`variant` · `sqlparser::ast::GranteeName::UserHost` · sqlparser 0.62.0

```rust
UserHost
```

Source: `src/ast/mod.rs:7487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A MySQL user/host pair such as 'root'@'%'

<a id="op-c82ff773a606b0fb7b2690d7"></a>
## clone

`function` · `sqlparser::ast::GranteeName::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GranteeName
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7480, 17], "end": [7480, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac47cfaabf22310fa6215352"></a>
## cmp

`function` · `sqlparser::ast::GranteeName::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GranteeName) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7480, 51], "end": [7480, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1e3a171d053e55d011c3435"></a>
## deserialize

`function` · `sqlparser::ast::GranteeName::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7481, 49], "end": [7481, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7481`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ebd39c193255ed13648319"></a>
## eq

`function` · `sqlparser::ast::GranteeName::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GranteeName) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7480, 24], "end": [7480, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b2bf75537f0ce9559694da6"></a>
## fmt

`function` · `sqlparser::ast::GranteeName::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7480, 10], "end": [7480, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdfbf14b05ad055edd1f4ec8"></a>
## fmt

`function` · `sqlparser::ast::GranteeName::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7495, 1], "end": [7504, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e335eeee777353cdbe708027"></a>
## hash

`function` · `sqlparser::ast::GranteeName::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7480, 56], "end": [7480, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27914080706ba5039a2bdc01"></a>
## partial_cmp

`function` · `sqlparser::ast::GranteeName::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GranteeName) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7480, 35], "end": [7480, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c8198e129f05af7fb8c7edd"></a>
## serialize

`function` · `sqlparser::ast::GranteeName::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7481, 38], "end": [7481, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7481`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53b7f5535f49668cec0339b4"></a>
## visit

`function` · `sqlparser::ast::GranteeName::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7482, 47], "end": [7482, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b02ee917fdf04883457da751"></a>
## visit

`function` · `sqlparser::ast::GranteeName::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteeName", "path": "GranteeName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7482, 40], "end": [7482, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
