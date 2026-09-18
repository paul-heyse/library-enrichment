# `datafusion_sql::planner::object_name_to_table_reference`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.planner.object_name_to_table_reference.json).

<a id="op-7d8ece37897b38ad3966789d"></a>
## object_name_to_table_reference

`function` · `datafusion_sql::planner::object_name_to_table_reference` · datafusion-sql 55.1.0

```rust
fn object_name_to_table_reference(object_name: sqlparser::ast::ObjectName, enable_normalization: bool) -> datafusion_common::Result<datafusion_common::TableReference>
```

Source: `src/planner.rs:938`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Create a [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) after normalizing the specified ObjectName

Examples
```text
['foo']          -> Bare { table: "foo" }
['"foo.bar"]]    -> Bare { table: "foo.bar" }
['foo', 'Bar']   -> Partial { schema: "foo", table: "bar" } <-- note lower case "bar"
['foo', 'bar']   -> Partial { schema: "foo", table: "bar" }
['foo', '"Bar"'] -> Partial { schema: "foo", table: "Bar" }
```
