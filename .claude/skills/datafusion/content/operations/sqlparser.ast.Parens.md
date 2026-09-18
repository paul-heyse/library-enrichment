# `sqlparser::ast::Parens`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Parens.json).

<a id="op-239ce1f60d19c7b290877cb1"></a>
## Parens

`struct` · `sqlparser::ast::Parens` · sqlparser 0.62.0

```rust
struct Parens<T>
```

Source: `src/ast/mod.rs:207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A item `T` enclosed in a pair of parentheses

<a id="op-54a61c426d3b05db58c83291"></a>
## Target

`assoc_type` · `sqlparser::ast::Parens::Target` · sqlparser 0.62.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [234, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/mod.rs:229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0deca6c84df335ee2b93592f"></a>
## clone

`function` · `sqlparser::ast::Parens::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Parens<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 17], "end": [204, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-876ece3da4eaff74545103df"></a>
## closing_token

`struct_field` · `sqlparser::ast::Parens::closing_token` · sqlparser 0.62.0

```rust
closing_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:213`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the closing parenthesis token, i.e. `)`

<a id="op-6023b9fafe90d150055680d8"></a>
## cmp

`function` · `sqlparser::ast::Parens::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Parens<T>) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Ord", "path": "$crate::cmp::Ord"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 51], "end": [204, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00a79ce2ba920683af0d5146"></a>
## content

`struct_field` · `sqlparser::ast::Parens::content` · sqlparser 0.62.0

```rust
content: T
```

Source: `src/ast/mod.rs:211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

content enclosed in parentheses

<a id="op-d965958a1a8f10cb85794d57"></a>
## deref

`function` · `sqlparser::ast::Parens::deref` · sqlparser 0.62.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 1], "end": [234, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ast/mod.rs:231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f36f9f469ea89568aa2e1eb6"></a>
## deref_mut

`function` · `sqlparser::ast::Parens::deref_mut` · sqlparser 0.62.0

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [236, 1], "end": [240, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::ops::deref::DerefMut", "path": "DerefMut"}, "trait_path": "core::ops::deref::DerefMut"}`

Source: `src/ast/mod.rs:237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-352d63c7e19ffc4527559570"></a>
## deserialize

`function` · `sqlparser::ast::Parens::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "_serde::Deserialize"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [205, 49], "end": [205, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f022345a517a48823d9b7c17"></a>
## eq

`function` · `sqlparser::ast::Parens::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Parens<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 24], "end": [204, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49bbc44df29533c33d42d1ef"></a>
## fmt

`function` · `sqlparser::ast::Parens::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 10], "end": [204, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b6fb78f1a3a747add2bdf5d"></a>
## hash

`function` · `sqlparser::ast::Parens::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "$crate::hash::Hash"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 56], "end": [204, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb270aa4e58f2b208a3710fb"></a>
## opening_token

`struct_field` · `sqlparser::ast::Parens::opening_token` · sqlparser 0.62.0

```rust
opening_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/mod.rs:209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the opening parenthesis token, i.e. `(`

<a id="op-b887cebdb09a6ac1cb28d7d4"></a>
## partial_cmp

`function` · `sqlparser::ast::Parens::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Parens<T>) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "$crate::cmp::PartialOrd"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 35], "end": [204, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-451b147b9d9a7ed3d7ae7fe2"></a>
## serialize

`function` · `sqlparser::ast::Parens::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "_serde::Serialize"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [205, 38], "end": [205, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fa054a7412d35b017c7d28e"></a>
## span

`function` · `sqlparser::ast::Parens::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "super::Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [113, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a93be3d70d3a1644a308cf15"></a>
## visit

`function` · `sqlparser::ast::Parens::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "sqlparser::ast::Visit"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 40], "end": [206, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c86f9c89071bfaec34c8fae7"></a>
## visit

`function` · `sqlparser::ast::Parens::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "sqlparser::ast::VisitMut"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 47], "end": [206, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0b475e9a7ccef2730945b86"></a>
## with_empty_span

`function` · `sqlparser::ast::Parens::with_empty_span` · sqlparser 0.62.0

```rust
fn with_empty_span(content: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "sqlparser::ast::Parens", "path": "Parens"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [216, 1], "end": [226, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Constructor wrapping `content` into `Parens` with an empty span;
useful for testing purposes.
