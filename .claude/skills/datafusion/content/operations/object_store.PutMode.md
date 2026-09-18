# `object_store::PutMode`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.PutMode.json).

<a id="op-6d1a652a18d92397a0c86baf"></a>
## PutMode

`enum` · `object_store::PutMode` · object_store 0.13.2

```rust
enum PutMode
```

Source: `src/lib.rs:1702`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure preconditions for the put operation

<a id="op-4f5db36c087aea67ef6a720d"></a>
## Create

`variant` · `object_store::PutMode::Create` · object_store 0.13.2

```rust
Create
```

Source: `src/lib.rs:1708`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform an atomic write operation, returning [`Error::AlreadyExists`](../operations/object_store.Error.md#op-aa9eb7b8d711d881e621142c) if an
object already exists at the provided path

<a id="op-323b100687e6ba36403a82cc"></a>
## Overwrite

`variant` · `object_store::PutMode::Overwrite` · object_store 0.13.2

```rust
Overwrite
```

Source: `src/lib.rs:1705`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform an atomic write operation, overwriting any object present at the provided path

<a id="op-70734561c8e87e7a3096b1d1"></a>
## Update

`variant` · `object_store::PutMode::Update` · object_store 0.13.2

```rust
Update
```

Source: `src/lib.rs:1711`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Perform an atomic write operation if the current version of the object matches the
provided [`UpdateVersion`](../operations/object_store.UpdateVersion.md#op-2dd35a5349d63ec0f7adeebb), returning [`Error::Precondition`](../operations/object_store.Error.md#op-530afde8313e7ef1611c8b0a) otherwise

<a id="op-bd0cc23c22431777bdb2dc6b"></a>
## clone

`function` · `object_store::PutMode::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PutMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMode", "path": "PutMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1701, 17], "end": [1701, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1701`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58e47577c3dd736c5aeefcde"></a>
## default

`function` · `object_store::PutMode::default` · object_store 0.13.2

```rust
fn default() -> PutMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMode", "path": "PutMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1701, 39], "end": [1701, 46], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1701`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95c356e37b4928e408c9f8c7"></a>
## eq

`function` · `object_store::PutMode::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &PutMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMode", "path": "PutMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1701, 24], "end": [1701, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1701`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f967c5d4a54c61e797eeb289"></a>
## fmt

`function` · `object_store::PutMode::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMode", "path": "PutMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1701, 10], "end": [1701, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1701`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
