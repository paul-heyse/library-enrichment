# `sqlparser::dialect::duckdb::DuckDbDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.duckdb.DuckDbDialect.json).

<a id="op-5e297cba31b2badeb4e514f9"></a>
## DuckDbDialect

`struct` · `sqlparser::dialect::duckdb::DuckDbDialect` · sqlparser 0.62.0

```rust
struct DuckDbDialect
```

Source: `src/dialect/duckdb.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [DuckDB](https://duckdb.org/)

<a id="op-df71d15235a96f3a4993fd3a"></a>
## allow_extract_single_quotes

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::allow_extract_single_quotes` · sqlparser 0.62.0

```rust
fn allow_extract_single_quotes(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns true if this dialect allows the `EXTRACT` function to use single quotes in the part being extracted.

<a id="op-74ecd167b60d681b571279a3"></a>
## clone

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DuckDbDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 26], "end": [21, 31], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/duckdb.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14ae4fe2f2938ffa4d848dd4"></a>
## cmp

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DuckDbDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 72], "end": [21, 75], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/duckdb.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39da1c5a82861c8532636351"></a>
## default

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::default` · sqlparser 0.62.0

```rust
fn default() -> DuckDbDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 17], "end": [21, 24], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/duckdb.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15027bff41cad3786089956b"></a>
## deserialize

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 56], "end": [22, 74], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/duckdb.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b50521ca32e3c7b438f2d13"></a>
## eq

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DuckDbDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 39], "end": [21, 48], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/duckdb.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d1933212a959686ede0ed63"></a>
## fmt

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/duckdb.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5e40146bcf0cc789822e939"></a>
## hash

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 54], "end": [21, 58], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/duckdb.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9adb96324c33611f78002b9"></a>
## is_identifier_part

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6144b5f5b39bb478ddb6f63b"></a>
## is_identifier_start

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79faefe36dcd1fb117288b19"></a>
## partial_cmp

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DuckDbDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 60], "end": [21, 70], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/duckdb.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-120d1b35ef0aa4fed20cd3d7"></a>
## serialize

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/duckdb.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46e67c9679a7aa96c62cf59e"></a>
## support_map_literal_syntax

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::support_map_literal_syntax` · sqlparser 0.62.0

```rust
fn support_map_literal_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee98a863c432e0f94b6a86ac"></a>
## supports_array_typedef_with_brackets

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_array_typedef_with_brackets` · sqlparser 0.62.0

```rust
fn supports_array_typedef_with_brackets(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:95`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9566b4f15f5ab35049484e5e"></a>
## supports_bitwise_shift_operators

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_bitwise_shift_operators` · sqlparser 0.62.0

```rust
fn supports_bitwise_shift_operators(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e7bdd415e981e8f9fc92e3a"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-702bc87e8301f29faf8131a7"></a>
## supports_detach

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_detach` · sqlparser 0.62.0

```rust
fn supports_detach(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://duckdb.org/docs/sql/statements/attach#detach-syntax>

<a id="op-dad999de28bcf76af2215702"></a>
## supports_dictionary_syntax

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_dictionary_syntax` · sqlparser 0.62.0

```rust
fn supports_dictionary_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-216b845a75177729df81172c"></a>
## supports_explain_with_utility_options

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_explain_with_utility_options` · sqlparser 0.62.0

```rust
fn supports_explain_with_utility_options(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:85`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-692cfa42a24943c175e0c3a7"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85caa48a870c5ddbf59d2c93"></a>
## supports_from_first_select

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_from_first_select` · sqlparser 0.62.0

```rust
fn supports_from_first_select(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-506ee45bd56de02cbd24544d"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8390e06a654f9be904e07d2"></a>
## supports_install

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_install` · sqlparser 0.62.0

```rust
fn supports_install(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://duckdb.org/docs/extensions/overview>

<a id="op-ae9a9fd470738f2cd007e0f5"></a>
## supports_lambda_functions

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_lambda_functions` · sqlparser 0.62.0

```rust
fn supports_lambda_functions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://duckdb.org/docs/stable/sql/functions/lambda>

<a id="op-28812fc6f6e22ace3a340692"></a>
## supports_load_extension

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_load_extension` · sqlparser 0.62.0

```rust
fn supports_load_extension(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:90`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See DuckDB <https://duckdb.org/docs/sql/statements/load_and_install.html#load>

<a id="op-280ceff9546f4ae29c6da675"></a>
## supports_named_fn_args_with_assignment_operator

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_named_fn_args_with_assignment_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_assignment_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e780dab76581aa14f866b0d"></a>
## supports_named_fn_args_with_eq_operator

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_named_fn_args_with_eq_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_eq_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c35a9137bbc5eff03a5cb250"></a>
## supports_notnull_operator

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_notnull_operator` · sqlparser 0.62.0

```rust
fn supports_notnull_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DuckDB supports `NOTNULL` as an alias for `IS NOT NULL`,
see DuckDB Comparisons <https://duckdb.org/docs/stable/sql/expressions/comparison_operators#between-and-is-not-null>

<a id="op-6dee4e16a1739269609a891a"></a>
## supports_order_by_all

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_order_by_all` · sqlparser 0.62.0

```rust
fn supports_order_by_all(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See DuckDB <https://duckdb.org/docs/sql/query_syntax/orderby.html#order-by-all-examples>

<a id="op-84f0f517af22ed208b768224"></a>
## supports_select_wildcard_exclude

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_select_wildcard_exclude` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_exclude(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd2e9177867b99167b8fcd9c"></a>
## supports_select_wildcard_replace

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_select_wildcard_replace` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_replace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://duckdb.org/docs/sql/query_syntax/select#replace-clause>

<a id="op-2410203e0051fa69f90ea436"></a>
## supports_trailing_commas

`function` · `sqlparser::dialect::duckdb::DuckDbDialect::supports_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_trailing_commas(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::duckdb::DuckDbDialect", "path": "DuckDbDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [136, 2], "filename": "src/dialect/duckdb.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/duckdb.rs:27`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
