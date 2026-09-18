# `parquet_variant::builder::list::ListState`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.list.ListState.json).

<a id="op-895f73e6d091682a722e491f"></a>
## ListState

`struct` · `parquet_variant::builder::list::ListState` · parquet-variant 59.3.0

```rust
struct ListState<'a>
```

Source: `src/builder/list.rs:233`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Internal state for list building

<a id="op-76267dd41981687e96b46594"></a>
## fmt

`function` · `parquet_variant::builder::list::ListState::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::builder::list::ListState", "path": "ListState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 10], "end": [232, 15], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/list.rs:232`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b878557a2be8d70726395501"></a>
## rollback

`function` · `parquet_variant::builder::list::ListState::rollback` · parquet-variant 59.3.0

```rust
fn rollback(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::builder::list::ListState", "path": "ListState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [243, 2], "filename": "src/builder/list.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}, "trait_path": "parquet_variant::builder::BuilderSpecificState"}`

Source: `src/builder/list.rs:240`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
