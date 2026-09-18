# `sqlparser::ast::data_type::ArrayElemTypeDef`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.ArrayElemTypeDef.json).

<a id="op-5e415d650d1dd718df9c1c03"></a>
## ArrayElemTypeDef

`enum` · `sqlparser::ast::data_type::ArrayElemTypeDef` · sqlparser 0.62.0

```rust
enum ArrayElemTypeDef
```

Source: `src/ast/data_type.rs:1140`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the data type of the elements in an array (if any) as well as
the syntax used to declare the array.

For example: Bigquery/Hive use `ARRAY<INT>` whereas snowflake uses ARRAY.

<a id="op-185f948934a735feb58b23f8"></a>
## AngleBracket

`variant` · `sqlparser::ast::data_type::ArrayElemTypeDef::AngleBracket` · sqlparser 0.62.0

```rust
AngleBracket
```

Source: `src/ast/data_type.rs:1144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Angle-bracket style, e.g. `ARRAY<INT>`.

<a id="op-6325ba8e40cd0db8689cdfc0"></a>
## None

`variant` · `sqlparser::ast::data_type::ArrayElemTypeDef::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/data_type.rs:1142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use `ARRAY` style without an explicit element type.

<a id="op-97425f78a40b80ee10bd79f3"></a>
## Parenthesis

`variant` · `sqlparser::ast::data_type::ArrayElemTypeDef::Parenthesis` · sqlparser 0.62.0

```rust
Parenthesis
```

Source: `src/ast/data_type.rs:1148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parenthesis style, e.g. `Array(Int64)`.

<a id="op-113f219a659abf3f9d64e96e"></a>
## SquareBracket

`variant` · `sqlparser::ast::data_type::ArrayElemTypeDef::SquareBracket` · sqlparser 0.62.0

```rust
SquareBracket
```

Source: `src/ast/data_type.rs:1146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Square-bracket style, e.g. `INT[]` or `INT[2]`.

<a id="op-6b762977a1fe0aca4da53745"></a>
## clone

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ArrayElemTypeDef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 17], "end": [1137, 22], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27dab1ad48c93ee7ee1846ff"></a>
## cmp

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ArrayElemTypeDef) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 51], "end": [1137, 54], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62d86ac730d4a688b0fbc58d"></a>
## deserialize

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1138, 49], "end": [1138, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:1138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27593a87a56a9c3f773ed75f"></a>
## eq

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ArrayElemTypeDef) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 24], "end": [1137, 33], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07ddf996ccd92ffe92dead8a"></a>
## fmt

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 10], "end": [1137, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34de6f2f55a230ed8ddbad81"></a>
## hash

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 56], "end": [1137, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-584066bccbb3c3e492a88158"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ArrayElemTypeDef) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 35], "end": [1137, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:1137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a797359cfe33fc2087e07bf"></a>
## serialize

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1138, 38], "end": [1138, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:1138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91dab52a30b8d1b8d21c0d7e"></a>
## visit

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1139, 40], "end": [1139, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:1139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edc87e744673204b55c0eab1"></a>
## visit

`function` · `sqlparser::ast::data_type::ArrayElemTypeDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ArrayElemTypeDef", "path": "ArrayElemTypeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1139, 47], "end": [1139, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:1139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
