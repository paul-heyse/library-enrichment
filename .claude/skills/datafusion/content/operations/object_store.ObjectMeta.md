# `object_store::ObjectMeta`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.ObjectMeta.json).

<a id="op-84641755fb7ee613fe92d518"></a>
## ObjectMeta

`struct` · `object_store::ObjectMeta` · object_store 0.13.2

```rust
struct ObjectMeta
```

Source: `src/lib.rs:1420`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The metadata that describes an object.

<a id="op-4e7bd2c8554ea053ba0c8ae1"></a>
## clone

`function` · `object_store::ObjectMeta::clone` · object_store 0.13.2

```rust
fn clone(&self) -> ObjectMeta
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::ObjectMeta", "path": "ObjectMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 17], "end": [1419, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1419`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a70e4320ada32dc25584f111"></a>
## e_tag

`struct_field` · `object_store::ObjectMeta::e_tag` · object_store 0.13.2

```rust
e_tag: Option<String>
```

Source: `src/lib.rs:1432`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The unique identifier for the object

<https://datatracker.ietf.org/doc/html/rfc9110#name-etag>

<a id="op-f42f4da16cb81e51025fb47f"></a>
## eq

`function` · `object_store::ObjectMeta::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &ObjectMeta) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::ObjectMeta", "path": "ObjectMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 24], "end": [1419, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1419`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74d4fe02823a5807dcaee9b2"></a>
## fmt

`function` · `object_store::ObjectMeta::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::ObjectMeta", "path": "ObjectMeta"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1419, 10], "end": [1419, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1419`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b900d252287f5df98ae7985"></a>
## last_modified

`struct_field` · `object_store::ObjectMeta::last_modified` · object_store 0.13.2

```rust
last_modified: chrono::DateTime<chrono::Utc>
```

Source: `src/lib.rs:1424`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The last modified time

<a id="op-5ab93ebccaace96515ec795d"></a>
## location

`struct_field` · `object_store::ObjectMeta::location` · object_store 0.13.2

```rust
location: path::Path
```

Source: `src/lib.rs:1422`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The full path to the object

<a id="op-af70233788b833d3e3c779c9"></a>
## size

`struct_field` · `object_store::ObjectMeta::size` · object_store 0.13.2

```rust
size: u64
```

Source: `src/lib.rs:1428`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The size in bytes of the object.

Note this is not `usize` as `object_store` supports 32-bit architectures such as WASM

<a id="op-5a84f7075b76b5709e3fb494"></a>
## version

`struct_field` · `object_store::ObjectMeta::version` · object_store 0.13.2

```rust
version: Option<String>
```

Source: `src/lib.rs:1434`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A version indicator for this object
