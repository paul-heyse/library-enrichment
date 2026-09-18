# `arrow_ipc::writer::DictionaryHandling`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.writer.DictionaryHandling.json).

<a id="op-0d1a5683db89e04b47a91db9"></a>
## DictionaryHandling

`enum` · `arrow_ipc::writer::DictionaryHandling` · arrow-ipc 59.3.0

```rust
enum DictionaryHandling
```

Source: `src/writer.rs:1330`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Controls how dictionaries are handled in Arrow IPC messages

<a id="op-d91a44f52f08e1a08726a551"></a>
## Delta

`variant` · `arrow_ipc::writer::DictionaryHandling::Delta` · arrow-ipc 59.3.0

```rust
Delta
```

Source: `src/writer.rs:1339`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Send only new dictionary values since the last batch (delta encoding)

When a dictionary is first encountered, the entire dictionary is sent.
For subsequent batches, only values that are new (not previously sent)
are transmitted with the `isDelta` flag set to true.

<a id="op-3cf14935a65a060adc661351"></a>
## Resend

`variant` · `arrow_ipc::writer::DictionaryHandling::Resend` · arrow-ipc 59.3.0

```rust
Resend
```

Source: `src/writer.rs:1333`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Send the entire dictionary every time it is encountered (default)

<a id="op-7c0ff67b796c24f9269fdd38"></a>
## clone

`function` · `arrow_ipc::writer::DictionaryHandling::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> DictionaryHandling
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryHandling", "path": "DictionaryHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1329, 17], "end": [1329, 22], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/writer.rs:1329`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cb42d775c4bc9f1baf6fee9"></a>
## default

`function` · `arrow_ipc::writer::DictionaryHandling::default` · arrow-ipc 59.3.0

```rust
fn default() -> DictionaryHandling
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryHandling", "path": "DictionaryHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1329, 45], "end": [1329, 52], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/writer.rs:1329`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f291edbf02f15f65283ea38"></a>
## eq

`function` · `arrow_ipc::writer::DictionaryHandling::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &DictionaryHandling) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryHandling", "path": "DictionaryHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1329, 30], "end": [1329, 39], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/writer.rs:1329`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7dc898dc4935271424fa343"></a>
## fmt

`function` · `arrow_ipc::writer::DictionaryHandling::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_ipc::writer::DictionaryHandling", "path": "DictionaryHandling"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1329, 10], "end": [1329, 15], "filename": "src/writer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/writer.rs:1329`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
