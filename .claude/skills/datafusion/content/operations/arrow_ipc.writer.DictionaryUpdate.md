# `arrow_ipc::writer::DictionaryUpdate`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.DictionaryUpdate.json).

<a id="op-185c5cfa9bdf75582a5c26d0"></a>
## DictionaryUpdate

`enum` · `arrow_ipc::writer::DictionaryUpdate` · arrow-ipc 59.3.0

```rust
enum DictionaryUpdate
```

Source: `src/writer.rs:1344`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Describes what kind of update took place after a call to [`DictionaryTracker::insert`](../operations/arrow_ipc.writer.DictionaryTracker.md#op-6b1b645edb897ad0cd1fb3fd).

<a id="op-28d012fcf9c62910b1ee37c6"></a>
## Delta

`variant` · `arrow_ipc::writer::DictionaryUpdate::Delta` · arrow-ipc 59.3.0

```rust
Delta
```

Source: `src/writer.rs:1353`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Dictionary was updated, ArrayData is the delta between old and new

<a id="op-0ecbf141895490427d1b2754"></a>
## New

`variant` · `arrow_ipc::writer::DictionaryUpdate::New` · arrow-ipc 59.3.0

```rust
New
```

Source: `src/writer.rs:1349`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No dictionary was present in the tracker

<a id="op-62ad6c8b756e076dbbcb3412"></a>
## None

`variant` · `arrow_ipc::writer::DictionaryUpdate::None` · arrow-ipc 59.3.0

```rust
None
```

Source: `src/writer.rs:1347`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No dictionary was written, the dictionary was identical to what was already
in the tracker.

<a id="op-6369a0d7e54ec0b9fd69bade"></a>
## Replaced

`variant` · `arrow_ipc::writer::DictionaryUpdate::Replaced` · arrow-ipc 59.3.0

```rust
Replaced
```

Source: `src/writer.rs:1351`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Dictionary was replaced with the new data

<a id="op-74fe6c6bb530730187286f60"></a>
## clone

`function` · `arrow_ipc::writer::DictionaryUpdate::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> DictionaryUpdate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryUpdate", "path": "DictionaryUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 17], "end": [1343, 22], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer.rs:1343`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eebb8a8dfba3f57706b88d49"></a>
## fmt

`function` · `arrow_ipc::writer::DictionaryUpdate::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryUpdate", "path": "DictionaryUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1343, 10], "end": [1343, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:1343`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
