# `sqlparser::ast::BeginEndStatements`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.BeginEndStatements.json).

<a id="op-4a9f177f63ebc5b12ce09172"></a>
## BeginEndStatements

`struct` · `sqlparser::ast::BeginEndStatements` · sqlparser 0.62.0

```rust
struct BeginEndStatements
```

Source: `src/ast/mod.rs:2839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a list of statements enclosed within `BEGIN` and `END` keywords.
Example:
```sql
BEGIN
    SELECT 1;
    SELECT 2;
END
```

<a id="op-2bbb8de0219e590e56194ea0"></a>
## begin_token

`struct_field` · `sqlparser::ast::BeginEndStatements::begin_token` · sqlparser 0.62.0

```rust
begin_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:2841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token representing the `BEGIN` keyword (may include span info).

<a id="op-90b4242f8df88902cd5b42ba"></a>
## clone

`function` · `sqlparser::ast::BeginEndStatements::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> BeginEndStatements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2836, 17], "end": [2836, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d98f9106c902e8d03e84563e"></a>
## cmp

`function` · `sqlparser::ast::BeginEndStatements::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &BeginEndStatements) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2836, 51], "end": [2836, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3465ae17ddb93b17dc03db95"></a>
## deserialize

`function` · `sqlparser::ast::BeginEndStatements::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2837, 49], "end": [2837, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-265ccefe5294762efee0ce53"></a>
## end_token

`struct_field` · `sqlparser::ast::BeginEndStatements::end_token` · sqlparser 0.62.0

```rust
end_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:2845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token representing the `END` keyword (may include span info).

<a id="op-b27a0c461c04581a512b02a8"></a>
## eq

`function` · `sqlparser::ast::BeginEndStatements::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &BeginEndStatements) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2836, 24], "end": [2836, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d4694bc456dd9c9ee707c53"></a>
## fmt

`function` · `sqlparser::ast::BeginEndStatements::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2836, 10], "end": [2836, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7ea10363cfa30e0ea61f2d7"></a>
## fmt

`function` · `sqlparser::ast::BeginEndStatements::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2848, 1], "end": [2867, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bcb59a82c1f8a000d9a8e35"></a>
## hash

`function` · `sqlparser::ast::BeginEndStatements::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2836, 56], "end": [2836, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53900e6f1562d20de1dd6b2e"></a>
## partial_cmp

`function` · `sqlparser::ast::BeginEndStatements::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &BeginEndStatements) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2836, 35], "end": [2836, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eef81c788ac11442f9727e2a"></a>
## serialize

`function` · `sqlparser::ast::BeginEndStatements::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2837, 38], "end": [2837, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23ef1a8348941c8907d52208"></a>
## span

`function` · `sqlparser::ast::BeginEndStatements::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "super::BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2417, 1], "end": [2430, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61f3ca3922ff0a596add7886"></a>
## statements

`struct_field` · `sqlparser::ast::BeginEndStatements::statements` · sqlparser 0.62.0

```rust
statements: Vec<Statement>
```

Source: `src/ast/mod.rs:2843`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Statements contained within the block.

<a id="op-6896f7122a7380e9af6e223a"></a>
## visit

`function` · `sqlparser::ast::BeginEndStatements::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2838, 47], "end": [2838, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2838`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b189447e01489c5b42b145b6"></a>
## visit

`function` · `sqlparser::ast::BeginEndStatements::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::BeginEndStatements", "path": "BeginEndStatements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2838, 40], "end": [2838, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2838`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
