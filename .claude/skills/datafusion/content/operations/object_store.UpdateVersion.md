# `object_store::UpdateVersion`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.UpdateVersion.json).

<a id="op-2dd35a5349d63ec0f7adeebb"></a>
## UpdateVersion

`struct` · `object_store::UpdateVersion` · object_store 0.13.2

```rust
struct UpdateVersion
```

Source: `src/lib.rs:1719`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Uniquely identifies a version of an object to update

Stores will use differing combinations of `e_tag` and `version` to provide conditional
updates, and it is therefore recommended applications preserve both

<a id="op-4de1388574f737e16d1e961a"></a>
## clone

`function` · `object_store::UpdateVersion::clone` · object_store 0.13.2

```rust
fn clone(&self) -> UpdateVersion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::UpdateVersion", "path": "UpdateVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1718, 17], "end": [1718, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1718`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-383e253d2a7b9ce8a1ed5003"></a>
## e_tag

`struct_field` · `object_store::UpdateVersion::e_tag` · object_store 0.13.2

```rust
e_tag: Option<String>
```

Source: `src/lib.rs:1723`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The unique identifier for the newly created object

<https://datatracker.ietf.org/doc/html/rfc9110#name-etag>

<a id="op-c13b96e8472bf9facea077d5"></a>
## eq

`function` · `object_store::UpdateVersion::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &UpdateVersion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::UpdateVersion", "path": "UpdateVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1718, 24], "end": [1718, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1718`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dba5d2980339cb7393681a7f"></a>
## fmt

`function` · `object_store::UpdateVersion::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::UpdateVersion", "path": "UpdateVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1718, 10], "end": [1718, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1718`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5eea8dbd1ebae702a5a5f1a3"></a>
## from

`function` · `object_store::UpdateVersion::from` · object_store 0.13.2

```rust
fn from(value: PutResult) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::UpdateVersion", "path": "UpdateVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1728, 1], "end": [1735, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::PutResult", "path": "PutResult"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:1729`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32b4feebe53aa504f428c028"></a>
## version

`struct_field` · `object_store::UpdateVersion::version` · object_store 0.13.2

```rust
version: Option<String>
```

Source: `src/lib.rs:1725`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A version indicator for the newly created object
