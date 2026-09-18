# `sqlparser::dialect`

Crate `sqlparser` · 3 public items · structured records in [`model/sqlparser.dialect.json`](../model/sqlparser.dialect.json)

## Precedence

`enum` · `sqlparser::dialect::Precedence`

```rust
enum Precedence
```

**Variants**: `Period`, `DoubleColon`, `AtTz`, `MulDivModOp`, `PlusMinus`, `Xor`, `Ampersand`, `Caret`, `Pipe`, `Colon`, `Between`, `Eq`, `Like`, `Is`, `PgOther`, `UnaryNot`, `And`, `Or`

**Derives**: Clone, Copy, Debug

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.Precedence.md).


Operators for which precedence must be defined.

Higher number -> higher precedence.
See expression parsing for how these values are used.

---

## dialect_from_str

`function` · `sqlparser::dialect::dialect_from_str`

```rust
fn dialect_from_str(dialect_name: impl AsRef<str>) -> Option<Box<dyn Dialect>>
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.dialect_from_str.md).


Returns the built in [`Dialect`] corresponding to `dialect_name`.

See [`Dialect`] documentation for an example.

---

## Dialect

`trait` · `sqlparser::dialect::Dialect`

```rust
trait Dialect: Debug + Any
```

**Implementors** (16)

- `sqlparser::dialect::ansi::AnsiDialect`
- `sqlparser::dialect::bigquery::BigQueryDialect`
- `sqlparser::dialect::clickhouse::ClickHouseDialect`
- `sqlparser::dialect::databricks::DatabricksDialect`
- `sqlparser::dialect::duckdb::DuckDbDialect`
- `sqlparser::dialect::generic::GenericDialect`
- `sqlparser::dialect::hive::HiveDialect`
- `sqlparser::dialect::mssql::MsSqlDialect`
- `sqlparser::dialect::mysql::MySqlDialect`
- `sqlparser::dialect::oracle::OracleDialect`
- `sqlparser::dialect::postgresql::PostgreSqlDialect`
- `sqlparser::dialect::redshift::RedshiftSqlDialect`
- `sqlparser::dialect::snowflake::SnowflakeDialect`
- `sqlparser::dialect::spark::SparkSqlDialect`
- `sqlparser::dialect::sqlite::SQLiteDialect`
- `sqlparser::dialect::teradata::TeradataDialect`

**Methods** (161)

```rust
fn allow_extract_custom(&self) -> bool
fn allow_extract_single_quotes(&self) -> bool
fn convert_type_before_value(&self) -> bool
fn describe_requires_table_keyword(&self) -> bool
fn dialect(&self) -> TypeId
fn get_next_precedence(&self, _parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
fn get_next_precedence_default(&self, parser: &Parser<'_>) -> Result<u8, ParserError>
fn get_reserved_grantees_types(&self) -> &[GranteesType]
fn get_reserved_keywords_for_select_item_operator(&self) -> &[Keyword]
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
fn ignores_wildcard_escapes(&self) -> bool
fn is_column_alias(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
fn is_custom_operator_part(&self, _ch: char) -> bool
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_generating_function_name(&self, _ident: &Ident, _name_parts: &[ObjectNamePart]) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn is_nested_delimited_identifier_start(&self, _ch: char) -> bool
fn is_reserved_for_identifier(&self, kw: Keyword) -> bool
fn is_select_item_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
fn is_table_alias(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
fn is_table_factor(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
fn parse_column_option(&self, _parser: &mut Parser<'_>) -> Result<Option<Result<Option<ColumnOption>, ParserError>>, ParserError>
fn parse_infix(&self, _parser: &mut Parser<'_>, _expr: &Expr, _precedence: u8) -> Option<Result<Expr, ParserError>>
fn parse_prefix(&self, _parser: &mut Parser<'_>) -> Option<Result<Expr, ParserError>>
fn parse_statement(&self, _parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
fn peek_nested_delimited_identifier_quotes(&self, _chars: Peekable<Chars<'_>>) -> Option<(char, Option<char>)>
fn prec_unknown(&self) -> u8
fn prec_value(&self, prec: Precedence) -> u8
fn require_interval_qualifier(&self) -> bool
fn requires_single_line_comment_whitespace(&self) -> bool
fn support_map_literal_syntax(&self) -> bool
fn supports_alter_column_type_using(&self) -> bool
fn supports_array_join_syntax(&self) -> bool
fn supports_array_typedef_with_brackets(&self) -> bool
fn supports_array_typedef_without_element_type(&self) -> bool
fn supports_asc_desc_in_column_definition(&self) -> bool
fn supports_bang_not_operator(&self) -> bool
fn supports_binary_kw_as_cast(&self) -> bool
fn supports_bitwise_shift_operators(&self) -> bool
fn supports_boolean_literals(&self) -> bool
fn supports_column_definition_trailing_commas(&self) -> bool
fn supports_comma_separated_drop_column_list(&self) -> bool
fn supports_comma_separated_set_assignments(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_comment_on(&self) -> bool
fn supports_comment_optimizer_hint(&self) -> bool
fn supports_connect_by(&self) -> bool
fn supports_constraint_keyword_without_name(&self) -> bool
fn supports_create_index_with_clause(&self) -> bool
fn supports_create_table_like_parenthesized(&self) -> bool
fn supports_create_table_multi_schema_info_sources(&self) -> bool
fn supports_create_table_select(&self) -> bool
fn supports_create_table_using(&self) -> bool
fn supports_create_view_comment_syntax(&self) -> bool
fn supports_cross_join_constraint(&self) -> bool
fn supports_cte_without_as(&self) -> bool
fn supports_data_type_signed_suffix(&self) -> bool
fn supports_detach(&self) -> bool
fn supports_dictionary_syntax(&self) -> bool
fn supports_dollar_as_money_prefix(&self) -> bool
fn supports_dollar_placeholder(&self) -> bool
fn supports_double_ampersand_operator(&self) -> bool
fn supports_empty_projections(&self) -> bool
fn supports_end_transaction_modifier(&self) -> bool
fn supports_eq_alias_assignment(&self) -> bool
fn supports_execute_immediate(&self) -> bool
fn supports_explain_with_utility_options(&self) -> bool
fn supports_extract_comma_syntax(&self) -> bool
fn supports_factorial_operator(&self) -> bool
fn supports_filter_during_aggregation(&self) -> bool
fn supports_from_first_insert(&self) -> bool
fn supports_from_first_select(&self) -> bool
fn supports_from_trailing_commas(&self) -> bool
fn supports_geometric_types(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_group_by_with_modifier(&self) -> bool
fn supports_in_empty_list(&self) -> bool
fn supports_insert_format(&self) -> bool
fn supports_insert_set(&self) -> bool
fn supports_insert_table_alias(&self) -> bool
fn supports_insert_table_function(&self) -> bool
fn supports_insert_table_query(&self) -> bool
fn supports_install(&self) -> bool
fn supports_interpolate(&self) -> bool
fn supports_interval_options(&self) -> bool
fn supports_key_column_option(&self) -> bool
fn supports_lambda_functions(&self) -> bool
fn supports_left_associative_joins_without_parens(&self) -> bool
fn supports_limit_by(&self) -> bool
fn supports_limit_comma(&self) -> bool
fn supports_listen_notify(&self) -> bool
fn supports_load_data(&self) -> bool
fn supports_load_extension(&self) -> bool
fn supports_long_type_as_bigint(&self) -> bool
fn supports_map_literal_with_angle_brackets(&self) -> bool
fn supports_match_against(&self) -> bool
fn supports_match_recognize(&self) -> bool
fn supports_multiline_comment_hints(&self) -> bool
fn supports_named_fn_args_with_assignment_operator(&self) -> bool
fn supports_named_fn_args_with_colon_operator(&self) -> bool
fn supports_named_fn_args_with_eq_operator(&self) -> bool
fn supports_named_fn_args_with_expr_name(&self) -> bool
fn supports_named_fn_args_with_rarrow_operator(&self) -> bool
fn supports_nested_comments(&self) -> bool
fn supports_notnull_operator(&self) -> bool
fn supports_numeric_literal_underscores(&self) -> bool
fn supports_numeric_prefix(&self) -> bool
fn supports_object_name_double_dot_notation(&self) -> bool
fn supports_optimize_table(&self) -> bool
fn supports_order_by_all(&self) -> bool
fn supports_outer_join_operator(&self) -> bool
fn supports_parens_around_table_factor(&self) -> bool
fn supports_parenthesized_set_variables(&self) -> bool
fn supports_partiql(&self) -> bool
fn supports_partition_by_after_order_by(&self) -> bool
fn supports_pipe_operator(&self) -> bool
fn supports_prewhere(&self) -> bool
fn supports_projection_trailing_commas(&self) -> bool
fn supports_quote_delimited_string(&self) -> bool
fn supports_select_exclude(&self) -> bool
fn supports_select_expr_star(&self) -> bool
fn supports_select_format(&self) -> bool
fn supports_select_item_multi_column_alias(&self) -> bool
fn supports_select_modifiers(&self) -> bool
fn supports_select_wildcard_except(&self) -> bool
fn supports_select_wildcard_exclude(&self) -> bool
fn supports_select_wildcard_ilike(&self) -> bool
fn supports_select_wildcard_rename(&self) -> bool
fn supports_select_wildcard_replace(&self) -> bool
fn supports_select_wildcard_with_alias(&self) -> bool
fn supports_semantic_view_table_factor(&self) -> bool
fn supports_set_names(&self) -> bool
fn supports_set_stmt_without_operator(&self) -> bool
fn supports_settings(&self) -> bool
fn supports_show_like_before_in(&self) -> bool
fn supports_space_separated_column_options(&self) -> bool
fn supports_start_transaction_modifier(&self) -> bool
fn supports_string_escape_constant(&self) -> bool
fn supports_string_literal_backslash_escape(&self) -> bool
fn supports_string_literal_concatenation(&self) -> bool
fn supports_string_literal_concatenation_with_newline(&self) -> bool
fn supports_struct_literal(&self) -> bool
fn supports_subquery_as_function_arg(&self) -> bool
fn supports_table_hints(&self) -> bool
fn supports_table_sample_before_alias(&self) -> bool
fn supports_table_versioning(&self) -> bool
fn supports_top_before_distinct(&self) -> bool
fn supports_trailing_commas(&self) -> bool
fn supports_triple_quoted_string(&self) -> bool
fn supports_try_convert(&self) -> bool
fn supports_unicode_string_literal(&self) -> bool
fn supports_update_order_by(&self) -> bool
fn supports_user_host_grantee(&self) -> bool
fn supports_values_as_table_factor(&self) -> bool
fn supports_window_clause_named_window_reference(&self) -> bool
fn supports_window_function_null_treatment_arg(&self) -> bool
fn supports_with_fill(&self) -> bool
fn supports_within_after_array_aggregation(&self) -> bool
fn supports_xml_expressions(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.Dialect.md).


Encapsulates the differences between SQL implementations.

# SQL Dialects

SQL implementations deviate from one another, either due to
custom extensions or various historical reasons. This trait
encapsulates the parsing differences between dialects.

[`GenericDialect`] is the most permissive dialect, and parses the union of
all the other dialects, when there is no ambiguity. However, it does not
currently allow `CREATE TABLE` statements without types specified for all
columns; use [`SQLiteDialect`] if you require that.

# Examples
Most users create a [`Dialect`] directly, as shown on the [module
level documentation]:

```
# use sqlparser::dialect::AnsiDialect;
let dialect = AnsiDialect {};
```

It is also possible to dynamically create a [`Dialect`] from its
name. For example:

```
# use sqlparser::dialect::{AnsiDialect, dialect_from_str};
let dialect = dialect_from_str("ansi").unwrap();

// Parsed dialect is an instance of `AnsiDialect`:
assert!(dialect.is::<AnsiDialect>());
```

[module level documentation]: crate

---
