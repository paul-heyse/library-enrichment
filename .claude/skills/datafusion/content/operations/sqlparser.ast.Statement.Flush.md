# `sqlparser::ast::Statement::Flush`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Flush.json).

<a id="op-c1b601b6db726db096f6c5f1"></a>
## channel

`struct_field` · `sqlparser::ast::Statement::Flush::channel` · sqlparser 0.62.0

```rust
channel: Option<String>
```

Source: `src/ast/mod.rs:4068`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional channel name used for flush operations.

<a id="op-6bb5579e894ddf23a963aea9"></a>
## export

`struct_field` · `sqlparser::ast::Statement::Flush::export` · sqlparser 0.62.0

```rust
export: bool
```

Source: `src/ast/mod.rs:4072`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether this is an export flush operation.

<a id="op-02666614ddca2b1b5adaa901"></a>
## location

`struct_field` · `sqlparser::ast::Statement::Flush::location` · sqlparser 0.62.0

```rust
location: Option<FlushLocation>
```

Source: `src/ast/mod.rs:4066`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional flush location (dialect-specific).

<a id="op-99eccc255bb72a3b46d600b9"></a>
## object_type

`struct_field` · `sqlparser::ast::Statement::Flush::object_type` · sqlparser 0.62.0

```rust
object_type: FlushType
```

Source: `src/ast/mod.rs:4064`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The specific flush option or object to flush.

<a id="op-4292ab2df441fd17452d1869"></a>
## read_lock

`struct_field` · `sqlparser::ast::Statement::Flush::read_lock` · sqlparser 0.62.0

```rust
read_lock: bool
```

Source: `src/ast/mod.rs:4070`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether a read lock was requested.

<a id="op-15871ed8b2b0aaf80a6f2d9f"></a>
## tables

`struct_field` · `sqlparser::ast::Statement::Flush::tables` · sqlparser 0.62.0

```rust
tables: Vec<ObjectName>
```

Source: `src/ast/mod.rs:4074`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of tables involved in the flush.
