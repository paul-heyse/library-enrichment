# `sqlparser::dialect::teradata::TeradataDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.teradata.TeradataDialect.json).

<a id="op-cd59fa754c1a022b4521336f"></a>
## TeradataDialect

`struct` · `sqlparser::dialect::teradata::TeradataDialect` · sqlparser 0.62.0

```rust
struct TeradataDialect
```

Source: `src/dialect/teradata.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Teradata](https://docs.teradata.com/).

<a id="op-13692efd14e658894dac06be"></a>
## clone

`function` · `sqlparser::dialect::teradata::TeradataDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TeradataDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 26], "end": [21, 31], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/teradata.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a514e81fb0c2d9d33c7f8011"></a>
## cmp

`function` · `sqlparser::dialect::teradata::TeradataDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TeradataDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 72], "end": [21, 75], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/teradata.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f11ed864a890f3d39a25fd16"></a>
## default

`function` · `sqlparser::dialect::teradata::TeradataDialect::default` · sqlparser 0.62.0

```rust
fn default() -> TeradataDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 17], "end": [21, 24], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/teradata.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efb0232d45130acf9dc84580"></a>
## deserialize

`function` · `sqlparser::dialect::teradata::TeradataDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 56], "end": [22, 74], "filename": "src/dialect/teradata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/teradata.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81b17b3df297e3f340cbcaef"></a>
## eq

`function` · `sqlparser::dialect::teradata::TeradataDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TeradataDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 39], "end": [21, 48], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/teradata.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78912e6e3b7909bab595027b"></a>
## fmt

`function` · `sqlparser::dialect::teradata::TeradataDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/teradata.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b32ff15773f0dfbfece10478"></a>
## hash

`function` · `sqlparser::dialect::teradata::TeradataDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 54], "end": [21, 58], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/teradata.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bc5a23151c2b941598b93ad"></a>
## identifier_quote_style

`function` · `sqlparser::dialect::teradata::TeradataDialect::identifier_quote_style` · sqlparser 0.62.0

```rust
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:27`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Fundamentals/Basic-SQL-Syntax/Object-Names>

<a id="op-b3e6543d69684265b5153d96"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::teradata::TeradataDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:32`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Fundamentals/Basic-SQL-Syntax/Working-with-Unicode-Delimited-Identifiers>

<a id="op-ff23be6fce8013c1dc6c6672"></a>
## is_identifier_part

`function` · `sqlparser::dialect::teradata::TeradataDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-768bf9728a423124681d5297"></a>
## is_identifier_start

`function` · `sqlparser::dialect::teradata::TeradataDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:37`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/International-Character-Set-Support/Managing-International-Language-Support/Object-Names/Rules-for-Object-Naming>

<a id="op-db6bbaf99c58b0461edb5402"></a>
## partial_cmp

`function` · `sqlparser::dialect::teradata::TeradataDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TeradataDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 60], "end": [21, 70], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/teradata.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2baed53bb42000555f9cb8d"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::teradata::TeradataDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:59`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Data-Types-and-Literals/Data-Literals/Interval-Literals>

<a id="op-349aef46e14ca8151c91eaf7"></a>
## serialize

`function` · `sqlparser::dialect::teradata::TeradataDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/teradata.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-968fb0ddcfce017ade311bdb"></a>
## supports_boolean_literals

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_boolean_literals` · sqlparser 0.62.0

```rust
fn supports_boolean_literals(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Teradata has no native `BOOLEAN` data type.

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Data-Types-and-Literals>

<a id="op-9ea77ddbc33fd2f81383d375"></a>
## supports_comment_on

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_comment_on` · sqlparser 0.62.0

```rust
fn supports_comment_on(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Data-Definition-Language-Syntax-and-Examples/Comment-Help-and-Show-Statements/COMMENT-Comment-Placing-Form>

<a id="op-6c665e27a7c3e2431e6678de"></a>
## supports_create_table_select

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_create_table_select` · sqlparser 0.62.0

```rust
fn supports_create_table_select(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Data-Definition-Language-Syntax-and-Examples/Table-Statements/CREATE-TABLE-and-CREATE-TABLE-AS>

<a id="op-d8b6600263d044513ae87658"></a>
## supports_execute_immediate

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_execute_immediate` · sqlparser 0.62.0

```rust
fn supports_execute_immediate(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Stored-Procedures-and-Embedded-SQL/Dynamic-Embedded-SQL-Statements/Dynamic-SQL-Statement-Syntax/EXECUTE-IMMEDIATE>

<a id="op-b2bb2c0e4bae3e26a3e2f5c8"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Data-Manipulation-Language/SELECT-Statements/GROUP-BY-Clause/GROUP-BY-Clause-Syntax>

<a id="op-7bd18f74a239f64bc98eee17"></a>
## supports_string_literal_concatenation

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_string_literal_concatenation` · sqlparser 0.62.0

```rust
fn supports_string_literal_concatenation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Data-Types-and-Literals/Data-Literals/Character-String-Literals>

<a id="op-ca861f9508f2eede6e20119c"></a>
## supports_top_before_distinct

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_top_before_distinct` · sqlparser 0.62.0

```rust
fn supports_top_before_distinct(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Data-Manipulation-Language/SELECT-Statements/Select-List-Syntax/TOP-Clause>

<a id="op-66731e05196f034a31d3831d"></a>
## supports_window_function_null_treatment_arg

`function` · `sqlparser::dialect::teradata::TeradataDialect::supports_window_function_null_treatment_arg` · sqlparser 0.62.0

```rust
fn supports_window_function_null_treatment_arg(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::teradata::TeradataDialect", "path": "TeradataDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [92, 2], "filename": "src/dialect/teradata.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/teradata.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.teradata.com/r/Enterprise_IntelliFlex_VMware/SQL-Functions-Expressions-and-Predicates/Ordered-Analytical/Window-Aggregate-Functions>
