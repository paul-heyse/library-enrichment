# `sqlparser::ast::value::TrimWhereField`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.TrimWhereField.json).

<a id="op-e711960a81e5df7bf864046b"></a>
## TrimWhereField

`enum` · `sqlparser::ast::value::TrimWhereField` · sqlparser 0.62.0

```rust
enum TrimWhereField
```

Source: `src/ast/value.rs:691`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The side on which `TRIM` should be applied.

Corresponds to `TRIM(BOTH|LEADING|TRAILING)` SQL syntax.

<a id="op-6c84307767e05f8397de6035"></a>
## Both

`variant` · `sqlparser::ast::value::TrimWhereField::Both` · sqlparser 0.62.0

```rust
Both
```

Source: `src/ast/value.rs:693`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`BOTH` (trim from both ends)

<a id="op-9c10792af852b6bc9cadbe99"></a>
## Leading

`variant` · `sqlparser::ast::value::TrimWhereField::Leading` · sqlparser 0.62.0

```rust
Leading
```

Source: `src/ast/value.rs:695`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LEADING` (trim from start)

<a id="op-7bc05c479406041fc8dc7dfb"></a>
## Trailing

`variant` · `sqlparser::ast::value::TrimWhereField::Trailing` · sqlparser 0.62.0

```rust
Trailing
```

Source: `src/ast/value.rs:697`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TRAILING` (trim from end)

<a id="op-87134a82355f259b83179256"></a>
## clone

`function` · `sqlparser::ast::value::TrimWhereField::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TrimWhereField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 17], "end": [688, 22], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/value.rs:688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce9b0edffabe91cb7be26063"></a>
## cmp

`function` · `sqlparser::ast::value::TrimWhereField::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TrimWhereField) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 57], "end": [688, 60], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/value.rs:688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf100e63c89528481a9d1b05"></a>
## deserialize

`function` · `sqlparser::ast::value::TrimWhereField::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 49], "end": [689, 60], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/value.rs:689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a51407b65e0a743ecb72591f"></a>
## eq

`function` · `sqlparser::ast::value::TrimWhereField::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TrimWhereField) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 30], "end": [688, 39], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/value.rs:688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d43f26f56433a35aabdeb84c"></a>
## fmt

`function` · `sqlparser::ast::value::TrimWhereField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 10], "end": [688, 15], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/value.rs:688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df6bbc52dd0d6a41e3a0795f"></a>
## fmt

`function` · `sqlparser::ast::value::TrimWhereField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [709, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/value.rs:701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45bf9d8dcf357fbb8f369ecb"></a>
## hash

`function` · `sqlparser::ast::value::TrimWhereField::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 62], "end": [688, 66], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/value.rs:688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee4a08e85f21e5fb32d49a21"></a>
## partial_cmp

`function` · `sqlparser::ast::value::TrimWhereField::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TrimWhereField) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 45], "end": [688, 55], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/value.rs:688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-156d74f1dcd55ed7ea1a48ea"></a>
## serialize

`function` · `sqlparser::ast::value::TrimWhereField::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [689, 38], "end": [689, 47], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/value.rs:689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d2c6f944872acd45cb6679a"></a>
## visit

`function` · `sqlparser::ast::value::TrimWhereField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [690, 40], "end": [690, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/value.rs:690`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d3e1b8c544f97632328bd43"></a>
## visit

`function` · `sqlparser::ast::value::TrimWhereField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::TrimWhereField", "path": "TrimWhereField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [690, 47], "end": [690, 55], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/value.rs:690`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
