# `datafusion_sql::unparser::dialect::Dialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.Dialect.json).

<a id="op-141206b8b6a3273ae8b53463"></a>
## Dialect

`trait` · `datafusion_sql::unparser::dialect::Dialect` · datafusion-sql 55.1.0

```rust
trait Dialect: Send + Sync
```

Source: `src/unparser/dialect.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

`Dialect` to use for Unparsing

The default dialect tries to avoid quoting identifiers unless necessary (e.g. `a` instead of `"a"`)
but this behavior can be overridden as needed

**Note**: This trait will eventually be replaced by the Dialect in the SQLparser package

See <https://github.com/sqlparser-rs/sqlparser-rs/pull/1170>
See also the discussion in <https://github.com/apache/datafusion/pull/10625>

<a id="op-fb589120ca28768524e9a48a"></a>
## character_length_style

`function` · `datafusion_sql::unparser::dialect::Dialect::character_length_style` · datafusion-sql 55.1.0

```rust
fn character_length_style(&self) -> CharacterLengthStyle
```

Source: `src/unparser/dialect.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The character length extraction style to use: `CharacterLengthStyle`

<a id="op-466474dcaef5269863e4b8f8"></a>
## col_alias_overrides

`function` · `datafusion_sql::unparser::dialect::Dialect::col_alias_overrides` · datafusion-sql 55.1.0

```rust
fn col_alias_overrides(&self, _alias: &str) -> Result<Option<String>>
```

Source: `src/unparser/dialect.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allows the dialect to override column alias unparsing if the dialect has specific rules.
Returns None if the default unparsing should be used, or Some(String) if there is
a custom implementation for the alias.

<a id="op-2cac5bdbc403b4e7fd5bb8ab"></a>
## date32_cast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::date32_cast_dtype` · datafusion-sql 55.1.0

```rust
fn date32_cast_dtype(&self) -> ast::DataType
```

Source: `src/unparser/dialect.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The SQL type to use for Arrow Date32 unparsing
Most dialects use Date, but some, like SQLite require TEXT

<a id="op-2e2bad7864dbc025f1846c13"></a>
## date_field_extract_style

`function` · `datafusion_sql::unparser::dialect::Dialect::date_field_extract_style` · datafusion-sql 55.1.0

```rust
fn date_field_extract_style(&self) -> DateFieldExtractStyle
```

Source: `src/unparser/dialect.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The date field extract style to use: `DateFieldExtractStyle`

<a id="op-0b5331b7c4a7dbe27e7f06a4"></a>
## distinct_from_style

`function` · `datafusion_sql::unparser::dialect::Dialect::distinct_from_style` · datafusion-sql 55.1.0

```rust
fn distinct_from_style(&self) -> DistinctFromStyle
```

Source: `src/unparser/dialect.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The style to use when unparsing DISTINCT FROM style expressions

<a id="op-ea77940681db18b3e3b90b46"></a>
## division_operator

`function` · `datafusion_sql::unparser::dialect::Dialect::division_operator` · datafusion-sql 55.1.0

```rust
fn division_operator(&self) -> BinaryOperator
```

Source: `src/unparser/dialect.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The division operator for the dialect
Most dialect uses ` BinaryOperator::Divide` (/)
But DuckDB dialect uses `BinaryOperator::DuckIntegerDivide` (//)

<a id="op-d4330675d6d2e15f283a07a1"></a>
## float64_ast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::float64_ast_dtype` · datafusion-sql 55.1.0

```rust
fn float64_ast_dtype(&self) -> ast::DataType
```

Source: `src/unparser/dialect.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Does the dialect use DOUBLE PRECISION to represent Float64 rather than DOUBLE?
E.g. Postgres uses DOUBLE PRECISION instead of DOUBLE

<a id="op-80954beaf62e9a9f679de5c4"></a>
## full_qualified_col

`function` · `datafusion_sql::unparser::dialect::Dialect::full_qualified_col` · datafusion-sql 55.1.0

```rust
fn full_qualified_col(&self) -> bool
```

Source: `src/unparser/dialect.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allow to unparse a qualified column with a full qualified name
(e.g. catalog_name.schema_name.table_name.column_name)
Otherwise, the column will be unparsed with only the table name and column name
(e.g. table_name.column_name)

<a id="op-572c2ef177bcf5eeac48641e"></a>
## higher_order_function_to_sql_overrides

`function` · `datafusion_sql::unparser::dialect::Dialect::higher_order_function_to_sql_overrides` · datafusion-sql 55.1.0

```rust
fn higher_order_function_to_sql_overrides(&self, _unparser: &Unparser<'_>, _func_name: &str, _args: &[Expr]) -> Result<Option<ast::Expr>>
```

Source: `src/unparser/dialect.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allows the dialect to override higher order function unparsing if the dialect has specific rules.
Returns None if the default unparsing should be used, or Some(ast::Expr) if there is
a custom implementation for the function.

<a id="op-4354c0fccda13c069a05d4d3"></a>
## identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::Dialect::identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Source: `src/unparser/dialect.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Return the character used to quote identifiers.

<a id="op-1c4cd27f9026820e4626060c"></a>
## int32_cast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::int32_cast_dtype` · datafusion-sql 55.1.0

```rust
fn int32_cast_dtype(&self) -> ast::DataType
```

Source: `src/unparser/dialect.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The SQL type to use for Arrow Int32 unparsing
Most dialects use Integer, but some, like MySQL, require SIGNED

<a id="op-adf64e818090786af421bf76"></a>
## int64_cast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::int64_cast_dtype` · datafusion-sql 55.1.0

```rust
fn int64_cast_dtype(&self) -> ast::DataType
```

Source: `src/unparser/dialect.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The SQL type to use for Arrow Int64 unparsing
Most dialects use BigInt, but some, like MySQL, require SIGNED

<a id="op-248ea9f142dbbe690e1314cd"></a>
## int8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::int8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn int8_cast_dtype(&self) -> ast::DataType
```

Source: `src/unparser/dialect.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The SQL type to use for Arrow Int8 unparsing
Most dialects use TinyInt, but PostgreSQL prefers SmallInt

<a id="op-54725a5d5db2f30d10a17ec1"></a>
## interval_style

`function` · `datafusion_sql::unparser::dialect::Dialect::interval_style` · datafusion-sql 55.1.0

```rust
fn interval_style(&self) -> IntervalStyle
```

Source: `src/unparser/dialect.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7803b785d1407e25a6e88a98"></a>
## large_utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::large_utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn large_utf8_cast_dtype(&self) -> ast::DataType
```

Source: `src/unparser/dialect.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The SQL type to use for Arrow LargeUtf8 unparsing
Most dialects use TEXT, but some, like MySQL, require CHAR

<a id="op-97a57eedab4c6165724b4a6d"></a>
## requires_derived_table_alias

`function` · `datafusion_sql::unparser::dialect::Dialect::requires_derived_table_alias` · datafusion-sql 55.1.0

```rust
fn requires_derived_table_alias(&self) -> bool
```

Source: `src/unparser/dialect.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether the dialect requires a table alias for any subquery in the FROM clause
This affects behavior when deriving logical plans for Sort, Limit, etc.

<a id="op-c4c14690dc4271fc557a534f"></a>
## scalar_function_to_sql_overrides

`function` · `datafusion_sql::unparser::dialect::Dialect::scalar_function_to_sql_overrides` · datafusion-sql 55.1.0

```rust
fn scalar_function_to_sql_overrides(&self, _unparser: &Unparser<'_>, _func_name: &str, _args: &[Expr]) -> Result<Option<ast::Expr>>
```

Source: `src/unparser/dialect.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allows the dialect to override scalar function unparsing if the dialect has specific rules.
Returns None if the default unparsing should be used, or Some(ast::Expr) if there is
a custom implementation for the function.

<a id="op-ffe28b6f247908c6d2e018a7"></a>
## string_literal_to_sql

`function` · `datafusion_sql::unparser::dialect::Dialect::string_literal_to_sql` · datafusion-sql 55.1.0

```rust
fn string_literal_to_sql(&self, _s: &str) -> Option<ast::Expr>
```

Source: `src/unparser/dialect.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Override the default string literal unparsing.

Returns `Some(ast::Expr)` to replace the default single-quoted string,
or `None` to use the default behavior.

For example, MSSQL requires non-ASCII strings to use national string
literal syntax (`N'datafusion資料融合'`).

<a id="op-3642f1a2dbc81a7911282f93"></a>
## supports_column_alias_in_table_alias

`function` · `datafusion_sql::unparser::dialect::Dialect::supports_column_alias_in_table_alias` · datafusion-sql 55.1.0

```rust
fn supports_column_alias_in_table_alias(&self) -> bool
```

Source: `src/unparser/dialect.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Does the dialect support specifying column aliases as part of alias table definition?
(SELECT col1, col2 from my_table) AS my_table_alias(col1_alias, col2_alias)

<a id="op-906db47faf00de7da27e4d4b"></a>
## supports_empty_select_list

`function` · `datafusion_sql::unparser::dialect::Dialect::supports_empty_select_list` · datafusion-sql 55.1.0

```rust
fn supports_empty_select_list(&self) -> bool
```

Source: `src/unparser/dialect.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether the dialect supports an empty select list such as `SELECT FROM table`.

An empty select list returns rows without any column data, which is useful for:
- Counting rows: `SELECT FROM users WHERE active = true` (combined with `COUNT(*)`)
- Testing row existence without retrieving column data
- Performance optimization when only row counts or existence checks are needed

# Default

Returns `false` for maximum compatibility across SQL dialects. When `false`,
the unparser falls back to `SELECT 1 FROM table`.

# Implementation Note

Specific dialects should override this method to return `true` if they support
the empty select list syntax (e.g., PostgreSQL).

# Example SQL Output

```sql
-- When supported:
SELECT FROM users WHERE active = true;

-- Fallback when unsupported:
SELECT 1 FROM users WHERE active = true;
```

<a id="op-97cd3cbd42fc5690052ea602"></a>
## supports_nulls_first_in_sort

`function` · `datafusion_sql::unparser::dialect::Dialect::supports_nulls_first_in_sort` · datafusion-sql 55.1.0

```rust
fn supports_nulls_first_in_sort(&self) -> bool
```

Source: `src/unparser/dialect.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Does the dialect support specifying `NULLS FIRST/LAST` in `ORDER BY` clauses?

<a id="op-ff859c2d4f0ed5304c9442fa"></a>
## supports_qualify

`function` · `datafusion_sql::unparser::dialect::Dialect::supports_qualify` · datafusion-sql 55.1.0

```rust
fn supports_qualify(&self) -> bool
```

Source: `src/unparser/dialect.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allows the dialect to support the QUALIFY clause

Some dialects, like Postgres, do not support the QUALIFY clause

<a id="op-ce32a59adb294c71a416f358"></a>
## timestamp_cast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::timestamp_cast_dtype` · datafusion-sql 55.1.0

```rust
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, tz: &Option<Arc<str>>) -> ast::DataType
```

Source: `src/unparser/dialect.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The SQL type to use for Timestamp unparsing
Most dialects use Timestamp, but some, like MySQL, require Datetime
Some dialects like Dremio does not support WithTimeZone and requires always Timestamp

<a id="op-cb6c9578c11c943f2f7806e2"></a>
## timestamp_with_tz_to_string

`function` · `datafusion_sql::unparser::dialect::Dialect::timestamp_with_tz_to_string` · datafusion-sql 55.1.0

```rust
fn timestamp_with_tz_to_string(&self, dt: DateTime<Tz>, _unit: TimeUnit) -> String
```

Source: `src/unparser/dialect.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allows the dialect to override logic of formatting datetime with tz into string.

<a id="op-5394a5fdd6ad1e9d9b01fe2a"></a>
## unnest_as_lateral_flatten

`function` · `datafusion_sql::unparser::dialect::Dialect::unnest_as_lateral_flatten` · datafusion-sql 55.1.0

```rust
fn unnest_as_lateral_flatten(&self) -> bool
```

Source: `src/unparser/dialect.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Unparse the unnest plan as `LATERAL FLATTEN(INPUT => expr, ...)`.

Snowflake uses FLATTEN as a table function instead of the SQL-standard UNNEST.
When this returns `true`, the unparser emits
`LATERAL FLATTEN(INPUT => <col>, OUTER => <bool>)` in the FROM clause.

<a id="op-77a35cc4cff1b96e87c00c5e"></a>
## unnest_as_table_factor

`function` · `datafusion_sql::unparser::dialect::Dialect::unnest_as_table_factor` · datafusion-sql 55.1.0

```rust
fn unnest_as_table_factor(&self) -> bool
```

Source: `src/unparser/dialect.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allow to unparse the unnest plan as [ast::TableFactor::UNNEST](../operations/sqlparser.ast.query.TableFactor.md#op-09f539ef76985a39f91066ce).

Some dialects like BigQuery require UNNEST to be used in the FROM clause but
the LogicalPlan planner always puts UNNEST in the SELECT clause. This flag allows
to unparse the UNNEST plan as [ast::TableFactor::UNNEST](../operations/sqlparser.ast.query.TableFactor.md#op-09f539ef76985a39f91066ce) instead of a subquery.

<a id="op-9b6164eeb2a846b6a45cc041"></a>
## use_array_keyword_for_array_literals

`function` · `datafusion_sql::unparser::dialect::Dialect::use_array_keyword_for_array_literals` · datafusion-sql 55.1.0

```rust
fn use_array_keyword_for_array_literals(&self) -> bool
```

Source: `src/unparser/dialect.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Whether array literals should be rendered with the `ARRAY[...]` keyword.

<a id="op-7b6203520f3e658be595ebd8"></a>
## use_timestamp_for_date64

`function` · `datafusion_sql::unparser::dialect::Dialect::use_timestamp_for_date64` · datafusion-sql 55.1.0

```rust
fn use_timestamp_for_date64(&self) -> bool
```

Source: `src/unparser/dialect.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Does the dialect use TIMESTAMP to represent Date64 rather than DATETIME?
E.g. Trino, Athena and Dremio does not have DATETIME data type

<a id="op-d9c3150a416f0ca87506dce3"></a>
## utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::Dialect::utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn utf8_cast_dtype(&self) -> ast::DataType
```

Source: `src/unparser/dialect.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The SQL type to use for Arrow Utf8 unparsing
Most dialects use VARCHAR, but some, like MySQL, require CHAR

<a id="op-4c61eb8811748452e5f58225"></a>
## window_func_support_window_frame

`function` · `datafusion_sql::unparser::dialect::Dialect::window_func_support_window_frame` · datafusion-sql 55.1.0

```rust
fn window_func_support_window_frame(&self, _func_name: &str, _start_bound: &WindowFrameBound, _end_bound: &WindowFrameBound) -> bool
```

Source: `src/unparser/dialect.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Allows the dialect to choose to omit window frame in unparsing
based on function name and window frame bound
Returns false if specific function name / window frame bound indicates no window frame is needed in unparsing

<a id="op-8b598fe1f9ef666671888d89"></a>
## with_custom_scalar_overrides

`function` · `datafusion_sql::unparser::dialect::Dialect::with_custom_scalar_overrides` · datafusion-sql 55.1.0

```rust
fn with_custom_scalar_overrides(self, _handlers: Vec<(&str, ScalarFnToSqlHandler)>) -> Self where Self: Sized
```

Source: `src/unparser/dialect.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Extends the dialect's default rules for unparsing scalar functions.
This is useful for supporting application-specific UDFs or custom engine extensions.
