# `sqlparser::ast::Statement::CopyIntoSnowflake`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CopyIntoSnowflake.json).

<a id="op-9353be6933022b6d3b08ffda"></a>
## copy_options

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::copy_options` · sqlparser 0.62.0

```rust
copy_options: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:3658`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional copy options.

<a id="op-d6b59a48bd3f30cac181e1e0"></a>
## file_format

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::file_format` · sqlparser 0.62.0

```rust
file_format: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:3656`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

File format options.

<a id="op-65353344d416bb7e8716f2da"></a>
## files

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::files` · sqlparser 0.62.0

```rust
files: Option<Vec<String>>
```

Source: `src/ast/mod.rs:3652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of specific file names to load.

<a id="op-13ef098d6e2598ee2a94c009"></a>
## from_obj

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::from_obj` · sqlparser 0.62.0

```rust
from_obj: Option<ObjectName>
```

Source: `src/ast/mod.rs:3642`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional source object name (staged data).

<a id="op-35e1e59f4c8a3bf55d9d9c2d"></a>
## from_obj_alias

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::from_obj_alias` · sqlparser 0.62.0

```rust
from_obj_alias: Option<Ident>
```

Source: `src/ast/mod.rs:3644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the source object.

<a id="op-ff18ae25e00d2938dc56765f"></a>
## from_query

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::from_query` · sqlparser 0.62.0

```rust
from_query: Option<Box<Query>>
```

Source: `src/ast/mod.rs:3650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional source query instead of a staged object.

<a id="op-a2d0dfa7d737747662d6836b"></a>
## from_transformations

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::from_transformations` · sqlparser 0.62.0

```rust
from_transformations: Option<Vec<helpers::stmt_data_loading::StageLoadSelectItemKind>>
```

Source: `src/ast/mod.rs:3648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of transformations applied when loading.

<a id="op-763b4d74ee6f3db269012c73"></a>
## into

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::into` · sqlparser 0.62.0

```rust
into: ObjectName
```

Source: `src/ast/mod.rs:3638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target object for the COPY INTO operation.

<a id="op-d42df858d92c26663acbc2bf"></a>
## into_columns

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::into_columns` · sqlparser 0.62.0

```rust
into_columns: Option<Vec<Ident>>
```

Source: `src/ast/mod.rs:3640`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of target columns.

<a id="op-4512ddcc96d83f79698d5690"></a>
## kind

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::kind` · sqlparser 0.62.0

```rust
kind: CopyIntoSnowflakeKind
```

Source: `src/ast/mod.rs:3636`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Kind of COPY INTO operation (table or location).

<a id="op-df7977b475f4995f6a57b96a"></a>
## partition

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::partition` · sqlparser 0.62.0

```rust
partition: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:3662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partition expression for loading.

<a id="op-3b6e25881b6a7780d70166d1"></a>
## pattern

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::pattern` · sqlparser 0.62.0

```rust
pattern: Option<String>
```

Source: `src/ast/mod.rs:3654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional filename matching pattern.

<a id="op-e5a34507a1e3118fdb34a4bc"></a>
## stage_params

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::stage_params` · sqlparser 0.62.0

```rust
stage_params: ast::helpers::stmt_data_loading::StageParamsObject
```

Source: `src/ast/mod.rs:3646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Stage-specific parameters (e.g., credentials, path).

<a id="op-e50b10803d88023393e7a09e"></a>
## validation_mode

`struct_field` · `sqlparser::ast::Statement::CopyIntoSnowflake::validation_mode` · sqlparser 0.62.0

```rust
validation_mode: Option<String>
```

Source: `src/ast/mod.rs:3660`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional validation mode string.
