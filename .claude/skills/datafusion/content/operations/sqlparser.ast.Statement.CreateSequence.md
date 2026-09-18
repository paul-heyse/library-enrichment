# `sqlparser::ast::Statement::CreateSequence`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateSequence.json).

<a id="op-1132a908e572fd0943a647d5"></a>
## data_type

`struct_field` · `sqlparser::ast::Statement::CreateSequence::data_type` · sqlparser 0.62.0

```rust
data_type: Option<DataType>
```

Source: `src/ast/mod.rs:4690`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional data type for the sequence.

<a id="op-51ebd35de80a5d6d94e79266"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::Statement::CreateSequence::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:4686`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF NOT EXISTS` flag.

<a id="op-112b9a77c06dca4bfc90d957"></a>
## name

`struct_field` · `sqlparser::ast::Statement::CreateSequence::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:4688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sequence name.

<a id="op-4ce7e33c247730f2fbd7ed6a"></a>
## owned_by

`struct_field` · `sqlparser::ast::Statement::CreateSequence::owned_by` · sqlparser 0.62.0

```rust
owned_by: Option<ObjectName>
```

Source: `src/ast/mod.rs:4694`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `OWNED BY` target.

<a id="op-71752c65872b346a40e23faf"></a>
## sequence_options

`struct_field` · `sqlparser::ast::Statement::CreateSequence::sequence_options` · sqlparser 0.62.0

```rust
sequence_options: Vec<SequenceOptions>
```

Source: `src/ast/mod.rs:4692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sequence options (INCREMENT, MINVALUE, etc.).

<a id="op-65dda507d381e0518652906b"></a>
## temporary

`struct_field` · `sqlparser::ast::Statement::CreateSequence::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/mod.rs:4684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the sequence is temporary.
