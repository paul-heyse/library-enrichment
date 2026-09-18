# `sqlparser::ast::visitor`

Crate `sqlparser` · 10 public items · structured records in [`model/sqlparser.ast.visitor.json`](../model/sqlparser.ast.visitor.json)

## visit_expressions

`function` · `sqlparser::ast::visitor::visit_expressions`

```rust
fn visit_expressions<V, E, F>(v: &V, f: F) -> core::ops::ControlFlow<E> where V: Visit, F: FnMut(&ast::Expr) -> core::ops::ControlFlow<E>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.visit_expressions.md).


Invokes the provided closure on all expressions (e.g. `1 + 2`) present in `v`

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{visit_expressions};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit all expressions
let mut visited = vec![];
visit_expressions(&statements, |expr| {
  visited.push(format!("EXPR: {}", expr));
  ControlFlow::<()>::Continue(())
});

let expected : Vec<_> = [
  "EXPR: a",
  "EXPR: x IN (SELECT y FROM bar)",
  "EXPR: x",
  "EXPR: y",
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visited, expected);
```

---

## visit_expressions_mut

`function` · `sqlparser::ast::visitor::visit_expressions_mut`

```rust
fn visit_expressions_mut<V, E, F>(v: &mut V, f: F) -> core::ops::ControlFlow<E> where V: VisitMut, F: FnMut(&mut ast::Expr) -> core::ops::ControlFlow<E>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.visit_expressions_mut.md).


Invokes the provided closure iteratively with a mutable reference to all expressions
present in `v`.

This performs a depth-first search, so if the closure mutates the expression

# Example

## Remove all select limits in sub-queries
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{Expr, visit_expressions_mut, visit_statements_mut};
# use core::ops::ControlFlow;
let sql = "SELECT (SELECT y FROM z LIMIT 9) FROM t LIMIT 3";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

// Remove all select limits in sub-queries
visit_expressions_mut(&mut statements, |expr| {
  if let Expr::Subquery(q) = expr {
     q.limit_clause = None;
  }
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT (SELECT y FROM z) FROM t LIMIT 3");
```

## Wrap column name in function call

This demonstrates how to effectively replace an expression with another more complicated one
that references the original. This example avoids unnecessary allocations by using the
[`std::mem`] family of functions.

```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::*;
# use core::ops::ControlFlow;
let sql = "SELECT x, y FROM t";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

visit_expressions_mut(&mut statements, |expr| {
  if matches!(expr, Expr::Identifier(col_name) if col_name.value == "x") {
    let old_expr = std::mem::replace(expr, Expr::value(Value::Null));
    *expr = Expr::Function(Function {
          name: ObjectName::from(vec![Ident::new("f")]),
          uses_odbc_syntax: false,
          args: FunctionArguments::List(FunctionArgumentList {
              duplicate_treatment: None,
              args: vec![FunctionArg::Unnamed(FunctionArgExpr::Expr(old_expr))],
              clauses: vec![],
          }),
          null_treatment: None,
          filter: None,
          over: None,
          parameters: FunctionArguments::None,
          within_group: vec![],
     });
  }
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT f(x), y FROM t");
```

---

## visit_relations

`function` · `sqlparser::ast::visitor::visit_relations`

```rust
fn visit_relations<V, E, F>(v: &V, f: F) -> core::ops::ControlFlow<E> where V: Visit, F: FnMut(&ast::ObjectName) -> core::ops::ControlFlow<E>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.visit_relations.md).


Invokes the provided closure on all relations (e.g. table names) present in `v`

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{visit_relations};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit statements, capturing relations (table names)
let mut visited = vec![];
visit_relations(&statements, |relation| {
  visited.push(format!("RELATION: {}", relation));
  ControlFlow::<()>::Continue(())
});

let expected : Vec<_> = [
  "RELATION: foo",
  "RELATION: bar",
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visited, expected);
```

---

## visit_relations_mut

`function` · `sqlparser::ast::visitor::visit_relations_mut`

```rust
fn visit_relations_mut<V, E, F>(v: &mut V, f: F) -> core::ops::ControlFlow<E> where V: VisitMut, F: FnMut(&mut ast::ObjectName) -> core::ops::ControlFlow<E>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.visit_relations_mut.md).


Invokes the provided closure with a mutable reference to all relations (e.g. table names)
present in `v`.

When the closure mutates its argument, the new mutated relation will not be visited again.

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{ObjectName, ObjectNamePart, Ident, visit_relations_mut};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit statements, renaming table foo to bar
visit_relations_mut(&mut statements, |table| {
  table.0[0] = ObjectNamePart::Identifier(Ident::new("bar"));
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT a FROM bar");
```

---

## visit_statements

`function` · `sqlparser::ast::visitor::visit_statements`

```rust
fn visit_statements<V, E, F>(v: &V, f: F) -> core::ops::ControlFlow<E> where V: Visit, F: FnMut(&ast::Statement) -> core::ops::ControlFlow<E>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.visit_statements.md).


Invokes the provided closure iteratively with a mutable reference to all statements
present in `v` (e.g. `SELECT`, `CREATE TABLE`, etc).

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{visit_statements};
# use core::ops::ControlFlow;
let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar); CREATE TABLE baz(q int)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// visit all statements
let mut visited = vec![];
visit_statements(&statements, |stmt| {
  visited.push(format!("STATEMENT: {}", stmt));
  ControlFlow::<()>::Continue(())
});

let expected : Vec<_> = [
  "STATEMENT: SELECT a FROM foo WHERE x IN (SELECT y FROM bar)",
  "STATEMENT: CREATE TABLE baz (q INT)"
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visited, expected);
```

---

## visit_statements_mut

`function` · `sqlparser::ast::visitor::visit_statements_mut`

```rust
fn visit_statements_mut<V, E, F>(v: &mut V, f: F) -> core::ops::ControlFlow<E> where V: VisitMut, F: FnMut(&mut ast::Statement) -> core::ops::ControlFlow<E>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.visit_statements_mut.md).


Invokes the provided closure on all statements (e.g. `SELECT`, `CREATE TABLE`, etc) present in `v`

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{Statement, visit_statements_mut};
# use core::ops::ControlFlow;
let sql = "SELECT x FROM foo LIMIT 9+$limit; SELECT * FROM t LIMIT f()";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

// Remove all select limits in outer statements (not in sub-queries)
visit_statements_mut(&mut statements, |stmt| {
  if let Statement::Query(q) = stmt {
     q.limit_clause = None;
  }
  ControlFlow::<()>::Continue(())
});

assert_eq!(statements[0].to_string(), "SELECT x FROM foo");
assert_eq!(statements[1].to_string(), "SELECT * FROM t");
```

---

## Visit

`trait` · `sqlparser::ast::visitor::Visit`

```rust
trait Visit
```

**Implementors** (520)

- `alloc::boxed::Box`
- `alloc::string::String`
- `alloc::vec::Vec`
- `core::option::Option`
- `sqlparser::ast::AccessExpr`
- `sqlparser::ast::Action`
- `sqlparser::ast::ActionApplyType`
- `sqlparser::ast::ActionCreateObjectType`
- `sqlparser::ast::ActionExecuteObjectType`
- `sqlparser::ast::ActionManageType`
- `sqlparser::ast::ActionModifyType`
- `sqlparser::ast::ActionMonitorType`
- `sqlparser::ast::AddDropSync`
- `sqlparser::ast::AlterUser`
- `sqlparser::ast::AlterUserAddMfaMethodOtp`
- `sqlparser::ast::AlterUserAddRoleDelegation`
- `sqlparser::ast::AlterUserModifyMfaMethod`
- `sqlparser::ast::AlterUserPassword`
- `sqlparser::ast::AlterUserRemoveRoleDelegation`
- `sqlparser::ast::AlterUserSetPolicy`
- `sqlparser::ast::Analyze`
- `sqlparser::ast::AnalyzeFormat`
- `sqlparser::ast::AnalyzeFormatKind`
- `sqlparser::ast::ArgMode`
- `sqlparser::ast::Array`
- `sqlparser::ast::Assignment`
- `sqlparser::ast::AssignmentTarget`
- `sqlparser::ast::AttachDuckDBDatabaseOption`
- `sqlparser::ast::BeginEndStatements`
- `sqlparser::ast::BeginTransactionKind`
- `sqlparser::ast::CascadeOption`
- `sqlparser::ast::CaseStatement`
- `sqlparser::ast::CaseWhen`
- `sqlparser::ast::CastFormat`
- `sqlparser::ast::CastKind`
- `sqlparser::ast::CatalogSyncNamespaceMode`
- `sqlparser::ast::CeilFloorKind`
- `sqlparser::ast::CloseCursor`
- `sqlparser::ast::ClusteredIndex`
- `sqlparser::ast::CommentDef`
- `sqlparser::ast::CommentObject`
- `sqlparser::ast::ConditionalStatementBlock`
- `sqlparser::ast::ConditionalStatements`
- `sqlparser::ast::ConflictTarget`
- `sqlparser::ast::ConstraintReferenceMatchKind`
- `sqlparser::ast::ContactEntry`
- `sqlparser::ast::ContextModifier`
- `sqlparser::ast::CopyIntoSnowflakeKind`
- `sqlparser::ast::CopyLegacyCsvOption`
- `sqlparser::ast::CopyLegacyOption`
- `sqlparser::ast::CopyOption`
- `sqlparser::ast::CopySource`
- `sqlparser::ast::CopyTarget`
- `sqlparser::ast::CreateFunctionBody`
- `sqlparser::ast::CreateFunctionUsing`
- `sqlparser::ast::CreateServerOption`
- `sqlparser::ast::CreateServerStatement`
- `sqlparser::ast::CreateTableLike`
- `sqlparser::ast::CreateTableLikeDefaults`
- `sqlparser::ast::CreateTableLikeKind`
- `sqlparser::ast::CreateTableOptions`
- `sqlparser::ast::CreateUser`
- `sqlparser::ast::CreateViewAlgorithm`
- `sqlparser::ast::CreateViewParams`
- `sqlparser::ast::CreateViewSecurity`
- `sqlparser::ast::CurrentGrantsKind`
- `sqlparser::ast::Declare`
- `sqlparser::ast::DeclareAssignment`
- `sqlparser::ast::DeclareType`
- `sqlparser::ast::DenyStatement`
- `sqlparser::ast::DescribeAlias`
- `sqlparser::ast::DictionaryField`
- `sqlparser::ast::DiscardObject`
- `sqlparser::ast::DoUpdate`
- `sqlparser::ast::DropDomain`
- `sqlparser::ast::DuplicateTreatment`
- `sqlparser::ast::ExceptionWhen`
- `sqlparser::ast::ExportData`
- `sqlparser::ast::Expr`
- `sqlparser::ast::ExtractSyntax`
- `sqlparser::ast::FetchDirection`
- `sqlparser::ast::FetchPosition`
- `sqlparser::ast::FileFormat`
- `sqlparser::ast::FileSize`
- `sqlparser::ast::FileSizeUnit`
- `sqlparser::ast::FlushLocation`
- `sqlparser::ast::FlushType`
- `sqlparser::ast::FromTable`
- `sqlparser::ast::Function`
- `sqlparser::ast::FunctionArg`
- `sqlparser::ast::FunctionArgExpr`
- `sqlparser::ast::FunctionArgOperator`
- `sqlparser::ast::FunctionArgumentClause`
- `sqlparser::ast::FunctionArgumentList`
- `sqlparser::ast::FunctionArguments`
- `sqlparser::ast::FunctionBehavior`
- `sqlparser::ast::FunctionCalledOnNull`
- `sqlparser::ast::FunctionDefinitionSetParam`
- `sqlparser::ast::FunctionDesc`
- `sqlparser::ast::FunctionDeterminismSpecifier`
- `sqlparser::ast::FunctionParallel`
- `sqlparser::ast::FunctionSecurity`
- `sqlparser::ast::FunctionSetValue`
- `sqlparser::ast::GrantObjects`
- `sqlparser::ast::Grantee`
- `sqlparser::ast::GranteeName`
- `sqlparser::ast::GranteesType`
- `sqlparser::ast::HavingBound`
- `sqlparser::ast::HavingBoundKind`
- `sqlparser::ast::HiveDelimiter`
- `sqlparser::ast::HiveDescribeFormat`
- `sqlparser::ast::HiveDistributionStyle`
- `sqlparser::ast::HiveFormat`
- `sqlparser::ast::HiveIOFormat`
- `sqlparser::ast::HiveLoadDataFormat`
- `sqlparser::ast::HiveRowDelimiter`
- `sqlparser::ast::HiveRowFormat`
- `sqlparser::ast::HiveSetLocation`
- `sqlparser::ast::IamRoleKind`
- `sqlparser::ast::Ident`
- `sqlparser::ast::IfStatement`
- `sqlparser::ast::InitializeKind`
- `sqlparser::ast::InsertAliases`
- `sqlparser::ast::Interval`
- `sqlparser::ast::JsonNullClause`
- `sqlparser::ast::JsonPath`
- `sqlparser::ast::JsonPathElem`
- `sqlparser::ast::JsonReturningClause`
- `sqlparser::ast::KillType`
- `sqlparser::ast::LambdaFunction`
- `sqlparser::ast::LambdaFunctionParameter`
- `sqlparser::ast::LambdaSyntax`
- `sqlparser::ast::ListAggOnOverflow`
- `sqlparser::ast::Lock`
- `sqlparser::ast::LockTable`
- `sqlparser::ast::LockTableMode`
- `sqlparser::ast::LockTableTarget`
- `sqlparser::ast::LockTableType`
- `sqlparser::ast::MacroArg`
- `sqlparser::ast::MacroDefinition`
- `sqlparser::ast::Map`
- `sqlparser::ast::MapEntry`
- `sqlparser::ast::MemberOf`
- `sqlparser::ast::Method`
- `sqlparser::ast::MfaMethodKind`
- `sqlparser::ast::MinMaxValue`
- `sqlparser::ast::MySQLColumnPosition`
- `sqlparser::ast::MysqlInsertPriority`
- `sqlparser::ast::NamedParenthesizedList`
- `sqlparser::ast::NullInclusion`
- `sqlparser::ast::NullTreatment`
- `sqlparser::ast::ObjectName`
- `sqlparser::ast::ObjectNamePart`
- `sqlparser::ast::ObjectNamePartFunction`
- `sqlparser::ast::ObjectType`
- `sqlparser::ast::OnCommit`
- `sqlparser::ast::OnConflict`
- `sqlparser::ast::OnConflictAction`
- `sqlparser::ast::OnInsert`
- `sqlparser::ast::OneOrManyWithParens`
- `sqlparser::ast::OpenStatement`
- `sqlparser::ast::OperateFunctionArg`
- `sqlparser::ast::OptimizerHint`
- `sqlparser::ast::OptimizerHintStyle`
- `sqlparser::ast::Parens`
- `sqlparser::ast::PartitionRangeDirection`
- `sqlparser::ast::Password`
- `sqlparser::ast::PrintStatement`
- `sqlparser::ast::Privileges`
- `sqlparser::ast::RaisErrorOption`
- `sqlparser::ast::RaiseStatement`
- `sqlparser::ast::RaiseStatementValue`
- `sqlparser::ast::RefreshModeKind`
- `sqlparser::ast::RenameTable`
- `sqlparser::ast::Reset`
- `sqlparser::ast::ResetStatement`
- `sqlparser::ast::ReturnStatement`
- `sqlparser::ast::ReturnStatementValue`
- `sqlparser::ast::RowAccessPolicy`
- `sqlparser::ast::SchemaName`
- `sqlparser::ast::SearchModifier`
- `sqlparser::ast::SecretOption`
- `sqlparser::ast::SequenceOptions`
- `sqlparser::ast::SessionParamStatsTopic`
- `sqlparser::ast::SessionParamValue`
- `sqlparser::ast::Set`
- `sqlparser::ast::SetAssignment`
- `sqlparser::ast::SetSessionAuthorizationParam`
- `sqlparser::ast::SetSessionAuthorizationParamKind`
- `sqlparser::ast::SetSessionParamGeneric`
- `sqlparser::ast::SetSessionParamIdentityInsert`
- `sqlparser::ast::SetSessionParamKind`
- `sqlparser::ast::SetSessionParamOffsets`
- `sqlparser::ast::SetSessionParamStatistics`
- `sqlparser::ast::ShowCharset`
- `sqlparser::ast::ShowCreateObject`
- `sqlparser::ast::ShowObjects`
- `sqlparser::ast::ShowStatementFilter`
- `sqlparser::ast::ShowStatementFilterPosition`
- `sqlparser::ast::ShowStatementIn`
- `sqlparser::ast::ShowStatementInClause`
- `sqlparser::ast::ShowStatementInParentType`
- `sqlparser::ast::ShowStatementOptions`
- `sqlparser::ast::SqlOption`
- `sqlparser::ast::SqliteOnConflict`
- `sqlparser::ast::Statement`
- `sqlparser::ast::StorageLifecyclePolicy`
- `sqlparser::ast::StorageSerializationPolicy`
- `sqlparser::ast::StorageType`
- `sqlparser::ast::StructField`
- `sqlparser::ast::Subscript`
- `sqlparser::ast::TableAliasWithoutColumns`
- `sqlparser::ast::TableObject`
- `sqlparser::ast::TableOptionsClustered`
- `sqlparser::ast::TablespaceOption`
- `sqlparser::ast::Tag`
- `sqlparser::ast::ThrowStatement`
- `sqlparser::ast::TransactionAccessMode`
- `sqlparser::ast::TransactionIsolationLevel`
- `sqlparser::ast::TransactionMode`
- `sqlparser::ast::TransactionModifier`
- `sqlparser::ast::TruncateIdentityOption`
- `sqlparser::ast::TruncateTableTarget`
- `sqlparser::ast::TypedString`
- `sqlparser::ast::UnionField`
- `sqlparser::ast::UnloadPartitionBy`
- `sqlparser::ast::UserPolicyKind`
- `sqlparser::ast::UtilityOption`
- `sqlparser::ast::VacuumStatement`
- `sqlparser::ast::WaitForStatement`
- `sqlparser::ast::WaitForType`
- `sqlparser::ast::WhileStatement`
- `sqlparser::ast::WindowFrame`
- `sqlparser::ast::WindowFrameBound`
- `sqlparser::ast::WindowFrameUnits`
- `sqlparser::ast::WindowSpec`
- `sqlparser::ast::WindowType`
- `sqlparser::ast::WrappedCollection`
- `sqlparser::ast::data_type::ArrayElemTypeDef`
- `sqlparser::ast::data_type::BinaryLength`
- `sqlparser::ast::data_type::CharLengthUnits`
- `sqlparser::ast::data_type::CharacterLength`
- `sqlparser::ast::data_type::DataType`
- `sqlparser::ast::data_type::EnumMember`
- `sqlparser::ast::data_type::ExactNumberInfo`
- `sqlparser::ast::data_type::GeometricTypeKind`
- `sqlparser::ast::data_type::IntervalFields`
- `sqlparser::ast::data_type::StructBracketKind`
- `sqlparser::ast::data_type::TimezoneInfo`
- `sqlparser::ast::dcl::AlterRoleOperation`
- `sqlparser::ast::dcl::CreateRole`
- `sqlparser::ast::dcl::Grant`
- `sqlparser::ast::dcl::ResetConfig`
- `sqlparser::ast::dcl::Revoke`
- `sqlparser::ast::dcl::RoleOption`
- `sqlparser::ast::dcl::SecondaryRoles`
- `sqlparser::ast::dcl::SetConfigValue`
- `sqlparser::ast::dcl::Use`
- `sqlparser::ast::ddl::Alignment`
- `sqlparser::ast::ddl::AlterCollation`
- `sqlparser::ast::ddl::AlterCollationOperation`
- `sqlparser::ast::ddl::AlterColumnOperation`
- `sqlparser::ast::ddl::AlterConnectorOwner`
- `sqlparser::ast::ddl::AlterFunction`
- `sqlparser::ast::ddl::AlterFunctionAction`
- `sqlparser::ast::ddl::AlterFunctionKind`
- `sqlparser::ast::ddl::AlterFunctionOperation`
- `sqlparser::ast::ddl::AlterIndexOperation`
- `sqlparser::ast::ddl::AlterOperator`
- `sqlparser::ast::ddl::AlterOperatorClass`
- `sqlparser::ast::ddl::AlterOperatorClassOperation`
- `sqlparser::ast::ddl::AlterOperatorFamily`
- `sqlparser::ast::ddl::AlterOperatorFamilyOperation`
- `sqlparser::ast::ddl::AlterOperatorOperation`
- `sqlparser::ast::ddl::AlterPolicy`
- `sqlparser::ast::ddl::AlterPolicyOperation`
- `sqlparser::ast::ddl::AlterSchema`
- `sqlparser::ast::ddl::AlterSchemaOperation`
- `sqlparser::ast::ddl::AlterTable`
- `sqlparser::ast::ddl::AlterTableAlgorithm`
- `sqlparser::ast::ddl::AlterTableLock`
- `sqlparser::ast::ddl::AlterTableOperation`
- `sqlparser::ast::ddl::AlterTableType`
- `sqlparser::ast::ddl::AlterType`
- `sqlparser::ast::ddl::AlterTypeAddValue`
- `sqlparser::ast::ddl::AlterTypeAddValuePosition`
- `sqlparser::ast::ddl::AlterTypeOperation`
- `sqlparser::ast::ddl::AlterTypeRename`
- `sqlparser::ast::ddl::AlterTypeRenameValue`
- `sqlparser::ast::ddl::ClusteredBy`
- `sqlparser::ast::ddl::ColumnDef`
- `sqlparser::ast::ddl::ColumnOption`
- `sqlparser::ast::ddl::ColumnOptionDef`
- `sqlparser::ast::ddl::ColumnOptions`
- `sqlparser::ast::ddl::ColumnPolicy`
- `sqlparser::ast::ddl::ColumnPolicyProperty`
- `sqlparser::ast::ddl::ConstraintCharacteristics`
- `sqlparser::ast::ddl::CreateCollation`
- `sqlparser::ast::ddl::CreateCollationDefinition`
- `sqlparser::ast::ddl::CreateConnector`
- `sqlparser::ast::ddl::CreateDomain`
- `sqlparser::ast::ddl::CreateExtension`
- `sqlparser::ast::ddl::CreateFunction`
- `sqlparser::ast::ddl::CreateIndex`
- `sqlparser::ast::ddl::CreateOperator`
- `sqlparser::ast::ddl::CreateOperatorClass`
- `sqlparser::ast::ddl::CreateOperatorFamily`
- `sqlparser::ast::ddl::CreatePolicy`
- `sqlparser::ast::ddl::CreatePolicyCommand`
- `sqlparser::ast::ddl::CreatePolicyType`
- `sqlparser::ast::ddl::CreateTable`
- `sqlparser::ast::ddl::CreateTrigger`
- `sqlparser::ast::ddl::CreateView`
- `sqlparser::ast::ddl::Deduplicate`
- `sqlparser::ast::ddl::DeferrableInitial`
- `sqlparser::ast::ddl::DistStyle`
- `sqlparser::ast::ddl::DropBehavior`
- `sqlparser::ast::ddl::DropExtension`
- `sqlparser::ast::ddl::DropFunction`
- `sqlparser::ast::ddl::DropOperator`
- `sqlparser::ast::ddl::DropOperatorClass`
- `sqlparser::ast::ddl::DropOperatorFamily`
- `sqlparser::ast::ddl::DropOperatorSignature`
- `sqlparser::ast::ddl::DropPolicy`
- `sqlparser::ast::ddl::DropTrigger`
- `sqlparser::ast::ddl::ForValues`
- `sqlparser::ast::ddl::FunctionReturnType`
- `sqlparser::ast::ddl::GeneratedAs`
- `sqlparser::ast::ddl::GeneratedExpressionMode`
- `sqlparser::ast::ddl::IdentityParameters`
- `sqlparser::ast::ddl::IdentityProperty`
- `sqlparser::ast::ddl::IdentityPropertyFormatKind`
- `sqlparser::ast::ddl::IdentityPropertyKind`
- `sqlparser::ast::ddl::IdentityPropertyOrder`
- `sqlparser::ast::ddl::IndexColumn`
- `sqlparser::ast::ddl::IndexOption`
- `sqlparser::ast::ddl::IndexType`
- `sqlparser::ast::ddl::KeyOrIndexDisplay`
- `sqlparser::ast::ddl::Msck`
- `sqlparser::ast::ddl::NullsDistinctOption`
- `sqlparser::ast::ddl::OperatorArgTypes`
- `sqlparser::ast::ddl::OperatorClassItem`
- `sqlparser::ast::ddl::OperatorFamilyDropItem`
- `sqlparser::ast::ddl::OperatorFamilyItem`
- `sqlparser::ast::ddl::OperatorOption`
- `sqlparser::ast::ddl::OperatorPurpose`
- `sqlparser::ast::ddl::Owner`
- `sqlparser::ast::ddl::Partition`
- `sqlparser::ast::ddl::PartitionBoundValue`
- `sqlparser::ast::ddl::ProcedureParam`
- `sqlparser::ast::ddl::ReferentialAction`
- `sqlparser::ast::ddl::RenameTableNameKind`
- `sqlparser::ast::ddl::ReplicaIdentity`
- `sqlparser::ast::ddl::TagsColumnOption`
- `sqlparser::ast::ddl::TriggerObjectKind`
- `sqlparser::ast::ddl::Truncate`
- `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef`
- `sqlparser::ast::ddl::UserDefinedTypeInternalLength`
- `sqlparser::ast::ddl::UserDefinedTypeRangeOption`
- `sqlparser::ast::ddl::UserDefinedTypeRepresentation`
- `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption`
- `sqlparser::ast::ddl::UserDefinedTypeStorage`
- `sqlparser::ast::ddl::ViewColumnDef`
- `sqlparser::ast::dml::Delete`
- `sqlparser::ast::dml::Insert`
- `sqlparser::ast::dml::Merge`
- `sqlparser::ast::dml::MergeAction`
- `sqlparser::ast::dml::MergeClause`
- `sqlparser::ast::dml::MergeClauseKind`
- `sqlparser::ast::dml::MergeInsertExpr`
- `sqlparser::ast::dml::MergeInsertKind`
- `sqlparser::ast::dml::MergeUpdateExpr`
- `sqlparser::ast::dml::MultiTableInsertIntoClause`
- `sqlparser::ast::dml::MultiTableInsertType`
- `sqlparser::ast::dml::MultiTableInsertValue`
- `sqlparser::ast::dml::MultiTableInsertValues`
- `sqlparser::ast::dml::MultiTableInsertWhenClause`
- `sqlparser::ast::dml::OutputClause`
- `sqlparser::ast::dml::Update`
- `sqlparser::ast::helpers::attached_token::AttachedToken`
- `sqlparser::ast::helpers::key_value_options::KeyValueOption`
- `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind`
- `sqlparser::ast::helpers::key_value_options::KeyValueOptions`
- `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter`
- `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder`
- `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder`
- `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand`
- `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem`
- `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind`
- `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject`
- `sqlparser::ast::operator::BinaryOperator`
- `sqlparser::ast::operator::UnaryOperator`
- `sqlparser::ast::query::AfterMatchSkip`
- `sqlparser::ast::query::ConnectByKind`
- `sqlparser::ast::query::Cte`
- `sqlparser::ast::query::CteAsMaterialized`
- `sqlparser::ast::query::Distinct`
- `sqlparser::ast::query::EmptyMatchesMode`
- `sqlparser::ast::query::ExceptSelectItem`
- `sqlparser::ast::query::ExcludeSelectItem`
- `sqlparser::ast::query::ExprWithAlias`
- `sqlparser::ast::query::ExprWithAliasAndOrderBy`
- `sqlparser::ast::query::Fetch`
- `sqlparser::ast::query::ForClause`
- `sqlparser::ast::query::ForJson`
- `sqlparser::ast::query::ForXml`
- `sqlparser::ast::query::FormatClause`
- `sqlparser::ast::query::GroupByExpr`
- `sqlparser::ast::query::GroupByWithModifier`
- `sqlparser::ast::query::IdentWithAlias`
- `sqlparser::ast::query::IlikeSelectItem`
- `sqlparser::ast::query::InputFormatClause`
- `sqlparser::ast::query::Interpolate`
- `sqlparser::ast::query::InterpolateExpr`
- `sqlparser::ast::query::Join`
- `sqlparser::ast::query::JoinConstraint`
- `sqlparser::ast::query::JoinOperator`
- `sqlparser::ast::query::JsonTableColumn`
- `sqlparser::ast::query::JsonTableColumnErrorHandling`
- `sqlparser::ast::query::JsonTableNamedColumn`
- `sqlparser::ast::query::JsonTableNestedColumn`
- `sqlparser::ast::query::LateralView`
- `sqlparser::ast::query::LimitClause`
- `sqlparser::ast::query::LockClause`
- `sqlparser::ast::query::LockType`
- `sqlparser::ast::query::MatchRecognizePattern`
- `sqlparser::ast::query::MatchRecognizeSymbol`
- `sqlparser::ast::query::Measure`
- `sqlparser::ast::query::NamedWindowDefinition`
- `sqlparser::ast::query::NamedWindowExpr`
- `sqlparser::ast::query::NonBlock`
- `sqlparser::ast::query::Offset`
- `sqlparser::ast::query::OffsetRows`
- `sqlparser::ast::query::OpenJsonTableColumn`
- `sqlparser::ast::query::OrderBy`
- `sqlparser::ast::query::OrderByExpr`
- `sqlparser::ast::query::OrderByKind`
- `sqlparser::ast::query::OrderByOptions`
- `sqlparser::ast::query::PipeOperator`
- `sqlparser::ast::query::PivotValueSource`
- `sqlparser::ast::query::ProjectionSelect`
- `sqlparser::ast::query::Query`
- `sqlparser::ast::query::RenameSelectItem`
- `sqlparser::ast::query::RepetitionQuantifier`
- `sqlparser::ast::query::ReplaceSelectElement`
- `sqlparser::ast::query::ReplaceSelectItem`
- `sqlparser::ast::query::RowsPerMatch`
- `sqlparser::ast::query::Select`
- `sqlparser::ast::query::SelectFlavor`
- `sqlparser::ast::query::SelectInto`
- `sqlparser::ast::query::SelectItem`
- `sqlparser::ast::query::SelectItemQualifiedWildcardKind`
- `sqlparser::ast::query::SelectModifiers`
- `sqlparser::ast::query::SetExpr`
- `sqlparser::ast::query::SetOperator`
- `sqlparser::ast::query::SetQuantifier`
- `sqlparser::ast::query::Setting`
- `sqlparser::ast::query::SymbolDefinition`
- `sqlparser::ast::query::Table`
- `sqlparser::ast::query::TableAlias`
- `sqlparser::ast::query::TableAliasColumnDef`
- `sqlparser::ast::query::TableFactor`
- `sqlparser::ast::query::TableFunctionArgs`
- `sqlparser::ast::query::TableIndexHintForClause`
- `sqlparser::ast::query::TableIndexHintType`
- `sqlparser::ast::query::TableIndexHints`
- `sqlparser::ast::query::TableIndexType`
- `sqlparser::ast::query::TableSample`
- `sqlparser::ast::query::TableSampleBucket`
- `sqlparser::ast::query::TableSampleKind`
- `sqlparser::ast::query::TableSampleMethod`
- `sqlparser::ast::query::TableSampleModifier`
- `sqlparser::ast::query::TableSampleQuantity`
- `sqlparser::ast::query::TableSampleSeed`
- `sqlparser::ast::query::TableSampleSeedModifier`
- `sqlparser::ast::query::TableSampleUnit`
- `sqlparser::ast::query::TableVersion`
- `sqlparser::ast::query::TableWithJoins`
- `sqlparser::ast::query::Top`
- `sqlparser::ast::query::TopQuantity`
- `sqlparser::ast::query::UpdateTableFromKind`
- `sqlparser::ast::query::ValueTableMode`
- `sqlparser::ast::query::Values`
- `sqlparser::ast::query::WildcardAdditionalOptions`
- `sqlparser::ast::query::With`
- `sqlparser::ast::query::WithFill`
- `sqlparser::ast::query::XmlNamespaceDefinition`
- `sqlparser::ast::query::XmlPassingArgument`
- `sqlparser::ast::query::XmlPassingClause`
- `sqlparser::ast::query::XmlTableColumn`
- `sqlparser::ast::query::XmlTableColumnOption`
- `sqlparser::ast::table_constraints::CheckConstraint`
- `sqlparser::ast::table_constraints::ConstraintUsingIndex`
- `sqlparser::ast::table_constraints::ForeignKeyConstraint`
- `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint`
- `sqlparser::ast::table_constraints::IndexConstraint`
- `sqlparser::ast::table_constraints::PrimaryKeyConstraint`
- `sqlparser::ast::table_constraints::TableConstraint`
- `sqlparser::ast::table_constraints::UniqueConstraint`
- `sqlparser::ast::trigger::TriggerEvent`
- `sqlparser::ast::trigger::TriggerExecBody`
- `sqlparser::ast::trigger::TriggerExecBodyType`
- `sqlparser::ast::trigger::TriggerObject`
- `sqlparser::ast::trigger::TriggerPeriod`
- `sqlparser::ast::trigger::TriggerReferencing`
- `sqlparser::ast::trigger::TriggerReferencingType`
- `sqlparser::ast::value::DateTimeField`
- `sqlparser::ast::value::DollarQuotedString`
- `sqlparser::ast::value::NormalizationForm`
- `sqlparser::ast::value::QuoteDelimitedString`
- `sqlparser::ast::value::TrimWhereField`
- `sqlparser::ast::value::Value`
- `sqlparser::ast::value::ValueWithSpan`
- `sqlparser::keywords::Keyword`
- `sqlparser::tokenizer::Location`
- `sqlparser::tokenizer::Span`
- `sqlparser::tokenizer::Token`
- `sqlparser::tokenizer::TokenWithSpan`
- `sqlparser::tokenizer::Whitespace`
- `sqlparser::tokenizer::Word`

**Methods** (1)

```rust
fn visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.Visit.md).


A type that can be visited by a [`Visitor`]. See [`Visitor`] for
recursively visiting parsed SQL statements.

# Note

This trait should be automatically derived for sqlparser AST nodes
using the [Visit](sqlparser_derive::Visit) proc macro.

```text
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
```

---

## VisitMut

`trait` · `sqlparser::ast::visitor::VisitMut`

```rust
trait VisitMut
```

**Implementors** (520)

- `alloc::boxed::Box`
- `alloc::string::String`
- `alloc::vec::Vec`
- `core::option::Option`
- `sqlparser::ast::AccessExpr`
- `sqlparser::ast::Action`
- `sqlparser::ast::ActionApplyType`
- `sqlparser::ast::ActionCreateObjectType`
- `sqlparser::ast::ActionExecuteObjectType`
- `sqlparser::ast::ActionManageType`
- `sqlparser::ast::ActionModifyType`
- `sqlparser::ast::ActionMonitorType`
- `sqlparser::ast::AddDropSync`
- `sqlparser::ast::AlterUser`
- `sqlparser::ast::AlterUserAddMfaMethodOtp`
- `sqlparser::ast::AlterUserAddRoleDelegation`
- `sqlparser::ast::AlterUserModifyMfaMethod`
- `sqlparser::ast::AlterUserPassword`
- `sqlparser::ast::AlterUserRemoveRoleDelegation`
- `sqlparser::ast::AlterUserSetPolicy`
- `sqlparser::ast::Analyze`
- `sqlparser::ast::AnalyzeFormat`
- `sqlparser::ast::AnalyzeFormatKind`
- `sqlparser::ast::ArgMode`
- `sqlparser::ast::Array`
- `sqlparser::ast::Assignment`
- `sqlparser::ast::AssignmentTarget`
- `sqlparser::ast::AttachDuckDBDatabaseOption`
- `sqlparser::ast::BeginEndStatements`
- `sqlparser::ast::BeginTransactionKind`
- `sqlparser::ast::CascadeOption`
- `sqlparser::ast::CaseStatement`
- `sqlparser::ast::CaseWhen`
- `sqlparser::ast::CastFormat`
- `sqlparser::ast::CastKind`
- `sqlparser::ast::CatalogSyncNamespaceMode`
- `sqlparser::ast::CeilFloorKind`
- `sqlparser::ast::CloseCursor`
- `sqlparser::ast::ClusteredIndex`
- `sqlparser::ast::CommentDef`
- `sqlparser::ast::CommentObject`
- `sqlparser::ast::ConditionalStatementBlock`
- `sqlparser::ast::ConditionalStatements`
- `sqlparser::ast::ConflictTarget`
- `sqlparser::ast::ConstraintReferenceMatchKind`
- `sqlparser::ast::ContactEntry`
- `sqlparser::ast::ContextModifier`
- `sqlparser::ast::CopyIntoSnowflakeKind`
- `sqlparser::ast::CopyLegacyCsvOption`
- `sqlparser::ast::CopyLegacyOption`
- `sqlparser::ast::CopyOption`
- `sqlparser::ast::CopySource`
- `sqlparser::ast::CopyTarget`
- `sqlparser::ast::CreateFunctionBody`
- `sqlparser::ast::CreateFunctionUsing`
- `sqlparser::ast::CreateServerOption`
- `sqlparser::ast::CreateServerStatement`
- `sqlparser::ast::CreateTableLike`
- `sqlparser::ast::CreateTableLikeDefaults`
- `sqlparser::ast::CreateTableLikeKind`
- `sqlparser::ast::CreateTableOptions`
- `sqlparser::ast::CreateUser`
- `sqlparser::ast::CreateViewAlgorithm`
- `sqlparser::ast::CreateViewParams`
- `sqlparser::ast::CreateViewSecurity`
- `sqlparser::ast::CurrentGrantsKind`
- `sqlparser::ast::Declare`
- `sqlparser::ast::DeclareAssignment`
- `sqlparser::ast::DeclareType`
- `sqlparser::ast::DenyStatement`
- `sqlparser::ast::DescribeAlias`
- `sqlparser::ast::DictionaryField`
- `sqlparser::ast::DiscardObject`
- `sqlparser::ast::DoUpdate`
- `sqlparser::ast::DropDomain`
- `sqlparser::ast::DuplicateTreatment`
- `sqlparser::ast::ExceptionWhen`
- `sqlparser::ast::ExportData`
- `sqlparser::ast::Expr`
- `sqlparser::ast::ExtractSyntax`
- `sqlparser::ast::FetchDirection`
- `sqlparser::ast::FetchPosition`
- `sqlparser::ast::FileFormat`
- `sqlparser::ast::FileSize`
- `sqlparser::ast::FileSizeUnit`
- `sqlparser::ast::FlushLocation`
- `sqlparser::ast::FlushType`
- `sqlparser::ast::FromTable`
- `sqlparser::ast::Function`
- `sqlparser::ast::FunctionArg`
- `sqlparser::ast::FunctionArgExpr`
- `sqlparser::ast::FunctionArgOperator`
- `sqlparser::ast::FunctionArgumentClause`
- `sqlparser::ast::FunctionArgumentList`
- `sqlparser::ast::FunctionArguments`
- `sqlparser::ast::FunctionBehavior`
- `sqlparser::ast::FunctionCalledOnNull`
- `sqlparser::ast::FunctionDefinitionSetParam`
- `sqlparser::ast::FunctionDesc`
- `sqlparser::ast::FunctionDeterminismSpecifier`
- `sqlparser::ast::FunctionParallel`
- `sqlparser::ast::FunctionSecurity`
- `sqlparser::ast::FunctionSetValue`
- `sqlparser::ast::GrantObjects`
- `sqlparser::ast::Grantee`
- `sqlparser::ast::GranteeName`
- `sqlparser::ast::GranteesType`
- `sqlparser::ast::HavingBound`
- `sqlparser::ast::HavingBoundKind`
- `sqlparser::ast::HiveDelimiter`
- `sqlparser::ast::HiveDescribeFormat`
- `sqlparser::ast::HiveDistributionStyle`
- `sqlparser::ast::HiveFormat`
- `sqlparser::ast::HiveIOFormat`
- `sqlparser::ast::HiveLoadDataFormat`
- `sqlparser::ast::HiveRowDelimiter`
- `sqlparser::ast::HiveRowFormat`
- `sqlparser::ast::HiveSetLocation`
- `sqlparser::ast::IamRoleKind`
- `sqlparser::ast::Ident`
- `sqlparser::ast::IfStatement`
- `sqlparser::ast::InitializeKind`
- `sqlparser::ast::InsertAliases`
- `sqlparser::ast::Interval`
- `sqlparser::ast::JsonNullClause`
- `sqlparser::ast::JsonPath`
- `sqlparser::ast::JsonPathElem`
- `sqlparser::ast::JsonReturningClause`
- `sqlparser::ast::KillType`
- `sqlparser::ast::LambdaFunction`
- `sqlparser::ast::LambdaFunctionParameter`
- `sqlparser::ast::LambdaSyntax`
- `sqlparser::ast::ListAggOnOverflow`
- `sqlparser::ast::Lock`
- `sqlparser::ast::LockTable`
- `sqlparser::ast::LockTableMode`
- `sqlparser::ast::LockTableTarget`
- `sqlparser::ast::LockTableType`
- `sqlparser::ast::MacroArg`
- `sqlparser::ast::MacroDefinition`
- `sqlparser::ast::Map`
- `sqlparser::ast::MapEntry`
- `sqlparser::ast::MemberOf`
- `sqlparser::ast::Method`
- `sqlparser::ast::MfaMethodKind`
- `sqlparser::ast::MinMaxValue`
- `sqlparser::ast::MySQLColumnPosition`
- `sqlparser::ast::MysqlInsertPriority`
- `sqlparser::ast::NamedParenthesizedList`
- `sqlparser::ast::NullInclusion`
- `sqlparser::ast::NullTreatment`
- `sqlparser::ast::ObjectName`
- `sqlparser::ast::ObjectNamePart`
- `sqlparser::ast::ObjectNamePartFunction`
- `sqlparser::ast::ObjectType`
- `sqlparser::ast::OnCommit`
- `sqlparser::ast::OnConflict`
- `sqlparser::ast::OnConflictAction`
- `sqlparser::ast::OnInsert`
- `sqlparser::ast::OneOrManyWithParens`
- `sqlparser::ast::OpenStatement`
- `sqlparser::ast::OperateFunctionArg`
- `sqlparser::ast::OptimizerHint`
- `sqlparser::ast::OptimizerHintStyle`
- `sqlparser::ast::Parens`
- `sqlparser::ast::PartitionRangeDirection`
- `sqlparser::ast::Password`
- `sqlparser::ast::PrintStatement`
- `sqlparser::ast::Privileges`
- `sqlparser::ast::RaisErrorOption`
- `sqlparser::ast::RaiseStatement`
- `sqlparser::ast::RaiseStatementValue`
- `sqlparser::ast::RefreshModeKind`
- `sqlparser::ast::RenameTable`
- `sqlparser::ast::Reset`
- `sqlparser::ast::ResetStatement`
- `sqlparser::ast::ReturnStatement`
- `sqlparser::ast::ReturnStatementValue`
- `sqlparser::ast::RowAccessPolicy`
- `sqlparser::ast::SchemaName`
- `sqlparser::ast::SearchModifier`
- `sqlparser::ast::SecretOption`
- `sqlparser::ast::SequenceOptions`
- `sqlparser::ast::SessionParamStatsTopic`
- `sqlparser::ast::SessionParamValue`
- `sqlparser::ast::Set`
- `sqlparser::ast::SetAssignment`
- `sqlparser::ast::SetSessionAuthorizationParam`
- `sqlparser::ast::SetSessionAuthorizationParamKind`
- `sqlparser::ast::SetSessionParamGeneric`
- `sqlparser::ast::SetSessionParamIdentityInsert`
- `sqlparser::ast::SetSessionParamKind`
- `sqlparser::ast::SetSessionParamOffsets`
- `sqlparser::ast::SetSessionParamStatistics`
- `sqlparser::ast::ShowCharset`
- `sqlparser::ast::ShowCreateObject`
- `sqlparser::ast::ShowObjects`
- `sqlparser::ast::ShowStatementFilter`
- `sqlparser::ast::ShowStatementFilterPosition`
- `sqlparser::ast::ShowStatementIn`
- `sqlparser::ast::ShowStatementInClause`
- `sqlparser::ast::ShowStatementInParentType`
- `sqlparser::ast::ShowStatementOptions`
- `sqlparser::ast::SqlOption`
- `sqlparser::ast::SqliteOnConflict`
- `sqlparser::ast::Statement`
- `sqlparser::ast::StorageLifecyclePolicy`
- `sqlparser::ast::StorageSerializationPolicy`
- `sqlparser::ast::StorageType`
- `sqlparser::ast::StructField`
- `sqlparser::ast::Subscript`
- `sqlparser::ast::TableAliasWithoutColumns`
- `sqlparser::ast::TableObject`
- `sqlparser::ast::TableOptionsClustered`
- `sqlparser::ast::TablespaceOption`
- `sqlparser::ast::Tag`
- `sqlparser::ast::ThrowStatement`
- `sqlparser::ast::TransactionAccessMode`
- `sqlparser::ast::TransactionIsolationLevel`
- `sqlparser::ast::TransactionMode`
- `sqlparser::ast::TransactionModifier`
- `sqlparser::ast::TruncateIdentityOption`
- `sqlparser::ast::TruncateTableTarget`
- `sqlparser::ast::TypedString`
- `sqlparser::ast::UnionField`
- `sqlparser::ast::UnloadPartitionBy`
- `sqlparser::ast::UserPolicyKind`
- `sqlparser::ast::UtilityOption`
- `sqlparser::ast::VacuumStatement`
- `sqlparser::ast::WaitForStatement`
- `sqlparser::ast::WaitForType`
- `sqlparser::ast::WhileStatement`
- `sqlparser::ast::WindowFrame`
- `sqlparser::ast::WindowFrameBound`
- `sqlparser::ast::WindowFrameUnits`
- `sqlparser::ast::WindowSpec`
- `sqlparser::ast::WindowType`
- `sqlparser::ast::WrappedCollection`
- `sqlparser::ast::data_type::ArrayElemTypeDef`
- `sqlparser::ast::data_type::BinaryLength`
- `sqlparser::ast::data_type::CharLengthUnits`
- `sqlparser::ast::data_type::CharacterLength`
- `sqlparser::ast::data_type::DataType`
- `sqlparser::ast::data_type::EnumMember`
- `sqlparser::ast::data_type::ExactNumberInfo`
- `sqlparser::ast::data_type::GeometricTypeKind`
- `sqlparser::ast::data_type::IntervalFields`
- `sqlparser::ast::data_type::StructBracketKind`
- `sqlparser::ast::data_type::TimezoneInfo`
- `sqlparser::ast::dcl::AlterRoleOperation`
- `sqlparser::ast::dcl::CreateRole`
- `sqlparser::ast::dcl::Grant`
- `sqlparser::ast::dcl::ResetConfig`
- `sqlparser::ast::dcl::Revoke`
- `sqlparser::ast::dcl::RoleOption`
- `sqlparser::ast::dcl::SecondaryRoles`
- `sqlparser::ast::dcl::SetConfigValue`
- `sqlparser::ast::dcl::Use`
- `sqlparser::ast::ddl::Alignment`
- `sqlparser::ast::ddl::AlterCollation`
- `sqlparser::ast::ddl::AlterCollationOperation`
- `sqlparser::ast::ddl::AlterColumnOperation`
- `sqlparser::ast::ddl::AlterConnectorOwner`
- `sqlparser::ast::ddl::AlterFunction`
- `sqlparser::ast::ddl::AlterFunctionAction`
- `sqlparser::ast::ddl::AlterFunctionKind`
- `sqlparser::ast::ddl::AlterFunctionOperation`
- `sqlparser::ast::ddl::AlterIndexOperation`
- `sqlparser::ast::ddl::AlterOperator`
- `sqlparser::ast::ddl::AlterOperatorClass`
- `sqlparser::ast::ddl::AlterOperatorClassOperation`
- `sqlparser::ast::ddl::AlterOperatorFamily`
- `sqlparser::ast::ddl::AlterOperatorFamilyOperation`
- `sqlparser::ast::ddl::AlterOperatorOperation`
- `sqlparser::ast::ddl::AlterPolicy`
- `sqlparser::ast::ddl::AlterPolicyOperation`
- `sqlparser::ast::ddl::AlterSchema`
- `sqlparser::ast::ddl::AlterSchemaOperation`
- `sqlparser::ast::ddl::AlterTable`
- `sqlparser::ast::ddl::AlterTableAlgorithm`
- `sqlparser::ast::ddl::AlterTableLock`
- `sqlparser::ast::ddl::AlterTableOperation`
- `sqlparser::ast::ddl::AlterTableType`
- `sqlparser::ast::ddl::AlterType`
- `sqlparser::ast::ddl::AlterTypeAddValue`
- `sqlparser::ast::ddl::AlterTypeAddValuePosition`
- `sqlparser::ast::ddl::AlterTypeOperation`
- `sqlparser::ast::ddl::AlterTypeRename`
- `sqlparser::ast::ddl::AlterTypeRenameValue`
- `sqlparser::ast::ddl::ClusteredBy`
- `sqlparser::ast::ddl::ColumnDef`
- `sqlparser::ast::ddl::ColumnOption`
- `sqlparser::ast::ddl::ColumnOptionDef`
- `sqlparser::ast::ddl::ColumnOptions`
- `sqlparser::ast::ddl::ColumnPolicy`
- `sqlparser::ast::ddl::ColumnPolicyProperty`
- `sqlparser::ast::ddl::ConstraintCharacteristics`
- `sqlparser::ast::ddl::CreateCollation`
- `sqlparser::ast::ddl::CreateCollationDefinition`
- `sqlparser::ast::ddl::CreateConnector`
- `sqlparser::ast::ddl::CreateDomain`
- `sqlparser::ast::ddl::CreateExtension`
- `sqlparser::ast::ddl::CreateFunction`
- `sqlparser::ast::ddl::CreateIndex`
- `sqlparser::ast::ddl::CreateOperator`
- `sqlparser::ast::ddl::CreateOperatorClass`
- `sqlparser::ast::ddl::CreateOperatorFamily`
- `sqlparser::ast::ddl::CreatePolicy`
- `sqlparser::ast::ddl::CreatePolicyCommand`
- `sqlparser::ast::ddl::CreatePolicyType`
- `sqlparser::ast::ddl::CreateTable`
- `sqlparser::ast::ddl::CreateTrigger`
- `sqlparser::ast::ddl::CreateView`
- `sqlparser::ast::ddl::Deduplicate`
- `sqlparser::ast::ddl::DeferrableInitial`
- `sqlparser::ast::ddl::DistStyle`
- `sqlparser::ast::ddl::DropBehavior`
- `sqlparser::ast::ddl::DropExtension`
- `sqlparser::ast::ddl::DropFunction`
- `sqlparser::ast::ddl::DropOperator`
- `sqlparser::ast::ddl::DropOperatorClass`
- `sqlparser::ast::ddl::DropOperatorFamily`
- `sqlparser::ast::ddl::DropOperatorSignature`
- `sqlparser::ast::ddl::DropPolicy`
- `sqlparser::ast::ddl::DropTrigger`
- `sqlparser::ast::ddl::ForValues`
- `sqlparser::ast::ddl::FunctionReturnType`
- `sqlparser::ast::ddl::GeneratedAs`
- `sqlparser::ast::ddl::GeneratedExpressionMode`
- `sqlparser::ast::ddl::IdentityParameters`
- `sqlparser::ast::ddl::IdentityProperty`
- `sqlparser::ast::ddl::IdentityPropertyFormatKind`
- `sqlparser::ast::ddl::IdentityPropertyKind`
- `sqlparser::ast::ddl::IdentityPropertyOrder`
- `sqlparser::ast::ddl::IndexColumn`
- `sqlparser::ast::ddl::IndexOption`
- `sqlparser::ast::ddl::IndexType`
- `sqlparser::ast::ddl::KeyOrIndexDisplay`
- `sqlparser::ast::ddl::Msck`
- `sqlparser::ast::ddl::NullsDistinctOption`
- `sqlparser::ast::ddl::OperatorArgTypes`
- `sqlparser::ast::ddl::OperatorClassItem`
- `sqlparser::ast::ddl::OperatorFamilyDropItem`
- `sqlparser::ast::ddl::OperatorFamilyItem`
- `sqlparser::ast::ddl::OperatorOption`
- `sqlparser::ast::ddl::OperatorPurpose`
- `sqlparser::ast::ddl::Owner`
- `sqlparser::ast::ddl::Partition`
- `sqlparser::ast::ddl::PartitionBoundValue`
- `sqlparser::ast::ddl::ProcedureParam`
- `sqlparser::ast::ddl::ReferentialAction`
- `sqlparser::ast::ddl::RenameTableNameKind`
- `sqlparser::ast::ddl::ReplicaIdentity`
- `sqlparser::ast::ddl::TagsColumnOption`
- `sqlparser::ast::ddl::TriggerObjectKind`
- `sqlparser::ast::ddl::Truncate`
- `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef`
- `sqlparser::ast::ddl::UserDefinedTypeInternalLength`
- `sqlparser::ast::ddl::UserDefinedTypeRangeOption`
- `sqlparser::ast::ddl::UserDefinedTypeRepresentation`
- `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption`
- `sqlparser::ast::ddl::UserDefinedTypeStorage`
- `sqlparser::ast::ddl::ViewColumnDef`
- `sqlparser::ast::dml::Delete`
- `sqlparser::ast::dml::Insert`
- `sqlparser::ast::dml::Merge`
- `sqlparser::ast::dml::MergeAction`
- `sqlparser::ast::dml::MergeClause`
- `sqlparser::ast::dml::MergeClauseKind`
- `sqlparser::ast::dml::MergeInsertExpr`
- `sqlparser::ast::dml::MergeInsertKind`
- `sqlparser::ast::dml::MergeUpdateExpr`
- `sqlparser::ast::dml::MultiTableInsertIntoClause`
- `sqlparser::ast::dml::MultiTableInsertType`
- `sqlparser::ast::dml::MultiTableInsertValue`
- `sqlparser::ast::dml::MultiTableInsertValues`
- `sqlparser::ast::dml::MultiTableInsertWhenClause`
- `sqlparser::ast::dml::OutputClause`
- `sqlparser::ast::dml::Update`
- `sqlparser::ast::helpers::attached_token::AttachedToken`
- `sqlparser::ast::helpers::key_value_options::KeyValueOption`
- `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind`
- `sqlparser::ast::helpers::key_value_options::KeyValueOptions`
- `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter`
- `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder`
- `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder`
- `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand`
- `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem`
- `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind`
- `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject`
- `sqlparser::ast::operator::BinaryOperator`
- `sqlparser::ast::operator::UnaryOperator`
- `sqlparser::ast::query::AfterMatchSkip`
- `sqlparser::ast::query::ConnectByKind`
- `sqlparser::ast::query::Cte`
- `sqlparser::ast::query::CteAsMaterialized`
- `sqlparser::ast::query::Distinct`
- `sqlparser::ast::query::EmptyMatchesMode`
- `sqlparser::ast::query::ExceptSelectItem`
- `sqlparser::ast::query::ExcludeSelectItem`
- `sqlparser::ast::query::ExprWithAlias`
- `sqlparser::ast::query::ExprWithAliasAndOrderBy`
- `sqlparser::ast::query::Fetch`
- `sqlparser::ast::query::ForClause`
- `sqlparser::ast::query::ForJson`
- `sqlparser::ast::query::ForXml`
- `sqlparser::ast::query::FormatClause`
- `sqlparser::ast::query::GroupByExpr`
- `sqlparser::ast::query::GroupByWithModifier`
- `sqlparser::ast::query::IdentWithAlias`
- `sqlparser::ast::query::IlikeSelectItem`
- `sqlparser::ast::query::InputFormatClause`
- `sqlparser::ast::query::Interpolate`
- `sqlparser::ast::query::InterpolateExpr`
- `sqlparser::ast::query::Join`
- `sqlparser::ast::query::JoinConstraint`
- `sqlparser::ast::query::JoinOperator`
- `sqlparser::ast::query::JsonTableColumn`
- `sqlparser::ast::query::JsonTableColumnErrorHandling`
- `sqlparser::ast::query::JsonTableNamedColumn`
- `sqlparser::ast::query::JsonTableNestedColumn`
- `sqlparser::ast::query::LateralView`
- `sqlparser::ast::query::LimitClause`
- `sqlparser::ast::query::LockClause`
- `sqlparser::ast::query::LockType`
- `sqlparser::ast::query::MatchRecognizePattern`
- `sqlparser::ast::query::MatchRecognizeSymbol`
- `sqlparser::ast::query::Measure`
- `sqlparser::ast::query::NamedWindowDefinition`
- `sqlparser::ast::query::NamedWindowExpr`
- `sqlparser::ast::query::NonBlock`
- `sqlparser::ast::query::Offset`
- `sqlparser::ast::query::OffsetRows`
- `sqlparser::ast::query::OpenJsonTableColumn`
- `sqlparser::ast::query::OrderBy`
- `sqlparser::ast::query::OrderByExpr`
- `sqlparser::ast::query::OrderByKind`
- `sqlparser::ast::query::OrderByOptions`
- `sqlparser::ast::query::PipeOperator`
- `sqlparser::ast::query::PivotValueSource`
- `sqlparser::ast::query::ProjectionSelect`
- `sqlparser::ast::query::Query`
- `sqlparser::ast::query::RenameSelectItem`
- `sqlparser::ast::query::RepetitionQuantifier`
- `sqlparser::ast::query::ReplaceSelectElement`
- `sqlparser::ast::query::ReplaceSelectItem`
- `sqlparser::ast::query::RowsPerMatch`
- `sqlparser::ast::query::Select`
- `sqlparser::ast::query::SelectFlavor`
- `sqlparser::ast::query::SelectInto`
- `sqlparser::ast::query::SelectItem`
- `sqlparser::ast::query::SelectItemQualifiedWildcardKind`
- `sqlparser::ast::query::SelectModifiers`
- `sqlparser::ast::query::SetExpr`
- `sqlparser::ast::query::SetOperator`
- `sqlparser::ast::query::SetQuantifier`
- `sqlparser::ast::query::Setting`
- `sqlparser::ast::query::SymbolDefinition`
- `sqlparser::ast::query::Table`
- `sqlparser::ast::query::TableAlias`
- `sqlparser::ast::query::TableAliasColumnDef`
- `sqlparser::ast::query::TableFactor`
- `sqlparser::ast::query::TableFunctionArgs`
- `sqlparser::ast::query::TableIndexHintForClause`
- `sqlparser::ast::query::TableIndexHintType`
- `sqlparser::ast::query::TableIndexHints`
- `sqlparser::ast::query::TableIndexType`
- `sqlparser::ast::query::TableSample`
- `sqlparser::ast::query::TableSampleBucket`
- `sqlparser::ast::query::TableSampleKind`
- `sqlparser::ast::query::TableSampleMethod`
- `sqlparser::ast::query::TableSampleModifier`
- `sqlparser::ast::query::TableSampleQuantity`
- `sqlparser::ast::query::TableSampleSeed`
- `sqlparser::ast::query::TableSampleSeedModifier`
- `sqlparser::ast::query::TableSampleUnit`
- `sqlparser::ast::query::TableVersion`
- `sqlparser::ast::query::TableWithJoins`
- `sqlparser::ast::query::Top`
- `sqlparser::ast::query::TopQuantity`
- `sqlparser::ast::query::UpdateTableFromKind`
- `sqlparser::ast::query::ValueTableMode`
- `sqlparser::ast::query::Values`
- `sqlparser::ast::query::WildcardAdditionalOptions`
- `sqlparser::ast::query::With`
- `sqlparser::ast::query::WithFill`
- `sqlparser::ast::query::XmlNamespaceDefinition`
- `sqlparser::ast::query::XmlPassingArgument`
- `sqlparser::ast::query::XmlPassingClause`
- `sqlparser::ast::query::XmlTableColumn`
- `sqlparser::ast::query::XmlTableColumnOption`
- `sqlparser::ast::table_constraints::CheckConstraint`
- `sqlparser::ast::table_constraints::ConstraintUsingIndex`
- `sqlparser::ast::table_constraints::ForeignKeyConstraint`
- `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint`
- `sqlparser::ast::table_constraints::IndexConstraint`
- `sqlparser::ast::table_constraints::PrimaryKeyConstraint`
- `sqlparser::ast::table_constraints::TableConstraint`
- `sqlparser::ast::table_constraints::UniqueConstraint`
- `sqlparser::ast::trigger::TriggerEvent`
- `sqlparser::ast::trigger::TriggerExecBody`
- `sqlparser::ast::trigger::TriggerExecBodyType`
- `sqlparser::ast::trigger::TriggerObject`
- `sqlparser::ast::trigger::TriggerPeriod`
- `sqlparser::ast::trigger::TriggerReferencing`
- `sqlparser::ast::trigger::TriggerReferencingType`
- `sqlparser::ast::value::DateTimeField`
- `sqlparser::ast::value::DollarQuotedString`
- `sqlparser::ast::value::NormalizationForm`
- `sqlparser::ast::value::QuoteDelimitedString`
- `sqlparser::ast::value::TrimWhereField`
- `sqlparser::ast::value::Value`
- `sqlparser::ast::value::ValueWithSpan`
- `sqlparser::keywords::Keyword`
- `sqlparser::tokenizer::Location`
- `sqlparser::tokenizer::Span`
- `sqlparser::tokenizer::Token`
- `sqlparser::tokenizer::TokenWithSpan`
- `sqlparser::tokenizer::Whitespace`
- `sqlparser::tokenizer::Word`

**Methods** (1)

```rust
fn visit<V: VisitorMut>(&mut self, visitor: &mut V) -> ControlFlow<V::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.VisitMut.md).


A type that can be visited by a [`VisitorMut`]. See [`VisitorMut`] for
recursively visiting parsed SQL statements.

# Note

This trait should be automatically derived for sqlparser AST nodes
using the [VisitMut](sqlparser_derive::VisitMut) proc macro.

```text
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
```

---

## Visitor

`trait` · `sqlparser::ast::visitor::Visitor`

```rust
trait Visitor
```

**Methods** (14)

```rust
fn post_visit_expr(&mut self, _expr: &Expr) -> ControlFlow<Self::Break>
fn post_visit_query(&mut self, _query: &Query) -> ControlFlow<Self::Break>
fn post_visit_relation(&mut self, _relation: &ObjectName) -> ControlFlow<Self::Break>
fn post_visit_select(&mut self, _select: &Select) -> ControlFlow<Self::Break>
fn post_visit_statement(&mut self, _statement: &Statement) -> ControlFlow<Self::Break>
fn post_visit_table_factor(&mut self, _table_factor: &TableFactor) -> ControlFlow<Self::Break>
fn post_visit_value(&mut self, _value: &ValueWithSpan) -> ControlFlow<Self::Break>
fn pre_visit_expr(&mut self, _expr: &Expr) -> ControlFlow<Self::Break>
fn pre_visit_query(&mut self, _query: &Query) -> ControlFlow<Self::Break>
fn pre_visit_relation(&mut self, _relation: &ObjectName) -> ControlFlow<Self::Break>
fn pre_visit_select(&mut self, _select: &Select) -> ControlFlow<Self::Break>
fn pre_visit_statement(&mut self, _statement: &Statement) -> ControlFlow<Self::Break>
fn pre_visit_table_factor(&mut self, _table_factor: &TableFactor) -> ControlFlow<Self::Break>
fn pre_visit_value(&mut self, _value: &ValueWithSpan) -> ControlFlow<Self::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.Visitor.md).


A visitor that can be used to walk an AST tree.

`pre_visit_` methods are invoked before visiting all children of the
node and `post_visit_` methods are invoked after visiting all
children of the node.

# See also

These methods provide a more concise way of visiting nodes of a certain type:
* [visit_relations]
* [visit_expressions]
* [visit_statements]

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{Visit, Visitor, ObjectName, Expr};
# use core::ops::ControlFlow;
// A structure that records statements and relations
#[derive(Default)]
struct V {
   visited: Vec<String>,
}

// Visit relations and exprs before children are visited (depth first walk)
// Note you can also visit statements and visit exprs after children have been visited
impl Visitor for V {
  type Break = ();

  fn pre_visit_relation(&mut self, relation: &ObjectName) -> ControlFlow<Self::Break> {
    self.visited.push(format!("PRE: RELATION: {}", relation));
    ControlFlow::Continue(())
  }

  fn pre_visit_expr(&mut self, expr: &Expr) -> ControlFlow<Self::Break> {
    self.visited.push(format!("PRE: EXPR: {}", expr));
    ControlFlow::Continue(())
  }
}

let sql = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statements = Parser::parse_sql(&GenericDialect{}, sql)
   .unwrap();

// Drive the visitor through the AST
let mut visitor = V::default();
statements.visit(&mut visitor);

// The visitor has visited statements and expressions in pre-traversal order
let expected : Vec<_> = [
  "PRE: EXPR: a",
  "PRE: RELATION: foo",
  "PRE: EXPR: x IN (SELECT y FROM bar)",
  "PRE: EXPR: x",
  "PRE: EXPR: y",
  "PRE: RELATION: bar",
]
  .into_iter().map(|s| s.to_string()).collect();

assert_eq!(visitor.visited, expected);
```

---

## VisitorMut

`trait` · `sqlparser::ast::visitor::VisitorMut`

```rust
trait VisitorMut
```

**Methods** (14)

```rust
fn post_visit_expr(&mut self, _expr: &mut Expr) -> ControlFlow<Self::Break>
fn post_visit_query(&mut self, _query: &mut Query) -> ControlFlow<Self::Break>
fn post_visit_relation(&mut self, _relation: &mut ObjectName) -> ControlFlow<Self::Break>
fn post_visit_select(&mut self, _select: &mut Select) -> ControlFlow<Self::Break>
fn post_visit_statement(&mut self, _statement: &mut Statement) -> ControlFlow<Self::Break>
fn post_visit_table_factor(&mut self, _table_factor: &mut TableFactor) -> ControlFlow<Self::Break>
fn post_visit_value(&mut self, _value: &mut ValueWithSpan) -> ControlFlow<Self::Break>
fn pre_visit_expr(&mut self, _expr: &mut Expr) -> ControlFlow<Self::Break>
fn pre_visit_query(&mut self, _query: &mut Query) -> ControlFlow<Self::Break>
fn pre_visit_relation(&mut self, _relation: &mut ObjectName) -> ControlFlow<Self::Break>
fn pre_visit_select(&mut self, _select: &mut Select) -> ControlFlow<Self::Break>
fn pre_visit_statement(&mut self, _statement: &mut Statement) -> ControlFlow<Self::Break>
fn pre_visit_table_factor(&mut self, _table_factor: &mut TableFactor) -> ControlFlow<Self::Break>
fn pre_visit_value(&mut self, _value: &mut ValueWithSpan) -> ControlFlow<Self::Break>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.visitor.VisitorMut.md).


A visitor that can be used to mutate an AST tree.

`pre_visit_` methods are invoked before visiting all children of the
node and `post_visit_` methods are invoked after visiting all
children of the node.

# See also

These methods provide a more concise way of visiting nodes of a certain type:
* [visit_relations_mut]
* [visit_expressions_mut]
* [visit_statements_mut]

# Example
```
# use sqlparser::parser::Parser;
# use sqlparser::dialect::GenericDialect;
# use sqlparser::ast::{VisitMut, VisitorMut, ObjectName, Expr, Ident};
# use core::ops::ControlFlow;

// A visitor that replaces "to_replace" with "replaced" in all expressions
struct Replacer;

// Visit each expression after its children have been visited
impl VisitorMut for Replacer {
  type Break = ();

  fn post_visit_expr(&mut self, expr: &mut Expr) -> ControlFlow<Self::Break> {
    if let Expr::Identifier(Ident{ value, ..}) = expr {
        *value = value.replace("to_replace", "replaced")
    }
    ControlFlow::Continue(())
  }
}

let sql = "SELECT to_replace FROM foo where to_replace IN (SELECT to_replace FROM bar)";
let mut statements = Parser::parse_sql(&GenericDialect{}, sql).unwrap();

// Drive the visitor through the AST
statements.visit(&mut Replacer);

assert_eq!(statements[0].to_string(), "SELECT replaced FROM foo WHERE replaced IN (SELECT replaced FROM bar)");
```

---
