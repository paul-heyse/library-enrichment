# `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.stmt_data_loading.StageParamsObject.json).

<a id="op-92d55df86955bcf07fb69251"></a>
## StageParamsObject

`struct` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject` · sqlparser 0.62.0

```rust
struct StageParamsObject
```

Source: `src/ast/helpers/stmt_data_loading.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parameters for a named stage object used in data loading/unloading.

<a id="op-15b119acf19b3ec9c5f47499"></a>
## clone

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StageParamsObject
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 22], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/stmt_data_loading.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ea0fdd0a50ff37f02478523"></a>
## cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StageParamsObject) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 51], "end": [34, 54], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/stmt_data_loading.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc2a61f3956693ec8d1ddbac"></a>
## credentials

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::credentials` · sqlparser 0.62.0

```rust
credentials: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/helpers/stmt_data_loading.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Credentials for accessing the stage.

<a id="op-fba66d41f9196d774f800f92"></a>
## deserialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 49], "end": [35, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03532aa6fb8cdc854625ebd0"></a>
## encryption

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::encryption` · sqlparser 0.62.0

```rust
encryption: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/helpers/stmt_data_loading.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Encryption-related key/value options.

<a id="op-2142cab66d8c4a6ea1eb0e91"></a>
## endpoint

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::endpoint` · sqlparser 0.62.0

```rust
endpoint: Option<String>
```

Source: `src/ast/helpers/stmt_data_loading.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional endpoint string.

<a id="op-6daf7c21dc3a32f7535bc5ad"></a>
## eq

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StageParamsObject) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 24], "end": [34, 33], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/stmt_data_loading.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d2ee03a239415106be9b4a8"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [111, 2], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/helpers/stmt_data_loading.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1c94c133cdb93f0ee21f73f"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/stmt_data_loading.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c4d474dbf391bd6b027634a"></a>
## hash

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 56], "end": [34, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/stmt_data_loading.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d35dc85d6fb18df1635a0297"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StageParamsObject) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 39], "end": [34, 49], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/stmt_data_loading.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d26b65a7463275526dae7c1b"></a>
## serialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 38], "end": [35, 47], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84365a2ad403c668ef13ec1e"></a>
## storage_integration

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::storage_integration` · sqlparser 0.62.0

```rust
storage_integration: Option<String>
```

Source: `src/ast/helpers/stmt_data_loading.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage integration identifier.

<a id="op-daecefee1f7b0a0d260972e0"></a>
## url

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::url` · sqlparser 0.62.0

```rust
url: Option<String>
```

Source: `src/ast/helpers/stmt_data_loading.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional URL for the stage.

<a id="op-a93b55aab96fcfe30fa8f829"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 40], "end": [36, 45], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/stmt_data_loading.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9825c0bd9b2b305170d3b88"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageParamsObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageParamsObject", "path": "StageParamsObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 47], "end": [36, 55], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/stmt_data_loading.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
