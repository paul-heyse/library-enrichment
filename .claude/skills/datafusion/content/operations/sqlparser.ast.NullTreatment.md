# `sqlparser::ast::NullTreatment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.NullTreatment.json).

<a id="op-fd81823ba9595c5f6be8245e"></a>
## NullTreatment

`enum` · `sqlparser::ast::NullTreatment` · sqlparser 0.62.0

```rust
enum NullTreatment
```

Source: `src/ast/mod.rs:2391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies Ignore / Respect NULL within window functions.
For example
`FIRST_VALUE(column2) IGNORE NULLS OVER (PARTITION BY column1)`
How NULL values are treated in certain window functions.

<a id="op-bf1bd07ec2d5ad5ebc9fcbee"></a>
## IgnoreNulls

`variant` · `sqlparser::ast::NullTreatment::IgnoreNulls` · sqlparser 0.62.0

```rust
IgnoreNulls
```

Source: `src/ast/mod.rs:2393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Ignore NULL values (e.g. `IGNORE NULLS`).

<a id="op-758b8740f8e864892c2ebdcc"></a>
## RespectNulls

`variant` · `sqlparser::ast::NullTreatment::RespectNulls` · sqlparser 0.62.0

```rust
RespectNulls
```

Source: `src/ast/mod.rs:2395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Respect NULL values (e.g. `RESPECT NULLS`).

<a id="op-84396fc757d431b46361cdfc"></a>
## clone

`function` · `sqlparser::ast::NullTreatment::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NullTreatment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2387, 23], "end": [2387, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a989c130b71d1cf5e028702f"></a>
## cmp

`function` · `sqlparser::ast::NullTreatment::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NullTreatment) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2387, 57], "end": [2387, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8de1653c4a86d839db5d42ab"></a>
## deserialize

`function` · `sqlparser::ast::NullTreatment::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2388, 49], "end": [2388, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a43871a2cbd48a09e5891e38"></a>
## eq

`function` · `sqlparser::ast::NullTreatment::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NullTreatment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2387, 30], "end": [2387, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0633988465811e342a586fb9"></a>
## fmt

`function` · `sqlparser::ast::NullTreatment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2398, 1], "end": [2405, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9781d4040af87260cd6b9b87"></a>
## fmt

`function` · `sqlparser::ast::NullTreatment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2387, 10], "end": [2387, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88a111213e34da17fb42ee95"></a>
## hash

`function` · `sqlparser::ast::NullTreatment::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2387, 62], "end": [2387, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bcf72f4f3b98cc1db5010fc"></a>
## partial_cmp

`function` · `sqlparser::ast::NullTreatment::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NullTreatment) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2387, 41], "end": [2387, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a51903c73d6003f71db4a71b"></a>
## serialize

`function` · `sqlparser::ast::NullTreatment::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2388, 38], "end": [2388, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c25f11296c8af2f2867acb82"></a>
## visit

`function` · `sqlparser::ast::NullTreatment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2389, 40], "end": [2389, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e5d0e29fcf4c5410f055c2"></a>
## visit

`function` · `sqlparser::ast::NullTreatment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullTreatment", "path": "NullTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2389, 47], "end": [2389, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
