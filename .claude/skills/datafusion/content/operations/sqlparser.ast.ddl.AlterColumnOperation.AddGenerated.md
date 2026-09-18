# `sqlparser::ast::ddl::AlterColumnOperation::AddGenerated`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterColumnOperation.AddGenerated.json).

<a id="op-4e482981016467c9cf67147a"></a>
## generated_as

`struct_field` · `sqlparser::ast::ddl::AlterColumnOperation::AddGenerated::generated_as` · sqlparser 0.62.0

```rust
generated_as: Option<GeneratedAs>
```

Source: `src/ast/ddl.rs:1296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `GENERATED AS` specifier (e.g. `ALWAYS` or `BY DEFAULT`).

<a id="op-f3b3b9c96155b3cabad0cf15"></a>
## sequence_options

`struct_field` · `sqlparser::ast::ddl::AlterColumnOperation::AddGenerated::sequence_options` · sqlparser 0.62.0

```rust
sequence_options: Option<Vec<ast::SequenceOptions>>
```

Source: `src/ast/ddl.rs:1298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional sequence options for identity generation.
