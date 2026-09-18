# `object_store::CopyMode`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.CopyMode.json).

<a id="op-5e9ca7dad569b06b1370d098"></a>
## CopyMode

`enum` · `object_store::CopyMode` · object_store 0.13.2

```rust
enum CopyMode
```

Source: `src/lib.rs:1880`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure preconditions for the copy operation

<a id="op-c5accc46eb090edf710fde22"></a>
## Create

`variant` · `object_store::CopyMode::Create` · object_store 0.13.2

```rust
Create
```

Source: `src/lib.rs:1886`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform an atomic write operation, returning [`Error::AlreadyExists`](../operations/object_store.Error.md#op-aa9eb7b8d711d881e621142c) if an
object already exists at the provided path

<a id="op-9939caed182218c2d9a60d36"></a>
## Overwrite

`variant` · `object_store::CopyMode::Overwrite` · object_store 0.13.2

```rust
Overwrite
```

Source: `src/lib.rs:1883`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform an atomic write operation, overwriting any object present at the provided path

<a id="op-0a7fad53f467e5cde0b56356"></a>
## clone

`function` · `object_store::CopyMode::clone` · object_store 0.13.2

```rust
fn clone(&self) -> CopyMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyMode", "path": "CopyMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1879, 17], "end": [1879, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1879`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-431b5d7e611430462200b2ac"></a>
## default

`function` · `object_store::CopyMode::default` · object_store 0.13.2

```rust
fn default() -> CopyMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyMode", "path": "CopyMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1879, 45], "end": [1879, 52], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1879`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d3fbe0ad8b932a3eaa99a56"></a>
## eq

`function` · `object_store::CopyMode::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &CopyMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyMode", "path": "CopyMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1879, 30], "end": [1879, 39], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1879`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0651e8d29f121035e5d18224"></a>
## fmt

`function` · `object_store::CopyMode::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyMode", "path": "CopyMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1879, 10], "end": [1879, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1879`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
