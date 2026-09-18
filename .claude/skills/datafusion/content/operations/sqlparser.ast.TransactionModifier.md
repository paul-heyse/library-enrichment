# `sqlparser::ast::TransactionModifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TransactionModifier.json).

<a id="op-dd1ce80a9ff8605bcd21dc70"></a>
## TransactionModifier

`enum` · `sqlparser::ast::TransactionModifier` · sqlparser 0.62.0

```rust
enum TransactionModifier
```

Source: `src/ast/mod.rs:9115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modifier for the transaction in the `BEGIN` syntax

SQLite: <https://sqlite.org/lang_transaction.html>
MS-SQL: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/try-catch-transact-sql>

<a id="op-51aecec20d601a9ece7d1290"></a>
## Catch

`variant` · `sqlparser::ast::TransactionModifier::Catch` · sqlparser 0.62.0

```rust
Catch
```

Source: `src/ast/mod.rs:9125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CATCH block modifier (MS-SQL style TRY/CATCH).

<a id="op-fb88131e1a87e7fd3e588b02"></a>
## Deferred

`variant` · `sqlparser::ast::TransactionModifier::Deferred` · sqlparser 0.62.0

```rust
Deferred
```

Source: `src/ast/mod.rs:9117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DEFERRED transaction modifier.

<a id="op-3fdb28c743d6b8f4e55ba3b4"></a>
## Exclusive

`variant` · `sqlparser::ast::TransactionModifier::Exclusive` · sqlparser 0.62.0

```rust
Exclusive
```

Source: `src/ast/mod.rs:9121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

EXCLUSIVE transaction modifier.

<a id="op-2fb702d406949db1f085b841"></a>
## Immediate

`variant` · `sqlparser::ast::TransactionModifier::Immediate` · sqlparser 0.62.0

```rust
Immediate
```

Source: `src/ast/mod.rs:9119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

IMMEDIATE transaction modifier.

<a id="op-de05911be9c0d7253619310f"></a>
## Try

`variant` · `sqlparser::ast::TransactionModifier::Try` · sqlparser 0.62.0

```rust
Try
```

Source: `src/ast/mod.rs:9123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TRY block modifier (MS-SQL style TRY/CATCH).

<a id="op-252251182deea3dad239ccad"></a>
## clone

`function` · `sqlparser::ast::TransactionModifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TransactionModifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9112, 23], "end": [9112, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-009bd5181ee6c19147b0d743"></a>
## cmp

`function` · `sqlparser::ast::TransactionModifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TransactionModifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9112, 57], "end": [9112, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-741326febf0e2a0e589c408f"></a>
## deserialize

`function` · `sqlparser::ast::TransactionModifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9113, 49], "end": [9113, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d770bb635f57cd69669abad2"></a>
## eq

`function` · `sqlparser::ast::TransactionModifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TransactionModifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9112, 30], "end": [9112, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48f8689302b65e5f8535182e"></a>
## fmt

`function` · `sqlparser::ast::TransactionModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9128, 1], "end": [9139, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6eadbd6223a8d06ce427df5"></a>
## fmt

`function` · `sqlparser::ast::TransactionModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9112, 10], "end": [9112, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac8544b61ec122bcf0e5b056"></a>
## hash

`function` · `sqlparser::ast::TransactionModifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9112, 62], "end": [9112, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd59195213054decd93437e7"></a>
## partial_cmp

`function` · `sqlparser::ast::TransactionModifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TransactionModifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9112, 41], "end": [9112, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ed37242aa5a4abe75799844"></a>
## serialize

`function` · `sqlparser::ast::TransactionModifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9113, 38], "end": [9113, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a2d5cde9a0f2c8960646d71"></a>
## visit

`function` · `sqlparser::ast::TransactionModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9114, 40], "end": [9114, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c21c930d9b7b6215f3393f8d"></a>
## visit

`function` · `sqlparser::ast::TransactionModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionModifier", "path": "TransactionModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9114, 47], "end": [9114, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
