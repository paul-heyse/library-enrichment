# `object_store::ListResult`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.ListResult.json).

<a id="op-f000e58500759e875fa8908e"></a>
## ListResult

`struct` · `object_store::ListResult` · object_store 0.13.2

```rust
struct ListResult
```

Source: `src/lib.rs:1411`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Result of a list call that includes objects, prefixes (directories) and a
token for the next set of results. Individual result sets may be limited to
1,000 objects based on the underlying object storage's limitations.

<a id="op-3c7415799cf5bdb2b1401501"></a>
## common_prefixes

`struct_field` · `object_store::ListResult::common_prefixes` · object_store 0.13.2

```rust
common_prefixes: Vec<path::Path>
```

Source: `src/lib.rs:1413`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Prefixes that are common (like directories)

<a id="op-a4c323a4fa71a950760bd4b0"></a>
## fmt

`function` · `object_store::ListResult::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::ListResult", "path": "ListResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 10], "end": [1410, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1410`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eed114f1cf4172a52267d3f0"></a>
## objects

`struct_field` · `object_store::ListResult::objects` · object_store 0.13.2

```rust
objects: Vec<ObjectMeta>
```

Source: `src/lib.rs:1415`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Object metadata for the listing
