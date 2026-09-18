# `sqlparser::ast::CreateFunctionUsing`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateFunctionUsing.json).

<a id="op-ed5d6a3d889464b1f5bff669"></a>
## CreateFunctionUsing

`enum` · `sqlparser::ast::CreateFunctionUsing` · sqlparser 0.62.0

```rust
enum CreateFunctionUsing
```

Source: `src/ast/mod.rs:10212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`USING` clause options for `CREATE FUNCTION` (e.g., JAR, FILE, ARCHIVE).

<a id="op-6ae95af7f05376228e6c238d"></a>
## Archive

`variant` · `sqlparser::ast::CreateFunctionUsing::Archive` · sqlparser 0.62.0

```rust
Archive
```

Source: `src/ast/mod.rs:10218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use an archive located at the given URI.

<a id="op-9bc1b81e5e1a4e7021756f9f"></a>
## File

`variant` · `sqlparser::ast::CreateFunctionUsing::File` · sqlparser 0.62.0

```rust
File
```

Source: `src/ast/mod.rs:10216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use a file located at the given URI.

<a id="op-6b2a88a90a55c0bdcac8c314"></a>
## Jar

`variant` · `sqlparser::ast::CreateFunctionUsing::Jar` · sqlparser 0.62.0

```rust
Jar
```

Source: `src/ast/mod.rs:10214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use a JAR file located at the given URI.

<a id="op-a89fe24df287368b94bd10cf"></a>
## clone

`function` · `sqlparser::ast::CreateFunctionUsing::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateFunctionUsing
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10208, 17], "end": [10208, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21cab84e9b1ade824d67012a"></a>
## cmp

`function` · `sqlparser::ast::CreateFunctionUsing::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateFunctionUsing) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10208, 51], "end": [10208, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dc5abe4590da8cedcc7648f"></a>
## deserialize

`function` · `sqlparser::ast::CreateFunctionUsing::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10209, 49], "end": [10209, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56b0f35dbbde0c648d38d46c"></a>
## eq

`function` · `sqlparser::ast::CreateFunctionUsing::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateFunctionUsing) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10208, 24], "end": [10208, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1925214246592aa5747b7cf1"></a>
## fmt

`function` · `sqlparser::ast::CreateFunctionUsing::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10208, 10], "end": [10208, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-331660d27ab6fec0c699c931"></a>
## fmt

`function` · `sqlparser::ast::CreateFunctionUsing::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10221, 1], "end": [10230, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10222`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-669f067bdfb4a52efc77c93d"></a>
## hash

`function` · `sqlparser::ast::CreateFunctionUsing::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10208, 56], "end": [10208, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf675077971ecb3a1b53a295"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateFunctionUsing::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateFunctionUsing) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10208, 35], "end": [10208, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63ce386bb0336a615c33edec"></a>
## serialize

`function` · `sqlparser::ast::CreateFunctionUsing::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10209, 38], "end": [10209, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a761d9ec5b6a7455ad3b5c"></a>
## visit

`function` · `sqlparser::ast::CreateFunctionUsing::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10210, 40], "end": [10210, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10210`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c4d71b96ddc035c40401f75"></a>
## visit

`function` · `sqlparser::ast::CreateFunctionUsing::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionUsing", "path": "CreateFunctionUsing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10210, 47], "end": [10210, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10210`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
