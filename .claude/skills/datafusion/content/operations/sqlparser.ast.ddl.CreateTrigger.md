# `sqlparser::ast::ddl::CreateTrigger`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateTrigger.json).

<a id="op-0d20726f7c1a342ab578756d"></a>
## CreateTrigger

`struct` · `sqlparser::ast::ddl::CreateTrigger` · sqlparser 0.62.0

```rust
struct CreateTrigger
```

Source: `src/ast/ddl.rs:3952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE TRIGGER

Examples:

```sql
CREATE TRIGGER trigger_name
BEFORE INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

Postgres: <https://www.postgresql.org/docs/current/sql-createtrigger.html>
SQL Server: <https://learn.microsoft.com/en-us/sql/t-sql/statements/create-trigger-transact-sql>

<a id="op-660c11d048909bba587ccb9d"></a>
## characteristics

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::characteristics` · sqlparser 0.62.0

```rust
characteristics: Option<ConstraintCharacteristics>
```

Source: `src/ast/ddl.rs:4051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The characteristic of the trigger, which include whether the trigger is `DEFERRABLE`, `INITIALLY DEFERRED`, or `INITIALLY IMMEDIATE`,

<a id="op-1519917b5170fb7cce2b4131"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateTrigger::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateTrigger
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3936, 17], "end": [3936, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f85ea27c416b9ddb07876f38"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateTrigger::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateTrigger) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3936, 51], "end": [3936, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7875670e6dd4f56cac2f54b"></a>
## condition

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::condition` · sqlparser 0.62.0

```rust
condition: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:4043`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triggering conditions

<a id="op-373e97b429ca14747f4b60af"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateTrigger::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3937, 49], "end": [3937, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3937`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-300c9c9f652e0ccebc811293"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateTrigger::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateTrigger) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3936, 24], "end": [3936, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d535d1a6a90b6ad68a5c07d7"></a>
## events

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::events` · sqlparser 0.62.0

```rust
events: Vec<ast::TriggerEvent>
```

Source: `src/ast/ddl.rs:4029`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multiple events can be specified using OR, such as `INSERT`, `UPDATE`, `DELETE`, or `TRUNCATE`.

<a id="op-74cc5d3567b1fc35258a9234"></a>
## exec_body

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::exec_body` · sqlparser 0.62.0

```rust
exec_body: Option<ast::TriggerExecBody>
```

Source: `src/ast/ddl.rs:4045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Execute logic block

<a id="op-952db9b2a82c79b192928ff0"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateTrigger::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4054, 1], "end": [4131, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8941b86d9f89e4faaafe1e3"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateTrigger::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3936, 10], "end": [3936, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13728f7548cafe77d53f7e6d"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateTrigger::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3936, 56], "end": [3936, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19fdf59dfdffb4f20df26cbe"></a>
## is_constraint

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::is_constraint` · sqlparser 0.62.0

```rust
is_constraint: bool
```

Source: `src/ast/ddl.rs:3985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `CONSTRAINT` keyword is used to create a trigger as a constraint.

<a id="op-2553b034a6780084103f6a7c"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:3987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the trigger to be created.

<a id="op-dad5d194d60e25f8a1fefa98"></a>
## or_alter

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::or_alter` · sqlparser 0.62.0

```rust
or_alter: bool
```

Source: `src/ast/ddl.rs:3956`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if this is a `CREATE OR ALTER TRIGGER` statement

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-trigger-transact-sql?view=sql-server-ver16#arguments)

<a id="op-194dc8d2c95f1a1112d88a92"></a>
## or_replace

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/ddl.rs:3983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `OR REPLACE` clause is used to re-create the trigger if it already exists.

Example:
```sql
CREATE OR REPLACE TRIGGER trigger_name
AFTER INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

<a id="op-20917bf98325f15ce11d9b89"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateTrigger::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateTrigger) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3936, 35], "end": [3936, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a07b9feb812275ded12875e8"></a>
## period

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::period` · sqlparser 0.62.0

```rust
period: Option<ast::TriggerPeriod>
```

Source: `src/ast/ddl.rs:4016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Determines whether the function is called before, after, or instead of the event.

Example of BEFORE:

```sql
CREATE TRIGGER trigger_name
BEFORE INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

Example of AFTER:

```sql
CREATE TRIGGER trigger_name
AFTER INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

Example of INSTEAD OF:

```sql
CREATE TRIGGER trigger_name
INSTEAD OF INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

<a id="op-a2263215382991b9cdda37fb"></a>
## period_before_table

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::period_before_table` · sqlparser 0.62.0

```rust
period_before_table: bool
```

Source: `src/ast/ddl.rs:4027`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the trigger period was specified before the target table name.
This does not refer to whether the period is BEFORE, AFTER, or INSTEAD OF,
but rather the position of the period clause in relation to the table name.

```sql
-- period_before_table == true: Postgres, MySQL, and standard SQL
CREATE TRIGGER t BEFORE INSERT ON table_name ...;
-- period_before_table == false: MSSQL
CREATE TRIGGER t ON table_name BEFORE INSERT ...;
```

<a id="op-ca56b881a1a4655131bec05b"></a>
## referenced_table_name

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::referenced_table_name` · sqlparser 0.62.0

```rust
referenced_table_name: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:4034`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The optional referenced table name that can be referenced via
the `FROM` keyword.

<a id="op-96280d58ee501f05ad94158a"></a>
## referencing

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::referencing` · sqlparser 0.62.0

```rust
referencing: Vec<ast::TriggerReferencing>
```

Source: `src/ast/ddl.rs:4036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This keyword immediately precedes the declaration of one or two relation names that provide access to the transition relations of the triggering statement.

<a id="op-28d619ac2a49328378143598"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateTrigger::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3937, 38], "end": [3937, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3937`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed14a3c6993d2da71f28fc91"></a>
## statements

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::statements` · sqlparser 0.62.0

```rust
statements: Option<ast::ConditionalStatements>
```

Source: `src/ast/ddl.rs:4049`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For SQL dialects with statement(s) for a body

<a id="op-66f762514f763bfd9aad2c66"></a>
## statements_as

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::statements_as` · sqlparser 0.62.0

```rust
statements_as: bool
```

Source: `src/ast/ddl.rs:4047`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For MSSQL and dialects where statements are preceded by `AS`

<a id="op-a1c7ec57b25a169d9e3b15cb"></a>
## table_name

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::table_name` · sqlparser 0.62.0

```rust
table_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4031`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The table on which the trigger is to be created.

<a id="op-5ee23b2bae8c1a6bfb45e65c"></a>
## temporary

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/ddl.rs:3973`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if this is a temporary trigger.

Examples:

```sql
CREATE TEMP TRIGGER trigger_name
```

or

```sql
CREATE TEMPORARY TRIGGER trigger_name;
CREATE TEMP TRIGGER trigger_name;
```

[SQLite](https://sqlite.org/lang_createtrigger.html#temp_triggers_on_non_temp_tables)

<a id="op-712eec93a31a138fec38e680"></a>
## trigger_object

`struct_field` · `sqlparser::ast::ddl::CreateTrigger::trigger_object` · sqlparser 0.62.0

```rust
trigger_object: Option<TriggerObjectKind>
```

Source: `src/ast/ddl.rs:4041`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This specifies whether the trigger function should be fired once for
every row affected by the trigger event, or just once per SQL statement.
This is optional in some SQL dialects, such as SQLite, and if not specified, in
those cases, the implied default is `FOR EACH ROW`.

<a id="op-4a71c4e7b1f348074d26d1db"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateTrigger::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3938, 40], "end": [3938, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3938`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d76a9695e62f71aa28f87b1d"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateTrigger::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3938, 47], "end": [3938, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3938`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
