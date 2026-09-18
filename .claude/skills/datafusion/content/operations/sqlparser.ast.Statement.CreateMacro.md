# `sqlparser::ast::Statement::CreateMacro`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateMacro.json).

<a id="op-2d4e8528fb366a1ffac1e3c7"></a>
## args

`struct_field` · `sqlparser::ast::Statement::CreateMacro::args` · sqlparser 0.62.0

```rust
args: Option<Vec<MacroArg>>
```

Source: `src/ast/mod.rs:4467`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional macro arguments.

<a id="op-84439407d76fd2e2b3429952"></a>
## definition

`struct_field` · `sqlparser::ast::Statement::CreateMacro::definition` · sqlparser 0.62.0

```rust
definition: MacroDefinition
```

Source: `src/ast/mod.rs:4469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Macro definition body.

<a id="op-76cf4c058cebba94411839ef"></a>
## name

`struct_field` · `sqlparser::ast::Statement::CreateMacro::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:4465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Macro name.

<a id="op-89d0a210305f68fb44abc826"></a>
## or_replace

`struct_field` · `sqlparser::ast::Statement::CreateMacro::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/mod.rs:4461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OR REPLACE` flag.

<a id="op-e7063e443d2728f6790133a8"></a>
## temporary

`struct_field` · `sqlparser::ast::Statement::CreateMacro::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/mod.rs:4463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether macro is temporary.
