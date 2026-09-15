# `sqlparser::ast::spans`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.ast.spans.json`](../model/sqlparser.ast.spans.json)

## Spanned

`trait` · `sqlparser::ast::spans::Spanned`

Also reachable as `sqlparser::ast::Spanned`

```rust
trait Spanned
```

**Implementors** (141)

- `sqlparser::ast::AccessExpr`
- `sqlparser::ast::Analyze`
- `sqlparser::ast::Array`
- `sqlparser::ast::Assignment`
- `sqlparser::ast::AssignmentTarget`
- `sqlparser::ast::BeginEndStatements`
- `sqlparser::ast::CaseStatement`
- `sqlparser::ast::ClusteredIndex`
- `sqlparser::ast::ConditionalStatementBlock`
- `sqlparser::ast::ConditionalStatements`
- `sqlparser::ast::ConflictTarget`
- `sqlparser::ast::CopySource`
- `sqlparser::ast::CreateTableOptions`
- `sqlparser::ast::DoUpdate`
- `sqlparser::ast::Expr`
- `sqlparser::ast::FromTable`
- `sqlparser::ast::Function`
- `sqlparser::ast::FunctionArg`
- `sqlparser::ast::FunctionArgExpr`
- `sqlparser::ast::FunctionArgumentClause`
- `sqlparser::ast::FunctionArgumentList`
- `sqlparser::ast::FunctionArguments`
- `sqlparser::ast::IfStatement`
- `sqlparser::ast::JsonPath`
- `sqlparser::ast::JsonPathElem`
- `sqlparser::ast::ObjectName`
- `sqlparser::ast::ObjectNamePart`
- `sqlparser::ast::OnConflict`
- `sqlparser::ast::OnConflictAction`
- `sqlparser::ast::OnInsert`
- `sqlparser::ast::OpenStatement`
- `sqlparser::ast::Parens`
- `sqlparser::ast::RaiseStatement`
- `sqlparser::ast::RaiseStatementValue`
- `sqlparser::ast::SqlOption`
- `sqlparser::ast::Statement`
- `sqlparser::ast::Subscript`
- `sqlparser::ast::TableObject`
- `sqlparser::ast::TableOptionsClustered`
- `sqlparser::ast::WhileStatement`
- `sqlparser::ast::comments::CommentWithSpan`
- `sqlparser::ast::dcl::CreateRole`
- `sqlparser::ast::dcl::Use`
- `sqlparser::ast::ddl::AlterCollation`
- `sqlparser::ast::ddl::AlterColumnOperation`
- `sqlparser::ast::ddl::AlterFunction`
- `sqlparser::ast::ddl::AlterIndexOperation`
- `sqlparser::ast::ddl::AlterOperatorClass`
- `sqlparser::ast::ddl::AlterOperatorFamily`
- `sqlparser::ast::ddl::AlterSchema`
- `sqlparser::ast::ddl::AlterSchemaOperation`
- `sqlparser::ast::ddl::AlterTable`
- `sqlparser::ast::ddl::AlterTableOperation`
- `sqlparser::ast::ddl::ColumnDef`
- `sqlparser::ast::ddl::ColumnOption`
- `sqlparser::ast::ddl::ColumnOptionDef`
- `sqlparser::ast::ddl::ColumnOptions`
- `sqlparser::ast::ddl::ConstraintCharacteristics`
- `sqlparser::ast::ddl::CreateCollation`
- `sqlparser::ast::ddl::CreateExtension`
- `sqlparser::ast::ddl::CreateIndex`
- `sqlparser::ast::ddl::CreateOperator`
- `sqlparser::ast::ddl::CreateOperatorClass`
- `sqlparser::ast::ddl::CreateOperatorFamily`
- `sqlparser::ast::ddl::CreateTable`
- `sqlparser::ast::ddl::CreateView`
- `sqlparser::ast::ddl::DropExtension`
- `sqlparser::ast::ddl::DropFunction`
- `sqlparser::ast::ddl::DropOperator`
- `sqlparser::ast::ddl::DropOperatorClass`
- `sqlparser::ast::ddl::DropOperatorFamily`
- `sqlparser::ast::ddl::ForValues`
- `sqlparser::ast::ddl::IndexColumn`
- `sqlparser::ast::ddl::Msck`
- `sqlparser::ast::ddl::Partition`
- `sqlparser::ast::ddl::PartitionBoundValue`
- `sqlparser::ast::ddl::ReferentialAction`
- `sqlparser::ast::ddl::RenameTableNameKind`
- `sqlparser::ast::ddl::Truncate`
- `sqlparser::ast::ddl::ViewColumnDef`
- `sqlparser::ast::dml::Delete`
- `sqlparser::ast::dml::Insert`
- `sqlparser::ast::dml::Merge`
- `sqlparser::ast::dml::MergeAction`
- `sqlparser::ast::dml::MergeClause`
- `sqlparser::ast::dml::MergeInsertExpr`
- `sqlparser::ast::dml::MergeUpdateExpr`
- `sqlparser::ast::dml::OutputClause`
- `sqlparser::ast::dml::Update`
- `sqlparser::ast::query::ConnectByKind`
- `sqlparser::ast::query::Cte`
- `sqlparser::ast::query::ExceptSelectItem`
- `sqlparser::ast::query::ExcludeSelectItem`
- `sqlparser::ast::query::ExprWithAlias`
- `sqlparser::ast::query::Fetch`
- `sqlparser::ast::query::GroupByExpr`
- `sqlparser::ast::query::IlikeSelectItem`
- `sqlparser::ast::query::Interpolate`
- `sqlparser::ast::query::InterpolateExpr`
- `sqlparser::ast::query::Join`
- `sqlparser::ast::query::JoinConstraint`
- `sqlparser::ast::query::JoinOperator`
- `sqlparser::ast::query::LateralView`
- `sqlparser::ast::query::LimitClause`
- `sqlparser::ast::query::MatchRecognizePattern`
- `sqlparser::ast::query::Measure`
- `sqlparser::ast::query::NamedWindowDefinition`
- `sqlparser::ast::query::Offset`
- `sqlparser::ast::query::OrderBy`
- `sqlparser::ast::query::OrderByExpr`
- `sqlparser::ast::query::PivotValueSource`
- `sqlparser::ast::query::ProjectionSelect`
- `sqlparser::ast::query::Query`
- `sqlparser::ast::query::RenameSelectItem`
- `sqlparser::ast::query::ReplaceSelectElement`
- `sqlparser::ast::query::ReplaceSelectItem`
- `sqlparser::ast::query::Select`
- `sqlparser::ast::query::SelectInto`
- `sqlparser::ast::query::SelectItem`
- `sqlparser::ast::query::SelectItemQualifiedWildcardKind`
- `sqlparser::ast::query::SetExpr`
- `sqlparser::ast::query::SymbolDefinition`
- `sqlparser::ast::query::TableAlias`
- `sqlparser::ast::query::TableAliasColumnDef`
- `sqlparser::ast::query::TableFactor`
- `sqlparser::ast::query::TableWithJoins`
- `sqlparser::ast::query::UpdateTableFromKind`
- `sqlparser::ast::query::Values`
- `sqlparser::ast::query::WildcardAdditionalOptions`
- `sqlparser::ast::query::With`
- `sqlparser::ast::query::WithFill`
- `sqlparser::ast::table_constraints::CheckConstraint`
- `sqlparser::ast::table_constraints::ConstraintUsingIndex`
- `sqlparser::ast::table_constraints::ForeignKeyConstraint`
- `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint`
- `sqlparser::ast::table_constraints::IndexConstraint`
- `sqlparser::ast::table_constraints::PrimaryKeyConstraint`
- `sqlparser::ast::table_constraints::TableConstraint`
- `sqlparser::ast::table_constraints::UniqueConstraint`
- `sqlparser::ast::value::ValueWithSpan`
- `sqlparser::tokenizer::TokenWithSpan`

**Methods** (1)

```rust
fn span(&self) -> Span
```

Trait for AST nodes that have a source location information.

# Notes:

Source [`Span`] are not yet complete. They may be missing:

1. keywords or other tokens
2. span information entirely, in which case they return [`Span::empty()`].

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

---
