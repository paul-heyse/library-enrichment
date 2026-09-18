# `object_store::GetResult`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.GetResult.json).

<a id="op-5bbf376d09191747a49aacb9"></a>
## GetResult

`struct` · `object_store::GetResult` · object_store 0.13.2

```rust
struct GetResult
```

Source: `src/lib.rs:1619`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Result for a get request

<a id="op-84d9c7bb8a60218c845b7eea"></a>
## attributes

`struct_field` · `object_store::GetResult::attributes` · object_store 0.13.2

```rust
attributes: Attributes
```

Source: `src/lib.rs:1629`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Additional object attributes

<a id="op-33e65ffd589773c3079b4e69"></a>
## bytes

`function` · `object_store::GetResult::bytes` · object_store 0.13.2

```rust
async fn bytes(self) -> Result<Bytes>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetResult", "path": "GetResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1654, 1], "end": [1698, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1656`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Collects the data into a [`Bytes`]

Unresolved upstream links (retained, not inferred): ``Bytes``.

<a id="op-4adf6fceccabfb1edd3af5e1"></a>
## fmt

`function` · `object_store::GetResult::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetResult", "path": "GetResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1618, 10], "end": [1618, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1618`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14f366726bc8c0d00052d1f4"></a>
## into_stream

`function` · `object_store::GetResult::into_stream` · object_store 0.13.2

```rust
fn into_stream(self) -> BoxStream<'static, Result<Bytes>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::GetResult", "path": "GetResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1654, 1], "end": [1698, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1688`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Converts this into a byte stream

If the `self.kind` is [`GetResultPayload::File`](../operations/object_store.GetResultPayload.md#op-0211051432dee3f6644c79df) will perform chunked reads of the file,
otherwise will return the [`GetResultPayload::Stream`](../operations/object_store.GetResultPayload.md#op-2872e38543a2098c316a9a32).

# Tokio Compatibility

Tokio discourages performing blocking IO on a tokio worker thread, however,
no major operating systems have stable async file APIs. Therefore if called from
a tokio context, this will use [`tokio::runtime::Handle::spawn_blocking`] to dispatch
IO to a blocking thread pool, much like `tokio::fs` does under-the-hood.

If not called from a tokio context, this will perform IO on the current thread with
no additional complexity or overheads

Unresolved upstream links (retained, not inferred): ``tokio::runtime::Handle::spawn_blocking``.

<a id="op-5086593151ad9db0e6f1aec1"></a>
## meta

`struct_field` · `object_store::GetResult::meta` · object_store 0.13.2

```rust
meta: ObjectMeta
```

Source: `src/lib.rs:1623`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518) for this object

<a id="op-767b4724a160f49198c53c5a"></a>
## payload

`struct_field` · `object_store::GetResult::payload` · object_store 0.13.2

```rust
payload: GetResultPayload
```

Source: `src/lib.rs:1621`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The [`GetResultPayload`](../operations/object_store.GetResultPayload.md#op-f0c3bc293967f4345e687d48)

<a id="op-24ca7c249ec8dabe507f1796"></a>
## range

`struct_field` · `object_store::GetResult::range` · object_store 0.13.2

```rust
range: std::ops::Range<u64>
```

Source: `src/lib.rs:1627`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The range of bytes returned by this request

Note this is not `usize` as `object_store` supports 32-bit architectures such as WASM
