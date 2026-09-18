# `object_store::list::PaginatedListResult`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.list.PaginatedListResult.json).

<a id="op-2b5d515754a7a733a318b0c7"></a>
## PaginatedListResult

`struct` · `object_store::list::PaginatedListResult` · object_store 0.13.2

```rust
struct PaginatedListResult
```

Source: `src/list.rs:59`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A [`ListResult`](../operations/object_store.ListResult.md#op-f000e58500759e875fa8908e) with optional pagination token

<a id="op-fc29e536ce4142437de120a5"></a>
## fmt

`function` · `object_store::list::PaginatedListResult::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::list::PaginatedListResult", "path": "PaginatedListResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/list.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/list.rs:58`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0442d4a51a74a314b16eb52"></a>
## page_token

`struct_field` · `object_store::list::PaginatedListResult::page_token` · object_store 0.13.2

```rust
page_token: Option<String>
```

Source: `src/list.rs:63`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If result set truncated, the pagination token to fetch next results

<a id="op-5640e0d6bccfac67ccea686b"></a>
## result

`struct_field` · `object_store::list::PaginatedListResult::result` · object_store 0.13.2

```rust
result: ListResult
```

Source: `src/list.rs:61`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The list result
