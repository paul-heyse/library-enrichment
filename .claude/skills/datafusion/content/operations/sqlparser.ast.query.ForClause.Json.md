# `sqlparser::ast::query::ForClause::Json`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ForClause.Json.json).

<a id="op-ebcab83c2e97c16da0fcaf9f"></a>
## for_json

`struct_field` · `sqlparser::ast::query::ForClause::Json::for_json` · sqlparser 0.62.0

```rust
for_json: ForJson
```

Source: `src/ast/query.rs:3841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

JSON mode (`AUTO` or `PATH`).

<a id="op-de0511a58a123c81411e33c3"></a>
## include_null_values

`struct_field` · `sqlparser::ast::query::ForClause::Json::include_null_values` · sqlparser 0.62.0

```rust
include_null_values: bool
```

Source: `src/ast/query.rs:3845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INCLUDE_NULL_VALUES` flag.

<a id="op-b695660dd6856d238665015c"></a>
## root

`struct_field` · `sqlparser::ast::query::ForClause::Json::root` · sqlparser 0.62.0

```rust
root: Option<String>
```

Source: `src/ast/query.rs:3843`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ROOT('...')` parameter.

<a id="op-1d64c46cbbb7cf2ff16043a1"></a>
## without_array_wrapper

`struct_field` · `sqlparser::ast::query::ForClause::Json::without_array_wrapper` · sqlparser 0.62.0

```rust
without_array_wrapper: bool
```

Source: `src/ast/query.rs:3847`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WITHOUT_ARRAY_WRAPPER` flag.
