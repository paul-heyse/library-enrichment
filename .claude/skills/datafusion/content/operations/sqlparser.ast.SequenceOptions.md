# `sqlparser::ast::SequenceOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SequenceOptions.json).

<a id="op-7f8839b99192af843eed378c"></a>
## SequenceOptions

`enum` · `sqlparser::ast::SequenceOptions` · sqlparser 0.62.0

```rust
enum SequenceOptions
```

Source: `src/ast/mod.rs:6402`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Can use to describe options in create sequence or table column type identity
```sql
[ INCREMENT [ BY ] increment ]
    [ MINVALUE minvalue | NO MINVALUE ] [ MAXVALUE maxvalue | NO MAXVALUE ]
    [ START [ WITH ] start ] [ CACHE cache ] [ [ NO ] CYCLE ]
```

<a id="op-d59efc91842620525b921263"></a>
## Cache

`variant` · `sqlparser::ast::SequenceOptions::Cache` · sqlparser 0.62.0

```rust
Cache
```

Source: `src/ast/mod.rs:6412`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CACHE <expr>` option.

<a id="op-d30d4ba8836740fc4c2b17e2"></a>
## Cycle

`variant` · `sqlparser::ast::SequenceOptions::Cycle` · sqlparser 0.62.0

```rust
Cycle
```

Source: `src/ast/mod.rs:6414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CYCLE` or `NO CYCLE` option.

<a id="op-ef869fd2074d872337e28a6e"></a>
## IncrementBy

`variant` · `sqlparser::ast::SequenceOptions::IncrementBy` · sqlparser 0.62.0

```rust
IncrementBy
```

Source: `src/ast/mod.rs:6404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INCREMENT [BY] <expr>` option; second value indicates presence of `BY` keyword.

<a id="op-0c3fbd92ad6ef9b67074c823"></a>
## MaxValue

`variant` · `sqlparser::ast::SequenceOptions::MaxValue` · sqlparser 0.62.0

```rust
MaxValue
```

Source: `src/ast/mod.rs:6408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MAXVALUE <expr>` or `NO MAXVALUE`.

<a id="op-2f958a12f59be429281689cd"></a>
## MinValue

`variant` · `sqlparser::ast::SequenceOptions::MinValue` · sqlparser 0.62.0

```rust
MinValue
```

Source: `src/ast/mod.rs:6406`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MINVALUE <expr>` or `NO MINVALUE`.

<a id="op-ae51c9bdd8dae4eeed6bd326"></a>
## StartWith

`variant` · `sqlparser::ast::SequenceOptions::StartWith` · sqlparser 0.62.0

```rust
StartWith
```

Source: `src/ast/mod.rs:6410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`START [WITH] <expr>`; second value indicates presence of `WITH`.

<a id="op-79ebbc27085d80ed0d93b5bd"></a>
## clone

`function` · `sqlparser::ast::SequenceOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SequenceOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6399, 17], "end": [6399, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cd6534521f35dcb6176a224"></a>
## cmp

`function` · `sqlparser::ast::SequenceOptions::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SequenceOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6399, 51], "end": [6399, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8434f071ff09c7e7d46e7fc2"></a>
## deserialize

`function` · `sqlparser::ast::SequenceOptions::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6400, 49], "end": [6400, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6400`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3009945a916c49fe27de87d2"></a>
## eq

`function` · `sqlparser::ast::SequenceOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SequenceOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6399, 24], "end": [6399, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-528b06a349dc09943d065b8a"></a>
## fmt

`function` · `sqlparser::ast::SequenceOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6417, 1], "end": [6456, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f87de7f365208369555120e8"></a>
## fmt

`function` · `sqlparser::ast::SequenceOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6399, 10], "end": [6399, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cbde4a062888d6c20aa7df5"></a>
## hash

`function` · `sqlparser::ast::SequenceOptions::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6399, 56], "end": [6399, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23f20a1e1051c13cf1acccaf"></a>
## partial_cmp

`function` · `sqlparser::ast::SequenceOptions::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SequenceOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6399, 35], "end": [6399, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4c9b1ff37541c29e5218da7"></a>
## serialize

`function` · `sqlparser::ast::SequenceOptions::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6400, 38], "end": [6400, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6400`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45aa2ccf988605f2051281c7"></a>
## visit

`function` · `sqlparser::ast::SequenceOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6401, 40], "end": [6401, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6401`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c53068d8863158c9948f179e"></a>
## visit

`function` · `sqlparser::ast::SequenceOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SequenceOptions", "path": "SequenceOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6401, 47], "end": [6401, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6401`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
