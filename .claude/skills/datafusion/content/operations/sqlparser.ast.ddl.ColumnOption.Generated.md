# `sqlparser::ast::ddl::ColumnOption::Generated`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ColumnOption.Generated.json).

<a id="op-209d898082b770065b0107b7"></a>
## generated_as

`struct_field` · `sqlparser::ast::ddl::ColumnOption::Generated::generated_as` · sqlparser 0.62.0

```rust
generated_as: GeneratedAs
```

Source: `src/ast/ddl.rs:1946`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

How the column is generated (e.g. `GENERATED ALWAYS`, `BY DEFAULT`, or expression-stored).

<a id="op-1153df09ac79db52ffd0072a"></a>
## generated_keyword

`struct_field` · `sqlparser::ast::ddl::ColumnOption::Generated::generated_keyword` · sqlparser 0.62.0

```rust
generated_keyword: bool
```

Source: `src/ast/ddl.rs:1954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

false if 'GENERATED ALWAYS' is skipped (option starts with AS)

<a id="op-a4225a68641caf74f001fd3f"></a>
## generation_expr

`struct_field` · `sqlparser::ast::ddl::ColumnOption::Generated::generation_expr` · sqlparser 0.62.0

```rust
generation_expr: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:1950`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional expression used to generate the column value.

<a id="op-71864e2a245e1f8b3ca73060"></a>
## generation_expr_mode

`struct_field` · `sqlparser::ast::ddl::ColumnOption::Generated::generation_expr_mode` · sqlparser 0.62.0

```rust
generation_expr_mode: Option<GeneratedExpressionMode>
```

Source: `src/ast/ddl.rs:1952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Mode of the generated expression (`VIRTUAL` or `STORED`) when `generation_expr` is present.

<a id="op-e9ffb43538245ab6df12b704"></a>
## sequence_options

`struct_field` · `sqlparser::ast::ddl::ColumnOption::Generated::sequence_options` · sqlparser 0.62.0

```rust
sequence_options: Option<Vec<ast::SequenceOptions>>
```

Source: `src/ast/ddl.rs:1948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sequence/identity options when generation is backed by a sequence.
