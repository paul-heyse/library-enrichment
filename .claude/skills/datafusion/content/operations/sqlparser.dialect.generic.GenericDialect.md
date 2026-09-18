# `sqlparser::dialect::generic::GenericDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.generic.GenericDialect.json).

<a id="op-e9e98e2bf43b55c23c54c302"></a>
## GenericDialect

`struct` · `sqlparser::dialect::generic::GenericDialect` · sqlparser 0.62.0

```rust
struct GenericDialect
```

Source: `src/dialect/generic.rs:24`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A permissive, general purpose [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b), which parses a wide variety of SQL
statements, from many different dialects.

<a id="op-a19b936442e5d0b0ae779bfa"></a>
## allow_extract_custom

`function` · `sqlparser::dialect::generic::GenericDialect::allow_extract_custom` · sqlparser 0.62.0

```rust
fn allow_extract_custom(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5be6228dc5651f0d8029e1c7"></a>
## allow_extract_single_quotes

`function` · `sqlparser::dialect::generic::GenericDialect::allow_extract_single_quotes` · sqlparser 0.62.0

```rust
fn allow_extract_single_quotes(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54f115dca1217d372dcb907b"></a>
## clone

`function` · `sqlparser::dialect::generic::GenericDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GenericDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 26], "end": [22, 31], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/generic.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae464f6c17cf29525e857c00"></a>
## cmp

`function` · `sqlparser::dialect::generic::GenericDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GenericDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 72], "end": [22, 75], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/generic.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96db8b4fc5ca4012cc5b16c3"></a>
## default

`function` · `sqlparser::dialect::generic::GenericDialect::default` · sqlparser 0.62.0

```rust
fn default() -> GenericDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 17], "end": [22, 24], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/generic.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12453296589043538095bc29"></a>
## deserialize

`function` · `sqlparser::dialect::generic::GenericDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 56], "end": [23, 74], "filename": "src/dialect/generic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/generic.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c2abe2f410bee109c35155e"></a>
## eq

`function` · `sqlparser::dialect::generic::GenericDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GenericDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 39], "end": [22, 48], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/generic.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff9a2e43eba0b8f5f6153e2a"></a>
## fmt

`function` · `sqlparser::dialect::generic::GenericDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 15], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/generic.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ecebd4215c988b54f5411a4"></a>
## hash

`function` · `sqlparser::dialect::generic::GenericDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 54], "end": [22, 58], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/generic.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fbc4b8c16b56944f32a08f1"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::generic::GenericDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:27`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc320589d0591e77f8f0d00b"></a>
## is_identifier_part

`function` · `sqlparser::dialect::generic::GenericDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd43a394d5a6aa1708a61ffe"></a>
## is_identifier_start

`function` · `sqlparser::dialect::generic::GenericDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6415bb8ac061742ff1afa0a0"></a>
## partial_cmp

`function` · `sqlparser::dialect::generic::GenericDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GenericDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 60], "end": [22, 70], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/generic.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a47443a3c3a0ab6114b51b72"></a>
## serialize

`function` · `sqlparser::dialect::generic::GenericDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 38], "end": [23, 54], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/generic.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04337720072d00d83689d732"></a>
## support_map_literal_syntax

`function` · `sqlparser::dialect::generic::GenericDialect::support_map_literal_syntax` · sqlparser 0.62.0

```rust
fn support_map_literal_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73a2a8a785685cca4e5b4f7a"></a>
## supports_array_join_syntax

`function` · `sqlparser::dialect::generic::GenericDialect::supports_array_join_syntax` · sqlparser 0.62.0

```rust
fn supports_array_join_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:52`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7312a4720cdba454cf1d1a85"></a>
## supports_array_typedef_with_brackets

`function` · `sqlparser::dialect::generic::GenericDialect::supports_array_typedef_with_brackets` · sqlparser 0.62.0

```rust
fn supports_array_typedef_with_brackets(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5094b91738299a14dba9108"></a>
## supports_asc_desc_in_column_definition

`function` · `sqlparser::dialect::generic::GenericDialect::supports_asc_desc_in_column_definition` · sqlparser 0.62.0

```rust
fn supports_asc_desc_in_column_definition(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01eb249ed7f2bbd968d1e2ad"></a>
## supports_bitwise_shift_operators

`function` · `sqlparser::dialect::generic::GenericDialect::supports_bitwise_shift_operators` · sqlparser 0.62.0

```rust
fn supports_bitwise_shift_operators(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08aab3c430f2b62a4e211b41"></a>
## supports_comma_separated_set_assignments

`function` · `sqlparser::dialect::generic::GenericDialect::supports_comma_separated_set_assignments` · sqlparser 0.62.0

```rust
fn supports_comma_separated_set_assignments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7f2034ab509bb790996004f"></a>
## supports_comma_separated_trim

`function` · `sqlparser::dialect::generic::GenericDialect::supports_comma_separated_trim` · sqlparser 0.62.0

```rust
fn supports_comma_separated_trim(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e642419b66ba863357a73dea"></a>
## supports_comment_on

`function` · `sqlparser::dialect::generic::GenericDialect::supports_comment_on` · sqlparser 0.62.0

```rust
fn supports_comment_on(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ee0f8195683c25836fcb65e"></a>
## supports_comment_optimizer_hint

`function` · `sqlparser::dialect::generic::GenericDialect::supports_comment_optimizer_hint` · sqlparser 0.62.0

```rust
fn supports_comment_optimizer_hint(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eba7b82552c2ca738c30198b"></a>
## supports_connect_by

`function` · `sqlparser::dialect::generic::GenericDialect::supports_connect_by` · sqlparser 0.62.0

```rust
fn supports_connect_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:68`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c8667841200d66adae9f6f6"></a>
## supports_constraint_keyword_without_name

`function` · `sqlparser::dialect::generic::GenericDialect::supports_constraint_keyword_without_name` · sqlparser 0.62.0

```rust
fn supports_constraint_keyword_without_name(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24eaca480ec4b9c26022a82d"></a>
## supports_create_index_with_clause

`function` · `sqlparser::dialect::generic::GenericDialect::supports_create_index_with_clause` · sqlparser 0.62.0

```rust
fn supports_create_index_with_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ea8476b1527b9a0be624c55"></a>
## supports_create_view_comment_syntax

`function` · `sqlparser::dialect::generic::GenericDialect::supports_create_view_comment_syntax` · sqlparser 0.62.0

```rust
fn supports_create_view_comment_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2280cdeb9f0a82ea7ac0468"></a>
## supports_cte_without_as

`function` · `sqlparser::dialect::generic::GenericDialect::supports_cte_without_as` · sqlparser 0.62.0

```rust
fn supports_cte_without_as(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c63330ad8058e9cec587d2b1"></a>
## supports_data_type_signed_suffix

`function` · `sqlparser::dialect::generic::GenericDialect::supports_data_type_signed_suffix` · sqlparser 0.62.0

```rust
fn supports_data_type_signed_suffix(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6c7753db8194508ecb767fb"></a>
## supports_detach

`function` · `sqlparser::dialect::generic::GenericDialect::supports_detach` · sqlparser 0.62.0

```rust
fn supports_detach(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae808f71d9f96cbde4e16b2a"></a>
## supports_dictionary_syntax

`function` · `sqlparser::dialect::generic::GenericDialect::supports_dictionary_syntax` · sqlparser 0.62.0

```rust
fn supports_dictionary_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b4ca67d91c5e90518bcd608"></a>
## supports_empty_projections

`function` · `sqlparser::dialect::generic::GenericDialect::supports_empty_projections` · sqlparser 0.62.0

```rust
fn supports_empty_projections(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dac17a8a25dc4d323fa63836"></a>
## supports_explain_with_utility_options

`function` · `sqlparser::dialect::generic::GenericDialect::supports_explain_with_utility_options` · sqlparser 0.62.0

```rust
fn supports_explain_with_utility_options(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37feb5f38dc9b1d679c86624"></a>
## supports_extract_comma_syntax

`function` · `sqlparser::dialect::generic::GenericDialect::supports_extract_comma_syntax` · sqlparser 0.62.0

```rust
fn supports_extract_comma_syntax(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95250b10a81108ebef7cf6a5"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::generic::GenericDialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0c0d8d75dcd72ead1ff7b8d"></a>
## supports_from_first_select

`function` · `sqlparser::dialect::generic::GenericDialect::supports_from_first_select` · sqlparser 0.62.0

```rust
fn supports_from_first_select(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75a6f55611125c38ce9190f3"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::generic::GenericDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04101c225c0d1d359101fc6c"></a>
## supports_group_by_with_modifier

`function` · `sqlparser::dialect::generic::GenericDialect::supports_group_by_with_modifier` · sqlparser 0.62.0

```rust
fn supports_group_by_with_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-369966332512d25f8b161719"></a>
## supports_install

`function` · `sqlparser::dialect::generic::GenericDialect::supports_install` · sqlparser 0.62.0

```rust
fn supports_install(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57d4493bb0f0b975b5f215be"></a>
## supports_interpolate

`function` · `sqlparser::dialect::generic::GenericDialect::supports_interpolate` · sqlparser 0.62.0

```rust
fn supports_interpolate(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:276`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55464659879f8fbe1fa7d3fa"></a>
## supports_interval_options

`function` · `sqlparser::dialect::generic::GenericDialect::supports_interval_options` · sqlparser 0.62.0

```rust
fn supports_interval_options(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5b5b2a2bc9b405cd3cf6d83"></a>
## supports_key_column_option

`function` · `sqlparser::dialect::generic::GenericDialect::supports_key_column_option` · sqlparser 0.62.0

```rust
fn supports_key_column_option(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:296`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af816627e0be451b39d0a306"></a>
## supports_left_associative_joins_without_parens

`function` · `sqlparser::dialect::generic::GenericDialect::supports_left_associative_joins_without_parens` · sqlparser 0.62.0

```rust
fn supports_left_associative_joins_without_parens(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa331140e1cdc394247809c8"></a>
## supports_limit_by

`function` · `sqlparser::dialect::generic::GenericDialect::supports_limit_by` · sqlparser 0.62.0

```rust
fn supports_limit_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6060a7e29840bbf57f37e2f5"></a>
## supports_limit_comma

`function` · `sqlparser::dialect::generic::GenericDialect::supports_limit_comma` · sqlparser 0.62.0

```rust
fn supports_limit_comma(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:140`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9ea64ac23025a809a4c0898"></a>
## supports_load_extension

`function` · `sqlparser::dialect::generic::GenericDialect::supports_load_extension` · sqlparser 0.62.0

```rust
fn supports_load_extension(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d256f6a83516b6910ffbf51"></a>
## supports_match_against

`function` · `sqlparser::dialect::generic::GenericDialect::supports_match_against` · sqlparser 0.62.0

```rust
fn supports_match_against(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-723856ad17816f9b32c21f46"></a>
## supports_match_recognize

`function` · `sqlparser::dialect::generic::GenericDialect::supports_match_recognize` · sqlparser 0.62.0

```rust
fn supports_match_recognize(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e58b66d685c5ef23ffdaa2e"></a>
## supports_multiline_comment_hints

`function` · `sqlparser::dialect::generic::GenericDialect::supports_multiline_comment_hints` · sqlparser 0.62.0

```rust
fn supports_multiline_comment_hints(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76274aad7a2718111fb09a45"></a>
## supports_named_fn_args_with_assignment_operator

`function` · `sqlparser::dialect::generic::GenericDialect::supports_named_fn_args_with_assignment_operator` · sqlparser 0.62.0

```rust
fn supports_named_fn_args_with_assignment_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec23a265a2ed4040b8f62f4d"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::generic::GenericDialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-feda1ee73d99de6d6f52d81d"></a>
## supports_optimize_table

`function` · `sqlparser::dialect::generic::GenericDialect::supports_optimize_table` · sqlparser 0.62.0

```rust
fn supports_optimize_table(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:252`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afafa612aed9d20ec7cb4325"></a>
## supports_parens_around_table_factor

`function` · `sqlparser::dialect::generic::GenericDialect::supports_parens_around_table_factor` · sqlparser 0.62.0

```rust
fn supports_parens_around_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-425398b18276326fb3a43045"></a>
## supports_parenthesized_set_variables

`function` · `sqlparser::dialect::generic::GenericDialect::supports_parenthesized_set_variables` · sqlparser 0.62.0

```rust
fn supports_parenthesized_set_variables(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22a764954f19534841e82791"></a>
## supports_partition_by_after_order_by

`function` · `sqlparser::dialect::generic::GenericDialect::supports_partition_by_after_order_by` · sqlparser 0.62.0

```rust
fn supports_partition_by_after_order_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a73b00711a926800ffb5eedc"></a>
## supports_pipe_operator

`function` · `sqlparser::dialect::generic::GenericDialect::supports_pipe_operator` · sqlparser 0.62.0

```rust
fn supports_pipe_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:76`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70b77710aff80a04a8855fbe"></a>
## supports_prewhere

`function` · `sqlparser::dialect::generic::GenericDialect::supports_prewhere` · sqlparser 0.62.0

```rust
fn supports_prewhere(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4e45d597886f6dbea83de38"></a>
## supports_projection_trailing_commas

`function` · `sqlparser::dialect::generic::GenericDialect::supports_projection_trailing_commas` · sqlparser 0.62.0

```rust
fn supports_projection_trailing_commas(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e95368ddd3bc440de6269d7"></a>
## supports_quote_delimited_string

`function` · `sqlparser::dialect::generic::GenericDialect::supports_quote_delimited_string` · sqlparser 0.62.0

```rust
fn supports_quote_delimited_string(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57f6b46d78132d51b769e803"></a>
## supports_select_format

`function` · `sqlparser::dialect::generic::GenericDialect::supports_select_format` · sqlparser 0.62.0

```rust
fn supports_select_format(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a97496e5c91cdca976777a2a"></a>
## supports_select_item_multi_column_alias

`function` · `sqlparser::dialect::generic::GenericDialect::supports_select_item_multi_column_alias` · sqlparser 0.62.0

```rust
fn supports_select_item_multi_column_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-339a16b9173f30947db3c023"></a>
## supports_select_wildcard_except

`function` · `sqlparser::dialect::generic::GenericDialect::supports_select_wildcard_except` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_except(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:100`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8da63502426467421dbb41de"></a>
## supports_select_wildcard_exclude

`function` · `sqlparser::dialect::generic::GenericDialect::supports_select_wildcard_exclude` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_exclude(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:224`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68e24d38e5e4ece31c494a33"></a>
## supports_select_wildcard_ilike

`function` · `sqlparser::dialect::generic::GenericDialect::supports_select_wildcard_ilike` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_ilike(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25b95bd7550495b50762f9ea"></a>
## supports_select_wildcard_rename

`function` · `sqlparser::dialect::generic::GenericDialect::supports_select_wildcard_rename` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_rename(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f440e444da6cd1b74ad8fea"></a>
## supports_select_wildcard_replace

`function` · `sqlparser::dialect::generic::GenericDialect::supports_select_wildcard_replace` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_replace(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:240`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2233b3420d97d68a11160d58"></a>
## supports_set_names

`function` · `sqlparser::dialect::generic::GenericDialect::supports_set_names` · sqlparser 0.62.0

```rust
fn supports_set_names(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c29ef8034591954dc2a7769d"></a>
## supports_settings

`function` · `sqlparser::dialect::generic::GenericDialect::supports_settings` · sqlparser 0.62.0

```rust
fn supports_settings(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-612a6c983d2428b6df6eb4b6"></a>
## supports_start_transaction_modifier

`function` · `sqlparser::dialect::generic::GenericDialect::supports_start_transaction_modifier` · sqlparser 0.62.0

```rust
fn supports_start_transaction_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-424bd28ff2b54a7b5669ee8c"></a>
## supports_string_escape_constant

`function` · `sqlparser::dialect::generic::GenericDialect::supports_string_escape_constant` · sqlparser 0.62.0

```rust
fn supports_string_escape_constant(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9685b6f6e2fdb20423236a44"></a>
## supports_struct_literal

`function` · `sqlparser::dialect::generic::GenericDialect::supports_struct_literal` · sqlparser 0.62.0

```rust
fn supports_struct_literal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f9c669a50e28e703315b035"></a>
## supports_try_convert

`function` · `sqlparser::dialect::generic::GenericDialect::supports_try_convert` · sqlparser 0.62.0

```rust
fn supports_try_convert(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ee3b552a962e9da6b7bf89b"></a>
## supports_unicode_string_literal

`function` · `sqlparser::dialect::generic::GenericDialect::supports_unicode_string_literal` · sqlparser 0.62.0

```rust
fn supports_unicode_string_literal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5560d3f3deed2643762abc9"></a>
## supports_update_order_by

`function` · `sqlparser::dialect::generic::GenericDialect::supports_update_order_by` · sqlparser 0.62.0

```rust
fn supports_update_order_by(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cff6a59172a12b09a1ff4b2"></a>
## supports_user_host_grantee

`function` · `sqlparser::dialect::generic::GenericDialect::supports_user_host_grantee` · sqlparser 0.62.0

```rust
fn supports_user_host_grantee(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0471fd514b6730e59da18f1"></a>
## supports_values_as_table_factor

`function` · `sqlparser::dialect::generic::GenericDialect::supports_values_as_table_factor` · sqlparser 0.62.0

```rust
fn supports_values_as_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5225d27338ee5d8bcb72b684"></a>
## supports_window_clause_named_window_reference

`function` · `sqlparser::dialect::generic::GenericDialect::supports_window_clause_named_window_reference` · sqlparser 0.62.0

```rust
fn supports_window_clause_named_window_reference(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:92`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f69f63a57128ce2f2e770aad"></a>
## supports_window_function_null_treatment_arg

`function` · `sqlparser::dialect::generic::GenericDialect::supports_window_function_null_treatment_arg` · sqlparser 0.62.0

```rust
fn supports_window_function_null_treatment_arg(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5da059d5eca260423b6f97e"></a>
## supports_with_fill

`function` · `sqlparser::dialect::generic::GenericDialect::supports_with_fill` · sqlparser 0.62.0

```rust
fn supports_with_fill(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f9324662b0eff6d942e4e39"></a>
## supports_xml_expressions

`function` · `sqlparser::dialect::generic::GenericDialect::supports_xml_expressions` · sqlparser 0.62.0

```rust
fn supports_xml_expressions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::generic::GenericDialect", "path": "GenericDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [315, 2], "filename": "src/dialect/generic.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/generic.rs:312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
