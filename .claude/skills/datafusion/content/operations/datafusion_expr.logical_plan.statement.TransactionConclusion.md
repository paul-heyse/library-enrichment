# `datafusion_expr::logical_plan::statement::TransactionConclusion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.TransactionConclusion.json).

<a id="op-15f5d78672b8c02483c2a685"></a>
## TransactionConclusion

`enum` · `datafusion_expr::logical_plan::statement::TransactionConclusion` · datafusion-expr 55.1.0

```rust
enum TransactionConclusion
```

Source: `src/logical_plan/statement.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates if a transaction was committed or aborted

<a id="op-4987bef1b3428ad77abde1b2"></a>
## Commit

`variant` · `datafusion_expr::logical_plan::statement::TransactionConclusion::Commit` · datafusion-expr 55.1.0

```rust
Commit
```

Source: `src/logical_plan/statement.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a73b37b7f340143106ca604"></a>
## Rollback

`variant` · `datafusion_expr::logical_plan::statement::TransactionConclusion::Rollback` · datafusion-expr 55.1.0

```rust
Rollback
```

Source: `src/logical_plan/statement.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce74bbf96c992a10790e0603"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::TransactionConclusion::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TransactionConclusion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionConclusion", "path": "TransactionConclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 10], "end": [149, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72d030f6526c8b0ced3d1668"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::TransactionConclusion::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TransactionConclusion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionConclusion", "path": "TransactionConclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 17], "end": [149, 26], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fddd7279d55c5c1b5c074fa"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::TransactionConclusion::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionConclusion", "path": "TransactionConclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 50], "end": [149, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c5f288cfcf1d0b074176471"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::TransactionConclusion::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionConclusion", "path": "TransactionConclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 44], "end": [149, 48], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed1b88c2c5522c61afaa06ab"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::TransactionConclusion::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &TransactionConclusion) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionConclusion", "path": "TransactionConclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 32], "end": [149, 42], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
