# `sqlparser::dialect::generic`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.generic.json`](../model/sqlparser.dialect.generic.json)

## GenericDialect

`struct` · `sqlparser::dialect::generic::GenericDialect`

Also reachable as `sqlparser::dialect::GenericDialect`

```rust
struct GenericDialect
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
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn support_map_literal_syntax(&self) -> bool
fn supports_array_join_syntax(&self) -> bool
fn supports_array_typedef_with_brackets(&self) -> bool
fn supports_asc_desc_in_column_definition(&self) -> bool
fn supports_bitwise_shift_operators(&self) -> bool
fn supports_comma_separated_set_assignments(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_comment_on(&self) -> bool
fn supports_comment_optimizer_hint(&self) -> bool
fn supports_connect_by(&self) -> bool
fn supports_constraint_keyword_without_name(&self) -> bool
fn supports_create_index_with_clause(&self) -> bool
fn supports_create_view_comment_syntax(&self) -> bool
fn supports_cte_without_as(&self) -> bool
fn supports_data_type_signed_suffix(&self) -> bool
fn supports_detach(&self) -> bool
fn supports_dictionary_syntax(&self) -> bool
fn supports_empty_projections(&self) -> bool
fn supports_explain_with_utility_options(&self) -> bool
fn supports_extract_comma_syntax(&self) -> bool
fn supports_filter_during_aggregation(&self) -> bool
fn supports_from_first_select(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_group_by_with_modifier(&self) -> bool
fn supports_install(&self) -> bool
fn supports_interpolate(&self) -> bool
fn supports_interval_options(&self) -> bool
fn supports_key_column_option(&self) -> bool
fn supports_left_associative_joins_without_parens(&self) -> bool
fn supports_limit_by(&self) -> bool
fn supports_limit_comma(&self) -> bool
fn supports_load_extension(&self) -> bool
fn supports_match_against(&self) -> bool
fn supports_match_recognize(&self) -> bool
fn supports_multiline_comment_hints(&self) -> bool
fn supports_named_fn_args_with_assignment_operator(&self) -> bool
fn supports_nested_comments(&self) -> bool
fn supports_optimize_table(&self) -> bool
fn supports_parens_around_table_factor(&self) -> bool
fn supports_parenthesized_set_variables(&self) -> bool
fn supports_partition_by_after_order_by(&self) -> bool
fn supports_pipe_operator(&self) -> bool
fn supports_prewhere(&self) -> bool
fn supports_projection_trailing_commas(&self) -> bool
fn supports_quote_delimited_string(&self) -> bool
fn supports_select_format(&self) -> bool
fn supports_select_item_multi_column_alias(&self) -> bool
fn supports_select_wildcard_except(&self) -> bool
fn supports_select_wildcard_exclude(&self) -> bool
fn supports_select_wildcard_ilike(&self) -> bool
fn supports_select_wildcard_rename(&self) -> bool
fn supports_select_wildcard_replace(&self) -> bool
fn supports_set_names(&self) -> bool
fn supports_settings(&self) -> bool
fn supports_start_transaction_modifier(&self) -> bool
fn supports_string_escape_constant(&self) -> bool
fn supports_struct_literal(&self) -> bool
fn supports_try_convert(&self) -> bool
fn supports_unicode_string_literal(&self) -> bool
fn supports_update_order_by(&self) -> bool
fn supports_user_host_grantee(&self) -> bool
fn supports_values_as_table_factor(&self) -> bool
fn supports_window_clause_named_window_reference(&self) -> bool
fn supports_window_function_null_treatment_arg(&self) -> bool
fn supports_with_fill(&self) -> bool
fn supports_xml_expressions(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.generic.GenericDialect.md).


A permissive, general purpose [`Dialect`], which parses a wide variety of SQL
statements, from many different dialects.

---
