# `object_store::GetResultPayload`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.GetResultPayload.json).

<a id="op-f0c3bc293967f4345e687d48"></a>
## GetResultPayload

`enum` · `object_store::GetResultPayload` · object_store 0.13.2

```rust
enum GetResultPayload
```

Source: `src/lib.rs:1636`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The kind of a [`GetResult`](../operations/object_store.GetResult.md#op-5bbf376d09191747a49aacb9)

This special cases the case of a local file, as some systems may
be able to optimise the case of a file already present on local disk

<a id="op-0211051432dee3f6644c79df"></a>
## File

`variant` · `object_store::GetResultPayload::File` · object_store 0.13.2

```rust
File
```

Source: `src/lib.rs:1639`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The file, path

<a id="op-2872e38543a2098c316a9a32"></a>
## Stream

`variant` · `object_store::GetResultPayload::Stream` · object_store 0.13.2

```rust
Stream
```

Source: `src/lib.rs:1641`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An opaque stream of bytes

<a id="op-e6898e7ddef0a6ae14719048"></a>
## fmt

`function` · `object_store::GetResultPayload::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetResultPayload", "path": "GetResultPayload"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1644, 1], "end": [1652, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1645`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
