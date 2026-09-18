# `sqlparser::parser`

Crate `sqlparser` · 6 public items · structured records in [`model/sqlparser.parser.json`](../model/sqlparser.parser.json)

## IsLateral

`enum` · `sqlparser::parser::IsLateral`

```rust
enum IsLateral
```

**Variants**: `Lateral`, `NotLateral`

[Full member, field, variant and typed contracts](../operations/sqlparser.parser.IsLateral.md).


Indicates if a table expression is lateral.

---

## IsOptional

`enum` · `sqlparser::parser::IsOptional`

```rust
enum IsOptional
```

**Variants**: `Optional`, `Mandatory`

**Derives**: Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/sqlparser.parser.IsOptional.md).


Indicates whether a parser element is optional or mandatory.

---

## ParserError

`enum` · `sqlparser::parser::ParserError`

```rust
enum ParserError
```

**Variants**: `TokenizerError`, `ParserError`, `RecursionLimitExceeded`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(e: TokenizerError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/sqlparser.parser.ParserError.md).


Errors produced by the SQL parser.

---

## WildcardExpr

`enum` · `sqlparser::parser::WildcardExpr`

```rust
enum WildcardExpr
```

**Variants**: `Expr`, `QualifiedWildcard`, `Wildcard`

[Full member, field, variant and typed contracts](../operations/sqlparser.parser.WildcardExpr.md).


Represents a wildcard expression used in SELECT lists.

---

## Parser

`struct` · `sqlparser::parser::Parser`

```rust
struct Parser<'a>
```

**Methods** (340)

```rust
fn advance_token(&mut self)
fn consume_token(&mut self, expected: &Token) -> bool
fn consume_tokens(&mut self, tokens: &[Token]) -> bool
fn expect_keyword(&mut self, expected: Keyword) -> Result<TokenWithSpan, ParserError>
fn expect_keyword_is(&mut self, expected: Keyword) -> Result<(), ParserError>
fn expect_keywords(&mut self, expected: &[Keyword]) -> Result<(), ParserError>
fn expect_one_of_keywords(&mut self, keywords: &[Keyword]) -> Result<Keyword, ParserError>
fn expect_token(&mut self, expected: &Token) -> Result<TokenWithSpan, ParserError>
fn expected<T>(&self, expected: &str, found: TokenWithSpan) -> Result<T, ParserError>
fn expected_at<T>(&self, expected: &str, index: usize) -> Result<T, ParserError>
fn expected_ref<T>(&self, expected: &str, found: &TokenWithSpan) -> Result<T, ParserError>
fn get_current_index(&self) -> usize
fn get_current_token(&self) -> &TokenWithSpan
fn get_next_precedence(&self) -> Result<u8, ParserError>
fn get_next_token(&self) -> &TokenWithSpan
fn get_previous_token(&self) -> &TokenWithSpan
fn index(&self) -> usize
fn into_tokens(self) -> Vec<TokenWithSpan>
fn maybe_parse<T, F>(&mut self, f: F) -> Result<Option<T>, ParserError> where F: FnMut(&mut Parser<'_>) -> Result<T, ParserError>
fn maybe_parse_connect_by(&mut self) -> Result<Vec<ConnectByKind>, ParserError>
fn maybe_parse_options(&mut self, keyword: Keyword) -> Result<Option<Vec<SqlOption>>, ParserError>
fn maybe_parse_table_alias(&mut self) -> Result<Option<TableAlias>, ParserError>
fn maybe_parse_table_version(&mut self) -> Result<Option<TableVersion>, ParserError>
fn new(dialect: &'a dyn Dialect) -> Self
fn next_token(&mut self) -> TokenWithSpan
fn next_token_is_temporal_unit(&mut self) -> bool
fn next_token_no_skip(&mut self) -> Option<&TokenWithSpan>
fn parse_actions_list(&mut self) -> Result<Vec<Action>, ParserError>
fn parse_all_or_distinct(&mut self) -> Result<Option<Distinct>, ParserError>
fn parse_alter(&mut self) -> Result<Statement, ParserError>
fn parse_alter_collation(&mut self) -> Result<AlterCollation, ParserError>
fn parse_alter_connector(&mut self) -> Result<Statement, ParserError>
fn parse_alter_function(&mut self, kind: AlterFunctionKind) -> Result<Statement, ParserError>
fn parse_alter_operator(&mut self) -> Result<AlterOperator, ParserError>
fn parse_alter_operator_class(&mut self) -> Result<AlterOperatorClass, ParserError>
fn parse_alter_operator_family(&mut self) -> Result<AlterOperatorFamily, ParserError>
fn parse_alter_policy(&mut self) -> Result<AlterPolicy, ParserError>
fn parse_alter_role(&mut self) -> Result<Statement, ParserError>
fn parse_alter_schema(&mut self) -> Result<Statement, ParserError>
fn parse_alter_table(&mut self, iceberg: bool) -> Result<Statement, ParserError>
fn parse_alter_table_add_projection(&mut self) -> Result<AlterTableOperation, ParserError>
fn parse_alter_table_operation(&mut self) -> Result<AlterTableOperation, ParserError>
fn parse_alter_type(&mut self) -> Result<Statement, ParserError>
fn parse_alter_user(&mut self) -> Result<AlterUser, ParserError>
fn parse_alter_view(&mut self) -> Result<Statement, ParserError>
fn parse_analyze(&mut self) -> Result<Analyze, ParserError>
fn parse_analyze_format(&mut self) -> Result<AnalyzeFormat, ParserError>
fn parse_array_expr(&mut self, named: bool) -> Result<Expr, ParserError>
fn parse_as_query(&mut self) -> Result<(bool, Box<Query>), ParserError>
fn parse_as_table(&mut self) -> Result<Table, ParserError>
fn parse_asc_desc(&mut self) -> Option<bool>
fn parse_assert(&mut self) -> Result<Statement, ParserError>
fn parse_assignment(&mut self) -> Result<Assignment, ParserError>
fn parse_assignment_target(&mut self) -> Result<AssignmentTarget, ParserError>
fn parse_attach_database(&mut self) -> Result<Statement, ParserError>
fn parse_attach_duckdb_database(&mut self) -> Result<Statement, ParserError>
fn parse_attach_duckdb_database_options(&mut self) -> Result<Vec<AttachDuckDBDatabaseOption>, ParserError>
fn parse_begin(&mut self) -> Result<Statement, ParserError>
fn parse_begin_exception_end(&mut self) -> Result<Statement, ParserError>
fn parse_between(&mut self, expr: Expr, negated: bool) -> Result<Expr, ParserError>
fn parse_big_query_declare(&mut self) -> Result<Statement, ParserError>
fn parse_binary_length(&mut self) -> Result<BinaryLength, ParserError>
fn parse_cache_table(&mut self) -> Result<Statement, ParserError>
fn parse_call(&mut self) -> Result<Statement, ParserError>
fn parse_case_expr(&mut self) -> Result<Expr, ParserError>
fn parse_case_stmt(&mut self) -> Result<CaseStatement, ParserError>
fn parse_cast_expr(&mut self, kind: CastKind) -> Result<Expr, ParserError>
fn parse_ceil_floor_expr(&mut self, is_ceil: bool) -> Result<Expr, ParserError>
fn parse_character_length(&mut self) -> Result<CharacterLength, ParserError>
fn parse_close(&mut self) -> Result<Statement, ParserError>
fn parse_column_def(&mut self) -> Result<ColumnDef, ParserError>
fn parse_columns(&mut self) -> Result<(Vec<ColumnDef>, Vec<TableConstraint>), ParserError>
fn parse_comma_separated<T, F>(&mut self, f: F) -> Result<Vec<T>, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
fn parse_comma_separated0<T, F>(&mut self, f: F, end_token: Token) -> Result<Vec<T>, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
fn parse_comment(&mut self) -> Result<Statement, ParserError>
fn parse_comment_value(&mut self) -> Result<String, ParserError>
fn parse_commit(&mut self) -> Result<Statement, ParserError>
fn parse_commit_rollback_chain(&mut self) -> Result<bool, ParserError>
fn parse_compound_expr(&mut self, root: Expr, chain: Vec<AccessExpr>) -> Result<Expr, ParserError>
fn parse_constraint_characteristics(&mut self) -> Result<Option<ConstraintCharacteristics>, ParserError>
fn parse_convert_expr(&mut self, is_try: bool) -> Result<Expr, ParserError>
fn parse_copy(&mut self) -> Result<Statement, ParserError>
fn parse_create(&mut self) -> Result<Statement, ParserError>
fn parse_create_collation(&mut self) -> Result<CreateCollation, ParserError>
fn parse_create_connector(&mut self) -> Result<CreateConnector, ParserError>
fn parse_create_database(&mut self) -> Result<Statement, ParserError>
fn parse_create_extension(&mut self) -> Result<CreateExtension, ParserError>
fn parse_create_external_table(&mut self, or_replace: bool) -> Result<CreateTable, ParserError>
fn parse_create_function(&mut self, or_alter: bool, or_replace: bool, temporary: bool) -> Result<Statement, ParserError>
fn parse_create_index(&mut self, unique: bool) -> Result<CreateIndex, ParserError>
fn parse_create_index_expr(&mut self) -> Result<IndexColumn, ParserError>
fn parse_create_macro(&mut self, or_replace: bool, temporary: bool) -> Result<Statement, ParserError>
fn parse_create_operator(&mut self) -> Result<CreateOperator, ParserError>
fn parse_create_operator_class(&mut self) -> Result<CreateOperatorClass, ParserError>
fn parse_create_operator_family(&mut self) -> Result<CreateOperatorFamily, ParserError>
fn parse_create_policy(&mut self) -> Result<CreatePolicy, ParserError>
fn parse_create_procedure(&mut self, or_alter: bool) -> Result<Statement, ParserError>
fn parse_create_role(&mut self) -> Result<CreateRole, ParserError>
fn parse_create_schema(&mut self) -> Result<Statement, ParserError>
fn parse_create_secret(&mut self, or_replace: bool, temporary: bool, persistent: bool) -> Result<Statement, ParserError>
fn parse_create_sequence(&mut self, temporary: bool) -> Result<Statement, ParserError>
fn parse_create_snapshot_table(&mut self) -> Result<CreateTable, ParserError>
fn parse_create_table(&mut self, or_replace: bool, temporary: bool, global: Option<bool>, transient: bool) -> Result<CreateTable, ParserError>
fn parse_create_trigger(&mut self, temporary: bool, or_alter: bool, or_replace: bool, is_constraint: bool) -> Result<CreateTrigger, ParserError>
fn parse_create_type(&mut self) -> Result<Statement, ParserError>
fn parse_create_type_enum(&mut self, name: ObjectName) -> Result<Statement, ParserError>
fn parse_create_view(&mut self, or_alter: bool, or_replace: bool, temporary: bool, create_view_params: Option<CreateViewParams>) -> Result<CreateView, ParserError>
fn parse_create_virtual_table(&mut self) -> Result<Statement, ParserError>
fn parse_cte(&mut self) -> Result<Cte, ParserError>
fn parse_data_type(&mut self) -> Result<DataType, ParserError>
fn parse_date_time_field(&mut self) -> Result<DateTimeField, ParserError>
fn parse_datetime_64(&mut self) -> Result<(u64, Option<String>), ParserError>
fn parse_deallocate(&mut self) -> Result<Statement, ParserError>
fn parse_declare(&mut self) -> Result<Statement, ParserError>
fn parse_delete(&mut self, delete_token: TokenWithSpan) -> Result<Statement, ParserError>
fn parse_deny(&mut self) -> Result<Statement, ParserError>
fn parse_derived_table_factor(&mut self, lateral: IsLateral) -> Result<TableFactor, ParserError>
fn parse_detach_duckdb_database(&mut self) -> Result<Statement, ParserError>
fn parse_discard(&mut self) -> Result<Statement, ParserError>
fn parse_drop(&mut self) -> Result<Statement, ParserError>
fn parse_drop_extension(&mut self) -> Result<Statement, ParserError>
fn parse_drop_operator(&mut self) -> Result<Statement, ParserError>
fn parse_drop_operator_class(&mut self) -> Result<Statement, ParserError>
fn parse_drop_operator_family(&mut self) -> Result<Statement, ParserError>
fn parse_drop_trigger(&mut self) -> Result<DropTrigger, ParserError>
fn parse_end(&mut self) -> Result<Statement, ParserError>
fn parse_enum_values(&mut self) -> Result<Vec<EnumMember>, ParserError>
fn parse_escape_char(&mut self) -> Result<Option<ValueWithSpan>, ParserError>
fn parse_exact_number_optional_precision_scale(&mut self) -> Result<ExactNumberInfo, ParserError>
fn parse_execute(&mut self) -> Result<Statement, ParserError>
fn parse_exists_expr(&mut self, negated: bool) -> Result<Expr, ParserError>
fn parse_explain(&mut self, describe_alias: DescribeAlias) -> Result<Statement, ParserError>
fn parse_expr(&mut self) -> Result<Expr, ParserError>
fn parse_expr_with_alias(&mut self) -> Result<ExprWithAlias, ParserError>
fn parse_expr_with_alias_and_order_by(&mut self) -> Result<ExprWithAliasAndOrderBy, ParserError>
fn parse_extract_expr(&mut self) -> Result<Expr, ParserError>
fn parse_fetch(&mut self) -> Result<Fetch, ParserError>
fn parse_fetch_statement(&mut self) -> Result<Statement, ParserError>
fn parse_file_format(&mut self) -> Result<FileFormat, ParserError>
fn parse_flush(&mut self) -> Result<Statement, ParserError>
fn parse_for_clause(&mut self) -> Result<Option<ForClause>, ParserError>
fn parse_for_json(&mut self) -> Result<ForClause, ParserError>
fn parse_for_xml(&mut self) -> Result<ForClause, ParserError>
fn parse_function(&mut self, name: ObjectName) -> Result<Expr, ParserError>
fn parse_function_args(&mut self) -> Result<FunctionArg, ParserError>
fn parse_grant(&mut self) -> Result<Grant, ParserError>
fn parse_grant_deny_revoke_privileges_objects(&mut self) -> Result<(Privileges, Option<GrantObjects>), ParserError>
fn parse_grant_permission(&mut self) -> Result<Action, ParserError>
fn parse_grantee_name(&mut self) -> Result<GranteeName, ParserError>
fn parse_hive_distribution(&mut self) -> Result<HiveDistributionStyle, ParserError>
fn parse_hive_formats(&mut self) -> Result<Option<HiveFormat>, ParserError>
fn parse_identifier(&mut self) -> Result<Ident, ParserError>
fn parse_identifier_with_alias(&mut self) -> Result<IdentWithAlias, ParserError>
fn parse_identifiers(&mut self) -> Result<Vec<Ident>, ParserError>
fn parse_if_stmt(&mut self) -> Result<IfStatement, ParserError>
fn parse_in(&mut self, expr: Expr, negated: bool) -> Result<Expr, ParserError>
fn parse_index_options(&mut self) -> Result<Vec<IndexOption>, ParserError>
fn parse_index_type(&mut self) -> Result<IndexType, ParserError>
fn parse_index_type_display(&mut self) -> KeyOrIndexDisplay
fn parse_infix(&mut self, expr: Expr, precedence: u8) -> Result<Expr, ParserError>
fn parse_input_format_clause(&mut self) -> Result<InputFormatClause, ParserError>
fn parse_insert(&mut self, insert_token: TokenWithSpan) -> Result<Statement, ParserError>
fn parse_insert_partition(&mut self) -> Result<Option<Vec<Expr>>, ParserError>
fn parse_install(&mut self) -> Result<Statement, ParserError>
fn parse_interpolation(&mut self) -> Result<InterpolateExpr, ParserError>
fn parse_interpolations(&mut self) -> Result<Option<Interpolate>, ParserError>
fn parse_interval(&mut self) -> Result<Expr, ParserError>
fn parse_join_constraint(&mut self, natural: bool) -> Result<JoinConstraint, ParserError>
fn parse_json_table_column_def(&mut self) -> Result<JsonTableColumn, ParserError>
fn parse_keyword(&mut self, expected: Keyword) -> bool
fn parse_keyword_separated<T, F>(&mut self, keyword: Keyword, f: F) -> Result<Vec<T>, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
fn parse_keyword_with_tokens(&mut self, expected: Keyword, tokens: &[Token]) -> bool
fn parse_keywords(&mut self, keywords: &[Keyword]) -> bool
fn parse_kill(&mut self) -> Result<Statement, ParserError>
fn parse_limit(&mut self) -> Result<Option<Expr>, ParserError>
fn parse_listagg_on_overflow(&mut self) -> Result<Option<ListAggOnOverflow>, ParserError>
fn parse_listen(&mut self) -> Result<Statement, ParserError>
fn parse_literal_string(&mut self) -> Result<String, ParserError>
fn parse_literal_uint(&mut self) -> Result<u64, ParserError>
fn parse_load(&mut self) -> Result<Statement, ParserError>
fn parse_load_data_table_format(&mut self) -> Result<Option<HiveLoadDataFormat>, ParserError>
fn parse_lock(&mut self) -> Result<LockClause, ParserError>
fn parse_lock_statement(&mut self) -> Result<Lock, ParserError>
fn parse_match_against(&mut self) -> Result<Expr, ParserError>
fn parse_match_kind(&mut self) -> Result<ConstraintReferenceMatchKind, ParserError>
fn parse_merge(&mut self, merge_token: TokenWithSpan) -> Result<Merge, ParserError>
fn parse_msck(&mut self) -> Result<Msck, ParserError>
fn parse_mssql_declare(&mut self) -> Result<Statement, ParserError>
fn parse_mssql_declare_stmt(&mut self) -> Result<Declare, ParserError>
fn parse_mssql_variable_declaration_expression(&mut self) -> Result<Option<DeclareAssignment>, ParserError>
fn parse_multi_dim_subscript(&mut self, chain: &mut Vec<AccessExpr>) -> Result<(), ParserError>
fn parse_multipart_identifier(&mut self) -> Result<Vec<Ident>, ParserError>
fn parse_named_window(&mut self) -> Result<NamedWindowDefinition, ParserError>
fn parse_not(&mut self) -> Result<Expr, ParserError>
fn parse_notify(&mut self) -> Result<Statement, ParserError>
fn parse_number(&mut self) -> Result<Expr, ParserError>
fn parse_number_value(&mut self) -> Result<ValueWithSpan, ParserError>
fn parse_object_name(&mut self, in_table_clause: bool) -> Result<ObjectName, ParserError>
fn parse_offset(&mut self) -> Result<Offset, ParserError>
fn parse_one_of_keywords(&mut self, keywords: &[Keyword]) -> Option<Keyword>
fn parse_openjson_table_column_def(&mut self) -> Result<OpenJsonTableColumn, ParserError>
fn parse_optimize_table(&mut self) -> Result<Statement, ParserError>
fn parse_option_clustered(&mut self) -> Result<SqlOption, ParserError>
fn parse_option_partition(&mut self) -> Result<SqlOption, ParserError>
fn parse_optional_alias(&mut self, reserved_kwds: &[Keyword]) -> Result<Option<Ident>, ParserError>
fn parse_optional_args(&mut self) -> Result<Vec<FunctionArg>, ParserError>
fn parse_optional_binary_length(&mut self) -> Result<Option<BinaryLength>, ParserError>
fn parse_optional_cast_format(&mut self) -> Result<Option<CastFormat>, ParserError>
fn parse_optional_character_length(&mut self) -> Result<Option<CharacterLength>, ParserError>
fn parse_optional_clustered_by(&mut self) -> Result<Option<ClusteredBy>, ParserError>
fn parse_optional_column_option(&mut self) -> Result<Option<ColumnOption>, ParserError>
fn parse_optional_create_function_using(&mut self) -> Result<Option<CreateFunctionUsing>, ParserError>
fn parse_optional_group_by(&mut self) -> Result<Option<GroupByExpr>, ParserError>
fn parse_optional_ident(&mut self) -> Result<Option<Ident>, ParserError>
fn parse_optional_index_option(&mut self) -> Result<Option<IndexOption>, ParserError>
fn parse_optional_inline_comment(&mut self) -> Result<Option<CommentDef>, ParserError>
fn parse_optional_order_by(&mut self) -> Result<Option<OrderBy>, ParserError>
fn parse_optional_precision(&mut self) -> Result<Option<u64>, ParserError>
fn parse_optional_precision_scale(&mut self) -> Result<(Option<u64>, Option<u64>), ParserError>
fn parse_optional_procedure_parameters(&mut self) -> Result<Option<Vec<ProcedureParam>>, ParserError>
fn parse_optional_select_item_except(&mut self) -> Result<Option<ExceptSelectItem>, ParserError>
fn parse_optional_select_item_exclude(&mut self) -> Result<Option<ExcludeSelectItem>, ParserError>
fn parse_optional_select_item_ilike(&mut self) -> Result<Option<IlikeSelectItem>, ParserError>
fn parse_optional_select_item_rename(&mut self) -> Result<Option<RenameSelectItem>, ParserError>
fn parse_optional_select_item_replace(&mut self) -> Result<Option<ReplaceSelectItem>, ParserError>
fn parse_optional_table_constraint(&mut self) -> Result<Option<TableConstraint>, ParserError>
fn parse_optional_time_zone(&mut self) -> Result<Option<ValueWithSpan>, ParserError>
fn parse_optional_type_modifiers(&mut self) -> Result<Option<Vec<String>>, ParserError>
fn parse_optional_using_then_index_type(&mut self) -> Result<Option<IndexType>, ParserError>
fn parse_options(&mut self, keyword: Keyword) -> Result<Vec<SqlOption>, ParserError>
fn parse_options_with_keywords(&mut self, keywords: &[Keyword]) -> Result<Vec<SqlOption>, ParserError>
fn parse_order_by_expr(&mut self) -> Result<OrderByExpr, ParserError>
fn parse_overlay_expr(&mut self) -> Result<Expr, ParserError>
fn parse_owner(&mut self) -> Result<Owner, ParserError>
fn parse_parenthesized<T, F>(&mut self, f: F) -> Result<T, ParserError> where F: FnMut(&mut Parser<'a>) -> Result<T, ParserError>
fn parse_parenthesized_column_list(&mut self, optional: IsOptional, allow_empty: bool) -> Result<Vec<Ident>, ParserError>
fn parse_parenthesized_compound_identifier_list(&mut self, optional: IsOptional, allow_empty: bool) -> Result<Vec<Expr>, ParserError>
fn parse_parenthesized_qualified_column_list(&mut self, optional: IsOptional, allow_empty: bool) -> Result<Vec<ObjectName>, ParserError>
fn parse_partition(&mut self) -> Result<Partition, ParserError>
fn parse_pg_cast(&mut self, expr: Expr) -> Result<Expr, ParserError>
fn parse_pg_create_server(&mut self) -> Result<Statement, ParserError>
fn parse_pivot_table_factor(&mut self, table: TableFactor) -> Result<TableFactor, ParserError>
fn parse_plain_options(&mut self) -> Result<Vec<SqlOption>, ParserError>
fn parse_position_expr(&mut self, ident: Ident) -> Result<Expr, ParserError>
fn parse_pragma(&mut self) -> Result<Statement, ParserError>
fn parse_precision(&mut self) -> Result<u64, ParserError>
fn parse_prefix(&mut self) -> Result<Expr, ParserError>
fn parse_prepare(&mut self) -> Result<Statement, ParserError>
fn parse_procedure_param(&mut self) -> Result<ProcedureParam, ParserError>
fn parse_projection(&mut self) -> Result<Vec<SelectItem>, ParserError>
fn parse_projection_select(&mut self) -> Result<ProjectionSelect, ParserError>
fn parse_query(&mut self) -> Result<Box<Query>, ParserError>
fn parse_query_body(&mut self, precedence: u8) -> Result<Box<SetExpr>, ParserError>
fn parse_raise_stmt(&mut self) -> Result<RaiseStatement, ParserError>
fn parse_raiserror(&mut self) -> Result<Statement, ParserError>
fn parse_raiserror_option(&mut self) -> Result<RaisErrorOption, ParserError>
fn parse_referential_action(&mut self) -> Result<ReferentialAction, ParserError>
fn parse_release(&mut self) -> Result<Statement, ParserError>
fn parse_rename(&mut self) -> Result<Statement, ParserError>
fn parse_replace(&mut self, replace_token: TokenWithSpan) -> Result<Statement, ParserError>
fn parse_replace_elements(&mut self) -> Result<ReplaceSelectElement, ParserError>
fn parse_revoke(&mut self) -> Result<Revoke, ParserError>
fn parse_rollback(&mut self) -> Result<Statement, ParserError>
fn parse_rollback_savepoint(&mut self) -> Result<Option<Ident>, ParserError>
fn parse_row_format(&mut self) -> Result<HiveRowFormat, ParserError>
fn parse_savepoint(&mut self) -> Result<Statement, ParserError>
fn parse_select(&mut self) -> Result<Select, ParserError>
fn parse_select_item(&mut self) -> Result<SelectItem, ParserError>
fn parse_set_operator(&mut self, token: &Token) -> Option<SetOperator>
fn parse_set_quantifier(&mut self, op: &Option<SetOperator>) -> SetQuantifier
fn parse_set_session_params(&mut self) -> Result<Statement, ParserError>
fn parse_show(&mut self) -> Result<Statement, ParserError>
fn parse_show_collation(&mut self) -> Result<Statement, ParserError>
fn parse_show_columns(&mut self, extended: bool, full: bool) -> Result<Statement, ParserError>
fn parse_show_create(&mut self) -> Result<Statement, ParserError>
fn parse_show_functions(&mut self) -> Result<Statement, ParserError>
fn parse_show_statement_filter(&mut self) -> Result<Option<ShowStatementFilter>, ParserError>
fn parse_snowflake_declare(&mut self) -> Result<Statement, ParserError>
fn parse_snowflake_variable_declaration_expression(&mut self) -> Result<Option<DeclareAssignment>, ParserError>
fn parse_sql(dialect: &dyn Dialect, sql: &str) -> Result<Vec<Statement>, ParserError>
fn parse_sql_option(&mut self) -> Result<SqlOption, ParserError>
fn parse_sql_with_comments(dialect: &'a dyn Dialect, sql: &str) -> Result<(Vec<Statement>, comments::Comments), ParserError>
fn parse_start_transaction(&mut self) -> Result<Statement, ParserError>
fn parse_statement(&mut self) -> Result<Statement, ParserError>
fn parse_statements(&mut self) -> Result<Vec<Statement>, ParserError>
fn parse_string_values(&mut self) -> Result<Vec<String>, ParserError>
fn parse_subexpr(&mut self, precedence: u8) -> Result<Expr, ParserError>
fn parse_substring(&mut self) -> Result<Expr, ParserError>
fn parse_tab_value(&mut self) -> Vec<Option<String>>
fn parse_table_and_joins(&mut self) -> Result<TableWithJoins, ParserError>
fn parse_table_factor(&mut self) -> Result<TableFactor, ParserError>
fn parse_table_object(&mut self) -> Result<TableObject, ParserError>
fn parse_throw(&mut self) -> Result<ThrowStatement, ParserError>
fn parse_time_functions(&mut self, name: ObjectName) -> Result<Expr, ParserError>
fn parse_top(&mut self) -> Result<Top, ParserError>
fn parse_transaction_modes(&mut self) -> Result<Vec<TransactionMode>, ParserError>
fn parse_trigger_event(&mut self) -> Result<TriggerEvent, ParserError>
fn parse_trigger_exec_body(&mut self) -> Result<TriggerExecBody, ParserError>
fn parse_trigger_period(&mut self) -> Result<TriggerPeriod, ParserError>
fn parse_trigger_referencing(&mut self) -> Result<Option<TriggerReferencing>, ParserError>
fn parse_trim_expr(&mut self) -> Result<Expr, ParserError>
fn parse_trim_where(&mut self) -> Result<TrimWhereField, ParserError>
fn parse_truncate(&mut self) -> Result<Truncate, ParserError>
fn parse_tsv(&mut self) -> Vec<Option<String>>
fn parse_uncache_table(&mut self) -> Result<Statement, ParserError>
fn parse_unicode_is_normalized(&mut self, expr: Expr) -> Result<Expr, ParserError>
fn parse_unlisten(&mut self) -> Result<Statement, ParserError>
fn parse_unload(&mut self) -> Result<Statement, ParserError>
fn parse_unpivot_table_factor(&mut self, table: TableFactor) -> Result<TableFactor, ParserError>
fn parse_update(&mut self, update_token: TokenWithSpan) -> Result<Statement, ParserError>
fn parse_use(&mut self) -> Result<Statement, ParserError>
fn parse_utility_options(&mut self) -> Result<Vec<UtilityOption>, ParserError>
fn parse_value(&mut self) -> Result<ValueWithSpan, ParserError>
fn parse_values(&mut self, allow_empty: bool, value_keyword: bool) -> Result<Values, ParserError>
fn parse_wildcard_additional_options(&mut self, wildcard_token: TokenWithSpan) -> Result<WildcardAdditionalOptions, ParserError>
fn parse_wildcard_expr(&mut self) -> Result<Expr, ParserError>
fn parse_window_frame(&mut self) -> Result<WindowFrame, ParserError>
fn parse_window_frame_bound(&mut self) -> Result<WindowFrameBound, ParserError>
fn parse_window_frame_units(&mut self) -> Result<WindowFrameUnits, ParserError>
fn parse_window_spec(&mut self) -> Result<WindowSpec, ParserError>
fn parse_with_fill(&mut self) -> Result<WithFill, ParserError>
fn peek_keyword(&self, expected: Keyword) -> bool
fn peek_nth_token(&self, n: usize) -> TokenWithSpan
fn peek_nth_token_no_skip(&self, n: usize) -> TokenWithSpan
fn peek_nth_token_ref(&self, n: usize) -> &TokenWithSpan
fn peek_one_of_keywords(&self, keywords: &[Keyword]) -> Option<Keyword>
fn peek_token(&self) -> TokenWithSpan
fn peek_token_no_skip(&self) -> TokenWithSpan
fn peek_token_ref(&self) -> &TokenWithSpan
fn peek_tokens<const N: usize>(&self) -> [Token; N]
fn peek_tokens_ref<const N: usize>(&self) -> [&TokenWithSpan; N]
fn peek_tokens_with_location<const N: usize>(&self) -> [TokenWithSpan; N]
fn prev_token(&mut self)
fn token_at(&self, index: usize) -> &TokenWithSpan
fn try_parse<T, F>(&mut self, f: F) -> Result<T, ParserError> where F: FnMut(&mut Parser<'_>) -> Result<T, ParserError>
fn try_with_sql(self, sql: &str) -> Result<Self, ParserError>
fn with_options(self, options: ParserOptions) -> Self
fn with_recursion_limit(self, recursion_limit: usize) -> Self
fn with_tokens(self, tokens: Vec<Token>) -> Self
fn with_tokens_with_locations(self, tokens: Vec<TokenWithSpan>) -> Self
```

[Full member, field, variant and typed contracts](../operations/sqlparser.parser.Parser.md).


A SQL Parser

This struct is the main entry point for parsing SQL queries.

# Functionality:
* Parsing SQL: see examples on [`Parser::new`] and [`Parser::parse_sql`]
* Controlling recursion: See [`Parser::with_recursion_limit`]
* Controlling parser options: See [`Parser::with_options`]
* Providing your own tokens: See [`Parser::with_tokens`]

# Internals

The parser uses a [`Tokenizer`] to tokenize the input SQL string into a
`Vec` of [`TokenWithSpan`]s and maintains an `index` to the current token
being processed. The token vec may contain multiple SQL statements.

* The "current" token is the token at `index - 1`
* The "next" token is the token at `index`
* The "previous" token is the token at `index - 2`

If `index` is equal to the length of the token stream, the 'next' token is
[`Token::EOF`].

For example, the SQL string "SELECT * FROM foo" will be tokenized into
following tokens:
```text
 [
   "SELECT", // token index 0
   " ",      // whitespace
   "*",
   " ",
   "FROM",
   " ",
   "foo"
  ]
```

---

## ParserOptions

`struct` · `sqlparser::parser::ParserOptions`

```rust
struct ParserOptions
```

**Fields**: `trailing_commas`, `unescape`, `require_semicolon_stmt_delimiter`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new() -> Self
fn with_trailing_commas(self, trailing_commas: bool) -> Self
fn with_unescape(self, unescape: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/sqlparser.parser.ParserOptions.md).


Options that control how the [`Parser`] parses SQL text

---
