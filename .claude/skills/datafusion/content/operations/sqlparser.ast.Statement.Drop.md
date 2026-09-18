# `sqlparser::ast::Statement::Drop`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Drop.json).

<a id="op-a37fba8ed422539f66a5be05"></a>
## cascade

`struct_field` · `sqlparser::ast::Statement::Drop::cascade` · sqlparser 0.62.0

```rust
cascade: bool
```

Source: `src/ast/mod.rs:3925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `CASCADE` was specified. This will be `false` when
`RESTRICT` or no drop behavior at all was specified.

<a id="op-299df621e2e5333ac0426e9a"></a>
## if_exists

`struct_field` · `sqlparser::ast::Statement::Drop::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:3920`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An optional `IF EXISTS` clause. (Non-standard.)

<a id="op-b9944d8583b457cddc4b8e96"></a>
## names

`struct_field` · `sqlparser::ast::Statement::Drop::names` · sqlparser 0.62.0

```rust
names: Vec<ObjectName>
```

Source: `src/ast/mod.rs:3922`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more objects to drop. (ANSI SQL requires exactly one.)

<a id="op-a35b6ac9a6845657d7d0515f"></a>
## object_type

`struct_field` · `sqlparser::ast::Statement::Drop::object_type` · sqlparser 0.62.0

```rust
object_type: ObjectType
```

Source: `src/ast/mod.rs:3918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of the object to drop: TABLE, VIEW, etc.

<a id="op-3597a4427486a2ccbbaebe31"></a>
## purge

`struct_field` · `sqlparser::ast::Statement::Drop::purge` · sqlparser 0.62.0

```rust
purge: bool
```

Source: `src/ast/mod.rs:3931`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive allows you specify whether the table's stored data will be
deleted along with the dropped table

<a id="op-a396f1106816cc6cbb9459c8"></a>
## restrict

`struct_field` · `sqlparser::ast::Statement::Drop::restrict` · sqlparser 0.62.0

```rust
restrict: bool
```

Source: `src/ast/mod.rs:3928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `RESTRICT` was specified. This will be `false` when
`CASCADE` or no drop behavior at all was specified.

<a id="op-367f039e65eddef79142adbd"></a>
## table

`struct_field` · `sqlparser::ast::Statement::Drop::table` · sqlparser 0.62.0

```rust
table: Option<ObjectName>
```

Source: `src/ast/mod.rs:3936`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific drop index syntax, which requires table specification
See <https://dev.mysql.com/doc/refman/8.4/en/drop-index.html>

<a id="op-e1cd1635c2f6e40af139af03"></a>
## temporary

`struct_field` · `sqlparser::ast::Statement::Drop::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/mod.rs:3933`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific "TEMPORARY" keyword
