# `datafusion_sql::parser::DFParserBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.DFParserBuilder.json).

<a id="op-68cc3b0ccb81d9520e0b99b9"></a>
## DFParserBuilder

`struct` · `datafusion_sql::parser::DFParserBuilder` · datafusion-sql 55.1.0

```rust
struct DFParserBuilder<'a, 'b>
```

Source: `src/parser.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Builder for [`DFParser`](../operations/datafusion_sql.parser.DFParser.md#op-c4bfa9a3c20364cfab8a7ca0)

# Example: Create and Parse SQL statements
```
# use datafusion_sql::parser::DFParserBuilder;
# use datafusion_common::Result;
# fn test() -> Result<()> {
let mut parser = DFParserBuilder::new("SELECT * FROM foo; SELECT 1 + 2").build()?;
// parse the SQL into DFStatements
let statements = parser.parse_statements()?;
assert_eq!(statements.len(), 2);
# Ok(())
# }
```

# Example: Create and Parse expression with a different dialect
```
# use datafusion_sql::parser::DFParserBuilder;
# use datafusion_common::Result;
# use datafusion_sql::sqlparser::dialect::MySqlDialect;
# use datafusion_sql::sqlparser::ast::Expr;
# fn test() -> Result<()> {
let dialect = MySqlDialect {}; // Parse using MySQL dialect
let mut parser = DFParserBuilder::new("1 + 2")
    .with_dialect(&dialect)
    .build()?;
// parse 1+2 into an sqlparser::ast::Expr
let res = parser.parse_expr()?;
assert!(matches!(res.expr, Expr::BinaryOp { .. }));
# Ok(())
# }
```

<a id="op-e6572b0d816b01d6b2debf75"></a>
## build

`function` · `datafusion_sql::parser::DFParserBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(self) -> Result<DFParser<'b>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParserBuilder", "path": "DFParserBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [499, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:474`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Build resulting parser

<a id="op-abfd619f0f84d4901fc234d2"></a>
## new

`function` · `datafusion_sql::parser::DFParserBuilder::new` · datafusion-sql 55.1.0

```rust
fn new(input: impl Into<ParserInput<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParserBuilder", "path": "DFParserBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [499, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Create a new parser builder for the specified tokens using the
[`GenericDialect`](../operations/sqlparser.dialect.generic.GenericDialect.md#op-e9e98e2bf43b55c23c54c302).

<a id="op-e5f6337fcc78cac67b54c162"></a>
## with_dialect

`function` · `datafusion_sql::parser::DFParserBuilder::with_dialect` · datafusion-sql 55.1.0

```rust
fn with_dialect(self, dialect: &'b dyn Dialect) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParserBuilder", "path": "DFParserBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [499, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Adjust the parser builder's dialect. Defaults to [`GenericDialect`](../operations/sqlparser.dialect.generic.GenericDialect.md#op-e9e98e2bf43b55c23c54c302)

<a id="op-0021420cf780064d0f769134"></a>
## with_recursion_limit

`function` · `datafusion_sql::parser::DFParserBuilder::with_recursion_limit` · datafusion-sql 55.1.0

```rust
fn with_recursion_limit(self, recursion_limit: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"lifetime": "'b"}], "constraints": []}}, "id": "datafusion_sql::parser::DFParserBuilder", "path": "DFParserBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'b"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [499, 2], "filename": "src/parser.rs"}, "trait": null, "trait_path": null}`

Source: `src/parser.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Adjust the recursion limit of sql parsing.  Defaults to 50
