# `object_store::RenameTargetMode`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.RenameTargetMode.json).

<a id="op-fc30f2bd3edb0f54097fba9b"></a>
## RenameTargetMode

`enum` · `object_store::RenameTargetMode` · object_store 0.13.2

```rust
enum RenameTargetMode
```

Source: `src/lib.rs:1950`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure preconditions for the target of rename operation.

Note though that the source location may or not be deleted at the same time in an atomic operation. There is
currently NO flag to control the atomicity of "delete source at the same time as creating the target".

<a id="op-87c3f92ad597674af4973fd0"></a>
## Create

`variant` · `object_store::RenameTargetMode::Create` · object_store 0.13.2

```rust
Create
```

Source: `src/lib.rs:1956`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform an atomic write operation of the target, returning [`Error::AlreadyExists`](../operations/object_store.Error.md#op-aa9eb7b8d711d881e621142c) if an
object already exists at the provided path.

<a id="op-b971b405af3ca2a6654323d8"></a>
## Overwrite

`variant` · `object_store::RenameTargetMode::Overwrite` · object_store 0.13.2

```rust
Overwrite
```

Source: `src/lib.rs:1953`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform a write operation on the target, overwriting any object present at the provided path.

<a id="op-26099f8c8ed4d0086aa6cff7"></a>
## clone

`function` · `object_store::RenameTargetMode::clone` · object_store 0.13.2

```rust
fn clone(&self) -> RenameTargetMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameTargetMode", "path": "RenameTargetMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1949, 17], "end": [1949, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1949`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22af3bba911ab0c84c992731"></a>
## default

`function` · `object_store::RenameTargetMode::default` · object_store 0.13.2

```rust
fn default() -> RenameTargetMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameTargetMode", "path": "RenameTargetMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1949, 45], "end": [1949, 52], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1949`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-891e9ac54321d298062e4342"></a>
## eq

`function` · `object_store::RenameTargetMode::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &RenameTargetMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameTargetMode", "path": "RenameTargetMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1949, 30], "end": [1949, 39], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1949`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6aea5b17e17edaec07dfd27b"></a>
## fmt

`function` · `object_store::RenameTargetMode::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameTargetMode", "path": "RenameTargetMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1949, 10], "end": [1949, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1949`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
