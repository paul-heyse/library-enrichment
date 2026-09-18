# `sqlparser::ast::Statement::Comment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Comment.json).

<a id="op-888e5971a9fdc737020ef2fe"></a>
## comment

`struct_field` · `sqlparser::ast::Statement::Comment::comment` · sqlparser 0.62.0

```rust
comment: Option<String>
```

Source: `src/ast/mod.rs:4301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional comment text (None to remove comment).

<a id="op-aaea35348a5c65085b0b088a"></a>
## if_exists

`struct_field` · `sqlparser::ast::Statement::Comment::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:4304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An optional `IF EXISTS` clause. (Non-standard.)
See <https://docs.snowflake.com/en/sql-reference/sql/comment>

<a id="op-397a80376ebf2643a8ff461a"></a>
## object_name

`struct_field` · `sqlparser::ast::Statement::Comment::object_name` · sqlparser 0.62.0

```rust
object_name: ObjectName
```

Source: `src/ast/mod.rs:4299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the object the comment applies to.

<a id="op-10312a83f12709b037d866fd"></a>
## object_type

`struct_field` · `sqlparser::ast::Statement::Comment::object_type` · sqlparser 0.62.0

```rust
object_type: CommentObject
```

Source: `src/ast/mod.rs:4297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type of object being commented (table, column, etc.).
