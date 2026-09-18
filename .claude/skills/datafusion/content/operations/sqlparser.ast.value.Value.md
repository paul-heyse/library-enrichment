# `sqlparser::ast::value::Value`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.Value.json).

<a id="op-bc448bc8bde6975348b4ad8b"></a>
## Value

`enum` · `sqlparser::ast::value::Value` · sqlparser 0.62.0

```rust
enum Value
```

Source: `src/ast/value.rs:139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Primitive SQL values such as number and string

<a id="op-f86e36c4fb4f1979376a8d16"></a>
## Boolean

`variant` · `sqlparser::ast::value::Value::Boolean` · sqlparser 0.62.0

```rust
Boolean
```

Source: `src/ast/value.rs:203`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Boolean value true or false

<a id="op-188d76393a757355e0205834"></a>
## DollarQuotedString

`variant` · `sqlparser::ast::value::Value::DollarQuotedString` · sqlparser 0.62.0

```rust
DollarQuotedString
```

Source: `src/ast/value.rs:152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dollar-quoted string literal, e.g. `$$...$$` or `$tag$...$tag$` (Postgres syntax).

<a id="op-b5d443c7be39f245d1588ba6"></a>
## DoubleQuotedByteStringLiteral

`variant` · `sqlparser::ast::value::Value::DoubleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
DoubleQuotedByteStringLiteral
```

Source: `src/ast/value.rs:170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

B"string value"

<a id="op-0d1f371c96b715b072c944ca"></a>
## DoubleQuotedRawStringLiteral

`variant` · `sqlparser::ast::value::Value::DoubleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
DoubleQuotedRawStringLiteral
```

Source: `src/ast/value.rs:182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double quoted literal with raw string prefix. Example `R"abc"`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-cbfcba5024d7a3212edc4917"></a>
## DoubleQuotedString

`variant` · `sqlparser::ast::value::Value::DoubleQuotedString` · sqlparser 0.62.0

```rust
DoubleQuotedString
```

Source: `src/ast/value.rs:201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Double quoted string literal, e.g. `"abc"`.

<a id="op-322b45a9202f8e0f2cebb055"></a>
## EscapedStringLiteral

`variant` · `sqlparser::ast::value::Value::EscapedStringLiteral` · sqlparser 0.62.0

```rust
EscapedStringLiteral
```

Source: `src/ast/value.rs:162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

e'string value' (postgres extension)
See [Postgres docs](https://www.postgresql.org/docs/8.3/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS)
for more details.

<a id="op-487d81a7e5e6189aca4b4886"></a>
## HexStringLiteral

`variant` · `sqlparser::ast::value::Value::HexStringLiteral` · sqlparser 0.62.0

```rust
HexStringLiteral
```

Source: `src/ast/value.rs:198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

X'hex value'

<a id="op-182c8d7e642cac3ae52ce0f4"></a>
## NationalQuoteDelimitedStringLiteral

`variant` · `sqlparser::ast::value::Value::NationalQuoteDelimitedStringLiteral` · sqlparser 0.62.0

```rust
NationalQuoteDelimitedStringLiteral
```

Source: `src/ast/value.rs:196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

"National" quote delimited literal. Examples `Q'{ab'c}'`, `Q'|ab'c|'`, `Q'|ab|c|'`
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Literals.html#GUID-1824CBAA-6E16-4921-B2A6-112FB02248DA)

<a id="op-971c45fe3e080abbcd9ea7e9"></a>
## NationalStringLiteral

`variant` · `sqlparser::ast::value::Value::NationalStringLiteral` · sqlparser 0.62.0

```rust
NationalStringLiteral
```

Source: `src/ast/value.rs:190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

N'string value'

<a id="op-d22605e2cf7cf56bbe95e24e"></a>
## Null

`variant` · `sqlparser::ast::value::Value::Null` · sqlparser 0.62.0

```rust
Null
```

Source: `src/ast/value.rs:205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NULL` value

<a id="op-dbaaff13c440f690fd32891d"></a>
## Number

`variant` · `sqlparser::ast::value::Value::Number` · sqlparser 0.62.0

```rust
Number
```

Source: `src/ast/value.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Numeric literal

<a id="op-9d3805526682d05ef8d12567"></a>
## Placeholder

`variant` · `sqlparser::ast::value::Value::Placeholder` · sqlparser 0.62.0

```rust
Placeholder
```

Source: `src/ast/value.rs:207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`?` or `$` Prepared statement arg placeholder

<a id="op-fe645ce168ae3916ceccf75a"></a>
## QuoteDelimitedStringLiteral

`variant` · `sqlparser::ast::value::Value::QuoteDelimitedStringLiteral` · sqlparser 0.62.0

```rust
QuoteDelimitedStringLiteral
```

Source: `src/ast/value.rs:193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Quote delimited literal. Examples `Q'{ab'c}'`, `Q'|ab'c|'`, `Q'|ab|c|'`
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Literals.html#GUID-1824CBAA-6E16-4921-B2A6-112FB02248DA)

<a id="op-ee3f5387d0b92404fb5ff4ff"></a>
## SingleQuotedByteStringLiteral

`variant` · `sqlparser::ast::value::Value::SingleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
SingleQuotedByteStringLiteral
```

Source: `src/ast/value.rs:168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

B'string value'

<a id="op-f3e33a30bf7372e0c9ca4641"></a>
## SingleQuotedRawStringLiteral

`variant` · `sqlparser::ast::value::Value::SingleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
SingleQuotedRawStringLiteral
```

Source: `src/ast/value.rs:179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Single quoted literal with raw string prefix. Example `R'abc'`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-d5d2399a31cc2ef4b4af6349"></a>
## SingleQuotedString

`variant` · `sqlparser::ast::value::Value::SingleQuotedString` · sqlparser 0.62.0

```rust
SingleQuotedString
```

Source: `src/ast/value.rs:150`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

'string value'

<a id="op-e77f46069581a3ba267e6d3c"></a>
## TripleDoubleQuotedByteStringLiteral

`variant` · `sqlparser::ast::value::Value::TripleDoubleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
TripleDoubleQuotedByteStringLiteral
```

Source: `src/ast/value.rs:176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple double quoted literal with byte string prefix. Example `B"""abc"""`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-5adc491195ea4b4a6cb5a3b8"></a>
## TripleDoubleQuotedRawStringLiteral

`variant` · `sqlparser::ast::value::Value::TripleDoubleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
TripleDoubleQuotedRawStringLiteral
```

Source: `src/ast/value.rs:188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple double quoted literal with raw string prefix. Example `R"""abc"""`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-10b7b7f60132f62044f4cb84"></a>
## TripleDoubleQuotedString

`variant` · `sqlparser::ast::value::Value::TripleDoubleQuotedString` · sqlparser 0.62.0

```rust
TripleDoubleQuotedString
```

Source: `src/ast/value.rs:158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple double quoted strings: Example """abc"""
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-975dfbddacce2820e611d564"></a>
## TripleSingleQuotedByteStringLiteral

`variant` · `sqlparser::ast::value::Value::TripleSingleQuotedByteStringLiteral` · sqlparser 0.62.0

```rust
TripleSingleQuotedByteStringLiteral
```

Source: `src/ast/value.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple single quoted literal with byte string prefix. Example `B'''abc'''`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-08d05481f86af0c6ca415e25"></a>
## TripleSingleQuotedRawStringLiteral

`variant` · `sqlparser::ast::value::Value::TripleSingleQuotedRawStringLiteral` · sqlparser 0.62.0

```rust
TripleSingleQuotedRawStringLiteral
```

Source: `src/ast/value.rs:185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple single quoted literal with raw string prefix. Example `R'''abc'''`
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-840f7e2df81ca5a267d021ab"></a>
## TripleSingleQuotedString

`variant` · `sqlparser::ast::value::Value::TripleSingleQuotedString` · sqlparser 0.62.0

```rust
TripleSingleQuotedString
```

Source: `src/ast/value.rs:155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Triple single quoted strings: Example '''abc'''
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a id="op-5a5f23e1cb53ba2301e9a345"></a>
## UnicodeStringLiteral

`variant` · `sqlparser::ast::value::Value::UnicodeStringLiteral` · sqlparser 0.62.0

```rust
UnicodeStringLiteral
```

Source: `src/ast/value.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

u&'string value' (postgres extension)
See [Postgres docs](https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-UESCAPE)
for more details.

<a id="op-92ac965bebe7c2e1a0ef63bd"></a>
## clone

`function` · `sqlparser::ast::value::Value::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Value
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 17], "end": [136, 22], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/value.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82980d888859958c4b3ab7ac"></a>
## cmp

`function` · `sqlparser::ast::value::Value::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Value) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 51], "end": [136, 54], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/value.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22f93322625e689f42d85857"></a>
## deserialize

`function` · `sqlparser::ast::value::Value::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 49], "end": [137, 60], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/value.rs:137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8a8b6f83b40bdfb64a1356d"></a>
## eq

`function` · `sqlparser::ast::value::Value::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Value) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 24], "end": [136, 33], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/value.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30119608f3e192d7f3628c4d"></a>
## fmt

`function` · `sqlparser::ast::value::Value::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [261, 1], "end": [293, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/value.rs:262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-469f00f8d5294e4d72e34212"></a>
## fmt

`function` · `sqlparser::ast::value::Value::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 10], "end": [136, 15], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/value.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbb112d51bf0c21c2053d0ab"></a>
## from

`function` · `sqlparser::ast::value::Value::from` · sqlparser 0.62.0

```rust
fn from(value: ValueWithSpan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [119, 2], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::ValueWithSpan", "path": "ValueWithSpan"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/value.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49038afaa26dd469e9c2c9d5"></a>
## hash

`function` · `sqlparser::ast::value::Value::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 56], "end": [136, 60], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/value.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4e6be7573e360b7aad48dfd"></a>
## into_string

`function` · `sqlparser::ast::value::Value::into_string` · sqlparser 0.62.0

```rust
fn into_string(self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [253, 2], "filename": "src/ast/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/value.rs:219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the underlying literal is a string, regardless of quote style, returns the associated string value

<a id="op-6f66b9fc7dd3e7d8a10d0372"></a>
## partial_cmp

`function` · `sqlparser::ast::value::Value::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Value) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 35], "end": [136, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/value.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-128c9df3c321b046f64dc092"></a>
## serialize

`function` · `sqlparser::ast::value::Value::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 38], "end": [137, 47], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/value.rs:137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47eeb6e7be08e04ea783d6b6"></a>
## visit

`function` · `sqlparser::ast::value::Value::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 47], "end": [138, 55], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/value.rs:138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b39defa8acfe57b6e87d53c3"></a>
## visit

`function` · `sqlparser::ast::value::Value::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 40], "end": [138, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/value.rs:138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-debb7e074f0dadbf3d8c1278"></a>
## with_empty_span

`function` · `sqlparser::ast::value::Value::with_empty_span` · sqlparser 0.62.0

```rust
fn with_empty_span(self) -> ValueWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [253, 2], "filename": "src/ast/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/value.rs:250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Convenience for attaching an empty span to this `Value`.

<a id="op-a2202cde2dcabd7d374f0878"></a>
## with_span

`function` · `sqlparser::ast::value::Value::with_span` · sqlparser 0.62.0

```rust
fn with_span(self, span: Span) -> ValueWithSpan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::Value", "path": "Value"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [253, 2], "filename": "src/ast/value.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/value.rs:245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attach the provided `span` to this `Value` and return `ValueWithSpan`.
