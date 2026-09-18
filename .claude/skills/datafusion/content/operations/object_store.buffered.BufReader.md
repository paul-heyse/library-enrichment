# `object_store::buffered::BufReader`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.buffered.BufReader.json).

<a id="op-4d2885c98cc2e56923365692"></a>
## BufReader

`struct` · `object_store::buffered::BufReader` · object_store 0.13.2

```rust
struct BufReader
```

Source: `src/buffered.rs:56`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An async-buffered reader compatible with the tokio IO traits

Internally this maintains a buffer of the requested size, and uses [`ObjectStoreExt::get_range`](../operations/object_store.ObjectStoreExt.md#op-c46044b5384a5c5c106c5137)
to populate its internal buffer once depleted. This buffer is cleared on seek.

Whilst simple, this interface will typically be outperformed by the native [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca)
methods that better map to the network APIs. This is because most object stores have
very [high first-byte latencies], on the order of 100-200ms, and so avoiding unnecessary
round-trips is critical to throughput.

Systems looking to sequentially scan a file should instead consider using [`ObjectStoreExt::get`],
or [`ObjectStore::get_opts`](../operations/object_store.ObjectStore.md#op-0eb8121eee6ac22c92ee9da0), or [`ObjectStoreExt::get_range`](../operations/object_store.ObjectStoreExt.md#op-c46044b5384a5c5c106c5137) to read a particular range.

Systems looking to read multiple ranges of a file should instead consider using
[`ObjectStore::get_ranges`](../operations/object_store.ObjectStore.md#op-89edf38f0aecae998b9e9357), which will optimise the vectored IO.

[high first-byte latencies]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/optimizing-performance.html
[`ObjectStoreExt::get`]: crate::ObjectStoreExt::get

<a id="op-80d386829395c2d9758ec69c"></a>
## consume

`function` · `object_store::buffered::BufReader::consume` · object_store 0.13.2

```rust
fn consume(Pin<&mut self>, amt: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [210, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_buf_read::AsyncBufRead", "path": "AsyncBufRead"}, "trait_path": "tokio::io::async_buf_read::AsyncBufRead"}`

Source: `src/buffered.rs:198`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9abdd1076c393bd742aa87b6"></a>
## fmt

`function` · `object_store::buffered::BufReader::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [79, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/buffered.rs:72`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d6a16580d0bb0b998d9c385"></a>
## new

`function` · `object_store::buffered::BufReader::new` · object_store 0.13.2

```rust
fn new(store: Arc<dyn ObjectStore>, meta: &ObjectMeta) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [135, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:89`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`BufReader`](../operations/object_store.buffered.BufReader.md#op-4d2885c98cc2e56923365692) from the provided [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518) and [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca)

<a id="op-02bed6d9259f174861a2e31e"></a>
## poll_complete

`function` · `object_store::buffered::BufReader::poll_complete` · object_store 0.13.2

```rust
fn poll_complete(Pin<&mut self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<u64>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [169, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_seek::AsyncSeek", "path": "AsyncSeek"}, "trait_path": "tokio::io::async_seek::AsyncSeek"}`

Source: `src/buffered.rs:166`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb1c69544edbcfe6216f3158"></a>
## poll_fill_buf

`function` · `object_store::buffered::BufReader::poll_fill_buf` · object_store 0.13.2

```rust
fn poll_fill_buf(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<std::io::Result<&[u8]>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 1], "end": [210, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_buf_read::AsyncBufRead", "path": "AsyncBufRead"}, "trait_path": "tokio::io::async_buf_read::AsyncBufRead"}`

Source: `src/buffered.rs:193`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a50076edc66209c2c2e80f1a"></a>
## poll_read

`function` · `object_store::buffered::BufReader::poll_read` · object_store 0.13.2

```rust
fn poll_read(Pin<&mut self>, cx: &mut Context<'_>, out: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [190, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_read::AsyncRead", "path": "AsyncRead"}, "trait_path": "tokio::io::async_read::AsyncRead"}`

Source: `src/buffered.rs:172`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f2138e1e07063d3512b9eb8"></a>
## start_seek

`function` · `object_store::buffered::BufReader::start_seek` · object_store 0.13.2

```rust
fn start_seek(Pin<&mut self>, position: SeekFrom) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [169, 2], "filename": "src/buffered.rs"}, "trait": {"args": null, "id": "tokio::io::async_seek::AsyncSeek", "path": "AsyncSeek"}, "trait_path": "tokio::io::async_seek::AsyncSeek"}`

Source: `src/buffered.rs:138`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4904a7632c0f40fa64c89422"></a>
## with_capacity

`function` · `object_store::buffered::BufReader::with_capacity` · object_store 0.13.2

```rust
fn with_capacity(store: Arc<dyn ObjectStore>, meta: &ObjectMeta, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::buffered::BufReader", "path": "BufReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [135, 2], "filename": "src/buffered.rs"}, "trait": null, "trait_path": null}`

Source: `src/buffered.rs:94`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`BufReader`](../operations/object_store.buffered.BufReader.md#op-4d2885c98cc2e56923365692) from the provided [`ObjectMeta`](../operations/object_store.ObjectMeta.md#op-84641755fb7ee613fe92d518), [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca), and `capacity`
