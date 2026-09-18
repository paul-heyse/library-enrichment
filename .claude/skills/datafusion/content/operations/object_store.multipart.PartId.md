# `object_store::multipart::PartId`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.multipart.PartId.json).

<a id="op-7d9f763a7604a6bb02e2bddf"></a>
## PartId

`struct` · `object_store::multipart::PartId` · object_store 0.13.2

```rust
struct PartId
```

Source: `src/multipart.rs:31`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Represents a part of a file that has been successfully uploaded in a multipart upload process.

<a id="op-80413e9139b82624aae4352b"></a>
## clone

`function` · `object_store::multipart::PartId::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PartId
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::multipart::PartId", "path": "PartId"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 22], "filename": "src/multipart.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/multipart.rs:30`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff145b2f416c9c4a6d115730"></a>
## content_id

`struct_field` · `object_store::multipart::PartId::content_id` · object_store 0.13.2

```rust
content_id: String
```

Source: `src/multipart.rs:33`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Id of this part

<a id="op-5a39f9a1cf1a734d1e397ed4"></a>
## fmt

`function` · `object_store::multipart::PartId::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::multipart::PartId", "path": "PartId"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/multipart.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/multipart.rs:30`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
