# `sqlparser::ast::CreateFunctionBody`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateFunctionBody.json).

<a id="op-b7a9173bc4d148daf0dd239d"></a>
## CreateFunctionBody

`enum` · `sqlparser::ast::CreateFunctionBody` · sqlparser 0.62.0

```rust
enum CreateFunctionBody
```

Source: `src/ast/mod.rs:10117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represent the expression body of a `CREATE FUNCTION` statement as well as
where within the statement, the body shows up.

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#syntax_11
[PostgreSQL]: https://www.postgresql.org/docs/15/sql-createfunction.html
[MsSql]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql

<a id="op-726d5636b16e61ee347122d3"></a>
## AsAfterOptions

`variant` · `sqlparser::ast::CreateFunctionBody::AsAfterOptions` · sqlparser 0.62.0

```rust
AsAfterOptions
```

Source: `src/ast/mod.rs:10154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A function body expression using the 'AS' keyword and shows up
after any `OPTIONS` clause.

Example:
```sql
CREATE FUNCTION myfunc(x FLOAT64, y FLOAT64) RETURNS FLOAT64
OPTIONS(description="desc")
AS (x * y);
```

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#syntax_11

<a id="op-9c1cb8ee5eabf817547a23e8"></a>
## AsBeforeOptions

`variant` · `sqlparser::ast::CreateFunctionBody::AsBeforeOptions` · sqlparser 0.62.0

```rust
AsBeforeOptions
```

Source: `src/ast/mod.rs:10130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A function body expression using the 'AS' keyword and shows up
before any `OPTIONS` clause.

Example:
```sql
CREATE FUNCTION myfunc(x FLOAT64, y FLOAT64) RETURNS FLOAT64
AS (x * y)
OPTIONS(description="desc");
```

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#syntax_11
[PostgreSQL]: https://www.postgresql.org/docs/current/sql-createfunction.html

<a id="op-791fcc1810ec19941710e2f0"></a>
## AsBeginEnd

`variant` · `sqlparser::ast::CreateFunctionBody::AsBeginEnd` · sqlparser 0.62.0

```rust
AsBeginEnd
```

Source: `src/ast/mod.rs:10170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function body with statements before the `RETURN` keyword.

Example:
```sql
CREATE FUNCTION my_scalar_udf(a INT, b INT)
RETURNS INT
AS
BEGIN
    DECLARE c INT;
    SET c = a + b;
    RETURN c;
END
```

[MsSql]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql

<a id="op-58fdd7d437a54a0d1a56c632"></a>
## AsReturnExpr

`variant` · `sqlparser::ast::CreateFunctionBody::AsReturnExpr` · sqlparser 0.62.0

```rust
AsReturnExpr
```

Source: `src/ast/mod.rs:10193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function body expression using the 'AS RETURN' keywords

Example:
```sql
CREATE FUNCTION myfunc(a INT, b INT)
RETURNS TABLE
AS RETURN (SELECT a + b AS sum);
```

[MsSql]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql

<a id="op-a7f2c78629f921d427470bf7"></a>
## AsReturnSelect

`variant` · `sqlparser::ast::CreateFunctionBody::AsReturnSelect` · sqlparser 0.62.0

```rust
AsReturnSelect
```

Source: `src/ast/mod.rs:10205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function body expression using the 'AS RETURN' keywords, with an un-parenthesized SELECT query

Example:
```sql
CREATE FUNCTION myfunc(a INT, b INT)
RETURNS TABLE
AS RETURN SELECT a + b AS sum;
```

[MsSql]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql?view=sql-server-ver16#select_stmt

<a id="op-ff5a3d7f1a92e3d625d8a9c7"></a>
## Return

`variant` · `sqlparser::ast::CreateFunctionBody::Return` · sqlparser 0.62.0

```rust
Return
```

Source: `src/ast/mod.rs:10181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function body expression using the 'RETURN' keyword.

Example:
```sql
CREATE FUNCTION myfunc(a INTEGER, IN b INTEGER = 1) RETURNS INTEGER
LANGUAGE SQL
RETURN a + b;
```

[PostgreSQL]: https://www.postgresql.org/docs/current/sql-createfunction.html

<a id="op-cb52bd1be544b2b67f3a6c51"></a>
## clone

`function` · `sqlparser::ast::CreateFunctionBody::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateFunctionBody
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10114, 17], "end": [10114, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a46264a4f39dc166a9480b4"></a>
## cmp

`function` · `sqlparser::ast::CreateFunctionBody::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateFunctionBody) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10114, 51], "end": [10114, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11be5702f0e15c143015280e"></a>
## deserialize

`function` · `sqlparser::ast::CreateFunctionBody::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10115, 49], "end": [10115, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09d64bc19490b6f12e1ee33a"></a>
## eq

`function` · `sqlparser::ast::CreateFunctionBody::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateFunctionBody) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10114, 24], "end": [10114, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6459e24398ae773637271ede"></a>
## fmt

`function` · `sqlparser::ast::CreateFunctionBody::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10114, 10], "end": [10114, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b7604bea920bf7b73ecce89"></a>
## hash

`function` · `sqlparser::ast::CreateFunctionBody::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10114, 56], "end": [10114, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f764de234a7e03fa1507b0d3"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateFunctionBody::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateFunctionBody) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10114, 35], "end": [10114, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2452427852ed39fa94464e60"></a>
## serialize

`function` · `sqlparser::ast::CreateFunctionBody::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10115, 38], "end": [10115, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-235ed808f3a04ed5c59a7d77"></a>
## visit

`function` · `sqlparser::ast::CreateFunctionBody::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10116, 40], "end": [10116, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd99da77d336460873c089cb"></a>
## visit

`function` · `sqlparser::ast::CreateFunctionBody::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateFunctionBody", "path": "CreateFunctionBody"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10116, 47], "end": [10116, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
