# `parquet_variant::builder::object::ObjectState`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.object.ObjectState.json).

<a id="op-1e6e5e4d58b4a531e24e085a"></a>
## ObjectState

`struct` · `parquet_variant::builder::object::ObjectState` · parquet-variant 59.3.0

```rust
struct ObjectState<'a>
```

Source: `src/builder/object.rs:342`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Internal state for object building

<a id="op-f06f2ceb74fdb6ea7e2976a3"></a>
## fmt

`function` · `parquet_variant::builder::object::ObjectState::fmt` · parquet-variant 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectState", "path": "ObjectState"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 10], "end": [341, 15], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/object.rs:341`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-779a8068c05c2824969ca2ed"></a>
## rollback

`function` · `parquet_variant::builder::object::ObjectState::rollback` · parquet-variant 59.3.0

```rust
fn rollback(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "parquet_variant::builder::object::ObjectState", "path": "ObjectState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 1], "end": [352, 2], "filename": "src/builder/object.rs"}, "trait": {"args": null, "id": "parquet_variant::builder::BuilderSpecificState", "path": "BuilderSpecificState"}, "trait_path": "parquet_variant::builder::BuilderSpecificState"}`

Source: `src/builder/object.rs:349`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
