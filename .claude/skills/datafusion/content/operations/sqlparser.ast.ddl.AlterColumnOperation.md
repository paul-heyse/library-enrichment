# `sqlparser::ast::ddl::AlterColumnOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterColumnOperation.json).

<a id="op-4ca281a04158a4b6137750c4"></a>
## AlterColumnOperation

`enum` · `sqlparser::ast::ddl::AlterColumnOperation` · sqlparser 0.62.0

```rust
enum AlterColumnOperation
```

Source: `src/ast/ddl.rs:1268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ALTER COLUMN` (`Statement::AlterTable`) operation

<a id="op-407f93265f7f35029096b856"></a>
## AddGenerated

`variant` · `sqlparser::ast::ddl::AlterColumnOperation::AddGenerated` · sqlparser 0.62.0

```rust
AddGenerated
```

Source: `src/ast/ddl.rs:1294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ADD GENERATED { ALWAYS | BY DEFAULT } AS IDENTITY [ ( sequence_options ) ]`

Note: this is a PostgreSQL-specific operation.

<a id="op-9265b61d8d561c019b5d8c30"></a>
## DropDefault

`variant` · `sqlparser::ast::ddl::AlterColumnOperation::DropDefault` · sqlparser 0.62.0

```rust
DropDefault
```

Source: `src/ast/ddl.rs:1280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP DEFAULT`

<a id="op-062745c5271d9aa58340f6cb"></a>
## DropNotNull

`variant` · `sqlparser::ast::ddl::AlterColumnOperation::DropNotNull` · sqlparser 0.62.0

```rust
DropNotNull
```

Source: `src/ast/ddl.rs:1272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP NOT NULL`

<a id="op-eed338022c5d79229625b4c4"></a>
## SetDataType

`variant` · `sqlparser::ast::ddl::AlterColumnOperation::SetDataType` · sqlparser 0.62.0

```rust
SetDataType
```

Source: `src/ast/ddl.rs:1282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[SET DATA] TYPE <data_type> [USING <expr>]`

<a id="op-0a450a1735ae1106c819df8a"></a>
## SetDefault

`variant` · `sqlparser::ast::ddl::AlterColumnOperation::SetDefault` · sqlparser 0.62.0

```rust
SetDefault
```

Source: `src/ast/ddl.rs:1275`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET DEFAULT <expr>`
Set the column default value.

<a id="op-3abbf717ddbe12ccc82ca0d6"></a>
## SetNotNull

`variant` · `sqlparser::ast::ddl::AlterColumnOperation::SetNotNull` · sqlparser 0.62.0

```rust
SetNotNull
```

Source: `src/ast/ddl.rs:1270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET NOT NULL`

<a id="op-f49c43382fe09439702e69ee"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterColumnOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterColumnOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1265, 17], "end": [1265, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-feb2a04c18aa00eb89584fb2"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterColumnOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterColumnOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1265, 51], "end": [1265, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-857fadf1a4c2ff98258e5ba6"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterColumnOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1266, 49], "end": [1266, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dab741f8f6929b82569f8c6"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterColumnOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterColumnOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1265, 24], "end": [1265, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6425c7ade1caf03d01c566ec"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterColumnOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1265, 10], "end": [1265, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c70dac4998b75bcc417716c6"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterColumnOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1302, 1], "end": [1351, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36c364a274991f43751dd19a"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterColumnOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1265, 56], "end": [1265, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3755cfebc00f59673435239"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterColumnOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterColumnOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1265, 35], "end": [1265, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e05ce14e039228ac6fdc467"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterColumnOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1266, 38], "end": [1266, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96d1220b2a5401da4b8b97ba"></a>
## span

`function` · `sqlparser::ast::ddl::AlterColumnOperation::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "super::AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [890, 1], "end": [905, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4aa5fa4d1aeb1cd6e6f46bd9"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterColumnOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1267, 40], "end": [1267, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7e944e7d0cc0a6b8281052d"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterColumnOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterColumnOperation", "path": "AlterColumnOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1267, 47], "end": [1267, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
