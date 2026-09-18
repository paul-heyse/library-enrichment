# `datafusion_expr::logical_plan::statement::TransactionEnd`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.TransactionEnd.json).

<a id="op-41e3f3b5c955e0cc1fd4d70f"></a>
## TransactionEnd

`struct` · `datafusion_expr::logical_plan::statement::TransactionEnd` · datafusion-expr 55.1.0

```rust
struct TransactionEnd
```

Source: `src/logical_plan/statement.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Indicator that any current transaction should be terminated

<a id="op-ed955bb6fb5bf7d7b079a330"></a>
## chain

`struct_field` · `datafusion_expr::logical_plan::statement::TransactionEnd::chain` · datafusion-expr 55.1.0

```rust
chain: bool
```

Source: `src/logical_plan/statement.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

if specified a new transaction is immediately started with same characteristics

<a id="op-e938ec74c8f7dd5a6fa9919b"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::TransactionEnd::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TransactionEnd
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionEnd", "path": "TransactionEnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 17], "end": [182, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6a9bb1af1a3e8386e2602a9"></a>
## conclusion

`struct_field` · `datafusion_expr::logical_plan::statement::TransactionEnd::conclusion` · datafusion-expr 55.1.0

```rust
conclusion: TransactionConclusion
```

Source: `src/logical_plan/statement.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

whether the transaction committed or aborted

<a id="op-5d51dcddb01dc08672f3211c"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::TransactionEnd::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &TransactionEnd) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionEnd", "path": "TransactionEnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 24], "end": [182, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-474bb4b8b5402f8b27325605"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::TransactionEnd::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionEnd", "path": "TransactionEnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 10], "end": [182, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d382ce379d9f9da878bb7af5"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::TransactionEnd::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionEnd", "path": "TransactionEnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 51], "end": [182, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5ca58855072d6368ac38247"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::TransactionEnd::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &TransactionEnd) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::TransactionEnd", "path": "TransactionEnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 35], "end": [182, 45], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
