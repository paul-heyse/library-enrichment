# `sqlparser::ast::RaiseStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.RaiseStatement.json).

<a id="op-23ff2b4a827bdc3b655accea"></a>
## RaiseStatement

`struct` · `sqlparser::ast::RaiseStatement` · sqlparser 0.62.0

```rust
struct RaiseStatement
```

Source: `src/ast/mod.rs:2883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `RAISE` statement.

Examples:
```sql
RAISE USING MESSAGE = 'error';

RAISE myerror;
```

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#raise)
[Snowflake](https://docs.snowflake.com/en/sql-reference/snowflake-scripting/raise)

<a id="op-9b286b2da1a5178e5a6801a8"></a>
## clone

`function` · `sqlparser::ast::RaiseStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RaiseStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2880, 17], "end": [2880, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ee3e1ab71061d19d54c7fac"></a>
## cmp

`function` · `sqlparser::ast::RaiseStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RaiseStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2880, 51], "end": [2880, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a00c15835088e6ffc2dad2f"></a>
## deserialize

`function` · `sqlparser::ast::RaiseStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2881, 49], "end": [2881, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f5363ba19c7aab139500b90"></a>
## eq

`function` · `sqlparser::ast::RaiseStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RaiseStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2880, 24], "end": [2880, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-657bdaa6d912393a49d4625a"></a>
## fmt

`function` · `sqlparser::ast::RaiseStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2880, 10], "end": [2880, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-754967b00486567097ea936d"></a>
## fmt

`function` · `sqlparser::ast::RaiseStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2888, 1], "end": [2899, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-983dd5433a65399505420b9f"></a>
## hash

`function` · `sqlparser::ast::RaiseStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2880, 56], "end": [2880, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caf62b952207bb6530fbd6c1"></a>
## partial_cmp

`function` · `sqlparser::ast::RaiseStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RaiseStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2880, 35], "end": [2880, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcd1aeb3fdc616a04f637a53"></a>
## serialize

`function` · `sqlparser::ast::RaiseStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2881, 38], "end": [2881, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63dc6d5f1c85ef7f4fc99b7d"></a>
## span

`function` · `sqlparser::ast::RaiseStatement::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "super::RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [790, 1], "end": [796, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:791`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fec67395f25ae0904e3cd050"></a>
## value

`struct_field` · `sqlparser::ast::RaiseStatement::value` · sqlparser 0.62.0

```rust
value: Option<RaiseStatementValue>
```

Source: `src/ast/mod.rs:2885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional value provided to the RAISE statement.

<a id="op-53ad4176b7b95d1319f94ca4"></a>
## visit

`function` · `sqlparser::ast::RaiseStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2882, 40], "end": [2882, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2882`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59c930111d5b5b20cdf39d88"></a>
## visit

`function` · `sqlparser::ast::RaiseStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2882, 47], "end": [2882, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2882`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
