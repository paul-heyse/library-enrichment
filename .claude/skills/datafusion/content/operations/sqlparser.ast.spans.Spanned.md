# `sqlparser::ast::spans::Spanned`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.spans.Spanned.json).

<a id="op-a414e73174d10b9c5448d654"></a>
## Spanned

`trait` · `sqlparser::ast::spans::Spanned` · sqlparser 0.62.0

```rust
trait Spanned
```

Source: `src/ast/spans.rs:95`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trait for AST nodes that have a source location information.

# Notes:

Source [`Span`](../operations/sqlparser.tokenizer.Span.md#op-c744756cf28c8276d407a53a) are not yet complete. They may be missing:

1. keywords or other tokens
2. span information entirely, in which case they return [`Span::empty()`](../operations/sqlparser.tokenizer.Span.md#op-901a002887b77badb7cad462).

Note Some impl blocks (rendered below) are annotated with which nodes are
missing spans. See [this ticket] for additional information and status.

[this ticket]: https://github.com/apache/datafusion-sqlparser-rs/issues/1548

# Example
```
# use sqlparser::parser::{Parser, ParserError};
# use sqlparser::ast::Spanned;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::tokenizer::Location;
# fn main() -> Result<(), ParserError> {
let dialect = GenericDialect {};
let sql = r#"SELECT *
  FROM table_1"#;
let statements = Parser::new(&dialect)
  .try_with_sql(sql)?
  .parse_statements()?;
// Get the span of the first statement (SELECT)
let span = statements[0].span();
// statement starts at line 1, column 1 (1 based, not 0 based)
assert_eq!(span.start, Location::new(1, 1));
// statement ends on line 2, column 15
assert_eq!(span.end, Location::new(2, 15));
# Ok(())
# }
```


<a id="op-365c50a80fa73f2b30d2d485"></a>
## span

`function` · `sqlparser::ast::spans::Spanned::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Source: `src/ast/spans.rs:100`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Return the [`Span`](../operations/sqlparser.tokenizer.Span.md#op-c744756cf28c8276d407a53a) (the minimum and maximum [`Location`]) for this AST
node, by recursively combining the spans of its children.

[`Location`]: crate::tokenizer::Location
