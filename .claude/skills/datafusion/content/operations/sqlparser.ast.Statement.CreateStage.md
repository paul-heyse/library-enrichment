# `sqlparser::ast::Statement::CreateStage`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateStage.json).

<a id="op-133c760211cc90c80591d4b5"></a>
## comment

`struct_field` · `sqlparser::ast::Statement::CreateStage::comment` · sqlparser 0.62.0

```rust
comment: Option<String>
```

Source: `src/ast/mod.rs:4493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional comment.

<a id="op-7b849fd6b9523a6a67498dde"></a>
## copy_options

`struct_field` · `sqlparser::ast::Statement::CreateStage::copy_options` · sqlparser 0.62.0

```rust
copy_options: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:4491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Copy options for stage.

<a id="op-e0a2cf2f20270a0a4b477135"></a>
## directory_table_params

`struct_field` · `sqlparser::ast::Statement::CreateStage::directory_table_params` · sqlparser 0.62.0

```rust
directory_table_params: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:4487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Directory table parameters.

<a id="op-ebb71e66721e040f0ef5af84"></a>
## file_format

`struct_field` · `sqlparser::ast::Statement::CreateStage::file_format` · sqlparser 0.62.0

```rust
file_format: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:4489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

File format options.

<a id="op-bfbc50d8efee1cc91021f2f3"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::Statement::CreateStage::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:4481`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF NOT EXISTS` flag.

<a id="op-65a98ef07260a017512c13de"></a>
## name

`struct_field` · `sqlparser::ast::Statement::CreateStage::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:4483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Stage name.

<a id="op-a9c20b30ba20f9d5d8cef015"></a>
## or_replace

`struct_field` · `sqlparser::ast::Statement::CreateStage::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/mod.rs:4477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OR REPLACE` flag for stage.

<a id="op-d175af93f003bed7084dd76c"></a>
## stage_params

`struct_field` · `sqlparser::ast::Statement::CreateStage::stage_params` · sqlparser 0.62.0

```rust
stage_params: ast::helpers::stmt_data_loading::StageParamsObject
```

Source: `src/ast/mod.rs:4485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Stage parameters.

<a id="op-5c06e58a5d62938ea27cbceb"></a>
## temporary

`struct_field` · `sqlparser::ast::Statement::CreateStage::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/mod.rs:4479`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether stage is temporary.
