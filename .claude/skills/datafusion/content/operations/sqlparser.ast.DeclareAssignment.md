# `sqlparser::ast::DeclareAssignment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DeclareAssignment.json).

<a id="op-3efe80f1892b64094f374277"></a>
## DeclareAssignment

`enum` · `sqlparser::ast::DeclareAssignment` · sqlparser 0.62.0

```rust
enum DeclareAssignment
```

Source: `src/ast/mod.rs:2966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents an expression assignment within a variable `DECLARE` statement.

Examples:
```sql
DECLARE variable_name := 42
DECLARE variable_name DEFAULT 42
```

<a id="op-276e0b6a88e1767e5e0ecf4e"></a>
## Default

`variant` · `sqlparser::ast::DeclareAssignment::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/mod.rs:2971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression assigned via the `DEFAULT` keyword

<a id="op-85c4cc9f72009db3978fbebb"></a>
## DuckAssignment

`variant` · `sqlparser::ast::DeclareAssignment::DuckAssignment` · sqlparser 0.62.0

```rust
DuckAssignment
```

Source: `src/ast/mod.rs:2979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression assigned via the `:=` syntax

Example:
```sql
DECLARE variable_name := 42;
```

<a id="op-68981a189fcb4ecd132c3c36"></a>
## Expr

`variant` · `sqlparser::ast::DeclareAssignment::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/mod.rs:2968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Plain expression specified.

<a id="op-26e6af74668efb1fc9fd3d8a"></a>
## For

`variant` · `sqlparser::ast::DeclareAssignment::For` · sqlparser 0.62.0

```rust
For
```

Source: `src/ast/mod.rs:2987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression via the `FOR` keyword

Example:
```sql
DECLARE c1 CURSOR FOR res
```

<a id="op-37626b93833368700e32f183"></a>
## MsSqlAssignment

`variant` · `sqlparser::ast::DeclareAssignment::MsSqlAssignment` · sqlparser 0.62.0

```rust
MsSqlAssignment
```

Source: `src/ast/mod.rs:2995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression via the `=` syntax.

Example:
```sql
DECLARE @variable AS INT = 100
```

<a id="op-935b89cf4ad8fbf947ab736b"></a>
## clone

`function` · `sqlparser::ast::DeclareAssignment::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DeclareAssignment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 17], "end": [2963, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88e281ed4e4c4c66ca947ee2"></a>
## cmp

`function` · `sqlparser::ast::DeclareAssignment::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DeclareAssignment) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 51], "end": [2963, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ee5519a7c7d1bc517ec60e9"></a>
## deserialize

`function` · `sqlparser::ast::DeclareAssignment::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2964, 49], "end": [2964, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-052ef0c032eb4ff15d5014a8"></a>
## eq

`function` · `sqlparser::ast::DeclareAssignment::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DeclareAssignment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 24], "end": [2963, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05a684935a3f5d92a6468cb0"></a>
## fmt

`function` · `sqlparser::ast::DeclareAssignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 10], "end": [2963, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-480e6f68cbb57dc4a137ab00"></a>
## fmt

`function` · `sqlparser::ast::DeclareAssignment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2998, 1], "end": [3018, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f528dfbd3648ad3889ca77dd"></a>
## hash

`function` · `sqlparser::ast::DeclareAssignment::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 56], "end": [2963, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0ec942d5cc9c55ef6fa1053"></a>
## partial_cmp

`function` · `sqlparser::ast::DeclareAssignment::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DeclareAssignment) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 35], "end": [2963, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-617b06fe2f444279327a1465"></a>
## serialize

`function` · `sqlparser::ast::DeclareAssignment::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2964, 38], "end": [2964, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01600484b5490cf7bb280013"></a>
## visit

`function` · `sqlparser::ast::DeclareAssignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2965, 40], "end": [2965, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ebcff3232ac91a6cee242e4"></a>
## visit

`function` · `sqlparser::ast::DeclareAssignment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareAssignment", "path": "DeclareAssignment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2965, 47], "end": [2965, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
