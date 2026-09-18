# `datafusion_sql::resolve::resolve_table_references`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.resolve.resolve_table_references.json).

<a id="op-d2b5d8c02c97a530ce7ab240"></a>
## resolve_table_references

`function` · `datafusion_sql::resolve::resolve_table_references` · datafusion-sql 55.1.0

```rust
fn resolve_table_references(statement: &parser::Statement, enable_ident_normalization: bool) -> datafusion_common::Result<(Vec<datafusion_common::TableReference>, Vec<datafusion_common::TableReference>)>
```

Source: `src/resolve.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Collects all tables and views referenced in the SQL statement. CTEs are collected separately.
This can be used to determine which tables need to be in the catalog for a query to be planned.

# Returns

A `(table_refs, ctes)` tuple, the first element contains table and view references and the second
element contains any CTE aliases that were defined and possibly referenced.

## Example

```
# use datafusion_sql::parser::DFParser;
# use datafusion_sql::resolve::resolve_table_references;
let query = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statement = DFParser::parse_sql(query).unwrap().pop_back().unwrap();
let (table_refs, ctes) = resolve_table_references(&statement, true).unwrap();
assert_eq!(table_refs.len(), 2);
assert_eq!(table_refs[0].to_string(), "bar");
assert_eq!(table_refs[1].to_string(), "foo");
assert_eq!(ctes.len(), 0);
```

## Example with CTEs  
  
```  
# use datafusion_sql::parser::DFParser;
# use datafusion_sql::resolve::resolve_table_references;
let query = "with my_cte as (values (1), (2)) SELECT * from my_cte;";
let statement = DFParser::parse_sql(query).unwrap().pop_back().unwrap();
let (table_refs, ctes) = resolve_table_references(&statement, true).unwrap();
assert_eq!(table_refs.len(), 0);
assert_eq!(ctes.len(), 1);
assert_eq!(ctes[0].to_string(), "my_cte");
```
