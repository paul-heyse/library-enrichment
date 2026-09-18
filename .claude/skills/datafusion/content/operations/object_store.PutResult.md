# `object_store::PutResult`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.PutResult.json).

<a id="op-4bd57a7c3bcdf5605a33d831"></a>
## PutResult

`struct` · `object_store::PutResult` · object_store 0.13.2

```rust
struct PutResult
```

Source: `src/lib.rs:1869`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Result for a put request

<a id="op-4c22250e2ec3344e6d98f987"></a>
## clone

`function` · `object_store::PutResult::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PutResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1868, 17], "end": [1868, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1868`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f042128f8239249934481b26"></a>
## e_tag

`struct_field` · `object_store::PutResult::e_tag` · object_store 0.13.2

```rust
e_tag: Option<String>
```

Source: `src/lib.rs:1873`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The unique identifier for the newly created object

<https://datatracker.ietf.org/doc/html/rfc9110#name-etag>

<a id="op-01a1f88522e7943e1f9523a7"></a>
## eq

`function` · `object_store::PutResult::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &PutResult) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1868, 24], "end": [1868, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1868`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f12d63597ffdd6ceceb253d3"></a>
## fmt

`function` · `object_store::PutResult::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutResult", "path": "PutResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1868, 10], "end": [1868, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1868`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ad4f2037dd1396dfd2297e3"></a>
## version

`struct_field` · `object_store::PutResult::version` · object_store 0.13.2

```rust
version: Option<String>
```

Source: `src/lib.rs:1875`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A version indicator for the newly created object
