# `sqlparser::ast::ddl::CreateFunction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateFunction.json).

<a id="op-8a6321fdd652c2fe6923cc21"></a>
## CreateFunction

`struct` · `sqlparser::ast::ddl::CreateFunction` · sqlparser 0.62.0

```rust
struct CreateFunction
```

Source: `src/ast/ddl.rs:3569`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE FUNCTION statement

<a id="op-e55c0fda0e4839bd14413917"></a>
## args

`struct_field` · `sqlparser::ast::ddl::CreateFunction::args` · sqlparser 0.62.0

```rust
args: Option<Vec<ast::OperateFunctionArg>>
```

Source: `src/ast/ddl.rs:3583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of arguments for the function.

<a id="op-f9f0aa57d8355af85b77efc8"></a>
## behavior

`struct_field` · `sqlparser::ast::ddl::CreateFunction::behavior` · sqlparser 0.62.0

```rust
behavior: Option<ast::FunctionBehavior>
```

Source: `src/ast/ddl.rs:3599`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Behavior attribute for the function

IMMUTABLE | STABLE | VOLATILE

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-b9d082425546daa8a552b7cc"></a>
## called_on_null

`struct_field` · `sqlparser::ast::ddl::CreateFunction::called_on_null` · sqlparser 0.62.0

```rust
called_on_null: Option<ast::FunctionCalledOnNull>
```

Source: `src/ast/ddl.rs:3603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CALLED ON NULL INPUT | RETURNS NULL ON NULL INPUT | STRICT

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-e8127ab60aed4c0601795894"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateFunction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3565, 17], "end": [3565, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-125c8baf3e7fba7cb955de20"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateFunction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateFunction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3565, 51], "end": [3565, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f713c6e38a539ccffbe8d3e3"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateFunction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3566, 49], "end": [3566, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3566`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-187cab72e64ec22ebce8136f"></a>
## determinism_specifier

`struct_field` · `sqlparser::ast::ddl::CreateFunction::determinism_specifier` · sqlparser 0.62.0

```rust
determinism_specifier: Option<ast::FunctionDeterminismSpecifier>
```

Source: `src/ast/ddl.rs:3629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determinism keyword used for non-sql UDF definitions.

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#syntax_11)

<a id="op-51b3e6ccb7361ecb6d27a371"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateFunction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3565, 24], "end": [3565, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11072ab8574384ea6575b3ef"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3646, 1], "end": [3725, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1816890329a4e3958a0a95d"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3565, 10], "end": [3565, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0e175ac03120a6aabd4f008"></a>
## function_body

`struct_field` · `sqlparser::ast::ddl::CreateFunction::function_body` · sqlparser 0.62.0

```rust
function_body: Option<ast::CreateFunctionBody>
```

Source: `src/ast/ddl.rs:3593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression that defines the function.

Examples:
```sql
AS ((SELECT 1))
AS "console.log();"
```

<a id="op-bdd2f65c71e010c6b8b79aa2"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateFunction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3565, 56], "end": [3565, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad4ec108e08c8f1394a5866b"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateFunction::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:3579`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if this is a `CREATE IF NOT EXISTS FUNCTION` statement

<a id="op-4510d60e68e24064ed45fc2f"></a>
## language

`struct_field` · `sqlparser::ast::ddl::CreateFunction::language` · sqlparser 0.62.0

```rust
language: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:3625`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Language used in a UDF definition.

Example:
```sql
CREATE FUNCTION foo() LANGUAGE js AS "console.log();"
```
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_a_javascript_udf)

<a id="op-79f18a7406e950f213e88bef"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateFunction::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:3581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the function to be created.

<a id="op-593747ae91cfc409b999450d"></a>
## options

`struct_field` · `sqlparser::ast::ddl::CreateFunction::options` · sqlparser 0.62.0

```rust
options: Option<Vec<ast::SqlOption>>
```

Source: `src/ast/ddl.rs:3633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of options for creating the function.

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#syntax_11)

<a id="op-182ced1da37acd173a26301c"></a>
## or_alter

`struct_field` · `sqlparser::ast::ddl::CreateFunction::or_alter` · sqlparser 0.62.0

```rust
or_alter: bool
```

Source: `src/ast/ddl.rs:3573`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if this is a `CREATE OR ALTER FUNCTION` statement

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql?view=sql-server-ver16#or-alter)

<a id="op-1f8ce991910425c24028724e"></a>
## or_replace

`struct_field` · `sqlparser::ast::ddl::CreateFunction::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/ddl.rs:3575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if this is a `CREATE OR REPLACE FUNCTION` statement

<a id="op-0147a6edf32a02bf191e1673"></a>
## parallel

`struct_field` · `sqlparser::ast::ddl::CreateFunction::parallel` · sqlparser 0.62.0

```rust
parallel: Option<ast::FunctionParallel>
```

Source: `src/ast/ddl.rs:3607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PARALLEL { UNSAFE | RESTRICTED | SAFE }

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-832f7b959a9c38117eb3c96f"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateFunction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3565, 35], "end": [3565, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09b906d161b07011e65111b5"></a>
## remote_connection

`struct_field` · `sqlparser::ast::ddl::CreateFunction::remote_connection` · sqlparser 0.62.0

```rust
remote_connection: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:3643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Connection resource for a remote function.

Example:
```sql
CREATE FUNCTION foo()
RETURNS FLOAT64
REMOTE WITH CONNECTION us.myconnection
```
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_a_remote_function)

<a id="op-3da624f40012d3c49716336d"></a>
## return_type

`struct_field` · `sqlparser::ast::ddl::CreateFunction::return_type` · sqlparser 0.62.0

```rust
return_type: Option<FunctionReturnType>
```

Source: `src/ast/ddl.rs:3585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The return type of the function.

<a id="op-3a735de933f1c2e2a9f36f1c"></a>
## security

`struct_field` · `sqlparser::ast::ddl::CreateFunction::security` · sqlparser 0.62.0

```rust
security: Option<ast::FunctionSecurity>
```

Source: `src/ast/ddl.rs:3611`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SECURITY { DEFINER | INVOKER }

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-2d341303c5188227c32d4662"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateFunction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3566, 38], "end": [3566, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3566`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-418741971d35c3417645f9b6"></a>
## set_params

`struct_field` · `sqlparser::ast::ddl::CreateFunction::set_params` · sqlparser 0.62.0

```rust
set_params: Vec<ast::FunctionDefinitionSetParam>
```

Source: `src/ast/ddl.rs:3615`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SET configuration_parameter clauses

[PostgreSQL](https://www.postgresql.org/docs/current/sql-createfunction.html)

<a id="op-0a62907995cb17a26e2bd987"></a>
## temporary

`struct_field` · `sqlparser::ast::ddl::CreateFunction::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/ddl.rs:3577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if this is a `CREATE TEMPORARY FUNCTION` statement

<a id="op-e97cecfa2e3018270fdb8072"></a>
## using

`struct_field` · `sqlparser::ast::ddl::CreateFunction::using` · sqlparser 0.62.0

```rust
using: Option<ast::CreateFunctionUsing>
```

Source: `src/ast/ddl.rs:3617`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

USING ... (Hive only)

<a id="op-73244228029703408bd4ffe7"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3567, 47], "end": [3567, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-770acf388e93be1c1f20b65b"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3567, 40], "end": [3567, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
