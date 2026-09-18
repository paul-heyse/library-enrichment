# `sqlparser::ast::CeilFloorKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CeilFloorKind.json).

<a id="op-8e45f4b9f5f662c4854c91e0"></a>
## CeilFloorKind

`enum` · `sqlparser::ast::CeilFloorKind` · sqlparser 0.62.0

```rust
enum CeilFloorKind
```

Source: `src/ast/mod.rs:817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The syntax used in a CEIL or FLOOR expression.

The `CEIL/FLOOR(<datetime value expression> TO <time unit>)` is an Amazon Kinesis Data Analytics extension.
See <https://docs.aws.amazon.com/kinesisanalytics/latest/sqlref/sql-reference-ceil.html> for
details.

Other dialects either support `CEIL/FLOOR( <expr> [, <scale>])` format or just
`CEIL/FLOOR(<expr>)`.

<a id="op-5b5b46fb32c0d2800a0763fc"></a>
## DateTimeField

`variant` · `sqlparser::ast::CeilFloorKind::DateTimeField` · sqlparser 0.62.0

```rust
DateTimeField
```

Source: `src/ast/mod.rs:819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CEIL( <expr> TO <DateTimeField>)`

<a id="op-f396d9016f8ebf15998bb649"></a>
## Scale

`variant` · `sqlparser::ast::CeilFloorKind::Scale` · sqlparser 0.62.0

```rust
Scale
```

Source: `src/ast/mod.rs:821`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CEIL( <expr> [, <scale>])`

<a id="op-39264d8d38567e980c5a0db4"></a>
## clone

`function` · `sqlparser::ast::CeilFloorKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CeilFloorKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 17], "end": [814, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33be3cad56ff4073aa213000"></a>
## cmp

`function` · `sqlparser::ast::CeilFloorKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CeilFloorKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 51], "end": [814, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-960d3c2e25bdc774d87837af"></a>
## deserialize

`function` · `sqlparser::ast::CeilFloorKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [815, 49], "end": [815, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-187d3f98ce665549634c0db9"></a>
## eq

`function` · `sqlparser::ast::CeilFloorKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CeilFloorKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 24], "end": [814, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-259144d5fd9673d50d464eec"></a>
## fmt

`function` · `sqlparser::ast::CeilFloorKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 10], "end": [814, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21dc916712f2476f7396497f"></a>
## hash

`function` · `sqlparser::ast::CeilFloorKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 56], "end": [814, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfd1fdd65bd9e05f9771f390"></a>
## partial_cmp

`function` · `sqlparser::ast::CeilFloorKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CeilFloorKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 35], "end": [814, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51873d4d93ce5d6d1586c709"></a>
## serialize

`function` · `sqlparser::ast::CeilFloorKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [815, 38], "end": [815, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-259b431cf421af7f92f17cbb"></a>
## visit

`function` · `sqlparser::ast::CeilFloorKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [816, 47], "end": [816, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:816`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51c186541511e883ba106844"></a>
## visit

`function` · `sqlparser::ast::CeilFloorKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CeilFloorKind", "path": "CeilFloorKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [816, 40], "end": [816, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:816`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
