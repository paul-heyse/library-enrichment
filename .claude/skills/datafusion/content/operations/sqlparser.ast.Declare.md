# `sqlparser::ast::Declare`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Declare.json).

<a id="op-6d298301af5303ecf999966d"></a>
## Declare

`struct` · `sqlparser::ast::Declare` · sqlparser 0.62.0

```rust
struct Declare
```

Source: `src/ast/mod.rs:3082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `DECLARE` statement.
[PostgreSQL] [Snowflake] [BigQuery]

Examples:
```sql
DECLARE variable_name := 42
DECLARE liahona CURSOR FOR SELECT * FROM films;
```

[PostgreSQL]: https://www.postgresql.org/docs/current/sql-declare.html
[Snowflake]: https://docs.snowflake.com/en/sql-reference/snowflake-scripting/declare
[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#declare

<a id="op-bcff262f987581d9a90f7f77"></a>
## assignment

`struct_field` · `sqlparser::ast::Declare::assignment` · sqlparser 0.62.0

```rust
assignment: Option<DeclareAssignment>
```

Source: `src/ast/mod.rs:3090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression being assigned to the declared variable.

<a id="op-761b4be24dcb583d0c633cc7"></a>
## binary

`struct_field` · `sqlparser::ast::Declare::binary` · sqlparser 0.62.0

```rust
binary: Option<bool>
```

Source: `src/ast/mod.rs:3094`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Causes the cursor to return data in binary rather than in text format.

<a id="op-839a4910ddaf9accf7e435c9"></a>
## clone

`function` · `sqlparser::ast::Declare::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Declare
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3079, 17], "end": [3079, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e5892c541a843c521db1e9c"></a>
## cmp

`function` · `sqlparser::ast::Declare::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Declare) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3079, 51], "end": [3079, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fb12f56dd2465257f86a7cb"></a>
## data_type

`struct_field` · `sqlparser::ast::Declare::data_type` · sqlparser 0.62.0

```rust
data_type: Option<DataType>
```

Source: `src/ast/mod.rs:3088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Data-type assigned to the declared variable.
Example: `DECLARE x INT64 DEFAULT 42;

<a id="op-c218061839e6dc2550b0cc12"></a>
## declare_type

`struct_field` · `sqlparser::ast::Declare::declare_type` · sqlparser 0.62.0

```rust
declare_type: Option<DeclareType>
```

Source: `src/ast/mod.rs:3092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the type of the declared variable.

<a id="op-0b1f767f8e2ea69d2b7262a8"></a>
## deserialize

`function` · `sqlparser::ast::Declare::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3080, 49], "end": [3080, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56452403c6dc083fed8d5d3c"></a>
## eq

`function` · `sqlparser::ast::Declare::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Declare) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3079, 24], "end": [3079, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b2994b0f411f1ca93805735"></a>
## fmt

`function` · `sqlparser::ast::Declare::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3111, 1], "end": [3171, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:3112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ed6ea0a588b6eea6523b1cb"></a>
## fmt

`function` · `sqlparser::ast::Declare::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3079, 10], "end": [3079, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-499cebaeeccc391308d73ee8"></a>
## for_query

`struct_field` · `sqlparser::ast::Declare::for_query` · sqlparser 0.62.0

```rust
for_query: Option<Box<Query>>
```

Source: `src/ast/mod.rs:3108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR <query>` clause in a CURSOR declaration.

<a id="op-4cfc28d32e5f086b3e13c7f9"></a>
## hash

`function` · `sqlparser::ast::Declare::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3079, 56], "end": [3079, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12caa0244030a7e7ffd49af4"></a>
## hold

`struct_field` · `sqlparser::ast::Declare::hold` · sqlparser 0.62.0

```rust
hold: Option<bool>
```

Source: `src/ast/mod.rs:3106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

None = Not specified
Some(true) = WITH HOLD, specifies that the cursor can continue to be used after the transaction that created it successfully commits
Some(false) = WITHOUT HOLD, specifies that the cursor cannot be used outside of the transaction that created it

<a id="op-1bdfdd72356e0a0e797773e0"></a>
## names

`struct_field` · `sqlparser::ast::Declare::names` · sqlparser 0.62.0

```rust
names: Vec<Ident>
```

Source: `src/ast/mod.rs:3085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name(s) being declared.
Example: `DECLARE a, b, c DEFAULT 42;

<a id="op-2296a4dbd1ab68b65fad4f14"></a>
## partial_cmp

`function` · `sqlparser::ast::Declare::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Declare) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3079, 35], "end": [3079, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-917eda869de7f715c9a24159"></a>
## scroll

`struct_field` · `sqlparser::ast::Declare::scroll` · sqlparser 0.62.0

```rust
scroll: Option<bool>
```

Source: `src/ast/mod.rs:3102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

None = Not specified
Some(true) = SCROLL
Some(false) = NO SCROLL

<a id="op-3839d4e0066906bf6b5837c3"></a>
## sensitive

`struct_field` · `sqlparser::ast::Declare::sensitive` · sqlparser 0.62.0

```rust
sensitive: Option<bool>
```

Source: `src/ast/mod.rs:3098`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

None = Not specified
Some(true) = INSENSITIVE
Some(false) = ASENSITIVE

<a id="op-0b6b9628eea49f9b57dea16b"></a>
## serialize

`function` · `sqlparser::ast::Declare::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3080, 38], "end": [3080, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d884fa69e967a9ae6c3f224"></a>
## visit

`function` · `sqlparser::ast::Declare::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3081, 47], "end": [3081, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b07f83972053b89b78023db6"></a>
## visit

`function` · `sqlparser::ast::Declare::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Declare", "path": "Declare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3081, 40], "end": [3081, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
