# `datafusion_expr::logical_plan::statement::TransactionAccessMode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.TransactionAccessMode.json).

<a id="op-366082b792d9fd13b6d8d227"></a>
## TransactionAccessMode

`enum` · `datafusion_expr::logical_plan::statement::TransactionAccessMode` · datafusion-expr 55.1.0

```rust
enum TransactionAccessMode
```

Source: `src/logical_plan/statement.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates if this transaction is allowed to write

<a id="op-560b98c46de1c6f1ced1dafc"></a>
## ReadOnly

`variant` · `datafusion_expr::logical_plan::statement::TransactionAccessMode::ReadOnly` · datafusion-expr 55.1.0

```rust
ReadOnly
```

Source: `src/logical_plan/statement.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-248d831bd1b131a19a448e03"></a>
## ReadWrite

`variant` · `datafusion_expr::logical_plan::statement::TransactionAccessMode::ReadWrite` · datafusion-expr 55.1.0

```rust
ReadWrite
```

Source: `src/logical_plan/statement.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe801b9ba33fe7ac3327a6f3"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::TransactionAccessMode::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TransactionAccessMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 10], "end": [156, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06076ea53ebdea79c3f17b2e"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::TransactionAccessMode::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TransactionAccessMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 17], "end": [156, 26], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b82088f8d33bba0742c3128"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::TransactionAccessMode::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 50], "end": [156, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ffb6a52a29980c0fee7e644"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::TransactionAccessMode::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 44], "end": [156, 48], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20f5b404d79fc25962b58257"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::TransactionAccessMode::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &TransactionAccessMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionAccessMode", "path": "TransactionAccessMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 32], "end": [156, 42], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
