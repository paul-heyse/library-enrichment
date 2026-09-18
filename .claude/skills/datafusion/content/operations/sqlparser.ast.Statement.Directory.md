# `sqlparser::ast::Statement::Directory`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Directory.json).

<a id="op-69a39a48bf49601c04c0e064"></a>
## file_format

`struct_field` · `sqlparser::ast::Statement::Directory::file_format` · sqlparser 0.62.0

```rust
file_format: Option<FileFormat>
```

Source: `src/ast/mod.rs:3590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional file format for the data.

<a id="op-3b4ccab6aa0848beb39f65a8"></a>
## local

`struct_field` · `sqlparser::ast::Statement::Directory::local` · sqlparser 0.62.0

```rust
local: bool
```

Source: `src/ast/mod.rs:3586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the directory is local to the server.

<a id="op-0097e271ab27572c47d51fba"></a>
## overwrite

`struct_field` · `sqlparser::ast::Statement::Directory::overwrite` · sqlparser 0.62.0

```rust
overwrite: bool
```

Source: `src/ast/mod.rs:3584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to overwrite existing files.

<a id="op-eb5cab3644b931938cd58471"></a>
## path

`struct_field` · `sqlparser::ast::Statement::Directory::path` · sqlparser 0.62.0

```rust
path: String
```

Source: `src/ast/mod.rs:3588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Path to the directory or files.

<a id="op-a2927d7bc5f5e6c380252474"></a>
## source

`struct_field` · `sqlparser::ast::Statement::Directory::source` · sqlparser 0.62.0

```rust
source: Box<Query>
```

Source: `src/ast/mod.rs:3592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Source query providing data to load.
