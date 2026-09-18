# `sqlparser::ast::ddl::DistStyle`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DistStyle.json).

<a id="op-6fd7418b962214dd6a9c5c91"></a>
## DistStyle

`enum` · `sqlparser::ast::ddl::DistStyle` · sqlparser 0.62.0

```rust
enum DistStyle
```

Source: `src/ast/ddl.rs:3472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift distribution style for `CREATE TABLE`.

See [Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_TABLE_NEW.html)

<a id="op-c9004ef121670c465e9a9293"></a>
## All

`variant` · `sqlparser::ast::ddl::DistStyle::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/ddl.rs:3480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTSTYLE ALL`

<a id="op-8af49c82d73e92bc7df167e6"></a>
## Auto

`variant` · `sqlparser::ast::ddl::DistStyle::Auto` · sqlparser 0.62.0

```rust
Auto
```

Source: `src/ast/ddl.rs:3474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTSTYLE AUTO`

<a id="op-68f36bb7026fbd6dd702a07e"></a>
## Even

`variant` · `sqlparser::ast::ddl::DistStyle::Even` · sqlparser 0.62.0

```rust
Even
```

Source: `src/ast/ddl.rs:3476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTSTYLE EVEN`

<a id="op-97858ef9c564ecd1978a3e02"></a>
## Key

`variant` · `sqlparser::ast::ddl::DistStyle::Key` · sqlparser 0.62.0

```rust
Key
```

Source: `src/ast/ddl.rs:3478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTSTYLE KEY`

<a id="op-37aee85b71ca8e7682037f18"></a>
## clone

`function` · `sqlparser::ast::ddl::DistStyle::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DistStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3469, 17], "end": [3469, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d08240683be398259907e0cf"></a>
## cmp

`function` · `sqlparser::ast::ddl::DistStyle::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DistStyle) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3469, 51], "end": [3469, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b9b73f5e9ec6263b61ed924"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DistStyle::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3470, 49], "end": [3470, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3340030d65629b96bb111e55"></a>
## eq

`function` · `sqlparser::ast::ddl::DistStyle::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DistStyle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3469, 24], "end": [3469, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0e8b8b9dcb0d841434aeb2e"></a>
## fmt

`function` · `sqlparser::ast::ddl::DistStyle::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3483, 1], "end": [3492, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1a9dc0dca4c78a037cbdbbd"></a>
## fmt

`function` · `sqlparser::ast::ddl::DistStyle::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3469, 10], "end": [3469, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3ec8246f3ec62443dc2c6bc"></a>
## hash

`function` · `sqlparser::ast::ddl::DistStyle::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3469, 56], "end": [3469, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-056aede5ba24f279ebc6db5d"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DistStyle::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DistStyle) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3469, 35], "end": [3469, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9736a874f9d1a53c85b6432f"></a>
## serialize

`function` · `sqlparser::ast::ddl::DistStyle::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3470, 38], "end": [3470, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2366bb238bb6592292aadc54"></a>
## visit

`function` · `sqlparser::ast::ddl::DistStyle::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3471, 40], "end": [3471, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f304604ee88e1fe7496e98e5"></a>
## visit

`function` · `sqlparser::ast::ddl::DistStyle::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DistStyle", "path": "DistStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3471, 47], "end": [3471, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
