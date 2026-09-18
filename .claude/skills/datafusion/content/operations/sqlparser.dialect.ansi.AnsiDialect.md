# `sqlparser::dialect::ansi::AnsiDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.ansi.AnsiDialect.json).

<a id="op-3fca1ca08ee71d5760477ca2"></a>
## AnsiDialect

`struct` · `sqlparser::dialect::ansi::AnsiDialect` · sqlparser 0.62.0

```rust
struct AnsiDialect
```

Source: `src/dialect/ansi.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [ANSI SQL](https://en.wikipedia.org/wiki/SQL:2011).

<a id="op-6dd6241ded096399a7a8489f"></a>
## clone

`function` · `sqlparser::dialect::ansi::AnsiDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AnsiDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 26], "end": [21, 31], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/ansi.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-504cefa7fc795886ebb2def8"></a>
## cmp

`function` · `sqlparser::dialect::ansi::AnsiDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AnsiDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 72], "end": [21, 75], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/ansi.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c27d0f4a6e52bc50f00d02f5"></a>
## default

`function` · `sqlparser::dialect::ansi::AnsiDialect::default` · sqlparser 0.62.0

```rust
fn default() -> AnsiDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 17], "end": [21, 24], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/ansi.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd6ce0cc6786ba6b5322dc8e"></a>
## deserialize

`function` · `sqlparser::dialect::ansi::AnsiDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 56], "end": [22, 74], "filename": "src/dialect/ansi.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/ansi.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc692d6cb0165b997c5aea2a"></a>
## eq

`function` · `sqlparser::dialect::ansi::AnsiDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AnsiDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 39], "end": [21, 48], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/ansi.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d83e483f4db1c0c5e1eb2d2c"></a>
## fmt

`function` · `sqlparser::dialect::ansi::AnsiDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/ansi.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75a2c38c9702eb9ae204178a"></a>
## hash

`function` · `sqlparser::dialect::ansi::AnsiDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 54], "end": [21, 58], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/ansi.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d22bfac1f4db4e5bfe6b4f"></a>
## is_identifier_part

`function` · `sqlparser::dialect::ansi::AnsiDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [42, 2], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/ansi.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcca7ae315c6b18ded36e153"></a>
## is_identifier_start

`function` · `sqlparser::dialect::ansi::AnsiDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [42, 2], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/ansi.rs:26`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1fe4170db2a6e7289545721"></a>
## partial_cmp

`function` · `sqlparser::dialect::ansi::AnsiDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AnsiDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 60], "end": [21, 70], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/ansi.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fdf0545a6f4b1a56f29e9f5"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::ansi::AnsiDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [42, 2], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/ansi.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6615b61bd380b56116117e33"></a>
## serialize

`function` · `sqlparser::dialect::ansi::AnsiDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/ansi.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-483b681197aa40e0eb151dcb"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::ansi::AnsiDialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::ansi::AnsiDialect", "path": "AnsiDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [42, 2], "filename": "src/dialect/ansi.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/ansi.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The SQL standard explicitly states that block comments nest.
