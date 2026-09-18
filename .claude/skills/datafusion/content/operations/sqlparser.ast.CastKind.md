# `sqlparser::ast::CastKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CastKind.json).

<a id="op-654043c421b9e745681c06c7"></a>
## CastKind

`enum` · `sqlparser::ast::CastKind` · sqlparser 0.62.0

```rust
enum CastKind
```

Source: `src/ast/mod.rs:749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The syntax used for in a cast expression.

<a id="op-02e3a77ceb860ea32a3e53e6"></a>
## Cast

`variant` · `sqlparser::ast::CastKind::Cast` · sqlparser 0.62.0

```rust
Cast
```

Source: `src/ast/mod.rs:751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The standard SQL cast syntax, e.g. `CAST(<expr> as <datatype>)`

<a id="op-eeb6eb0384b843a24c12bd52"></a>
## DoubleColon

`variant` · `sqlparser::ast::CastKind::DoubleColon` · sqlparser 0.62.0

```rust
DoubleColon
```

Source: `src/ast/mod.rs:762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<expr> :: <datatype>`

<a id="op-76978da5397683a19ab0b3c2"></a>
## SafeCast

`variant` · `sqlparser::ast::CastKind::SafeCast` · sqlparser 0.62.0

```rust
SafeCast
```

Source: `src/ast/mod.rs:760`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A cast that returns `NULL` on failure, bigQuery-specific ,  e.g. `SAFE_CAST(<expr> as <datatype>)`.

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/functions-and-operators#safe_casting>.

<a id="op-cdcce139198ca0b9259b2c13"></a>
## TryCast

`variant` · `sqlparser::ast::CastKind::TryCast` · sqlparser 0.62.0

```rust
TryCast
```

Source: `src/ast/mod.rs:756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A cast that returns `NULL` on failure, e.g. `TRY_CAST(<expr> as <datatype>)`.

See <https://docs.snowflake.com/en/sql-reference/functions/try_cast>.
See <https://learn.microsoft.com/en-us/sql/t-sql/functions/try-cast-transact-sql>.

<a id="op-52ca6ac502b1f69dffd25d5d"></a>
## clone

`function` · `sqlparser::ast::CastKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CastKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 17], "end": [746, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25f1080f9f0ac8c3f7a3353e"></a>
## cmp

`function` · `sqlparser::ast::CastKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CastKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 51], "end": [746, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-247e4a2b560adcba5d18dbd1"></a>
## deserialize

`function` · `sqlparser::ast::CastKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [747, 49], "end": [747, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:747`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a87b37c95ceeaeac5c59e0ef"></a>
## eq

`function` · `sqlparser::ast::CastKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CastKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 24], "end": [746, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8a49a8b962145803becbd10"></a>
## fmt

`function` · `sqlparser::ast::CastKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 10], "end": [746, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a133c7cb286695c5824fa7c7"></a>
## hash

`function` · `sqlparser::ast::CastKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 56], "end": [746, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59f46e541ac297abd4da208a"></a>
## partial_cmp

`function` · `sqlparser::ast::CastKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CastKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [746, 35], "end": [746, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-159d3bbbb407b6abe654aa85"></a>
## serialize

`function` · `sqlparser::ast::CastKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [747, 38], "end": [747, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:747`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00599cd7cf07e1194f512c7c"></a>
## visit

`function` · `sqlparser::ast::CastKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 40], "end": [748, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8885abe34f2a81bed2ac8671"></a>
## visit

`function` · `sqlparser::ast::CastKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CastKind", "path": "CastKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 47], "end": [748, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
