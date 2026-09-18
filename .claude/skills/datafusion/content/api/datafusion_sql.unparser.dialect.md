# `datafusion_sql::unparser::dialect`

Crate `datafusion-sql` · 15 public items · structured records in [`model/datafusion_sql.unparser.dialect.json`](../model/datafusion_sql.unparser.dialect.json)

## CharacterLengthStyle

`enum` · `datafusion_sql::unparser::dialect::CharacterLengthStyle`

```rust
enum CharacterLengthStyle
```

**Variants**: `Length`, `CharacterLength`

**Derives**: Clone, Copy, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.CharacterLengthStyle.md).


`CharacterLengthStyle` to use for unparsing

Different DBMSs uses different names for function calculating the number of characters in the string
`Length` style uses length(x)
`SQLStandard` style uses character_length(x)

---

## DateFieldExtractStyle

`enum` · `datafusion_sql::unparser::dialect::DateFieldExtractStyle`

```rust
enum DateFieldExtractStyle
```

**Variants**: `DatePart`, `Extract`, `Strftime`

**Derives**: Clone, Copy, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.DateFieldExtractStyle.md).


Datetime subfield extraction style for unparsing

`<https://www.postgresql.org/docs/current/functions-datetime.html#FUNCTIONS-DATETIME-EXTRACT>`
Different DBMSs follow different standards; popular ones are:
date_part('YEAR', date '2001-02-16')
EXTRACT(YEAR from date '2001-02-16')
Some DBMSs, like Postgres, support both, whereas others like MySQL require EXTRACT.

---

## DistinctFromStyle

`enum` · `datafusion_sql::unparser::dialect::DistinctFromStyle`

```rust
enum DistinctFromStyle
```

**Variants**: `FullText`, `Spaceship`

**Derives**: Clone, Copy, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.DistinctFromStyle.md).


`DistinctFromStyle` to use for unparsing `IsDistinctFrom` and `IsNotDistinctFrom` operators

---

## IntervalStyle

`enum` · `datafusion_sql::unparser::dialect::IntervalStyle`

```rust
enum IntervalStyle
```

**Variants**: `PostgresVerbose`, `SQLStandard`, `MySQL`

**Derives**: Clone, Copy

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.IntervalStyle.md).


`IntervalStyle` to use for unparsing

<https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-INTERVAL-INPUT>
different DBMS follows different standards, popular ones are:
postgres_verbose: '2 years 15 months 100 weeks 99 hours 123456789 milliseconds' which is
compatible with arrow display format, as well as duckdb
sql standard format is '1-2' for year-month, or '1 10:10:10.123456' for day-time
<https://www.contrib.andrew.cmu.edu/~shadow/sql/sql1992.txt>

---

## BigQueryDialect

`struct` · `datafusion_sql::unparser::dialect::BigQueryDialect`

```rust
struct BigQueryDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**Derives**: Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn col_alias_overrides(&self, alias: &str) -> Result<Option<String>>
fn date_field_extract_style(&self) -> DateFieldExtractStyle
fn float64_ast_dtype(&self) -> ast::DataType
fn identifier_quote_style(&self, _: &str) -> Option<char>
fn interval_style(&self) -> IntervalStyle
fn large_utf8_cast_dtype(&self) -> ast::DataType
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
fn supports_column_alias_in_table_alias(&self) -> bool
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, _tz: &Option<Arc<str>>) -> ast::DataType
fn unnest_as_table_factor(&self) -> bool
fn utf8_cast_dtype(&self) -> ast::DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.BigQueryDialect.md).


---

## CustomDialect

`struct` · `datafusion_sql::unparser::dialect::CustomDialect`

```rust
struct CustomDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**Derives**: Default

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn character_length_style(&self) -> CharacterLengthStyle
fn date32_cast_dtype(&self) -> ast::DataType
fn date_field_extract_style(&self) -> DateFieldExtractStyle
fn division_operator(&self) -> BinaryOperator
fn float64_ast_dtype(&self) -> ast::DataType
fn full_qualified_col(&self) -> bool
fn identifier_quote_style(&self, _: &str) -> Option<char>
fn int32_cast_dtype(&self) -> ast::DataType
fn int64_cast_dtype(&self) -> ast::DataType
fn int8_cast_dtype(&self) -> ast::DataType
fn interval_style(&self) -> IntervalStyle
fn large_utf8_cast_dtype(&self) -> ast::DataType
fn requires_derived_table_alias(&self) -> bool
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
fn supports_column_alias_in_table_alias(&self) -> bool
fn supports_nulls_first_in_sort(&self) -> bool
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, tz: &Option<Arc<str>>) -> ast::DataType
fn unnest_as_lateral_flatten(&self) -> bool
fn unnest_as_table_factor(&self) -> bool
fn use_timestamp_for_date64(&self) -> bool
fn utf8_cast_dtype(&self) -> ast::DataType
fn window_func_support_window_frame(&self, _func_name: &str, _start_bound: &WindowFrameBound, _end_bound: &WindowFrameBound) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.CustomDialect.md).


---

## CustomDialectBuilder

`struct` · `datafusion_sql::unparser::dialect::CustomDialectBuilder`

```rust
struct CustomDialectBuilder
```

**Derives**: Default

**Methods** (23)

```rust
fn build(self) -> CustomDialect
fn new() -> Self
fn with_character_length_style(self, character_length_style: CharacterLengthStyle) -> Self
fn with_date32_cast_dtype(self, date32_cast_dtype: ast::DataType) -> Self
fn with_date_field_extract_style(self, date_field_extract_style: DateFieldExtractStyle) -> Self
fn with_division_operator(self, division_operator: BinaryOperator) -> Self
fn with_float64_ast_dtype(self, float64_ast_dtype: ast::DataType) -> Self
fn with_full_qualified_col(self, full_qualified_col: bool) -> Self
fn with_identifier_quote_style(self, identifier_quote_style: char) -> Self
fn with_int32_cast_dtype(self, int32_cast_dtype: ast::DataType) -> Self
fn with_int64_cast_dtype(self, int64_cast_dtype: ast::DataType) -> Self
fn with_int8_cast_dtype(self, int8_cast_dtype: ast::DataType) -> Self
fn with_interval_style(self, interval_style: IntervalStyle) -> Self
fn with_large_utf8_cast_dtype(self, large_utf8_cast_dtype: ast::DataType) -> Self
fn with_requires_derived_table_alias(self, requires_derived_table_alias: bool) -> Self
fn with_supports_column_alias_in_table_alias(self, supports_column_alias_in_table_alias: bool) -> Self
fn with_supports_nulls_first_in_sort(self, supports_nulls_first_in_sort: bool) -> Self
fn with_timestamp_cast_dtype(self, timestamp_cast_dtype: ast::DataType, timestamp_tz_cast_dtype: ast::DataType) -> Self
fn with_unnest_as_lateral_flatten(self, unnest_as_lateral_flatten: bool) -> Self
fn with_unnest_as_table_factor(self, unnest_as_table_factor: bool) -> Self
fn with_use_timestamp_for_date64(self, use_timestamp_for_date64: bool) -> Self
fn with_utf8_cast_dtype(self, utf8_cast_dtype: ast::DataType) -> Self
fn with_window_func_support_window_frame(self, window_func_support_window_frame: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.CustomDialectBuilder.md).


`CustomDialectBuilder` to build `CustomDialect` using builder pattern


# Examples

Building a custom dialect with all default options set in CustomDialectBuilder::new()
but with `use_timestamp_for_date64` overridden to `true`

```
use datafusion_sql::unparser::dialect::CustomDialectBuilder;
let dialect = CustomDialectBuilder::new()
    .with_use_timestamp_for_date64(true)
    .build();
```

---

## DefaultDialect

`struct` · `datafusion_sql::unparser::dialect::DefaultDialect`

```rust
struct DefaultDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn identifier_quote_style(&self, identifier: &str) -> Option<char>
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.DefaultDialect.md).


---

## DuckDBDialect

`struct` · `datafusion_sql::unparser::dialect::DuckDBDialect`

```rust
struct DuckDBDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**Derives**: Default

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn character_length_style(&self) -> CharacterLengthStyle
fn distinct_from_style(&self) -> DistinctFromStyle
fn division_operator(&self) -> BinaryOperator
fn identifier_quote_style(&self, _: &str) -> Option<char>
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
fn with_custom_scalar_overrides(self, handlers: Vec<(&str, ScalarFnToSqlHandler)>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.DuckDBDialect.md).


---

## MySqlDialect

`struct` · `datafusion_sql::unparser::dialect::MySqlDialect`

```rust
struct MySqlDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn date_field_extract_style(&self) -> DateFieldExtractStyle
fn distinct_from_style(&self) -> DistinctFromStyle
fn identifier_quote_style(&self, _: &str) -> Option<char>
fn int32_cast_dtype(&self) -> ast::DataType
fn int64_cast_dtype(&self) -> ast::DataType
fn interval_style(&self) -> IntervalStyle
fn large_utf8_cast_dtype(&self) -> ast::DataType
fn requires_derived_table_alias(&self) -> bool
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
fn supports_nulls_first_in_sort(&self) -> bool
fn supports_qualify(&self) -> bool
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, _tz: &Option<Arc<str>>) -> ast::DataType
fn utf8_cast_dtype(&self) -> ast::DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.MySqlDialect.md).


---

## PostgreSqlDialect

`struct` · `datafusion_sql::unparser::dialect::PostgreSqlDialect`

```rust
struct PostgreSqlDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn distinct_from_style(&self) -> DistinctFromStyle
fn float64_ast_dtype(&self) -> ast::DataType
fn identifier_quote_style(&self, _: &str) -> Option<char>
fn int8_cast_dtype(&self) -> ast::DataType
fn interval_style(&self) -> IntervalStyle
fn requires_derived_table_alias(&self) -> bool
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
fn supports_empty_select_list(&self) -> bool
fn supports_qualify(&self) -> bool
fn use_array_keyword_for_array_literals(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.PostgreSqlDialect.md).


---

## SnowflakeDialect

`struct` · `datafusion_sql::unparser::dialect::SnowflakeDialect`

```rust
struct SnowflakeDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn identifier_quote_style(&self, _: &str) -> Option<char>
fn supports_column_alias_in_table_alias(&self) -> bool
fn supports_empty_select_list(&self) -> bool
fn supports_nulls_first_in_sort(&self) -> bool
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, tz: &Option<Arc<str>>) -> ast::DataType
fn unnest_as_lateral_flatten(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.SnowflakeDialect.md).


Dialect for Snowflake SQL.

Key differences from the default dialect:
- Uses double-quote identifier quoting
- Supports `NULLS FIRST`/`NULLS LAST` in `ORDER BY`
- Does not support empty select lists (`SELECT FROM t`)
- Does not support column aliases in table alias definitions
  (Snowflake accepts the syntax but silently ignores the renames in join contexts)
- Unparses `UNNEST` plans as `LATERAL FLATTEN(INPUT => expr, ...)`

---

## SqliteDialect

`struct` · `datafusion_sql::unparser::dialect::SqliteDialect`

```rust
struct SqliteDialect
```

**Implements**: `datafusion_sql::unparser::dialect::Dialect`

**via `datafusion_sql::unparser::dialect::Dialect`**

```rust
fn character_length_style(&self) -> CharacterLengthStyle
fn date32_cast_dtype(&self) -> ast::DataType
fn date_field_extract_style(&self) -> DateFieldExtractStyle
fn distinct_from_style(&self) -> DistinctFromStyle
fn identifier_quote_style(&self, _: &str) -> Option<char>
fn scalar_function_to_sql_overrides(&self, unparser: &Unparser<'_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>>
fn supports_column_alias_in_table_alias(&self) -> bool
fn supports_qualify(&self) -> bool
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, _tz: &Option<Arc<str>>) -> ast::DataType
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.SqliteDialect.md).


---

## Dialect

`trait` · `datafusion_sql::unparser::dialect::Dialect`

```rust
trait Dialect: Send + Sync
```

**Implementors** (8)

- `datafusion_sql::unparser::dialect::BigQueryDialect`
- `datafusion_sql::unparser::dialect::CustomDialect`
- `datafusion_sql::unparser::dialect::DefaultDialect`
- `datafusion_sql::unparser::dialect::DuckDBDialect`
- `datafusion_sql::unparser::dialect::MySqlDialect`
- `datafusion_sql::unparser::dialect::PostgreSqlDialect`
- `datafusion_sql::unparser::dialect::SnowflakeDialect`
- `datafusion_sql::unparser::dialect::SqliteDialect`

**Methods** (31)

```rust
fn character_length_style(&self) -> CharacterLengthStyle
fn col_alias_overrides(&self, _alias: &str) -> Result<Option<String>>
fn date32_cast_dtype(&self) -> ast::DataType
fn date_field_extract_style(&self) -> DateFieldExtractStyle
fn distinct_from_style(&self) -> DistinctFromStyle
fn division_operator(&self) -> BinaryOperator
fn float64_ast_dtype(&self) -> ast::DataType
fn full_qualified_col(&self) -> bool
fn higher_order_function_to_sql_overrides(&self, _unparser: &Unparser<'_>, _func_name: &str, _args: &[Expr]) -> Result<Option<ast::Expr>>
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
fn int32_cast_dtype(&self) -> ast::DataType
fn int64_cast_dtype(&self) -> ast::DataType
fn int8_cast_dtype(&self) -> ast::DataType
fn interval_style(&self) -> IntervalStyle
fn large_utf8_cast_dtype(&self) -> ast::DataType
fn requires_derived_table_alias(&self) -> bool
fn scalar_function_to_sql_overrides(&self, _unparser: &Unparser<'_>, _func_name: &str, _args: &[Expr]) -> Result<Option<ast::Expr>>
fn string_literal_to_sql(&self, _s: &str) -> Option<ast::Expr>
fn supports_column_alias_in_table_alias(&self) -> bool
fn supports_empty_select_list(&self) -> bool
fn supports_nulls_first_in_sort(&self) -> bool
fn supports_qualify(&self) -> bool
fn timestamp_cast_dtype(&self, _time_unit: &TimeUnit, tz: &Option<Arc<str>>) -> ast::DataType
fn timestamp_with_tz_to_string(&self, dt: DateTime<Tz>, _unit: TimeUnit) -> String
fn unnest_as_lateral_flatten(&self) -> bool
fn unnest_as_table_factor(&self) -> bool
fn use_array_keyword_for_array_literals(&self) -> bool
fn use_timestamp_for_date64(&self) -> bool
fn utf8_cast_dtype(&self) -> ast::DataType
fn window_func_support_window_frame(&self, _func_name: &str, _start_bound: &WindowFrameBound, _end_bound: &WindowFrameBound) -> bool
fn with_custom_scalar_overrides(self, _handlers: Vec<(&str, ScalarFnToSqlHandler)>) -> Self where Self: Sized
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.Dialect.md).


`Dialect` to use for Unparsing

The default dialect tries to avoid quoting identifiers unless necessary (e.g. `a` instead of `"a"`)
but this behavior can be overridden as needed

**Note**: This trait will eventually be replaced by the Dialect in the SQLparser package

See <https://github.com/sqlparser-rs/sqlparser-rs/pull/1170>
See also the discussion in <https://github.com/apache/datafusion/pull/10625>

---

## ScalarFnToSqlHandler

`type_alias` · `datafusion_sql::unparser::dialect::ScalarFnToSqlHandler`

```rust
type ScalarFnToSqlHandler = Box<dyn Fn(&super::Unparser<'_>, &[datafusion_expr::Expr]) -> datafusion_common::Result<Option<ast::Expr>> + Send + Sync>
```

[Full member, field, variant and typed contracts](../operations/datafusion_sql.unparser.dialect.ScalarFnToSqlHandler.md).


---
