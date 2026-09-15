# `sqlparser::dialect::snowflake`

Crate `sqlparser` · 2 public items · structured records in [`model/sqlparser.dialect.snowflake.json`](../model/sqlparser.dialect.snowflake.json)

## parse_snowflake_stage_name

`function` · `sqlparser::dialect::snowflake::parse_snowflake_stage_name`

Also reachable as `sqlparser::dialect::parse_snowflake_stage_name`

```rust
fn parse_snowflake_stage_name(parser: &mut parser::Parser<'_>) -> Result<ast::ObjectName, parser::ParserError>
```

Parses a Snowflake stage name, which may start with `@` for internal stages.
Examples: `@mystage`, `@namespace.stage`, `schema.table`

---

## SnowflakeDialect

`struct` · `sqlparser::dialect::snowflake::SnowflakeDialect`

Also reachable as `sqlparser::dialect::SnowflakeDialect`

```rust
struct SnowflakeDialect
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::dialect::Dialect`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::dialect::Dialect`**

```rust
fn allow_extract_custom(&self) -> bool
fn allow_extract_single_quotes(&self) -> bool
fn describe_requires_table_keyword(&self) -> bool
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
fn get_reserved_keywords_for_select_item_operator(&self) -> &[Keyword]
fn is_column_alias(&self, kw: &Keyword, parser: &mut Parser<'_>) -> bool
fn is_identifier_generating_function_name(&self, ident: &Ident, name_parts: &[ObjectNamePart]) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn is_reserved_for_identifier(&self, kw: Keyword) -> bool
fn is_table_alias(&self, kw: &Keyword, parser: &mut Parser<'_>) -> bool
fn is_table_factor(&self, kw: &Keyword, parser: &mut Parser<'_>) -> bool
fn parse_column_option(&self, parser: &mut Parser<'_>) -> Result<Option<Result<Option<ColumnOption>, ParserError>>, ParserError>
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
fn supports_array_typedef_without_element_type(&self) -> bool
fn supports_comma_separated_drop_column_list(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_comment_on(&self) -> bool
fn supports_connect_by(&self) -> bool
fn supports_create_view_comment_syntax(&self) -> bool
fn supports_dictionary_syntax(&self) -> bool
fn supports_execute_immediate(&self) -> bool
fn supports_extract_comma_syntax(&self) -> bool
fn supports_from_trailing_commas(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_lambda_functions(&self) -> bool
fn supports_left_associative_joins_without_parens(&self) -> bool
fn supports_match_recognize(&self) -> bool
fn supports_object_name_double_dot_notation(&self) -> bool
fn supports_outer_join_operator(&self) -> bool
fn supports_parens_around_table_factor(&self) -> bool
fn supports_parenthesized_set_variables(&self) -> bool
fn supports_partiql(&self) -> bool
fn supports_projection_trailing_commas(&self) -> bool
fn supports_select_expr_star(&self) -> bool
fn supports_select_wildcard_exclude(&self) -> bool
fn supports_select_wildcard_ilike(&self) -> bool
fn supports_select_wildcard_rename(&self) -> bool
fn supports_select_wildcard_replace(&self) -> bool
fn supports_semantic_view_table_factor(&self) -> bool
fn supports_show_like_before_in(&self) -> bool
fn supports_space_separated_column_options(&self) -> bool
fn supports_string_literal_backslash_escape(&self) -> bool
fn supports_subquery_as_function_arg(&self) -> bool
fn supports_table_versioning(&self) -> bool
fn supports_values_as_table_factor(&self) -> bool
fn supports_window_function_null_treatment_arg(&self) -> bool
fn supports_within_after_array_aggregation(&self) -> bool
```

A [`Dialect`] for [Snowflake](https://www.snowflake.com/)

---
