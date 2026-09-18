# `sqlparser::dialect::bigquery::BigQueryDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.bigquery.BigQueryDialect.json).

<a id="op-7c7f7b1819230de825d06dae"></a>
## BigQueryDialect

`struct` · `sqlparser::dialect::bigquery::BigQueryDialect` · sqlparser 0.62.0

```rust
struct BigQueryDialect
```

Source: `src/dialect/bigquery.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Google Bigquery](https://cloud.google.com/bigquery/)

<a id="op-acd3a457aba605a2a100479d"></a>
## clone

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> BigQueryDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 26], "end": [45, 31], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/bigquery.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be625a8c52f67f2c00af03c6"></a>
## cmp

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &BigQueryDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 72], "end": [45, 75], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/bigquery.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db4c4bf62dcce632f3909443"></a>
## default

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::default` · sqlparser 0.62.0

```rust
fn default() -> BigQueryDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 17], "end": [45, 24], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/bigquery.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d98cbf0861ff26b8e4118be"></a>
## deserialize

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 56], "end": [46, 74], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/bigquery.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-007dea305cdb5d9fe2d69702"></a>
## eq

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &BigQueryDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 39], "end": [45, 48], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/bigquery.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c40d2981eeeba913f7b0802c"></a>
## fmt

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/bigquery.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfcac3dff7aea5c509c18583"></a>
## hash

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 54], "end": [45, 58], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/bigquery.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81c00ea6df522feb6c8d28e1"></a>
## is_column_alias

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::is_column_alias` · sqlparser 0.62.0

```rust
fn is_column_alias(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14512347950ffcb1ff7dc227"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:66`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#identifiers>

<a id="op-fff23a17ef0bc06791c663c3"></a>
## is_identifier_part

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:86`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3be0346b61cd2eef7da76b3f"></a>
## is_identifier_start

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61d7021250d7cd4519f88637"></a>
## parse_statement

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::parse_statement` · sqlparser 0.62.0

```rust
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:50`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2869c180b537c052646bf24a"></a>
## partial_cmp

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &BigQueryDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 60], "end": [45, 70], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/bigquery.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-126706a9d74921fc7f2b7ab2"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ab06b36594a4f2a3cdbe4a9"></a>
## serialize

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 38], "end": [46, 54], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/bigquery.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cda66ed0d0e4c9b9ba92e45"></a>
## supports_column_definition_trailing_commas

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_column_definition_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_column_definition_trailing_commas(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_table_statement>

<a id="op-fbb3dd247bcfe521f17a6290"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e6b5ffbe982d93e9bd2e77e"></a>
## supports_create_table_multi_schema_info_sources

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_create_table_multi_schema_info_sources` · sqlparser 0.62.0

```rust
fn supports_create_table_multi_schema_info_sources(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-340b1942d3d29eda78bf1c65"></a>
## supports_execute_immediate

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_execute_immediate` · sqlparser 0.62.0

```rust
fn supports_execute_immediate(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#execute_immediate>

<a id="op-4deb089796d5fd7598e2ce8d"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:145`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc7790c8355cf19a8f09e2a4"></a>
## supports_parenthesized_set_variables

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_parenthesized_set_variables` · sqlparser 0.62.0

```rust
fn supports_parenthesized_set_variables(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#set)

<a id="op-377d55a559ff397a40f32cec"></a>
## supports_pipe_operator

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_pipe_operator` · sqlparser 0.62.0

```rust
fn supports_pipe_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-704c82bba5564e3bb392b642"></a>
## supports_projection_trailing_commas

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_projection_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_projection_trailing_commas(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:70`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8eb3f02ba09d64f68a534ad"></a>
## supports_select_expr_star

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_select_expr_star` · sqlparser 0.62.0

```rust
fn supports_select_expr_star(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_expression_star>

<a id="op-605836030b3a2d01864cf36a"></a>
## supports_select_wildcard_except

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_select_wildcard_except` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_except(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6b5f0cceebc6c1af9424c5b"></a>
## supports_select_wildcard_replace

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_select_wildcard_replace` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_replace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_replace>

<a id="op-8460078a4df7e3903317d4e5"></a>
## supports_string_literal_backslash_escape

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_string_literal_backslash_escape` · sqlparser 0.62.0

```rust
fn supports_string_literal_backslash_escape(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b22aa68a4465f71051d1d33"></a>
## supports_struct_literal

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_struct_literal` · sqlparser 0.62.0

```rust
fn supports_struct_literal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1ca5221964d520d7d0d9713"></a>
## supports_table_versioning

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_table_versioning` · sqlparser 0.62.0

```rust
fn supports_table_versioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:140`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10828bb43556c666667b899b"></a>
## supports_triple_quoted_string

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_triple_quoted_string` · sqlparser 0.62.0

```rust
fn supports_triple_quoted_string(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:91`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-2f083e246945067d13f23c59"></a>
## supports_window_clause_named_window_reference

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_window_clause_named_window_reference` · sqlparser 0.62.0

```rust
fn supports_window_clause_named_window_reference(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/window-function-calls#ref_named_window)

<a id="op-fd564646a3ebed8989f5768b"></a>
## supports_window_function_null_treatment_arg

`function` · `sqlparser::dialect::bigquery::BigQueryDialect::supports_window_function_null_treatment_arg` · sqlparser 0.62.0

```rust
fn supports_window_function_null_treatment_arg(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::bigquery::BigQueryDialect", "path": "BigQueryDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [169, 2], "filename": "src/dialect/bigquery.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/bigquery.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [doc](https://cloud.google.com/bigquery/docs/reference/standard-sql/navigation_functions#first_value)
