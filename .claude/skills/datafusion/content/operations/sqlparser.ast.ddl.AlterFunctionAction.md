# `sqlparser::ast::ddl::AlterFunctionAction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterFunctionAction.json).

<a id="op-3566a38cf2db8c0d108cf1cd"></a>
## AlterFunctionAction

`enum` · `sqlparser::ast::ddl::AlterFunctionAction` · sqlparser 0.62.0

```rust
enum AlterFunctionAction
```

Source: `src/ast/ddl.rs:5440`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function action in `ALTER FUNCTION ... action [ ... ] [ RESTRICT ]`.

<a id="op-eac86fb09103d75deec940aa"></a>
## Behavior

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Behavior` · sqlparser 0.62.0

```rust
Behavior
```

Source: `src/ast/ddl.rs:5444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IMMUTABLE` / `STABLE` / `VOLATILE`

<a id="op-5bf67d0fb7f8e90a0c723bf5"></a>
## CalledOnNull

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::CalledOnNull` · sqlparser 0.62.0

```rust
CalledOnNull
```

Source: `src/ast/ddl.rs:5442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CALLED ON NULL INPUT` / `RETURNS NULL ON NULL INPUT` / `STRICT`

<a id="op-c485ad15ef10ad0d4e5f69d5"></a>
## Cost

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Cost` · sqlparser 0.62.0

```rust
Cost
```

Source: `src/ast/ddl.rs:5457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`COST execution_cost`

<a id="op-35c0a13e02c5a7d410b99bab"></a>
## Leakproof

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Leakproof` · sqlparser 0.62.0

```rust
Leakproof
```

Source: `src/ast/ddl.rs:5446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ NOT ] LEAKPROOF`

<a id="op-6d00204f88778a7ed82b85e8"></a>
## Parallel

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Parallel` · sqlparser 0.62.0

```rust
Parallel
```

Source: `src/ast/ddl.rs:5455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PARALLEL { UNSAFE | RESTRICTED | SAFE }`

<a id="op-601bbf574d4e289e05258508"></a>
## Reset

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Reset` · sqlparser 0.62.0

```rust
Reset
```

Source: `src/ast/ddl.rs:5466`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RESET configuration_parameter` or `RESET ALL`

<a id="op-ab63756bde9de47a6d578d5e"></a>
## Rows

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Rows` · sqlparser 0.62.0

```rust
Rows
```

Source: `src/ast/ddl.rs:5459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROWS result_rows`

<a id="op-a062c7e51e877ee49b74248c"></a>
## Security

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Security` · sqlparser 0.62.0

```rust
Security
```

Source: `src/ast/ddl.rs:5448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ EXTERNAL ] SECURITY { DEFINER | INVOKER }`

<a id="op-21e7cc386fe7c4515c842bbc"></a>
## Set

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Set` · sqlparser 0.62.0

```rust
Set
```

Source: `src/ast/ddl.rs:5464`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET configuration_parameter { TO | = } { value | DEFAULT }`
or `SET configuration_parameter FROM CURRENT`

<a id="op-07bc81afed14290896080602"></a>
## Support

`variant` · `sqlparser::ast::ddl::AlterFunctionAction::Support` · sqlparser 0.62.0

```rust
Support
```

Source: `src/ast/ddl.rs:5461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SUPPORT support_function`

<a id="op-6e18c1a9792920e3a26a37f1"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterFunctionAction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterFunctionAction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5437, 17], "end": [5437, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb12380c03bc9e51130e58a4"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterFunctionAction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterFunctionAction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5437, 51], "end": [5437, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-655c4d4773721192bd1d5f1d"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterFunctionAction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5438, 49], "end": [5438, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fb630478a0522421ca1642a"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterFunctionAction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterFunctionAction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5437, 24], "end": [5437, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24b901206693b6609d96402c"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunctionAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5531, 1], "end": [5562, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5532`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fef5b0bbb5c080fd834ec0c2"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunctionAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5437, 10], "end": [5437, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc4e26d60a4976f2a14027d2"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterFunctionAction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5437, 56], "end": [5437, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da344732cf161c26b5840196"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterFunctionAction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterFunctionAction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5437, 35], "end": [5437, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d1ef43f9aedf711d87d998c"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterFunctionAction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5438, 38], "end": [5438, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10f49a1b1b01047896178bb3"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunctionAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5439, 47], "end": [5439, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eec17e8414c7c77a5194b18f"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunctionAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunctionAction", "path": "AlterFunctionAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5439, 40], "end": [5439, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
