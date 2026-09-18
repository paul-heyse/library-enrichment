# `sqlparser::ast::helpers::stmt_data_loading`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.stmt_data_loading.json).

<a id="op-cce7ee9ed2c7a539b062f8ba"></a>
## stmt_data_loading

`module` · `sqlparser::ast::helpers::stmt_data_loading` · sqlparser 0.62.0

```rust
mod stmt_data_loading
```

Source: `src/ast/helpers/stmt_data_loading.rs:18`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Helpers for data loading/unloading related statements (stages, PUT, COPY INTO).
AST types specific to loading and unloading syntax, like one available in Snowflake which
contains: STAGE ddl operations, PUT upload or COPY INTO
See [this page](https://docs.snowflake.com/en/sql-reference/commands-data-loading) for more details.
