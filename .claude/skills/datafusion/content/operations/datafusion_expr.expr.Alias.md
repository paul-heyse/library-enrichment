# `datafusion_expr::expr::Alias`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.Alias.json).

<a id="op-fb00e68bc21f4d90c6eed853"></a>
## Alias

`struct` · `datafusion_expr::expr::Alias` · datafusion-expr 55.1.0

```rust
struct Alias
```

Source: `src/expr.rs:716`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Alias expression

<a id="op-764c36cd3b843c58008d91fb"></a>
## clone

`function` · `datafusion_expr::expr::Alias::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Alias
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Alias", "path": "Alias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 10], "end": [715, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:715`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-448bee531bf553881f5ce0af"></a>
## eq

`function` · `datafusion_expr::expr::Alias::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Alias) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Alias", "path": "Alias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 17], "end": [715, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:715`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd4ee9f254f7271039689591"></a>
## expr

`struct_field` · `datafusion_expr::expr::Alias::expr` · datafusion-expr 55.1.0

```rust
expr: Box<Expr>
```

Source: `src/expr.rs:717`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02fc11dc36986b25614d59d6"></a>
## fmt

`function` · `datafusion_expr::expr::Alias::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Alias", "path": "Alias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [715, 32], "end": [715, 37], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:715`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0806dc8f0d5bec677a47ac3"></a>
## hash

`function` · `datafusion_expr::expr::Alias::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Alias", "path": "Alias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 1], "end": [729, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:724`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bac9aa59612f09c0d37ef74c"></a>
## metadata

`struct_field` · `datafusion_expr::expr::Alias::metadata` · datafusion-expr 55.1.0

```rust
metadata: Option<FieldMetadata>
```

Source: `src/expr.rs:720`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64df121c5c13ced03a772caf"></a>
## name

`struct_field` · `datafusion_expr::expr::Alias::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/expr.rs:719`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-845b9c50edfb49d4fa51a011"></a>
## new

`function` · `datafusion_expr::expr::Alias::new` · datafusion-expr 55.1.0

```rust
fn new(expr: Expr, relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Alias", "path": "Alias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 1], "end": [789, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:750`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create an alias with an optional schema/field qualifier.

<a id="op-dec4d313cdff9a1d8bdc2b83"></a>
## partial_cmp

`function` · `datafusion_expr::expr::Alias::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Alias", "path": "Alias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [731, 1], "end": [746, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:732`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01188b4743e2692e7daeed25"></a>
## relation

`struct_field` · `datafusion_expr::expr::Alias::relation` · datafusion-expr 55.1.0

```rust
relation: Option<datafusion_common::TableReference>
```

Source: `src/expr.rs:718`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-934f81d548d465ed56cc18de"></a>
## with_metadata

`function` · `datafusion_expr::expr::Alias::with_metadata` · datafusion-expr 55.1.0

```rust
fn with_metadata(self, metadata: Option<FieldMetadata>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::Alias", "path": "Alias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [748, 1], "end": [789, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:763`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
