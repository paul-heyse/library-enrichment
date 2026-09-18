# `datafusion_expr::logical_plan::statement::TransactionIsolationLevel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.TransactionIsolationLevel.json).

<a id="op-3d634b3ee60a79770af37c3c"></a>
## TransactionIsolationLevel

`enum` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel` · datafusion-expr 55.1.0

```rust
enum TransactionIsolationLevel
```

Source: `src/logical_plan/statement.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicates ANSI transaction isolation level

<a id="op-49233e07b43f8ec4162fda42"></a>
## ReadCommitted

`variant` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::ReadCommitted` · datafusion-expr 55.1.0

```rust
ReadCommitted
```

Source: `src/logical_plan/statement.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e354bf30fae481a0b91832f"></a>
## ReadUncommitted

`variant` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::ReadUncommitted` · datafusion-expr 55.1.0

```rust
ReadUncommitted
```

Source: `src/logical_plan/statement.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a824a1665b8a9ed16ff80d7f"></a>
## RepeatableRead

`variant` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::RepeatableRead` · datafusion-expr 55.1.0

```rust
RepeatableRead
```

Source: `src/logical_plan/statement.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a38f08b941e1440f7780891"></a>
## Serializable

`variant` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::Serializable` · datafusion-expr 55.1.0

```rust
Serializable
```

Source: `src/logical_plan/statement.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60541af47480512673a96f40"></a>
## Snapshot

`variant` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::Snapshot` · datafusion-expr 55.1.0

```rust
Snapshot
```

Source: `src/logical_plan/statement.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fb8c345011be5beb7c813bf"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TransactionIsolationLevel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 10], "end": [163, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-131a17fdfe345ed6e21d1988"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TransactionIsolationLevel) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 17], "end": [163, 26], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8605104d4aaf33e47a05da3"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 50], "end": [163, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a049b336238782d0d6ce1d29"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 44], "end": [163, 48], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbc96978a76c8ddcda039d1e"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::TransactionIsolationLevel::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &TransactionIsolationLevel) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionIsolationLevel", "path": "TransactionIsolationLevel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 32], "end": [163, 42], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
