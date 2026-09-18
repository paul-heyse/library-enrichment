# `sqlparser::ast::HavingBound`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HavingBound.json).

<a id="op-f703728db3a20e7d6b69e1b0"></a>
## HavingBound

`struct` · `sqlparser::ast::HavingBound` · sqlparser 0.62.0

```rust
struct HavingBound
```

Source: `src/ast/mod.rs:8429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `HAVING` clause in a call to `ANY_VALUE` on BigQuery.

<a id="op-9b02579f3b9dc9ada6794055"></a>
## 0

`struct_field` · `sqlparser::ast::HavingBound::0` · sqlparser 0.62.0

```rust
0: HavingBoundKind
```

Source: `src/ast/mod.rs:8429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78fb6d45e3977f2849965149"></a>
## 1

`struct_field` · `sqlparser::ast::HavingBound::1` · sqlparser 0.62.0

```rust
1: Expr
```

Source: `src/ast/mod.rs:8429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-435652f08769ccbd9c489109"></a>
## clone

`function` · `sqlparser::ast::HavingBound::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HavingBound
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8426, 17], "end": [8426, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0f23e6840840a6419b04773"></a>
## cmp

`function` · `sqlparser::ast::HavingBound::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HavingBound) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8426, 51], "end": [8426, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c5f53afb579141d5d46807d"></a>
## deserialize

`function` · `sqlparser::ast::HavingBound::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8427, 49], "end": [8427, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8427`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bed7e0b70cc8463a500b9d5"></a>
## eq

`function` · `sqlparser::ast::HavingBound::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HavingBound) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8426, 24], "end": [8426, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-458dd54ef1fbdcf971381be5"></a>
## fmt

`function` · `sqlparser::ast::HavingBound::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8431, 1], "end": [8435, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8432`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7c66a9dd56c7f905eb68a4e"></a>
## fmt

`function` · `sqlparser::ast::HavingBound::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8426, 10], "end": [8426, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7da77d1639127ff365e876c4"></a>
## hash

`function` · `sqlparser::ast::HavingBound::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8426, 56], "end": [8426, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6deab81dbe6ef69620372456"></a>
## partial_cmp

`function` · `sqlparser::ast::HavingBound::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HavingBound) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8426, 35], "end": [8426, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4db7b3dc2375fbc5918152b6"></a>
## serialize

`function` · `sqlparser::ast::HavingBound::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8427, 38], "end": [8427, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8427`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-107f97db13e0c3daa75df392"></a>
## visit

`function` · `sqlparser::ast::HavingBound::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8428, 40], "end": [8428, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8428`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-744d39cd7be275d11c2b521d"></a>
## visit

`function` · `sqlparser::ast::HavingBound::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBound", "path": "HavingBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8428, 47], "end": [8428, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8428`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
