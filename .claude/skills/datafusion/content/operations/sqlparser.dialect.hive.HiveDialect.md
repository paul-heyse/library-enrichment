# `sqlparser::dialect::hive::HiveDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.hive.HiveDialect.json).

<a id="op-2a14a2afa59282e124d348be"></a>
## HiveDialect

`struct` · `sqlparser::dialect::hive::HiveDialect` · sqlparser 0.62.0

```rust
struct HiveDialect
```

Source: `src/dialect/hive.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Hive](https://hive.apache.org/).

<a id="op-e359bb2fab50d3d35078c9f1"></a>
## clone

`function` · `sqlparser::dialect::hive::HiveDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 26], "end": [21, 31], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/hive.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8041d09c4a661a618d62cb8"></a>
## cmp

`function` · `sqlparser::dialect::hive::HiveDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 72], "end": [21, 75], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/hive.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f459152fe17c108b63b7b87"></a>
## default

`function` · `sqlparser::dialect::hive::HiveDialect::default` · sqlparser 0.62.0

```rust
fn default() -> HiveDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 17], "end": [21, 24], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/hive.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bb1db7b1303addca55213ec"></a>
## deserialize

`function` · `sqlparser::dialect::hive::HiveDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 56], "end": [22, 74], "filename": "src/dialect/hive.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/hive.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9d81959d34de35ee6a3807e"></a>
## eq

`function` · `sqlparser::dialect::hive::HiveDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 39], "end": [21, 48], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/hive.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82ae6069b5367f645fe85ea7"></a>
## fmt

`function` · `sqlparser::dialect::hive::HiveDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 10], "end": [21, 15], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/hive.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d4a5904b2acd889f29b679c"></a>
## hash

`function` · `sqlparser::dialect::hive::HiveDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 54], "end": [21, 58], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/hive.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08c9d9ee7f4598aaf46eb49c"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::hive::HiveDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:26`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73de48f9d09d41e9ab2893e4"></a>
## is_identifier_part

`function` · `sqlparser::dialect::hive::HiveDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31f58841c004892d22f683d3"></a>
## is_identifier_start

`function` · `sqlparser::dialect::hive::HiveDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df07fc5d0e4d21976dbc1d30"></a>
## partial_cmp

`function` · `sqlparser::dialect::hive::HiveDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [21, 60], "end": [21, 70], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/hive.rs:21`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1677841d6297d8e0572f10e4"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::hive::HiveDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:52`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a27943225c61a0bce9a4ca4"></a>
## serialize

`function` · `sqlparser::dialect::hive::HiveDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 38], "end": [22, 54], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/hive.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b71cfb8eed2fb1d6b558f7f3"></a>
## supports_bang_not_operator

`function` · `sqlparser::dialect::hive::HiveDialect::supports_bang_not_operator` · sqlparser 0.62.0

```rust
fn supports_bang_not_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362061#Tutorial-BuiltInOperators>

<a id="op-685274910fcd09b9172b2155"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::hive::HiveDialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a40b68e740f95fc5b6cd4ddd"></a>
## supports_from_first_insert

`function` · `sqlparser::dialect::hive::HiveDialect::supports_from_first_insert` · sqlparser 0.62.0

```rust
fn supports_from_first_insert(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://hive.apache.org/docs/latest/language/common-table-expression/>

<a id="op-c25ea7128d5239d9d9e83f9b"></a>
## supports_group_by_with_modifier

`function` · `sqlparser::dialect::hive::HiveDialect::supports_group_by_with_modifier` · sqlparser 0.62.0

```rust
fn supports_group_by_with_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=30151323#EnhancedAggregation,Cube,GroupingandRollup-CubesandRollupsr>

<a id="op-97abc1cf8454a03a5facfdb8"></a>
## supports_load_data

`function` · `sqlparser::dialect::hive::HiveDialect::supports_load_data` · sqlparser 0.62.0

```rust
fn supports_load_data(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362036#LanguageManualDML-Loadingfilesintotables>

<a id="op-a5c1cebd42640053628c101b"></a>
## supports_numeric_prefix

`function` · `sqlparser::dialect::hive::HiveDialect::supports_numeric_prefix` · sqlparser 0.62.0

```rust
fn supports_numeric_prefix(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea38cb9f718f34158e63f276"></a>
## supports_table_sample_before_alias

`function` · `sqlparser::dialect::hive::HiveDialect::supports_table_sample_before_alias` · sqlparser 0.62.0

```rust
fn supports_table_sample_before_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::hive::HiveDialect", "path": "HiveDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [82, 2], "filename": "src/dialect/hive.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/hive.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://cwiki.apache.org/confluence/display/hive/languagemanual+sampling>
