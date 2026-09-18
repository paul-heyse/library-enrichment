# `sqlparser::ast::Set::SingleAssignment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Set.SingleAssignment.json).

<a id="op-dc0b59acd2947eec4e54d05f"></a>
## hivevar

`struct_field` · `sqlparser::ast::Set::SingleAssignment::hivevar` · sqlparser 0.62.0

```rust
hivevar: bool
```

Source: `src/ast/mod.rs:3253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether this is a Hive-style `HIVEVAR:` assignment.

<a id="op-b8292cbfd64752d7010f1600"></a>
## scope

`struct_field` · `sqlparser::ast::Set::SingleAssignment::scope` · sqlparser 0.62.0

```rust
scope: Option<ContextModifier>
```

Source: `src/ast/mod.rs:3251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional scope modifier (`SESSION` / `LOCAL`).

<a id="op-71b2b0dccc5d2b5d87ec5650"></a>
## values

`struct_field` · `sqlparser::ast::Set::SingleAssignment::values` · sqlparser 0.62.0

```rust
values: Vec<Expr>
```

Source: `src/ast/mod.rs:3257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Values assigned to the variable.

<a id="op-31155d1bb55eb00757007abe"></a>
## variable

`struct_field` · `sqlparser::ast::Set::SingleAssignment::variable` · sqlparser 0.62.0

```rust
variable: ObjectName
```

Source: `src/ast/mod.rs:3255`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variable name to assign.
