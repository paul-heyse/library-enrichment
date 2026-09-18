# `datafusion_sql::planner::object_name_to_qualifier`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.planner.object_name_to_qualifier.json).

<a id="op-73bcd68fa500a79411fcaaaa"></a>
## object_name_to_qualifier

`function` · `datafusion_sql::planner::object_name_to_qualifier` · datafusion-sql 55.1.0

```rust
fn object_name_to_qualifier(sql_table_name: &sqlparser::ast::ObjectName, enable_normalization: bool) -> datafusion_common::Result<String>
```

Source: `src/planner.rs:1033`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Construct a WHERE qualifier suitable for e.g. information_schema filtering
from the provided object identifiers (catalog, schema and table names).
