# `datafusion_sql::unparser::dialect::CustomDialectBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.CustomDialectBuilder.json).

<a id="op-c58bb1c84acf862abfb9d9a2"></a>
## CustomDialectBuilder

`struct` · `datafusion_sql::unparser::dialect::CustomDialectBuilder` · datafusion-sql 55.1.0

```rust
struct CustomDialectBuilder
```

Source: `src/unparser/dialect.rs:1014`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

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

<a id="op-d9950ae8718d7dbbc73334f9"></a>
## build

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::build` · datafusion-sql 55.1.0

```rust
fn build(self) -> CustomDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1076`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4df0019ad59d07fb38f9add2"></a>
## default

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1039, 1], "end": [1043, 2], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unparser/dialect.rs:1040`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6a33989a3049ee4f7e358a4"></a>
## new

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::new` · datafusion-sql 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1046`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0978a5de0463ee27b1a3bbb"></a>
## with_character_length_style

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_character_length_style` · datafusion-sql 55.1.0

```rust
fn with_character_length_style(self, character_length_style: CharacterLengthStyle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1135`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific character_length_style listed in `CharacterLengthStyle`

<a id="op-d5c85a8f8aa37c96093b41e9"></a>
## with_date32_cast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_date32_cast_dtype` · datafusion-sql 55.1.0

```rust
fn with_date32_cast_dtype(self, date32_cast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1202`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e1d43947a21a43b2302557"></a>
## with_date_field_extract_style

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_date_field_extract_style` · datafusion-sql 55.1.0

```rust
fn with_date_field_extract_style(self, date_field_extract_style: DateFieldExtractStyle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1171`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific date field extract style listed in `DateFieldExtractStyle`

<a id="op-902c6244458753989e9e41f2"></a>
## with_division_operator

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_division_operator` · datafusion-sql 55.1.0

```rust
fn with_division_operator(self, division_operator: BinaryOperator) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1224`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1feb242c803acbad13fe40b3"></a>
## with_float64_ast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_float64_ast_dtype` · datafusion-sql 55.1.0

```rust
fn with_float64_ast_dtype(self, float64_ast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1150`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific SQL type for Float64 casting: DOUBLE, DOUBLE PRECISION, etc.

<a id="op-248fc50ceb42386da3a3585e"></a>
## with_full_qualified_col

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_full_qualified_col` · datafusion-sql 55.1.0

```rust
fn with_full_qualified_col(self, full_qualified_col: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1238`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect to allow full qualified column names

<a id="op-53026395cabd152ef7a96644"></a>
## with_identifier_quote_style

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_identifier_quote_style` · datafusion-sql 55.1.0

```rust
fn with_identifier_quote_style(self, identifier_quote_style: char) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1105`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific identifier quote style, e.g. '`', '"'

<a id="op-e121cedb3ff04e3844f8062f"></a>
## with_int32_cast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_int32_cast_dtype` · datafusion-sql 55.1.0

```rust
fn with_int32_cast_dtype(self, int32_cast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1186`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific SQL type for Int32 casting: Integer, SIGNED, etc.

<a id="op-5407d9b0b9f0a946c7572be2"></a>
## with_int64_cast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_int64_cast_dtype` · datafusion-sql 55.1.0

```rust
fn with_int64_cast_dtype(self, int64_cast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1180`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific SQL type for Int64 casting: BigInt, SIGNED, etc.

<a id="op-88a3b7e5c40a908ca190a435"></a>
## with_int8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_int8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn with_int8_cast_dtype(self, int8_cast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1144`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific SQL type for Int8 casting: TinyInt, SmallInt, etc.

<a id="op-fdf7546bf55f623c6c4b7848"></a>
## with_interval_style

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_interval_style` · datafusion-sql 55.1.0

```rust
fn with_interval_style(self, interval_style: IntervalStyle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1129`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific interval style listed in `IntervalStyle`

<a id="op-66ea748b90d62a8275e7d1e7"></a>
## with_large_utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_large_utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn with_large_utf8_cast_dtype(self, large_utf8_cast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1162`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific SQL type for LargeUtf8 casting: TEXT, CHAR, etc.

<a id="op-26f830e2085082c40fbae84b"></a>
## with_requires_derived_table_alias

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_requires_derived_table_alias` · datafusion-sql 55.1.0

```rust
fn with_requires_derived_table_alias(self, requires_derived_table_alias: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1216`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f367b98e3a18ae199e716faa"></a>
## with_supports_column_alias_in_table_alias

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_supports_column_alias_in_table_alias` · datafusion-sql 55.1.0

```rust
fn with_supports_column_alias_in_table_alias(self, supports_column_alias_in_table_alias: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1208`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect to support column aliases as part of alias table definition

<a id="op-2adb6f2331c156328e5dbc37"></a>
## with_supports_nulls_first_in_sort

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_supports_nulls_first_in_sort` · datafusion-sql 55.1.0

```rust
fn with_supports_nulls_first_in_sort(self, supports_nulls_first_in_sort: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1111`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect to support `NULLS FIRST` in `ORDER BY` clauses

<a id="op-91cfcf1244261f6a61b78810"></a>
## with_timestamp_cast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_timestamp_cast_dtype` · datafusion-sql 55.1.0

```rust
fn with_timestamp_cast_dtype(self, timestamp_cast_dtype: ast::DataType, timestamp_tz_cast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1192`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific SQL type for Timestamp casting: Timestamp, Datetime, etc.

<a id="op-a4b6db2e7e7043d7f49fa4c4"></a>
## with_unnest_as_lateral_flatten

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_unnest_as_lateral_flatten` · datafusion-sql 55.1.0

```rust
fn with_unnest_as_lateral_flatten(self, unnest_as_lateral_flatten: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1248`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ab6273ef4a793f758bd5287"></a>
## with_unnest_as_table_factor

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_unnest_as_table_factor` · datafusion-sql 55.1.0

```rust
fn with_unnest_as_table_factor(self, unnest_as_table_factor: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1243`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c1d58e40e5e548c88108a9c"></a>
## with_use_timestamp_for_date64

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_use_timestamp_for_date64` · datafusion-sql 55.1.0

```rust
fn with_use_timestamp_for_date64(self, use_timestamp_for_date64: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1120`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect to uses TIMESTAMP when casting Date64 rather than DATETIME

<a id="op-0ebb2468d03dc0a23e6fdb0f"></a>
## with_utf8_cast_dtype

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_utf8_cast_dtype` · datafusion-sql 55.1.0

```rust
fn with_utf8_cast_dtype(self, utf8_cast_dtype: ast::DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1156`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Customize the dialect with a specific SQL type for Utf8 casting: VARCHAR, CHAR, etc.

<a id="op-7688e25dcd8694b430928b76"></a>
## with_window_func_support_window_frame

`function` · `datafusion_sql::unparser::dialect::CustomDialectBuilder::with_window_func_support_window_frame` · datafusion-sql 55.1.0

```rust
fn with_window_func_support_window_frame(self, window_func_support_window_frame: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::CustomDialectBuilder", "path": "CustomDialectBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1045, 1], "end": [1255, 2], "filename": "src/unparser/dialect.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/dialect.rs:1229`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
